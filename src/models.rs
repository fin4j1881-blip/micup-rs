pub mod constants {
    pub const API_BASE: &str = "https://micup.gg/api/v10";
    pub const GATEWAY_URL: &str = "wss://micup.gg/gateway";
    pub const CDN_BASE: &str = "https://media.micup.gg";
}

pub mod extra;
pub use extra::{
    Attachment, AuditLog, AuditLogEntry, Ban, Emoji, Invite, Message, Reaction, Webhook,
};

use serde::{Deserialize, Serialize};
use std::fmt;

/// Micup / Fluxer ağ geçidi bot oturum bilgisi (`GET /gateway/bot`).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GatewayBotInfo {
    pub url: String,
    pub shards: u32,
    pub session_start_limit: SessionStartLimit,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionStartLimit {
    pub total: u32,
    pub remaining: u32,
    pub reset_after: u64,
    pub max_concurrency: u32,
}

/// Micup platform keşif belgesi (`GET /.well-known/fluxer`).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FluxerDiscovery {
    #[serde(default)]
    pub api_code_version: Option<u64>,
    pub endpoints: DiscoveryEndpoints,
    #[serde(default)]
    pub features: Option<serde_json::Value>,
    #[serde(default)]
    pub limits: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiscoveryEndpoints {
    pub api: String,
    pub api_public: String,
    pub gateway: String,
    pub media: String,
    #[serde(default)]
    pub upload_relay: Option<String>,
    #[serde(default)]
    pub static_cdn: Option<String>,
    #[serde(default)]
    pub webapp: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct Id(pub u64);

impl Id {
    pub const fn new(id: u64) -> Self {
        Self(id)
    }
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for Id {
    fn from(v: u64) -> Self {
        Self(v)
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct IdVisitor;

        impl<'de> serde::de::Visitor<'de> for IdVisitor {
            type Value = Id;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("bir u64 tamsayısı veya string formatında ID")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
                Ok(Id(v))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                u64::try_from(v).map(Id).map_err(serde::de::Error::custom)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.parse::<u64>().map(Id).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(IdVisitor)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct User {
    pub id: Id,
    pub username: String,
    #[serde(default)]
    pub discriminator: Option<String>,
    #[serde(default)]
    pub global_name: Option<String>,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default)]
    pub bot: Option<bool>,
    #[serde(default)]
    pub banner: Option<String>,
    #[serde(default)]
    pub accent_color: Option<u64>,
    #[serde(default)]
    pub flags: Option<i64>,
    #[serde(default)]
    pub public_flags: Option<i64>,
}

impl User {
    pub fn is_bot(&self) -> bool {
        self.bot.unwrap_or(false)
    }

    pub fn avatar_url(&self) -> String {
        crate::cdn::avatar_url(self.id.get(), self.avatar.as_deref(), 0)
    }

    pub fn banner_url(&self) -> Option<String> {
        self.banner
            .as_deref()
            .map(|b| crate::cdn::banner_url(self.id.get(), b))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Member {
    #[serde(default)]
    pub user: Option<User>,
    #[serde(default)]
    pub nick: Option<String>,
    #[serde(default)]
    pub roles: Vec<Id>,
    #[serde(default)]
    pub joined_at: Option<String>,
    #[serde(default)]
    pub communication_disabled_until: Option<String>,
    #[serde(default)]
    pub banner: Option<String>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Role {
    pub id: Id,
    pub name: String,
    #[serde(default)]
    pub color: u64,
    #[serde(default)]
    pub hoist: bool,
    #[serde(default)]
    pub position: i64,
    #[serde(default)]
    pub permissions: String,
    #[serde(default)]
    pub managed: bool,
    #[serde(default)]
    pub mentionable: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Channel {
    pub id: Id,
    #[serde(rename = "type")]
    pub kind: u64,
    #[serde(default)]
    pub guild_id: Option<Id>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub topic: Option<String>,
    #[serde(default)]
    pub parent_id: Option<Id>,
    #[serde(default)]
    pub position: Option<i64>,
}

impl Channel {
    pub fn is_text(&self) -> bool {
        self.kind == 0
    }
    pub fn is_voice(&self) -> bool {
        self.kind == 2
    }
    pub fn is_category(&self) -> bool {
        self.kind == 4
    }
}

/// Micup API'sinin `null` dönebildiği veya hiç göndermediği alanları tolere eden Sunucu yapısı.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Guild {
    pub id: Id,
    pub name: String,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub splash: Option<String>,
    #[serde(default)]
    pub banner: Option<String>,
    #[serde(default)]
    pub owner_id: Option<Id>,
    #[serde(default)]
    pub roles: Vec<Role>,
    #[serde(default)]
    pub emojis: Vec<Emoji>,
    #[serde(default)]
    pub features: Vec<String>,
    #[serde(default)]
    pub preferred_locale: Option<String>,
    #[serde(default)]
    pub premium_progress_bar_enabled: Option<bool>,
    #[serde(default)]
    pub approximate_member_count: Option<u64>,
}

impl Guild {
    pub fn icon_url(&self) -> Option<String> {
        self.icon
            .as_deref()
            .map(|h| crate::cdn::guild_icon_url(self.id.get(), h))
    }

    pub fn banner_url(&self) -> Option<String> {
        self.banner
            .as_deref()
            .map(|h| crate::cdn::guild_banner_url(self.id.get(), h))
    }

    pub fn splash_url(&self) -> Option<String> {
        self.splash
            .as_deref()
            .map(|h| crate::cdn::splash_url(self.id.get(), h))
    }
}
