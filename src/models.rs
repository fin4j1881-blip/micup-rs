pub mod constants {
    pub const API_BASE: &str = "https://micup.gg/api/v10";
    pub const GATEWAY_URL: &str = "wss://micup.gg/gateway";
    pub const CDN_BASE: &str = "https://media.micup.gg";
}

pub mod extra;
pub use extra::{
    Attachment, AuditLog, AuditLogEntry, Ban, Emoji, Invite, Message, Reaction, Webhook,
};

use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

/// Eksik veya null dönebilecek alanları türün varsayılan değerine eşleyen toleranslı deserializer.
pub fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

/// Micup / Fluxer ağ geçidi bot oturum bilgisi (`GET /gateway/bot`).
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct GatewayBotInfo {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub url: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub shards: u32,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub session_start_limit: SessionStartLimit,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SessionStartLimit {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub total: u32,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub remaining: u32,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub reset_after: u64,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub max_concurrency: u32,
}

/// Micup platform keşif belgesi (`GET /.well-known/fluxer`).
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FluxerDiscovery {
    #[serde(default)]
    pub api_code_version: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub endpoints: DiscoveryEndpoints,
    #[serde(default)]
    pub features: Option<serde_json::Value>,
    #[serde(default)]
    pub limits: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct DiscoveryEndpoints {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub api: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub api_public: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub gateway: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
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
    #[inline]
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

            fn visit_unit<E>(self) -> Result<Self::Value, E> {
                Ok(Id(0))
            }

            fn visit_none<E>(self) -> Result<Self::Value, E> {
                Ok(Id(0))
            }
        }

        deserializer.deserialize_any(IdVisitor)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct User {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub id: Id,
    #[serde(default, deserialize_with = "deserialize_null_default")]
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
    #[serde(default, alias = "locale")]
    pub preferred_locale: Option<String>,
    #[serde(default, alias = "premium_type")]
    pub premium_tier: Option<u64>,
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
    #[serde(default, deserialize_with = "deserialize_null_default")]
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
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub id: Id,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub color: u64,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub hoist: bool,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub position: i64,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub permissions: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub managed: bool,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub mentionable: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Channel {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub id: Id,
    #[serde(
        rename = "type",
        default,
        deserialize_with = "deserialize_null_default"
    )]
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
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub id: Id,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub name: String,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub splash: Option<String>,
    #[serde(default)]
    pub banner: Option<String>,
    #[serde(default)]
    pub owner_id: Option<Id>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub roles: Vec<Role>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub emojis: Vec<Emoji>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub features: Vec<String>,
    #[serde(default)]
    pub preferred_locale: Option<String>,
    #[serde(default)]
    pub premium_tier: Option<u64>,
    #[serde(default)]
    pub premium_progress_bar_enabled: Option<bool>,
    #[serde(default)]
    pub approximate_member_count: Option<u64>,
    #[serde(default)]
    pub description: Option<String>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_null_and_missing_fields() {
        let json = r#"{
            "id": "12345",
            "username": "testuser",
            "preferred_locale": null,
            "premium_tier": null
        }"#;
        let user: User = serde_json::from_str(json).expect("User deserialize failed");
        assert_eq!(user.id.get(), 12345);
        assert_eq!(user.username, "testuser");
        assert_eq!(user.preferred_locale, None);
        assert_eq!(user.premium_tier, None);

        let json_empty = r#"{"id": 12345, "username": "testuser"}"#;
        let user2: User = serde_json::from_str(json_empty).expect("User deserialize failed");
        assert_eq!(user2.preferred_locale, None);
        assert_eq!(user2.premium_tier, None);
    }

    #[test]
    fn test_guild_null_and_missing_fields() {
        let json = r#"{
            "id": "67890",
            "name": "Test Guild",
            "preferred_locale": null,
            "premium_tier": null,
            "icon": null,
            "splash": null,
            "banner": null
        }"#;
        let guild: Guild = serde_json::from_str(json).expect("Guild deserialize failed");
        assert_eq!(guild.id.get(), 67890);
        assert_eq!(guild.name, "Test Guild");
        assert_eq!(guild.preferred_locale, None);
        assert_eq!(guild.premium_tier, None);

        let json_with_values = r#"{
            "id": 67890,
            "name": "Test Guild",
            "preferred_locale": "tr-TR",
            "premium_tier": 2
        }"#;
        let guild2: Guild =
            serde_json::from_str(json_with_values).expect("Guild deserialize failed");
        assert_eq!(guild2.preferred_locale, Some("tr-TR".to_string()));
        assert_eq!(guild2.premium_tier, Some(2));
    }

    #[test]
    fn test_tolerant_models_all_nulls_and_missing() {
        let g: Guild = serde_json::from_str(r#"{"roles": null, "emojis": null, "features": null}"#)
            .expect("Guild tolerant deserialize failed");
        assert!(g.roles.is_empty());
        assert!(g.emojis.is_empty());
        assert!(g.features.is_empty());

        let m: Member =
            serde_json::from_str(r#"{"roles": null}"#).expect("Member tolerant deserialize failed");
        assert!(m.roles.is_empty());

        let r: Role = serde_json::from_str(r#"{"name": null, "permissions": null, "color": null}"#)
            .expect("Role tolerant deserialize failed");
        assert_eq!(r.name, "");
        assert_eq!(r.permissions, "");
        assert_eq!(r.color, 0);

        let c: Channel =
            serde_json::from_str(r#"{"type": null}"#).expect("Channel tolerant deserialize failed");
        assert_eq!(c.kind, 0);

        let gb: GatewayBotInfo =
            serde_json::from_str(r#"{}"#).expect("GatewayBotInfo tolerant deserialize failed");
        assert_eq!(gb.shards, 0);
        assert_eq!(gb.session_start_limit.total, 0);
    }
}
