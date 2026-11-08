//! Micup Gateway Olayları (Typed Gateway Events).
//!
//! Ağ geçidinden gelen `MESSAGE_CREATE`, `INTERACTION_CREATE`, `READY`,
//! `VOICE_STATE_UPDATE` gibi olayları tür güvenli (type-safe) enum yapılarına dönüştürür.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::interaction::Interaction;
use crate::models::extra::Message;
use crate::models::{Channel, Emoji, Guild, Id, Member, Role, User};

/// Bir kullanıcının ses kanalındaki durumu (`VOICE_STATE_UPDATE`).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VoiceState {
    #[serde(default)]
    pub guild_id: Option<Id>,
    pub channel_id: Option<Id>,
    pub user_id: Id,
    #[serde(default)]
    pub member: Option<Member>,
    pub session_id: String,
    #[serde(default)]
    pub deaf: bool,
    #[serde(default)]
    pub mute: bool,
    #[serde(default)]
    pub self_deaf: bool,
    #[serde(default)]
    pub self_mute: bool,
    #[serde(default)]
    pub self_video: bool,
    #[serde(default)]
    pub suppress: bool,
}

/// Mesaja tepki ekleme olayı (`MESSAGE_REACTION_ADD`).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageReactionAdd {
    pub user_id: Id,
    pub channel_id: Id,
    pub message_id: Id,
    #[serde(default)]
    pub guild_id: Option<Id>,
    #[serde(default)]
    pub member: Option<Member>,
    pub emoji: Emoji,
}

/// Mesajdan tepki silme olayı (`MESSAGE_REACTION_REMOVE`).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageReactionRemove {
    pub user_id: Id,
    pub channel_id: Id,
    pub message_id: Id,
    #[serde(default)]
    pub guild_id: Option<Id>,
    pub emoji: Emoji,
}

/// Mesaj silinme olayı (`MESSAGE_DELETE`).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageDelete {
    pub id: Id,
    pub channel_id: Id,
    #[serde(default)]
    pub guild_id: Option<Id>,
}

/// Sunucudan üye ayrılma olayı (`GUILD_MEMBER_REMOVE`).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuildMemberRemove {
    pub guild_id: Id,
    pub user: User,
}

/// Ses sunucusu bağlantı izni olayı (`VOICE_SERVER_UPDATE`).
///
/// Bot ses kanalına katıldığında Micup bu olayla LiveKit sunucu adresini (`endpoint`)
/// ve bağlantı anahtarını (`token`) bota iletir.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VoiceServerUpdate {
    pub token: String,
    pub endpoint: String,
    pub connection_id: String,
    pub channel_id: Id,
    #[serde(default)]
    pub guild_id: Option<Id>,
    #[serde(default)]
    pub e2ee_key: Option<String>,
}

/// Ağ geçidi bağlantısı kurulduğunda gelen `READY` paketi.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReadyEvent {
    pub v: u64,
    pub user: User,
    pub session_id: String,
    #[serde(default)]
    pub resume_gateway_url: Option<String>,
    #[serde(default)]
    pub guilds: Vec<serde_json::Value>,
}

/// Tür güvenli Gateway Olayı.
#[derive(Clone, Debug)]
pub enum Event {
    Ready(ReadyEvent),
    Resumed,
    MessageCreate(Message),
    MessageUpdate(Message),
    MessageDelete(MessageDelete),
    MessageReactionAdd(MessageReactionAdd),
    MessageReactionRemove(MessageReactionRemove),
    InteractionCreate(Interaction),
    GuildCreate(Guild),
    GuildUpdate(Guild),
    GuildDelete(Id),
    GuildMemberAdd(Member),
    GuildMemberUpdate(Member),
    GuildMemberRemove(GuildMemberRemove),
    GuildRoleCreate(Role),
    GuildRoleUpdate(Role),
    GuildRoleDelete {
        guild_id: Id,
        role_id: Id,
    },
    ChannelCreate(Channel),
    ChannelUpdate(Channel),
    ChannelDelete(Channel),
    VoiceStateUpdate(VoiceState),
    VoiceServerUpdate(VoiceServerUpdate),
    UserUpdate(User),
    /// Henüz özel olarak türleştirilmemiş veya platforma özgü diğer olaylar.
    Unknown {
        name: String,
        data: Value,
    },
}

impl Event {
    /// Olay adı ve ham JSON verisinden tür güvenli `Event` nesnesi üretir.
    pub fn parse(name: &str, data: Value) -> Self {
        match name {
            "READY" => serde_json::from_value::<ReadyEvent>(data.clone())
                .map(Event::Ready)
                .unwrap_or_else(|_| Event::Unknown {
                    name: name.to_string(),
                    data,
                }),
            "RESUMED" => Event::Resumed,
            "MESSAGE_CREATE" => serde_json::from_value::<Message>(data.clone())
                .map(Event::MessageCreate)
                .unwrap_or_else(|_| Event::Unknown {
                    name: name.to_string(),
                    data,
                }),
            "MESSAGE_UPDATE" => serde_json::from_value::<Message>(data.clone())
                .map(Event::MessageUpdate)
                .unwrap_or_else(|_| Event::Unknown {
                    name: name.to_string(),
                    data,
                }),
            "MESSAGE_DELETE" => serde_json::from_value::<MessageDelete>(data.clone())
                .map(Event::MessageDelete)
                .unwrap_or_else(|_| Event::Unknown {
                    name: name.to_string(),
                    data,
                }),
            "MESSAGE_REACTION_ADD" => serde_json::from_value::<MessageReactionAdd>(data.clone())
                .map(Event::MessageReactionAdd)
                .unwrap_or_else(|_| Event::Unknown {
                    name: name.to_string(),
                    data,
                }),
            "MESSAGE_REACTION_REMOVE" => {
                serde_json::from_value::<MessageReactionRemove>(data.clone())
                    .map(Event::MessageReactionRemove)
                    .unwrap_or_else(|_| Event::Unknown {
                        name: name.to_string(),
                        data,
                    })
            }
            "INTERACTION_CREATE" => serde_json::from_value::<Interaction>(data.clone())
                .map(Event::InteractionCreate)
                .unwrap_or_else(|_| Event::Unknown {
                    name: name.to_string(),
                    data,
                }),
            "GUILD_CREATE" => serde_json::from_value::<Guild>(data.clone())
                .map(Event::GuildCreate)
                .unwrap_or_else(|_| Event::Unknown {
                    name: name.to_string(),
                    data,
                }),
            "GUILD_UPDATE" => serde_json::from_value::<Guild>(data.clone())
                .map(Event::GuildUpdate)
                .unwrap_or_else(|_| Event::Unknown {
                    name: name.to_string(),
                    data,
                }),
            "GUILD_DELETE" => {
                if let Some(id_val) = data
                    .get("id")
                    .and_then(|v| v.as_str())
                    .and_then(|s| s.parse::<u64>().ok())
                {
                    Event::GuildDelete(Id(id_val))
                } else {
                    Event::Unknown {
                        name: name.to_string(),
                        data,
                    }
                }
            }
            "GUILD_MEMBER_ADD" => serde_json::from_value::<Member>(data.clone())
                .map(Event::GuildMemberAdd)
                .unwrap_or_else(|_| Event::Unknown {
                    name: name.to_string(),
                    data,
                }),
            "GUILD_MEMBER_UPDATE" => serde_json::from_value::<Member>(data.clone())
                .map(Event::GuildMemberUpdate)
                .unwrap_or_else(|_| Event::Unknown {
                    name: name.to_string(),
                    data,
                }),
            "GUILD_MEMBER_REMOVE" => serde_json::from_value::<GuildMemberRemove>(data.clone())
                .map(Event::GuildMemberRemove)
                .unwrap_or_else(|_| Event::Unknown {
                    name: name.to_string(),
                    data,
                }),
            "GUILD_ROLE_CREATE" => serde_json::from_value::<Role>(data.clone())
                .map(Event::GuildRoleCreate)
                .unwrap_or_else(|_| Event::Unknown {
                    name: name.to_string(),
                    data,
                }),
            "GUILD_ROLE_UPDATE" => serde_json::from_value::<Role>(data.clone())
                .map(Event::GuildRoleUpdate)
                .unwrap_or_else(|_| Event::Unknown {
                    name: name.to_string(),
                    data,
                }),
            "CHANNEL_CREATE" => serde_json::from_value::<Channel>(data.clone())
                .map(Event::ChannelCreate)
                .unwrap_or_else(|_| Event::Unknown {
                    name: name.to_string(),
                    data,
                }),
            "CHANNEL_UPDATE" => serde_json::from_value::<Channel>(data.clone())
                .map(Event::ChannelUpdate)
                .unwrap_or_else(|_| Event::Unknown {
                    name: name.to_string(),
                    data,
                }),
            "CHANNEL_DELETE" => serde_json::from_value::<Channel>(data.clone())
                .map(Event::ChannelDelete)
                .unwrap_or_else(|_| Event::Unknown {
                    name: name.to_string(),
                    data,
                }),
            "VOICE_STATE_UPDATE" => serde_json::from_value::<VoiceState>(data.clone())
                .map(Event::VoiceStateUpdate)
                .unwrap_or_else(|_| Event::Unknown {
                    name: name.to_string(),
                    data,
                }),
            "VOICE_SERVER_UPDATE" => serde_json::from_value::<VoiceServerUpdate>(data.clone())
                .map(Event::VoiceServerUpdate)
                .unwrap_or_else(|_| Event::Unknown {
                    name: name.to_string(),
                    data,
                }),
            "USER_UPDATE" => serde_json::from_value::<User>(data.clone())
                .map(Event::UserUpdate)
                .unwrap_or_else(|_| Event::Unknown {
                    name: name.to_string(),
                    data,
                }),
            _ => Event::Unknown {
                name: name.to_string(),
                data,
            },
        }
    }
}
