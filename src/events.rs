//! Micup Gateway Olayları (Typed Gateway Events).
//!
//! Ağ geçidinden gelen `MESSAGE_CREATE`, `INTERACTION_CREATE`, `READY`,
//! `VOICE_STATE_UPDATE` gibi olayları tür güvenli (type-safe) enum yapılarına dönüştürür.
//! Ayrıca 3B oda sahneleri, LiveKit ses/kamera akışları ve etkileşimler için `EventHandler`
//! arayüzünü ve tracing loglama altyapısını sağlar.

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{debug, trace, warn};

use crate::context::Context;
use crate::interaction::Interaction;
use crate::models::extra::Message;
use crate::models::{Channel, Emoji, Guild, Id, Member, Role, User};
use crate::stage::{VoiceRoomEffect, VoiceRoomStage};

/// Asenkron olay işleyici dönüş türü (`BoxFuture`).
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Profesyonel / yapısal botlar için genişletilmiş olay dinleyici arayüzü (Event Handler Trait).
///
/// Mevcut bot uygulamalarının derlemesini bozmamak için tüm metotlar varsayılan
/// boş gövdeye (`Box::pin(async {})`) sahiptir.
pub trait EventHandler: Send + Sync + 'static {
    /// Gateway bağlantısı kurulup bot oturumu hazır olduğunda tetiklenir (`READY`).
    fn on_ready<'a>(&'a self, _ctx: Context, _ready: ReadyEvent) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }

    /// Yeni bir sohbet mesajı gönderildiğinde tetiklenir (`MESSAGE_CREATE`).
    fn on_message<'a>(&'a self, _ctx: Context, _msg: Message) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }

    /// Mevcut bir mesaj güncellendiğinde / düzenlendiğinde tetiklenir (`MESSAGE_UPDATE`).
    fn on_message_update<'a>(&'a self, _ctx: Context, _msg: Message) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }

    /// Bir mesaj silindiğinde tetiklenir (`MESSAGE_DELETE`).
    fn on_message_delete<'a>(&'a self, _ctx: Context, _del: MessageDelete) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }

    /// Mesaja tepki (emoji) eklendiğinde tetiklenir (`MESSAGE_REACTION_ADD`).
    fn on_reaction_add<'a>(&'a self, _ctx: Context, _add: MessageReactionAdd) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }

    /// Mesajdan tepki silindiğinde tetiklenir (`MESSAGE_REACTION_REMOVE`).
    fn on_reaction_remove<'a>(
        &'a self,
        _ctx: Context,
        _rem: MessageReactionRemove,
    ) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }

    /// Slash komut, buton tıklaması veya modal gönderimi gibi etkileşimlerde tetiklenir (`INTERACTION_CREATE`).
    fn on_interaction<'a>(&'a self, _ctx: Context, _interaction: Interaction) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }

    /// Bir kullanıcının ses veya kamera durumunda değişiklik olduğunda tetiklenir (`VOICE_STATE_UPDATE`).
    ///
    /// Mikrofon susturma (`self_mute`), sağırlaştırma (`self_deaf`) ve kamera (`self_video`) durumlarını içerir.
    fn on_voice_state<'a>(&'a self, _ctx: Context, _voice: VoiceState) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }

    /// Ses sunucusu bağlantı ve LiveKit endpoint bilgisi geldiğinde tetiklenir (`VOICE_SERVER_UPDATE`).
    fn on_voice_server<'a>(&'a self, _ctx: Context, _vsu: VoiceServerUpdate) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }

    /// 3B Oda Sahnesi (Voice Room Stage) yapılandırması güncellendiğinde tetiklenir (`VOICE_ROOM_STAGE_UPDATE`).
    fn on_stage_update<'a>(&'a self, _ctx: Context, _stage: VoiceRoomStage) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }

    /// 3B odada tek seferlik sahne efekti (konfeti, spot ışığı, NPC hareketi) patlatıldığında tetiklenir.
    fn on_stage_effect<'a>(&'a self, _ctx: Context, _effect: VoiceRoomEffect) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }

    /// 3B Oda Sahnesi detaylı kanal olayı alındığında tetiklenir.
    fn on_voice_room_stage<'a>(
        &'a self,
        _ctx: Context,
        _update: VoiceRoomStageUpdate,
    ) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }

    /// Yeni bir kanal oluşturulduğunda tetiklenir (`CHANNEL_CREATE`).
    fn on_channel_create<'a>(&'a self, _ctx: Context, _channel: Channel) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }

    /// Bir kanal güncellendiğinde tetiklenir (`CHANNEL_UPDATE`).
    fn on_channel_update<'a>(&'a self, _ctx: Context, _channel: Channel) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }

    /// Bir kanal silindiğinde tetiklenir (`CHANNEL_DELETE`).
    fn on_channel_delete<'a>(&'a self, _ctx: Context, _channel: Channel) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }

    /// Bot yeni bir sunucuya katıldığında veya sunucu verisi yüklendiğinde tetiklenir (`GUILD_CREATE`).
    fn on_guild_create<'a>(&'a self, _ctx: Context, _guild: Guild) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }

    /// Sunucu ayarları güncellendiğinde tetiklenir (`GUILD_UPDATE`).
    fn on_guild_update<'a>(&'a self, _ctx: Context, _guild: Guild) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }

    /// Sunucu silindiğinde veya bot sunucudan çıkarıldığında tetiklenir (`GUILD_DELETE`).
    fn on_guild_delete<'a>(&'a self, _ctx: Context, _guild_id: Id) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }

    /// Sunucuya yeni bir üye katıldığında tetiklenir (`GUILD_MEMBER_ADD`).
    fn on_guild_member_add<'a>(&'a self, _ctx: Context, _member: Member) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }

    /// Sunucudaki bir üyenin rolleri veya profili güncellendiğinde tetiklenir (`GUILD_MEMBER_UPDATE`).
    fn on_guild_member_update<'a>(&'a self, _ctx: Context, _member: Member) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }

    /// Sunucudan bir üye ayrıldığında tetiklenir (`GUILD_MEMBER_REMOVE`).
    fn on_guild_member_remove<'a>(
        &'a self,
        _ctx: Context,
        _remove: GuildMemberRemove,
    ) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }

    /// Kullanıcı bilgileri güncellendiğinde tetiklenir (`USER_UPDATE`).
    fn on_user_update<'a>(&'a self, _ctx: Context, _user: User) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }

    /// Tüm ham ve tür güvenli Gateway olayları için genel yakalayıcı.
    fn on_event<'a>(&'a self, _ctx: Context, _event: Event) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }
}

/// 3B Oda Sahnesi Güncelleme Olayı (`VOICE_ROOM_STAGE_UPDATE`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VoiceRoomStageUpdate {
    pub channel_id: Id,
    #[serde(default)]
    pub guild_id: Option<Id>,
    #[serde(default)]
    pub stage: Option<VoiceRoomStage>,
}

/// 3B Oda Sahnesi Tek Seferlik Efekt Olayı (`VOICE_ROOM_STAGE_EFFECT`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VoiceRoomStageEffectEvent {
    pub channel_id: Id,
    #[serde(default)]
    pub guild_id: Option<Id>,
    pub effect: VoiceRoomEffect,
}

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

impl VoiceState {
    /// Kullanıcının kamerasının açık olup olmadığını döndürür (`self_video`).
    pub fn is_video_enabled(&self) -> bool {
        self.self_video
    }

    /// Kullanıcının sesinin kapalı olup olmadığını döndürür (`mute` veya `self_mute`).
    pub fn is_muted(&self) -> bool {
        self.mute || self.self_mute
    }

    /// Kullanıcının sağırlaştırılmış olup olmadığını döndürür (`deaf` veya `self_deaf`).
    pub fn is_deafened(&self) -> bool {
        self.deaf || self.self_deaf
    }
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

impl VoiceServerUpdate {
    /// LiveKit sunucu uç noktasını (URL) döndürür.
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// LiveKit bağlantı anahtarını (JWT token) döndürür.
    pub fn token(&self) -> &str {
        &self.token
    }
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
    /// Olayın adını veya tipini metin olarak döndürür.
    pub fn name(&self) -> &str {
        match self {
            Event::Ready(_) => "READY",
            Event::Resumed => "RESUMED",
            Event::MessageCreate(_) => "MESSAGE_CREATE",
            Event::MessageUpdate(_) => "MESSAGE_UPDATE",
            Event::MessageDelete(_) => "MESSAGE_DELETE",
            Event::MessageReactionAdd(_) => "MESSAGE_REACTION_ADD",
            Event::MessageReactionRemove(_) => "MESSAGE_REACTION_REMOVE",
            Event::InteractionCreate(_) => "INTERACTION_CREATE",
            Event::GuildCreate(_) => "GUILD_CREATE",
            Event::GuildUpdate(_) => "GUILD_UPDATE",
            Event::GuildDelete(_) => "GUILD_DELETE",
            Event::GuildMemberAdd(_) => "GUILD_MEMBER_ADD",
            Event::GuildMemberUpdate(_) => "GUILD_MEMBER_UPDATE",
            Event::GuildMemberRemove(_) => "GUILD_MEMBER_REMOVE",
            Event::GuildRoleCreate(_) => "GUILD_ROLE_CREATE",
            Event::GuildRoleUpdate(_) => "GUILD_ROLE_UPDATE",
            Event::GuildRoleDelete { .. } => "GUILD_ROLE_DELETE",
            Event::ChannelCreate(_) => "CHANNEL_CREATE",
            Event::ChannelUpdate(_) => "CHANNEL_UPDATE",
            Event::ChannelDelete(_) => "CHANNEL_DELETE",
            Event::VoiceStateUpdate(_) => "VOICE_STATE_UPDATE",
            Event::VoiceServerUpdate(_) => "VOICE_SERVER_UPDATE",
            Event::UserUpdate(_) => "USER_UPDATE",
            Event::Unknown { name, .. } => name.as_str(),
        }
    }

    /// Olayın bir ses veya kamera durumu güncellemesi olup olmadığını kontrol eder.
    pub fn voice_state(&self) -> Option<&VoiceState> {
        match self {
            Event::VoiceStateUpdate(ref vs) => Some(vs),
            _ => None,
        }
    }

    /// Olayın bir LiveKit ses sunucusu güncellemesi olup olmadığını kontrol eder.
    pub fn voice_server(&self) -> Option<&VoiceServerUpdate> {
        match self {
            Event::VoiceServerUpdate(ref vsu) => Some(vsu),
            _ => None,
        }
    }

    /// Olayın bir etkileşim (Interaction) olup olmadığını kontrol eder.
    pub fn interaction(&self) -> Option<&Interaction> {
        match self {
            Event::InteractionCreate(ref inter) => Some(inter),
            _ => None,
        }
    }

    /// Olay adı ve ham JSON verisinden tür güvenli `Event` nesnesi üretir.
    ///
    /// Olay akışı `tracing::debug` ve `tracing::trace` makroları kullanılarak detaylıca kaydedilir.
    pub fn parse(name: &str, data: Value) -> Self {
        trace!(event = %name, "Gateway olayı alınıyor ve ayrıştırılıyor");
        match name {
            "READY" => {
                debug!("Gateway READY olayı ayrıştırılıyor");
                serde_json::from_value::<ReadyEvent>(data.clone())
                    .map(|ev| {
                        trace!(username = %ev.user.username, session_id = %ev.session_id, "READY başarıyla ayrıştırıldı");
                        Event::Ready(ev)
                    })
                    .unwrap_or_else(|err| {
                        warn!(error = %err, "READY JSON ayrıştırma hatası");
                        Event::Unknown {
                            name: name.to_string(),
                            data,
                        }
                    })
            }
            "RESUMED" => {
                debug!("Gateway RESUMED olayı alındı");
                Event::Resumed
            }
            "MESSAGE_CREATE" => {
                debug!("MESSAGE_CREATE olayı ayrıştırılıyor");
                serde_json::from_value::<Message>(data.clone())
                    .map(|msg| {
                        trace!(message_id = %msg.id, "MESSAGE_CREATE başarıyla ayrıştırıldı");
                        Event::MessageCreate(msg)
                    })
                    .unwrap_or_else(|err| {
                        warn!(error = %err, "MESSAGE_CREATE JSON ayrıştırma hatası");
                        Event::Unknown {
                            name: name.to_string(),
                            data,
                        }
                    })
            }
            "MESSAGE_UPDATE" => {
                debug!("MESSAGE_UPDATE olayı ayrıştırılıyor");
                serde_json::from_value::<Message>(data.clone())
                    .map(|msg| {
                        trace!(message_id = %msg.id, "MESSAGE_UPDATE başarıyla ayrıştırıldı");
                        Event::MessageUpdate(msg)
                    })
                    .unwrap_or_else(|err| {
                        warn!(error = %err, "MESSAGE_UPDATE JSON ayrıştırma hatası");
                        Event::Unknown {
                            name: name.to_string(),
                            data,
                        }
                    })
            }
            "MESSAGE_DELETE" => {
                debug!("MESSAGE_DELETE olayı ayrıştırılıyor");
                serde_json::from_value::<MessageDelete>(data.clone())
                    .map(|del| {
                        trace!(message_id = %del.id, channel_id = %del.channel_id, "MESSAGE_DELETE başarıyla ayrıştırıldı");
                        Event::MessageDelete(del)
                    })
                    .unwrap_or_else(|err| {
                        warn!(error = %err, "MESSAGE_DELETE JSON ayrıştırma hatası");
                        Event::Unknown {
                            name: name.to_string(),
                            data,
                        }
                    })
            }
            "MESSAGE_REACTION_ADD" => {
                debug!("MESSAGE_REACTION_ADD olayı ayrıştırılıyor");
                serde_json::from_value::<MessageReactionAdd>(data.clone())
                    .map(|add| {
                        trace!(user_id = %add.user_id, message_id = %add.message_id, "MESSAGE_REACTION_ADD başarıyla ayrıştırıldı");
                        Event::MessageReactionAdd(add)
                    })
                    .unwrap_or_else(|err| {
                        warn!(error = %err, "MESSAGE_REACTION_ADD JSON ayrıştırma hatası");
                        Event::Unknown {
                            name: name.to_string(),
                            data,
                        }
                    })
            }
            "MESSAGE_REACTION_REMOVE" => {
                debug!("MESSAGE_REACTION_REMOVE olayı ayrıştırılıyor");
                serde_json::from_value::<MessageReactionRemove>(data.clone())
                    .map(|rem| {
                        trace!(user_id = %rem.user_id, message_id = %rem.message_id, "MESSAGE_REACTION_REMOVE başarıyla ayrıştırıldı");
                        Event::MessageReactionRemove(rem)
                    })
                    .unwrap_or_else(|err| {
                        warn!(error = %err, "MESSAGE_REACTION_REMOVE JSON ayrıştırma hatası");
                        Event::Unknown {
                            name: name.to_string(),
                            data,
                        }
                    })
            }
            "INTERACTION_CREATE" => {
                debug!("INTERACTION_CREATE olayı ayrıştırılıyor");
                serde_json::from_value::<Interaction>(data.clone())
                    .map(|interaction| {
                        trace!(interaction_id = %interaction.id, kind = ?interaction.kind, "INTERACTION_CREATE başarıyla ayrıştırıldı");
                        Event::InteractionCreate(interaction)
                    })
                    .unwrap_or_else(|err| {
                        warn!(error = %err, "INTERACTION_CREATE JSON ayrıştırma hatası");
                        Event::Unknown {
                            name: name.to_string(),
                            data,
                        }
                    })
            }
            "GUILD_CREATE" => {
                debug!("GUILD_CREATE olayı ayrıştırılıyor");
                serde_json::from_value::<Guild>(data.clone())
                    .map(|guild| {
                        trace!(guild_id = %guild.id, guild_name = %guild.name, "GUILD_CREATE başarıyla ayrıştırıldı");
                        Event::GuildCreate(guild)
                    })
                    .unwrap_or_else(|err| {
                        warn!(error = %err, "GUILD_CREATE JSON ayrıştırma hatası");
                        Event::Unknown {
                            name: name.to_string(),
                            data,
                        }
                    })
            }
            "GUILD_UPDATE" => {
                debug!("GUILD_UPDATE olayı ayrıştırılıyor");
                serde_json::from_value::<Guild>(data.clone())
                    .map(|guild| {
                        trace!(guild_id = %guild.id, guild_name = %guild.name, "GUILD_UPDATE başarıyla ayrıştırıldı");
                        Event::GuildUpdate(guild)
                    })
                    .unwrap_or_else(|err| {
                        warn!(error = %err, "GUILD_UPDATE JSON ayrıştırma hatası");
                        Event::Unknown {
                            name: name.to_string(),
                            data,
                        }
                    })
            }
            "GUILD_DELETE" => {
                debug!("GUILD_DELETE olayı ayrıştırılıyor");
                if let Some(id_val) = data
                    .get("id")
                    .and_then(|v| v.as_str())
                    .and_then(|s| s.parse::<u64>().ok())
                    .or_else(|| data.get("id").and_then(|v| v.as_u64()))
                {
                    trace!(guild_id = id_val, "GUILD_DELETE başarıyla ayrıştırıldı");
                    Event::GuildDelete(Id(id_val))
                } else {
                    warn!("GUILD_DELETE id alanı bulunamadı");
                    Event::Unknown {
                        name: name.to_string(),
                        data,
                    }
                }
            }
            "GUILD_MEMBER_ADD" => {
                debug!("GUILD_MEMBER_ADD olayı ayrıştırılıyor");
                serde_json::from_value::<Member>(data.clone())
                    .map(|member| {
                        trace!("GUILD_MEMBER_ADD başarıyla ayrıştırıldı");
                        Event::GuildMemberAdd(member)
                    })
                    .unwrap_or_else(|err| {
                        warn!(error = %err, "GUILD_MEMBER_ADD JSON ayrıştırma hatası");
                        Event::Unknown {
                            name: name.to_string(),
                            data,
                        }
                    })
            }
            "GUILD_MEMBER_UPDATE" => {
                debug!("GUILD_MEMBER_UPDATE olayı ayrıştırılıyor");
                serde_json::from_value::<Member>(data.clone())
                    .map(|member| {
                        trace!("GUILD_MEMBER_UPDATE başarıyla ayrıştırıldı");
                        Event::GuildMemberUpdate(member)
                    })
                    .unwrap_or_else(|err| {
                        warn!(error = %err, "GUILD_MEMBER_UPDATE JSON ayrıştırma hatası");
                        Event::Unknown {
                            name: name.to_string(),
                            data,
                        }
                    })
            }
            "GUILD_MEMBER_REMOVE" => {
                debug!("GUILD_MEMBER_REMOVE olayı ayrıştırılıyor");
                serde_json::from_value::<GuildMemberRemove>(data.clone())
                    .map(|gmr| {
                        trace!(guild_id = %gmr.guild_id, user_id = %gmr.user.id, "GUILD_MEMBER_REMOVE başarıyla ayrıştırıldı");
                        Event::GuildMemberRemove(gmr)
                    })
                    .unwrap_or_else(|err| {
                        warn!(error = %err, "GUILD_MEMBER_REMOVE JSON ayrıştırma hatası");
                        Event::Unknown {
                            name: name.to_string(),
                            data,
                        }
                    })
            }
            "GUILD_ROLE_CREATE" => {
                debug!("GUILD_ROLE_CREATE olayı ayrıştırılıyor");
                serde_json::from_value::<Role>(data.clone())
                    .map(|role| {
                        trace!(role_id = %role.id, role_name = %role.name, "GUILD_ROLE_CREATE başarıyla ayrıştırıldı");
                        Event::GuildRoleCreate(role)
                    })
                    .unwrap_or_else(|err| {
                        warn!(error = %err, "GUILD_ROLE_CREATE JSON ayrıştırma hatası");
                        Event::Unknown {
                            name: name.to_string(),
                            data,
                        }
                    })
            }
            "GUILD_ROLE_UPDATE" => {
                debug!("GUILD_ROLE_UPDATE olayı ayrıştırılıyor");
                serde_json::from_value::<Role>(data.clone())
                    .map(|role| {
                        trace!(role_id = %role.id, role_name = %role.name, "GUILD_ROLE_UPDATE başarıyla ayrıştırıldı");
                        Event::GuildRoleUpdate(role)
                    })
                    .unwrap_or_else(|err| {
                        warn!(error = %err, "GUILD_ROLE_UPDATE JSON ayrıştırma hatası");
                        Event::Unknown {
                            name: name.to_string(),
                            data,
                        }
                    })
            }
            "CHANNEL_CREATE" => {
                debug!("CHANNEL_CREATE olayı ayrıştırılıyor");
                serde_json::from_value::<Channel>(data.clone())
                    .map(|channel| {
                        trace!(channel_id = %channel.id, "CHANNEL_CREATE başarıyla ayrıştırıldı");
                        Event::ChannelCreate(channel)
                    })
                    .unwrap_or_else(|err| {
                        warn!(error = %err, "CHANNEL_CREATE JSON ayrıştırma hatası");
                        Event::Unknown {
                            name: name.to_string(),
                            data,
                        }
                    })
            }
            "CHANNEL_UPDATE" => {
                debug!("CHANNEL_UPDATE olayı ayrıştırılıyor");
                serde_json::from_value::<Channel>(data.clone())
                    .map(|channel| {
                        trace!(channel_id = %channel.id, "CHANNEL_UPDATE başarıyla ayrıştırıldı");
                        Event::ChannelUpdate(channel)
                    })
                    .unwrap_or_else(|err| {
                        warn!(error = %err, "CHANNEL_UPDATE JSON ayrıştırma hatası");
                        Event::Unknown {
                            name: name.to_string(),
                            data,
                        }
                    })
            }
            "CHANNEL_DELETE" => {
                debug!("CHANNEL_DELETE olayı ayrıştırılıyor");
                serde_json::from_value::<Channel>(data.clone())
                    .map(|channel| {
                        trace!(channel_id = %channel.id, "CHANNEL_DELETE başarıyla ayrıştırıldı");
                        Event::ChannelDelete(channel)
                    })
                    .unwrap_or_else(|err| {
                        warn!(error = %err, "CHANNEL_DELETE JSON ayrıştırma hatası");
                        Event::Unknown {
                            name: name.to_string(),
                            data,
                        }
                    })
            }
            "VOICE_STATE_UPDATE" => {
                debug!("VOICE_STATE_UPDATE ses/kamera durum olayı ayrıştırılıyor");
                serde_json::from_value::<VoiceState>(data.clone())
                    .map(|vs| {
                        trace!(
                            user_id = %vs.user_id,
                            self_video = vs.self_video,
                            self_mute = vs.self_mute,
                            self_deaf = vs.self_deaf,
                            "VOICE_STATE_UPDATE ses ve kamera durumu başarıyla ayrıştırıldı"
                        );
                        Event::VoiceStateUpdate(vs)
                    })
                    .unwrap_or_else(|err| {
                        warn!(error = %err, "VOICE_STATE_UPDATE JSON ayrıştırma hatası");
                        Event::Unknown {
                            name: name.to_string(),
                            data,
                        }
                    })
            }
            "VOICE_SERVER_UPDATE" => {
                debug!("VOICE_SERVER_UPDATE LiveKit ses sunucusu bağlantı olayı ayrıştırılıyor");
                serde_json::from_value::<VoiceServerUpdate>(data.clone())
                    .map(|vsu| {
                        trace!(
                            channel_id = %vsu.channel_id,
                            endpoint = %vsu.endpoint,
                            "VOICE_SERVER_UPDATE LiveKit uç noktası başarıyla ayrıştırıldı"
                        );
                        Event::VoiceServerUpdate(vsu)
                    })
                    .unwrap_or_else(|err| {
                        warn!(error = %err, "VOICE_SERVER_UPDATE JSON ayrıştırma hatası");
                        Event::Unknown {
                            name: name.to_string(),
                            data,
                        }
                    })
            }
            "VOICE_ROOM_STAGE_UPDATE"
            | "STAGE_INSTANCE_CREATE"
            | "STAGE_INSTANCE_UPDATE"
            | "STAGE_INSTANCE_DELETE" => {
                debug!(event = %name, "3B Oda Sahnesi (Voice Room Stage) olayı alındı");
                trace!(event = %name, payload = ?data, "3B sahne güncelleme verisi");
                Event::Unknown {
                    name: name.to_string(),
                    data,
                }
            }
            "USER_UPDATE" => {
                debug!("USER_UPDATE olayı ayrıştırılıyor");
                serde_json::from_value::<User>(data.clone())
                    .map(|user| {
                        trace!(user_id = %user.id, username = %user.username, "USER_UPDATE başarıyla ayrıştırıldı");
                        Event::UserUpdate(user)
                    })
                    .unwrap_or_else(|err| {
                        warn!(error = %err, "USER_UPDATE JSON ayrıştırma hatası");
                        Event::Unknown {
                            name: name.to_string(),
                            data,
                        }
                    })
            }
            _ => {
                debug!(event = %name, "Bilinmeyen veya henüz özel türleştirilmemiş olay");
                Event::Unknown {
                    name: name.to_string(),
                    data,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voice_state_flags_and_helpers() {
        let vs = VoiceState {
            guild_id: Some(Id(12345)),
            channel_id: Some(Id(67890)),
            user_id: Id(11111),
            member: None,
            session_id: "sess_voice".to_string(),
            deaf: false,
            mute: false,
            self_deaf: true,
            self_mute: false,
            self_video: true,
            suppress: false,
        };

        assert!(vs.is_video_enabled());
        assert!(!vs.is_muted());
        assert!(vs.is_deafened());
    }

    #[test]
    fn test_voice_server_update_helpers() {
        let vsu = VoiceServerUpdate {
            token: "livekit_jwt_token".to_string(),
            endpoint: "wss://livekit.micup.gg".to_string(),
            connection_id: "conn_123".to_string(),
            channel_id: Id(9999),
            guild_id: Some(Id(8888)),
            e2ee_key: None,
        };

        assert_eq!(vsu.endpoint(), "wss://livekit.micup.gg");
        assert_eq!(vsu.token(), "livekit_jwt_token");
    }

    #[test]
    fn test_stage_update_models_serde() {
        let update = VoiceRoomStageUpdate {
            channel_id: Id(1234),
            guild_id: Some(Id(5678)),
            stage: Some(VoiceRoomStage::default()),
        };

        let json = serde_json::to_string(&update).expect("Serileştirme hatası");
        let parsed: VoiceRoomStageUpdate = serde_json::from_str(&json).expect("Ayrıştırma hatası");
        assert_eq!(parsed.channel_id, Id(1234));
        assert!(parsed.stage.is_some());
    }

    #[test]
    fn test_event_parse_with_tracing() {
        let vs_json = serde_json::json!({
            "guild_id": "100",
            "channel_id": "200",
            "user_id": "300",
            "session_id": "sess_1",
            "self_video": true,
            "self_mute": true,
            "self_deaf": false
        });

        let ev = Event::parse("VOICE_STATE_UPDATE", vs_json);
        assert_eq!(ev.name(), "VOICE_STATE_UPDATE");
        let vs = ev.voice_state().expect("VoiceStateUpdate bekleniyordu");
        assert!(vs.is_video_enabled());
        assert!(vs.is_muted());

        let unknown_ev = Event::parse("VOICE_ROOM_STAGE_UPDATE", serde_json::json!({"v": 1}));
        assert_eq!(unknown_ev.name(), "VOICE_ROOM_STAGE_UPDATE");
    }

    struct CustomTestHandler;
    impl EventHandler for CustomTestHandler {}

    #[test]
    fn test_event_handler_trait_default_implementations() {
        let handler = CustomTestHandler;
        let _ = &handler;
    }
}
