//! Micup.gg 3B Oda Sahnesi (Voice Room Stage) modelleri ve yardımcıları.
//!
//! Micup'ta sesli kanallar 3B oda olarak açılabilir. Botlar odada NPC sunucu karakteri
//! olarak durabilir, TV'de neon/tahta/sade panolar veya YouTube videoları gösterebilir,
//! TV'ye butonlar koyabilir, kazanana spot ışığı tutabilir ve konfeti patlatabilir.
//! Ayrıca LiveKit tabanlı ses ve video/kamera akışları için bağlantı parametreleri
//! ve 3B uzamsal ses (spatial audio) pozisyonları desteklenir.

use crate::error::{Error, Result};
use crate::models::Id;
use serde::{Deserialize, Serialize};

pub const MANAGE_ROOM_STAGE_PERMISSION: u64 = 1 << 62;

// =========================================================================
// 3B Sahne Pozisyon, Rotasyon ve Uzamsal Modeller
// =========================================================================

/// 3B odadaki bir noktanın kartezyen uzaydaki koordinatları (x, y, z).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StagePosition {
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default)]
    pub z: f64,
}

impl StagePosition {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub const fn origin() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn distance_to(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Koordinatların geçerli (NaN veya Sonsuz olmayan) olup olmadığını doğrular.
    pub fn validate(&self) -> Result<()> {
        if self.x.is_nan() || self.y.is_nan() || self.z.is_nan() {
            tracing::warn!("3B sahne pozisyonu NaN koordinat içeriyor");
            return Err(Error::Stage("Pozisyon koordinatları NaN olamaz".into()));
        }
        if self.x.is_infinite() || self.y.is_infinite() || self.z.is_infinite() {
            tracing::warn!("3B sahne pozisyonu sonsuz koordinat içeriyor");
            return Err(Error::Stage("Pozisyon koordinatları sonsuz olamaz".into()));
        }
        Ok(())
    }
}

/// 3B odadaki bir nesnenin açısal yönelimi (Euler açıları: pitch, yaw, roll - derece cinsinden).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StageRotation {
    #[serde(default)]
    pub pitch: f64,
    #[serde(default)]
    pub yaw: f64,
    #[serde(default)]
    pub roll: f64,
}

impl StageRotation {
    pub const fn new(pitch: f64, yaw: f64, roll: f64) -> Self {
        Self { pitch, yaw, roll }
    }

    pub const fn yaw_only(yaw: f64) -> Self {
        Self {
            pitch: 0.0,
            yaw,
            roll: 0.0,
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.pitch.is_nan() || self.yaw.is_nan() || self.roll.is_nan() {
            tracing::warn!("3B sahne rotasyonu NaN açısı içeriyor");
            return Err(Error::Stage("Rotasyon açıları NaN olamaz".into()));
        }
        if self.pitch.is_infinite() || self.yaw.is_infinite() || self.roll.is_infinite() {
            tracing::warn!("3B sahne rotasyonu sonsuz açı içeriyor");
            return Err(Error::Stage("Rotasyon açıları sonsuz olamaz".into()));
        }
        Ok(())
    }
}

/// 3B sahne elemanının uzamsal konum, yönelim ve ölçek bilgisi.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StageTransform {
    #[serde(default)]
    pub position: StagePosition,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotation: Option<StageRotation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
}

impl StageTransform {
    pub fn new(position: StagePosition) -> Self {
        Self {
            position,
            rotation: None,
            scale: None,
        }
    }

    pub fn with_rotation(mut self, rotation: StageRotation) -> Self {
        self.rotation = Some(rotation);
        self
    }

    pub fn with_scale(mut self, scale: f64) -> Self {
        self.scale = Some(scale);
        self
    }

    pub fn validate(&self) -> Result<()> {
        self.position.validate()?;
        if let Some(ref rot) = self.rotation {
            rot.validate()?;
        }
        if let Some(s) = self.scale {
            if s <= 0.0 || s.is_nan() || s.is_infinite() {
                tracing::warn!(scale = s, "Geçersiz sahne transform ölçeği");
                return Err(Error::Stage(
                    "Ölçek değeri pozitif ve sonlu bir sayı olmalıdır".into(),
                ));
            }
        }
        Ok(())
    }
}

/// 3B uzamsal ses (Spatial Audio) yapılandırması.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StageSpatialAudio {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub position: StagePosition,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_distance: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distance_rolloff: Option<f64>,
}

impl StageSpatialAudio {
    pub fn new(position: StagePosition) -> Self {
        Self {
            enabled: true,
            position,
            max_distance: None,
            distance_rolloff: None,
        }
    }

    pub fn max_distance(mut self, dist: f64) -> Self {
        self.max_distance = Some(dist);
        self
    }

    pub fn rolloff(mut self, rolloff: f64) -> Self {
        self.distance_rolloff = Some(rolloff);
        self
    }

    pub fn validate(&self) -> Result<()> {
        self.position.validate()?;
        if let Some(d) = self.max_distance {
            if d <= 0.0 || d.is_nan() || d.is_infinite() {
                return Err(Error::Stage(
                    "Maksimum ses mesafesi sıfırdan büyük olmalıdır".into(),
                ));
            }
        }
        Ok(())
    }
}

/// 3B odadaki katılımcı / bot konumlandırma yuvası.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StageParticipantSlot {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Id>,
    #[serde(default)]
    pub slot_name: String,
    #[serde(default)]
    pub position: StagePosition,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotation: Option<StageRotation>,
}

impl StageParticipantSlot {
    pub fn new(slot_name: impl Into<String>, position: StagePosition) -> Self {
        Self {
            user_id: None,
            slot_name: slot_name.into(),
            position,
            rotation: None,
        }
    }

    pub fn with_user(mut self, user_id: impl Into<Id>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    pub fn with_rotation(mut self, rotation: StageRotation) -> Self {
        self.rotation = Some(rotation);
        self
    }

    pub fn validate(&self) -> Result<()> {
        self.position.validate()?;
        if let Some(ref r) = self.rotation {
            r.validate()?;
        }
        Ok(())
    }
}

// =========================================================================
// LiveKit Ses ve Kamera / Video Yapılandırma Modelleri
// =========================================================================

/// LiveKit video codec formatları.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum VideoCodec {
    #[default]
    #[serde(alias = "h264", alias = "H264")]
    H264,
    #[serde(alias = "vp8", alias = "VP8")]
    Vp8,
    #[serde(alias = "vp9", alias = "VP9")]
    Vp9,
    #[serde(alias = "av1", alias = "AV1")]
    Av1,
}

/// LiveKit ses kanalı / mikrofon yayın parametreleri.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LiveKitAudioConfig {
    #[serde(default = "default_true")]
    pub echo_cancellation: bool,
    #[serde(default = "default_true")]
    pub noise_suppression: bool,
    #[serde(default = "default_true")]
    pub auto_gain_control: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channels: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u32>,
}

fn default_true() -> bool {
    true
}

impl Default for LiveKitAudioConfig {
    fn default() -> Self {
        Self {
            echo_cancellation: true,
            noise_suppression: true,
            auto_gain_control: true,
            sample_rate: Some(48000),
            channels: Some(2),
            bitrate: Some(64000),
        }
    }
}

impl LiveKitAudioConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn speech() -> Self {
        Self {
            echo_cancellation: true,
            noise_suppression: true,
            auto_gain_control: true,
            sample_rate: Some(48000),
            channels: Some(1),
            bitrate: Some(32000),
        }
    }

    pub fn music() -> Self {
        Self {
            echo_cancellation: false,
            noise_suppression: false,
            auto_gain_control: false,
            sample_rate: Some(48000),
            channels: Some(2),
            bitrate: Some(128000),
        }
    }

    pub fn echo_cancellation(mut self, enabled: bool) -> Self {
        self.echo_cancellation = enabled;
        self
    }

    pub fn noise_suppression(mut self, enabled: bool) -> Self {
        self.noise_suppression = enabled;
        self
    }

    pub fn auto_gain_control(mut self, enabled: bool) -> Self {
        self.auto_gain_control = enabled;
        self
    }

    pub fn sample_rate(mut self, rate: u32) -> Self {
        self.sample_rate = Some(rate);
        self
    }

    pub fn channels(mut self, ch: u8) -> Self {
        self.channels = Some(ch);
        self
    }

    pub fn bitrate(mut self, bps: u32) -> Self {
        self.bitrate = Some(bps);
        self
    }

    pub fn validate(&self) -> Result<()> {
        if let Some(rate) = self.sample_rate {
            if rate == 0 || rate > 192000 {
                tracing::warn!(rate, "Geçersiz ses örnekleme hızı (sample rate)");
                return Err(Error::Voice(format!("Geçersiz sample rate: {rate}")));
            }
        }
        if let Some(ch) = self.channels {
            if ch == 0 || ch > 8 {
                tracing::warn!(ch, "Geçersiz ses kanal sayısı");
                return Err(Error::Voice(format!(
                    "Geçersiz kanal sayısı: {ch} (1-8 arası olmalı)"
                )));
            }
        }
        Ok(())
    }
}

/// LiveKit kamera / video yayın parametreleri.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LiveKitCameraConfig {
    #[serde(default)]
    pub width: u32,
    #[serde(default)]
    pub height: u32,
    #[serde(default)]
    pub frame_rate: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u32>,
    #[serde(default)]
    pub codec: VideoCodec,
    #[serde(default)]
    pub simulcast: bool,
}

impl Default for LiveKitCameraConfig {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
            frame_rate: 30,
            bitrate: Some(1_500_000),
            codec: VideoCodec::H264,
            simulcast: false,
        }
    }
}

impl LiveKitCameraConfig {
    pub fn new(width: u32, height: u32, frame_rate: u32) -> Self {
        Self {
            width,
            height,
            frame_rate,
            bitrate: None,
            codec: VideoCodec::default(),
            simulcast: false,
        }
    }

    pub fn hd_720p() -> Self {
        Self {
            width: 1280,
            height: 720,
            frame_rate: 30,
            bitrate: Some(1_500_000),
            codec: VideoCodec::H264,
            simulcast: false,
        }
    }

    pub fn fhd_1080p() -> Self {
        Self {
            width: 1920,
            height: 1080,
            frame_rate: 30,
            bitrate: Some(3_000_000),
            codec: VideoCodec::H264,
            simulcast: false,
        }
    }

    pub fn low_360p() -> Self {
        Self {
            width: 640,
            height: 360,
            frame_rate: 24,
            bitrate: Some(500_000),
            codec: VideoCodec::Vp8,
            simulcast: false,
        }
    }

    pub fn codec(mut self, codec: VideoCodec) -> Self {
        self.codec = codec;
        self
    }

    pub fn bitrate(mut self, bps: u32) -> Self {
        self.bitrate = Some(bps);
        self
    }

    pub fn simulcast(mut self, enabled: bool) -> Self {
        self.simulcast = enabled;
        self
    }

    pub fn validate(&self) -> Result<()> {
        if self.width == 0 || self.height == 0 {
            tracing::warn!(
                width = self.width,
                height = self.height,
                "Kamera çözünürlüğü sıfır olamaz"
            );
            return Err(Error::Voice(
                "Kamera genişlik ve yükseklik değerleri sıfırdan büyük olmalıdır".into(),
            ));
        }
        if self.frame_rate == 0 || self.frame_rate > 120 {
            tracing::warn!(fps = self.frame_rate, "Geçersiz kamera kare hızı (FPS)");
            return Err(Error::Voice(format!(
                "Kamera kare hızı 1 ile 120 arasında olmalıdır, verilen: {}",
                self.frame_rate
            )));
        }
        Ok(())
    }
}

/// LiveKit ses ve video sunucusuna bağlanmak için gereken parametreler.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LiveKitConnectionParams {
    /// LiveKit WebSocket endpoint URL'i (örn. `wss://livekit.micup.gg`).
    #[serde(default)]
    pub endpoint: String,
    /// LiveKit erişim anahtarı / JWT belirteci.
    #[serde(default)]
    pub token: String,
    /// İlgili ses kanalı / oda kimliği.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Id>,
    /// Sunucu (Guild) kimliği.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id>,
    /// Bağlantı oturum kimliği (`connection_id`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    /// Uçtan uca şifreleme anahtarı (E2EE), varsa.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub e2ee_key: Option<String>,
    /// Odaya bağlanırken otomatik ses/video akışlarına abone olma.
    #[serde(default)]
    pub auto_subscribe: bool,
}

impl LiveKitConnectionParams {
    pub fn new(endpoint: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            token: token.into(),
            channel_id: None,
            guild_id: None,
            connection_id: None,
            e2ee_key: None,
            auto_subscribe: true,
        }
    }

    pub fn with_channel(mut self, channel_id: impl Into<Id>) -> Self {
        self.channel_id = Some(channel_id.into());
        self
    }

    pub fn with_guild(mut self, guild_id: impl Into<Id>) -> Self {
        self.guild_id = Some(guild_id.into());
        self
    }

    pub fn with_connection_id(mut self, connection_id: impl Into<String>) -> Self {
        self.connection_id = Some(connection_id.into());
        self
    }

    pub fn with_e2ee(mut self, key: impl Into<String>) -> Self {
        self.e2ee_key = Some(key.into());
        self
    }

    pub fn auto_subscribe(mut self, auto: bool) -> Self {
        self.auto_subscribe = auto;
        self
    }

    /// Bağlantı parametrelerini doğrular.
    pub fn validate(&self) -> Result<()> {
        if self.endpoint.trim().is_empty() {
            tracing::warn!("LiveKit bağlantı endpoint'i boş olamaz");
            return Err(Error::Voice("LiveKit endpoint adresi boş olamaz".into()));
        }
        if !self.endpoint.starts_with("ws://") && !self.endpoint.starts_with("wss://") {
            tracing::warn!(endpoint = %self.endpoint, "Geçersiz LiveKit endpoint protokolü");
            return Err(Error::Voice(format!(
                "LiveKit endpoint ws:// veya wss:// protokolü ile başlamalıdır: {}",
                self.endpoint
            )));
        }
        if self.token.trim().is_empty() {
            tracing::warn!("LiveKit token boş olamaz");
            return Err(Error::Voice("LiveKit token boş olamaz".into()));
        }
        tracing::debug!(endpoint = %self.endpoint, "LiveKit bağlantı parametreleri doğrulandı");
        Ok(())
    }
}

impl From<crate::events::VoiceServerUpdate> for LiveKitConnectionParams {
    fn from(update: crate::events::VoiceServerUpdate) -> Self {
        Self {
            endpoint: update.endpoint,
            token: update.token,
            channel_id: Some(update.channel_id),
            guild_id: update.guild_id,
            connection_id: Some(update.connection_id),
            e2ee_key: update.e2ee_key,
            auto_subscribe: true,
        }
    }
}

impl From<&crate::events::VoiceServerUpdate> for LiveKitConnectionParams {
    fn from(update: &crate::events::VoiceServerUpdate) -> Self {
        Self {
            endpoint: update.endpoint.clone(),
            token: update.token.clone(),
            channel_id: Some(update.channel_id),
            guild_id: update.guild_id,
            connection_id: Some(update.connection_id.clone()),
            e2ee_key: update.e2ee_key.clone(),
            auto_subscribe: true,
        }
    }
}

/// LiveKit sesli ve görüntülü oda oturum yapılandırması.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LiveKitRoomConfig {
    #[serde(default)]
    pub connection: LiveKitConnectionParams,
    #[serde(default)]
    pub audio: LiveKitAudioConfig,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub camera: Option<LiveKitCameraConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spatial_audio: Option<StageSpatialAudio>,
}

impl LiveKitRoomConfig {
    pub fn new(connection: LiveKitConnectionParams) -> Self {
        Self {
            connection,
            audio: LiveKitAudioConfig::default(),
            camera: None,
            spatial_audio: None,
        }
    }

    pub fn with_audio(mut self, audio: LiveKitAudioConfig) -> Self {
        self.audio = audio;
        self
    }

    pub fn with_camera(mut self, camera: LiveKitCameraConfig) -> Self {
        self.camera = Some(camera);
        self
    }

    pub fn with_spatial_audio(mut self, spatial: StageSpatialAudio) -> Self {
        self.spatial_audio = Some(spatial);
        self
    }

    pub fn validate(&self) -> Result<()> {
        self.connection.validate()?;
        self.audio.validate()?;
        if let Some(ref cam) = self.camera {
            cam.validate()?;
        }
        if let Some(ref spatial) = self.spatial_audio {
            spatial.validate()?;
        }
        tracing::debug!("LiveKit oda yapılandırması başarıyla doğrulandı");
        Ok(())
    }
}

// =========================================================================
// TV Panosu ve Sahne TV Yapıları
// =========================================================================

/// TV panosunun görsel teması (Neon, Ahşap/Tahta veya Sade).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum BoardStyle {
    #[default]
    #[serde(alias = "Neon", alias = "NEON")]
    Neon,
    #[serde(alias = "Tahta", alias = "TAHTA")]
    Tahta,
    #[serde(alias = "Sade", alias = "SADE")]
    Sade,
}

/// Skor tablosu satırı.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ScoreboardEntry {
    /// Satırda görünen isim (maksimum 32 karakter).
    pub name: String,
    /// Skor puanı (-1.000.000.000 ile 1.000.000.000 arası).
    pub value: i64,
    /// Kullanıcı odaya bağlıysa avatarı ve adı doğrudan hesabından çekilir.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Id>,
}

impl ScoreboardEntry {
    pub fn new(name: impl Into<String>, value: i64) -> Self {
        Self {
            name: sanitize_stage_line(&name.into(), 32),
            value,
            user_id: None,
        }
    }

    pub fn with_user(mut self, user_id: u64) -> Self {
        self.user_id = Some(Id(user_id));
        self
    }
}

fn default_board_type() -> String {
    "board".to_string()
}

/// TV'de görünen pano yapısı.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageBoard {
    #[serde(default = "default_board_type")]
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<BoardStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent: Option<u32>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub scores: Vec<ScoreboardEntry>,
    /// Botun aynı kanaldaki bir mesajının ID'si. Bu mesajdaki butonlar (en fazla 5) TV'de görünür!
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Id>,
}

impl Default for StageBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl StageBoard {
    /// Yeni bir TV panosu oluşturur (varsayılan Neon tarzı).
    pub fn new() -> Self {
        Self {
            r#type: "board".to_string(),
            style: Some(BoardStyle::Neon),
            title: None,
            text: None,
            accent: None,
            scores: Vec::new(),
            message_id: None,
        }
    }

    /// Neon tarzında yeni bir TV panosu oluşturur.
    pub fn neon() -> Self {
        Self::new().style(BoardStyle::Neon)
    }

    /// Ahşap/tahta tarzında yeni bir TV panosu oluşturur.
    pub fn tahta() -> Self {
        Self::new().style(BoardStyle::Tahta)
    }

    /// Sade/minimal tarzda yeni bir TV panosu oluşturur.
    pub fn sade() -> Self {
        Self::new().style(BoardStyle::Sade)
    }

    pub fn style(mut self, style: BoardStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(sanitize_stage_line(&title.into(), 60));
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(sanitize_stage_block(&text.into(), 280, 4));
        self
    }

    pub fn accent(mut self, rgb: u32) -> Self {
        self.accent = Some(rgb);
        self
    }

    pub fn score(mut self, entry: ScoreboardEntry) -> Self {
        if self.scores.len() < 10 {
            self.scores.push(entry);
        }
        self
    }

    pub fn scores(mut self, entries: impl IntoIterator<Item = ScoreboardEntry>) -> Self {
        for entry in entries {
            if self.scores.len() >= 10 {
                break;
            }
            self.scores.push(entry);
        }
        self
    }

    pub fn message_buttons(mut self, message_id: u64) -> Self {
        self.message_id = Some(Id(message_id));
        self
    }

    /// Pano içeriğini ve kısıtlamalarını doğrular.
    pub fn validate(&self) -> Result<()> {
        if self.scores.len() > 10 {
            tracing::warn!(
                count = self.scores.len(),
                "Skor tablosu satır sınırı aşıldı"
            );
            return Err(Error::Stage(
                "Skor tablosu en fazla 10 satır içerebilir".into(),
            ));
        }
        if let Some(ref t) = self.title {
            if t.len() > 60 {
                return Err(Error::Stage("Pano başlığı 60 karakteri aşamaz".into()));
            }
        }
        if let Some(ref text) = self.text {
            if text.len() > 280 {
                return Err(Error::Stage("Pano metni 280 karakteri aşamaz".into()));
            }
        }
        Ok(())
    }
}

fn default_video_type() -> String {
    "video".to_string()
}

/// TV'de oynatılan YouTube videosu.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageVideo {
    #[serde(default = "default_video_type")]
    pub r#type: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
}

impl StageVideo {
    pub fn new(youtube_url: impl Into<String>) -> Self {
        Self {
            r#type: "video".to_string(),
            url: youtube_url.into(),
            position_ms: Some(0),
            paused: Some(false),
        }
    }

    pub fn url(mut self, youtube_url: impl Into<String>) -> Self {
        self.url = youtube_url.into();
        self
    }

    pub fn position(mut self, ms: u64) -> Self {
        self.position_ms = Some(ms);
        self
    }

    pub fn paused(mut self, paused: bool) -> Self {
        self.paused = Some(paused);
        self
    }

    pub fn validate(&self) -> Result<()> {
        if self.url.trim().is_empty() {
            tracing::warn!("Video URL'i boş olamaz");
            return Err(Error::Stage("Video URL'i boş olamaz".into()));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StageTv {
    Video(StageVideo),
    Board(StageBoard),
}

impl StageTv {
    pub fn validate(&self) -> Result<()> {
        match self {
            StageTv::Board(b) => b.validate(),
            StageTv::Video(v) => v.validate(),
        }
    }
}

/// Odada TV yanında duran sunucu NPC karakteri.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StageHost {
    /// Hazır görünüm anahtarı (örn: `ithp-kulaklik`, `ithp-gunluk`, `ithp-ceketli`, `ithp-gozluk`, `ithp-rahat`, `ithp-sapkali`, `ithp-sehirli`, `ithp-yazlik`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub character: Option<String>,
    /// Gömlek rengi (RGB tamsayı).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shirt: Option<u32>,
    /// 3B sahnedeki konumu.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<StagePosition>,
    /// 3B sahnedeki yönelimi / açısı.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotation: Option<StageRotation>,
    /// NPC'nin yerleştiği slot adı (örn: "center", "podium", "tv_left").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
}

impl StageHost {
    pub fn new(character_preset: impl Into<String>) -> Self {
        Self {
            character: Some(character_preset.into()),
            shirt: None,
            position: None,
            rotation: None,
            slot: None,
        }
    }

    pub fn shirt_color(mut self, rgb: u32) -> Self {
        self.shirt = Some(rgb);
        self
    }

    pub fn position(mut self, pos: StagePosition) -> Self {
        self.position = Some(pos);
        self
    }

    pub fn rotation(mut self, rot: StageRotation) -> Self {
        self.rotation = Some(rot);
        self
    }

    pub fn slot(mut self, slot_name: impl Into<String>) -> Self {
        self.slot = Some(slot_name.into());
        self
    }

    pub fn validate(&self) -> Result<()> {
        if let Some(ref p) = self.position {
            p.validate()?;
        }
        if let Some(ref r) = self.rotation {
            r.validate()?;
        }
        Ok(())
    }
}

/// Sahneyi kurmak veya güncellemek için kullanılan istek gövdesi.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SetVoiceRoomStage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tv: Option<StageTv>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<StageHost>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl_seconds: Option<u64>,
    /// 3B sahnedeki katılımcı yuvaları veya özel konumlandırmalar.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub slots: Vec<StageParticipantSlot>,
}

impl SetVoiceRoomStage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn board(mut self, board: StageBoard) -> Self {
        self.tv = Some(StageTv::Board(board));
        self
    }

    pub fn video(mut self, video: StageVideo) -> Self {
        self.tv = Some(StageTv::Video(video));
        self
    }

    pub fn host(mut self, host: StageHost) -> Self {
        self.host = Some(host);
        self
    }

    pub fn ttl(mut self, seconds: u64) -> Self {
        self.ttl_seconds = Some(seconds.clamp(60, 21600));
        self
    }

    pub fn slot(mut self, slot: StageParticipantSlot) -> Self {
        self.slots.push(slot);
        self
    }

    pub fn slots(mut self, slots: impl IntoIterator<Item = StageParticipantSlot>) -> Self {
        self.slots.extend(slots);
        self
    }

    /// Sahne yapılandırmasının platform kurallarına uygunluğunu doğrular.
    pub fn validate(&self) -> Result<()> {
        if let Some(ttl) = self.ttl_seconds {
            if !(60..=21600).contains(&ttl) {
                tracing::warn!(ttl, "Geçersiz sahne TTL süresi");
                return Err(Error::Stage(format!(
                    "TTL süresi 60 ile 21600 saniye arasında olmalıdır, verilen: {ttl}"
                )));
            }
        }
        if let Some(ref tv) = self.tv {
            tv.validate()?;
        }
        if let Some(ref host) = self.host {
            host.validate()?;
        }
        for slot in &self.slots {
            slot.validate()?;
        }
        tracing::debug!("SetVoiceRoomStage başarıyla doğrulandı");
        Ok(())
    }
}

/// O anki aktif sahnenin detayları.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VoiceRoomStage {
    #[serde(default)]
    pub v: u64,
    #[serde(default)]
    pub bot: StageBotAccount,
    #[serde(default)]
    pub written_at: u64,
    #[serde(default)]
    pub expires_at: u64,
    #[serde(default)]
    pub tv: Option<serde_json::Value>,
    #[serde(default)]
    pub host: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub slots: Vec<StageParticipantSlot>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct StageBotAccount {
    #[serde(default)]
    pub id: Id,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub avatar: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VoiceRoomStageResponse {
    #[serde(default)]
    pub stage: Option<VoiceRoomStage>,
}

/// Tek seferlik sahne anları (efektler).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum VoiceRoomEffect {
    #[serde(rename = "confetti")]
    Confetti,
    #[serde(rename = "spotlight")]
    Spotlight { user_id: Id },
    #[serde(rename = "host_emote", alias = "npc_move", alias = "npc_motion")]
    HostEmote {
        emote: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        emoji: Option<String>,
    },
}

impl VoiceRoomEffect {
    pub fn confetti() -> Self {
        Self::Confetti
    }

    pub fn spotlight(user_id: u64) -> Self {
        Self::Spotlight {
            user_id: Id(user_id),
        }
    }

    pub fn wave() -> Self {
        Self::HostEmote {
            emote: "wave".to_string(),
            emoji: None,
        }
    }

    pub fn dance() -> Self {
        Self::HostEmote {
            emote: "dance".to_string(),
            emoji: None,
        }
    }

    pub fn jump() -> Self {
        Self::HostEmote {
            emote: "jump".to_string(),
            emoji: None,
        }
    }

    pub fn emoji(unicode_emoji: impl Into<String>) -> Self {
        Self::HostEmote {
            emote: "emoji".to_string(),
            emoji: Some(unicode_emoji.into()),
        }
    }

    pub fn host_emote(emote: impl Into<String>) -> Self {
        Self::HostEmote {
            emote: emote.into(),
            emoji: None,
        }
    }

    pub fn npc_move(action: impl Into<String>) -> Self {
        Self::host_emote(action)
    }
}

// =========================================================================
// Metin Doğrulama ve Sanitizasyon
// =========================================================================

fn is_hidden_char(c: char) -> bool {
    let u = c as u32;
    (0x7f..=0x9f).contains(&u)
        || u == 0x061c
        || u == 0x200b
        || u == 0x200e
        || u == 0x200f
        || (0x2028..=0x202e).contains(&u)
        || u == 0x2060
        || (0x2066..=0x2069).contains(&u)
        || u == 0xfeff
}

/// Sahne tek satırlık metin kontrolü ve temizliği.
pub fn sanitize_stage_line(text: &str, max_len: usize) -> String {
    let clean: String = text
        .chars()
        .filter(|&c| c >= ' ' && !is_hidden_char(c))
        .collect();
    let trimmed = clean.trim();
    trimmed.chars().take(max_len).collect()
}

/// Sahne çok satırlı metin kontrolü ve temizliği (en fazla `max_lines` satır).
pub fn sanitize_stage_block(text: &str, max_len: usize, max_lines: usize) -> String {
    let clean: String = text
        .chars()
        .filter(|&c| (c >= ' ' || c == '\n') && !is_hidden_char(c))
        .collect();
    let lines: Vec<&str> = clean.lines().take(max_lines).collect();
    let combined = lines.join("\n");
    combined.chars().take(max_len).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_style_variants_and_serialization() {
        assert_eq!(
            serde_json::to_string(&BoardStyle::Neon).unwrap(),
            "\"neon\""
        );
        assert_eq!(
            serde_json::to_string(&BoardStyle::Tahta).unwrap(),
            "\"tahta\""
        );
        assert_eq!(
            serde_json::to_string(&BoardStyle::Sade).unwrap(),
            "\"sade\""
        );

        let neon: BoardStyle = serde_json::from_str("\"neon\"").unwrap();
        assert_eq!(neon, BoardStyle::Neon);
        let tahta: BoardStyle = serde_json::from_str("\"tahta\"").unwrap();
        assert_eq!(tahta, BoardStyle::Tahta);
        let sade: BoardStyle = serde_json::from_str("\"sade\"").unwrap();
        assert_eq!(sade, BoardStyle::Sade);

        // Büyük/küçük harf toleransı
        let neon_upper: BoardStyle = serde_json::from_str("\"Neon\"").unwrap();
        assert_eq!(neon_upper, BoardStyle::Neon);
    }

    #[test]
    fn test_stage_board_constructors_and_serialization() {
        let board_neon = StageBoard::neon().title("Neon Sahne").accent(0x00FF88);
        assert_eq!(board_neon.style, Some(BoardStyle::Neon));
        let val_neon = serde_json::to_value(&board_neon).unwrap();
        assert_eq!(val_neon["type"], "board");
        assert_eq!(val_neon["style"], "neon");
        assert_eq!(val_neon["title"], "Neon Sahne");
        assert_eq!(val_neon["accent"], 0x00FF88);

        let board_tahta = StageBoard::tahta().title("Ahşap Sahne");
        assert_eq!(board_tahta.style, Some(BoardStyle::Tahta));
        let val_tahta = serde_json::to_value(&board_tahta).unwrap();
        assert_eq!(val_tahta["style"], "tahta");

        let board_sade = StageBoard::sade().title("Minimal Sahne");
        assert_eq!(board_sade.style, Some(BoardStyle::Sade));
        let val_sade = serde_json::to_value(&board_sade).unwrap();
        assert_eq!(val_sade["style"], "sade");
    }

    #[test]
    fn test_stage_video_serialization() {
        let video = StageVideo::new("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
            .position(1500)
            .paused(true);
        let val = serde_json::to_value(&video).unwrap();
        assert_eq!(val["type"], "video");
        assert_eq!(val["url"], "https://www.youtube.com/watch?v=dQw4w9WgXcQ");
        assert_eq!(val["position_ms"], 1500);
        assert_eq!(val["paused"], true);

        let des: StageVideo = serde_json::from_value(val).unwrap();
        assert_eq!(des.url, video.url);
        assert_eq!(des.position_ms, Some(1500));
        assert_eq!(des.paused, Some(true));
    }

    #[test]
    fn test_voice_room_effects_serialization() {
        // Confetti
        let confetti = VoiceRoomEffect::confetti();
        let val_confetti = serde_json::to_value(&confetti).unwrap();
        assert_eq!(val_confetti["type"], "confetti");
        let des_confetti: VoiceRoomEffect = serde_json::from_value(val_confetti).unwrap();
        assert_eq!(des_confetti, confetti);

        // Spotlight
        let spotlight = VoiceRoomEffect::spotlight(987654321);
        let val_spotlight = serde_json::to_value(&spotlight).unwrap();
        assert_eq!(val_spotlight["type"], "spotlight");
        assert_eq!(val_spotlight["user_id"], "987654321");
        let des_spotlight: VoiceRoomEffect = serde_json::from_value(val_spotlight).unwrap();
        assert_eq!(des_spotlight, spotlight);

        // NPC hareketleri (dance, wave, jump, emoji, npc_move)
        let dance = VoiceRoomEffect::dance();
        let val_dance = serde_json::to_value(&dance).unwrap();
        assert_eq!(val_dance["type"], "host_emote");
        assert_eq!(val_dance["emote"], "dance");

        let wave = VoiceRoomEffect::wave();
        let val_wave = serde_json::to_value(&wave).unwrap();
        assert_eq!(val_wave["emote"], "wave");

        let jump = VoiceRoomEffect::jump();
        let val_jump = serde_json::to_value(&jump).unwrap();
        assert_eq!(val_jump["emote"], "jump");

        let emoji = VoiceRoomEffect::emoji("🎉");
        let val_emoji = serde_json::to_value(&emoji).unwrap();
        assert_eq!(val_emoji["emote"], "emoji");
        assert_eq!(val_emoji["emoji"], "🎉");

        let npc_move = VoiceRoomEffect::npc_move("spin");
        let val_move = serde_json::to_value(&npc_move).unwrap();
        assert_eq!(val_move["type"], "host_emote");
        assert_eq!(val_move["emote"], "spin");
    }

    #[test]
    fn test_stage_tv_polymorphism() {
        let board = StageBoard::neon().title("Test Board");
        let tv_board = StageTv::Board(board);
        let val_b = serde_json::to_value(&tv_board).unwrap();
        assert_eq!(val_b["type"], "board");

        let video = StageVideo::new("https://youtu.be/xyz");
        let tv_video = StageTv::Video(video);
        let val_v = serde_json::to_value(&tv_video).unwrap();
        assert_eq!(val_v["type"], "video");
        assert_eq!(val_v["url"], "https://youtu.be/xyz");

        let des_v: StageTv = serde_json::from_value(val_v).unwrap();
        match des_v {
            StageTv::Video(v) => assert_eq!(v.url, "https://youtu.be/xyz"),
            _ => panic!("Beklenen StageTv::Video gelmedi"),
        }
    }

    #[test]
    fn test_sanitization_helpers() {
        let dirty = "Merhaba\u{200B}\u{FEFF} Dünya!";
        let clean = sanitize_stage_line(dirty, 20);
        assert_eq!(clean, "Merhaba Dünya!");

        let multiline = "1\n2\n3\n4\n5\n6";
        let clean_block = sanitize_stage_block(multiline, 100, 3);
        assert_eq!(clean_block, "1\n2\n3");
    }

    #[test]
    fn test_3d_stage_positions_and_transforms() {
        let pos1 = StagePosition::new(1.0, 2.0, 3.0);
        let pos2 = StagePosition::new(4.0, 6.0, 3.0);
        assert_eq!(pos1.distance_to(&pos2), 5.0);

        let rot = StageRotation::new(0.0, 90.0, 0.0);
        let transform = StageTransform::new(pos1).with_rotation(rot).with_scale(1.5);
        assert!(transform.validate().is_ok());

        let invalid_pos = StagePosition::new(f64::NAN, 0.0, 0.0);
        assert!(invalid_pos.validate().is_err());

        let json = serde_json::to_value(&pos1).unwrap();
        assert_eq!(json["x"], 1.0);
        assert_eq!(json["y"], 2.0);
        assert_eq!(json["z"], 3.0);
    }

    #[test]
    fn test_livekit_connection_and_config() {
        let conn = LiveKitConnectionParams::new("wss://livekit.micup.gg", "jwt_token_sample")
            .with_channel(12345u64)
            .with_guild(67890u64)
            .with_e2ee("e2ee_secret");
        assert!(conn.validate().is_ok());

        let invalid_conn = LiveKitConnectionParams::new("http://invalid.url", "token");
        assert!(invalid_conn.validate().is_err());

        let empty_token = LiveKitConnectionParams::new("wss://livekit.micup.gg", "   ");
        assert!(empty_token.validate().is_err());

        let audio = LiveKitAudioConfig::speech();
        assert!(audio.validate().is_ok());
        assert_eq!(audio.channels, Some(1));

        let camera = LiveKitCameraConfig::hd_720p();
        assert!(camera.validate().is_ok());
        assert_eq!(camera.width, 1280);
        assert_eq!(camera.height, 720);

        let room_cfg = LiveKitRoomConfig::new(conn)
            .with_audio(audio)
            .with_camera(camera);
        assert!(room_cfg.validate().is_ok());
    }

    #[test]
    fn test_set_voice_room_stage_validation_and_slots() {
        let pos = StagePosition::new(0.0, 1.0, -2.0);
        let host = StageHost::new("ithp-kulaklik")
            .shirt_color(0xFF5500)
            .position(pos)
            .slot("podium");
        assert!(host.validate().is_ok());

        let slot = StageParticipantSlot::new("player_1", StagePosition::new(1.0, 0.0, 0.0))
            .with_user(111222u64);
        assert!(slot.validate().is_ok());

        let stage = SetVoiceRoomStage::new()
            .board(StageBoard::neon().title("Turnuva"))
            .host(host)
            .slot(slot)
            .ttl(1800);
        assert!(stage.validate().is_ok());

        let invalid_ttl = SetVoiceRoomStage::new().ttl(10);
        // ttl clamp uygulandığından 60 olur
        assert_eq!(invalid_ttl.ttl_seconds, Some(60));
    }
}
