//! Micup Gateway (WebSocket) bağlantı yöneticisi ve olay döngüsü.
//!
//! Micup / Fluxer WebSocket ağ geçidine bağlanır, Hello/Identify/Resume akışını yönetir,
//! zombi bağlantı kontrolüyle (Heartbeat ACK) kalp atışlarını sürdürür ve gelen olayları dinleyiciye aktarır.

use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, error, info, warn};

use crate::error::MicupError;
use crate::models::constants::GATEWAY_URL;

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayPayload {
    pub op: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GatewayPresence {
    pub status: String,
    pub custom_status: Option<String>,
}

impl Default for GatewayPresence {
    fn default() -> Self {
        Self {
            status: "online".to_string(),
            custom_status: None,
        }
    }
}

pub struct Shard {
    token: String,
    intents: u64,
    presence: GatewayPresence,
}

impl Shard {
    pub fn new(token: impl Into<String>, intents: u64) -> Self {
        Self {
            token: token.into(),
            intents,
            presence: GatewayPresence {
                status: "online".to_string(),
                custom_status: None,
            },
        }
    }

    /// Botun durumunu (online, idle, dnd) ve özel durum metnini ayarlar.
    pub fn with_presence(
        mut self,
        status: impl Into<String>,
        custom_status: Option<impl Into<String>>,
    ) -> Self {
        self.presence = GatewayPresence {
            status: status.into(),
            custom_status: custom_status.map(Into::into),
        };
        self
    }

    /// Gateway'e bağlanır, Identify/Resume gönderir, kalp atışlarını yönetir,
    /// bağlantı koparsa otomatik olarak Resume ile yeniden bağlanır.
    pub async fn run<F, Fut>(&self, mut on_event: F) -> Result<(), MicupError>
    where
        F: FnMut(String, Value) -> Fut,
        Fut: std::future::Future<Output = ()>,
    {
        let mut session_id: Option<String> = None;
        let mut resume_gateway_url: Option<String> = None;
        let mut last_seq: Option<u64> = None;

        loop {
            let base_url = resume_gateway_url.as_deref().unwrap_or(GATEWAY_URL);
            let url = format!("{}/?v=10&encoding=json", base_url);
            info!(url = %url, "Micup Gateway bağlantısı kuruluyor...");

            let ws_stream = match connect_async(&url).await {
                Ok((stream, _)) => {
                    info!("Micup Gateway TCP & TLS el sıkışması başarılı");
                    stream
                }
                Err(e) => {
                    error!(error = %e, "Gateway bağlantı hatası, 3 saniye sonra yeniden denenecek");
                    tokio::time::sleep(Duration::from_secs(3)).await;
                    continue;
                }
            };

            let (mut write, mut read) = ws_stream.split();
            let mut heartbeat_interval = Duration::from_millis(41250);
            let mut heartbeat_acked = true;

            // İlk Hello paketini bekle
            if let Some(Ok(Message::Text(text))) = read.next().await {
                if let Ok(payload) = serde_json::from_str::<GatewayPayload>(&text) {
                    if payload.op == 10 {
                        if let Some(d) = payload.d {
                            if let Some(interval) =
                                d.get("heartbeat_interval").and_then(|v| v.as_u64())
                            {
                                heartbeat_interval = Duration::from_millis(interval);
                                debug!(
                                    interval_ms = interval,
                                    "Hello alındı, kalp atışı süresi ayarlandı"
                                );
                            }
                        }
                    }
                }
            }

            // Resume veya Identify gönder
            if let (Some(sid), Some(seq)) = (session_id.as_deref(), last_seq) {
                info!(session_id = %sid, seq = seq, "Resume paketi gönderiliyor...");
                let resume = serde_json::json!({
                    "op": 6,
                    "d": {
                        "token": self.token,
                        "session_id": sid,
                        "seq": seq
                    }
                });
                if let Err(e) = write.send(Message::Text(resume.to_string())).await {
                    warn!(error = %e, "Resume gönderilemedi, yeniden bağlanılacak");
                    continue;
                }
            } else {
                info!("Identify gönderiliyor...");
                let mut activities = serde_json::json!([]);
                if let Some(custom) = &self.presence.custom_status {
                    activities = serde_json::json!([{
                        "name": "Custom Status",
                        "type": 4,
                        "state": custom
                    }]);
                }

                let identify = serde_json::json!({
                    "op": 2,
                    "d": {
                        "token": self.token,
                        "intents": self.intents,
                        "properties": {
                            "os": std::env::consts::OS,
                            "browser": "micup-rs",
                            "device": "micup-rs"
                        },
                        "presence": {
                            "status": self.presence.status,
                            "since": null,
                            "activities": activities,
                            "afk": false
                        }
                    }
                });
                if let Err(e) = write.send(Message::Text(identify.to_string())).await {
                    warn!(error = %e, "Identify gönderilemedi, yeniden bağlanılacak");
                    continue;
                }
            }

            let mut ticker = tokio::time::interval(heartbeat_interval);
            // İlk tick anında tetiklenir, bir kez atla
            ticker.tick().await;

            loop {
                tokio::select! {
                    _ = ticker.tick() => {
                        if !heartbeat_acked {
                            warn!("Son kalp atışına ACK alınamadı (zombi bağlantı tespit edildi), bağlantı yenileniyor");
                            break;
                        }
                        let heartbeat = serde_json::json!({
                            "op": 1,
                            "d": last_seq
                        });
                        heartbeat_acked = false;
                        if let Err(e) = write.send(Message::Text(heartbeat.to_string())).await {
                            warn!(error = %e, "Kalp atışı gönderilemedi");
                            break;
                        }
                    }
                    msg = read.next() => {
                        match msg {
                            Some(Ok(Message::Text(text))) => {
                                if let Ok(p) = serde_json::from_str::<GatewayPayload>(&text) {
                                    if let Some(s) = p.s {
                                        last_seq = Some(s);
                                    }
                                    match p.op {
                                        0 => {
                                            if let (Some(event_name), Some(data)) = (p.t, p.d) {
                                                if event_name == "READY" {
                                                    if let Some(sid) = data.get("session_id").and_then(|v| v.as_str()) {
                                                        session_id = Some(sid.to_string());
                                                        debug!(session_id = %sid, "Oturum kimliği kaydedildi");
                                                    }
                                                    if let Some(url) = data.get("resume_gateway_url").and_then(|v| v.as_str()) {
                                                        resume_gateway_url = Some(url.to_string());
                                                    }
                                                }
                                                on_event(event_name, data).await;
                                            }
                                        }
                                        1 => {
                                            // Ağ geçidinin talebi üzerine anlık Heartbeat yanıtı
                                            let hb = serde_json::json!({ "op": 1, "d": last_seq });
                                            let _ = write.send(Message::Text(hb.to_string())).await;
                                        }
                                        7 => {
                                            warn!("Opcode 7: Yeniden bağlanma (Reconnect) talep edildi");
                                            break;
                                        }
                                        9 => {
                                            let resumable = p.d.and_then(|v| v.as_bool()).unwrap_or(false);
                                            warn!(resumable = resumable, "Opcode 9: Oturum geçersiz (Invalid Session)");
                                            if !resumable {
                                                session_id = None;
                                                last_seq = None;
                                            }
                                            break;
                                        }
                                        11 => {
                                            heartbeat_acked = true;
                                            debug!("Heartbeat ACK alındı");
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            Some(Ok(Message::Close(frame))) => {
                                info!(frame = ?frame, "Gateway bağlantısı kapandı, yeniden bağlanılacak");
                                break;
                            }
                            Some(Err(e)) => {
                                error!(error = %e, "Gateway okuma hatası, yeniden bağlanılacak");
                                break;
                            }
                            None => {
                                warn!("Gateway akışı sonlandı, yeniden bağlanılacak");
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}
