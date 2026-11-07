//! Micup.gg 3B Oda Sahnesi (Voice Room Stage) modelleri ve yardımcıları.
//!
//! Micup'ta sesli kanallar 3B oda olarak açılabilir. Botlar odada NPC sunucu karakteri
//! olarak durabilir, TV'de neon/tahta/sade panolar veya YouTube videoları gösterebilir,
//! TV'ye butonlar koyabilir, kazanana spot ışığı tutabilir ve konfeti patlatabilir.

use crate::models::Id;
use serde::{Deserialize, Serialize};

pub const MANAGE_ROOM_STAGE_PERMISSION: u64 = 1 << 62;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BoardStyle {
    Neon,
    Tahta,
    Sade,
}

impl Default for BoardStyle {
    fn default() -> Self {
        Self::Neon
    }
}

/// Skor tablosu satırı.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
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

/// TV'de görünen pano yapısı.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct StageBoard {
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

impl StageBoard {
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

    pub fn message_buttons(mut self, message_id: u64) -> Self {
        self.message_id = Some(Id(message_id));
        self
    }
}

/// TV'de oynatılan YouTube videosu.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StageVideo {
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

    pub fn position(mut self, ms: u64) -> Self {
        self.position_ms = Some(ms);
        self
    }

    pub fn paused(mut self, paused: bool) -> Self {
        self.paused = Some(paused);
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StageTv {
    Board(StageBoard),
    Video(StageVideo),
}

/// Odada TV yanında duran sunucu NPC karakteri.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct StageHost {
    /// Hazır görünüm anahtarı (örn: `ithp-kulaklik`, `ithp-gunluk`, `ithp-ceketli`, `ithp-gozluk`, `ithp-rahat`, `ithp-sapkali`, `ithp-sehirli`, `ithp-yazlik`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character: Option<String>,
    /// Gömlek rengi (RGB tamsayı).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shirt: Option<u32>,
}

impl StageHost {
    pub fn new(character_preset: impl Into<String>) -> Self {
        Self {
            character: Some(character_preset.into()),
            shirt: None,
        }
    }

    pub fn shirt_color(mut self, rgb: u32) -> Self {
        self.shirt = Some(rgb);
        self
    }
}

/// Sahneyi kurmak veya güncellemek için kullanılan istek gövdesi.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SetVoiceRoomStage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tv: Option<StageTv>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<StageHost>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_seconds: Option<u64>,
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
}

/// O anki aktif sahnenin detayları.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VoiceRoomStage {
    pub v: u64,
    pub bot: StageBotAccount,
    pub written_at: u64,
    pub expires_at: u64,
    #[serde(default)]
    pub tv: Option<serde_json::Value>,
    #[serde(default)]
    pub host: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StageBotAccount {
    pub id: Id,
    pub name: String,
    pub avatar: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VoiceRoomStageResponse {
    pub stage: Option<VoiceRoomStage>,
}

/// Tek seferlik sahne anları (efektler).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum VoiceRoomEffect {
    #[serde(rename = "confetti")]
    Confetti,
    #[serde(rename = "spotlight")]
    Spotlight { user_id: Id },
    #[serde(rename = "host_emote")]
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
