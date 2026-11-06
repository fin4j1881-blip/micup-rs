//! Micup Gateway (WebSocket) bağlantı yöneticisi ve olay döngüsü.

use std::time::Duration;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
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

pub struct Shard {
    token: String,
    intents: u64,
}

impl Shard {
    pub fn new(token: impl Into<String>, intents: u64) -> Self {
        Self {
            token: token.into(),
            intents,
        }
    }

    /// Gateway'e bağlanır, Identify gönderir, kalp atışlarını (Heartbeat) yönetir
    /// ve gelen olayları callback üzerinden dinleyiciye iletir.
    pub async fn run<F, Fut>(&self, mut on_event: F) -> Result<(), MicupError>
    where
        F: FnMut(String, Value) -> Fut,
        Fut: std::future::Future<Output = ()>,
    {
        let url = format!("{}/?v=10&encoding=json", GATEWAY_URL);
        info!(url = %url, "Micup Gateway bağlantısı kuruluyor...");

        let (ws_stream, _) = connect_async(&url)
            .await
            .map_err(|e| MicupError::Gateway(e.to_string()))?;

        info!("Micup Gateway TCP & TLS el sıkışması başarılı");
        let (mut write, mut read) = ws_stream.split();

        let mut heartbeat_interval = Duration::from_millis(41250);
        let mut last_seq: Option<u64> = None;

        // İlk Hello paketini bekle
        if let Some(Ok(Message::Text(text))) = read.next().await {
            if let Ok(payload) = serde_json::from_str::<GatewayPayload>(&text) {
                if payload.op == 10 {
                    if let Some(d) = payload.d {
                        if let Some(interval) = d.get("heartbeat_interval").and_then(|v| v.as_u64()) {
                            heartbeat_interval = Duration::from_millis(interval);
                            debug!(interval_ms = interval, "Hello alındı, kalp atışı süresi ayarlandı");
                        }
                    }
                }
            }
        }

        // Identify gönder
        let identify = serde_json::json!({
            "op": 2,
            "d": {
                "token": self.token,
                "intents": self.intents,
                "properties": {
                    "os": std::env::consts::OS,
                    "browser": "micup-rs",
                    "device": "micup-rs"
                }
            }
        });
        write
            .send(Message::Text(identify.to_string()))
            .await
            .map_err(|e| MicupError::Gateway(e.to_string()))?;

        info!("Identify gönderildi, olay döngüsü başlatılıyor");

        let mut ticker = tokio::time::interval(heartbeat_interval);

        loop {
            tokio::select! {
                _ = ticker.tick() => {
                    let heartbeat = serde_json::json!({
                        "op": 1,
                        "d": last_seq
                    });
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
                                            on_event(event_name, data).await;
                                        }
                                    }
                                    1 => {
                                        // Anlık Heartbeat talebi
                                        let hb = serde_json::json!({ "op": 1, "d": last_seq });
                                        let _ = write.send(Message::Text(hb.to_string())).await;
                                    }
                                    7 | 9 => {
                                        warn!(op = p.op, "Yeniden bağlanma talep edildi");
                                        break;
                                    }
                                    11 => {
                                        debug!("Heartbeat ACK alındı");
                                    }
                                    _ => {}
                                }
                            }
                        }
                        Some(Ok(Message::Close(frame))) => {
                            info!(frame = ?frame, "Gateway bağlantısı kapandı");
                            break;
                        }
                        Some(Err(e)) => {
                            error!(error = %e, "Gateway okuma hatası");
                            break;
                        }
                        None => break,
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }
}
