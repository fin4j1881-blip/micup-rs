//! Micup ve Fluxer izin bitleri (Permissions).
//!
//! Micup, standart Discord izinlerinin yanı sıra `MANAGE_ROOM_STAGE` (`1 << 62`),
//! `PIN_MESSAGES` (`1 << 51`), `BYPASS_SLOWMODE` (`1 << 52`), `UPDATE_RTC_REGION` (`1 << 53`)
//! ve `VIEW_CHANNEL_MEMBERS` (`1 << 54`) gibi özel platform izinlerine sahiptir.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Permissions(pub u64);

impl Permissions {
    pub const CREATE_INSTANT_INVITE: u64 = 1 << 0;
    pub const KICK_MEMBERS: u64 = 1 << 1;
    pub const BAN_MEMBERS: u64 = 1 << 2;
    pub const ADMINISTRATOR: u64 = 1 << 3;
    pub const MANAGE_CHANNELS: u64 = 1 << 4;
    pub const MANAGE_GUILD: u64 = 1 << 5;
    pub const ADD_REACTIONS: u64 = 1 << 6;
    pub const VIEW_AUDIT_LOG: u64 = 1 << 7;
    pub const PRIORITY_SPEAKER: u64 = 1 << 8;
    pub const STREAM: u64 = 1 << 9;
    pub const VIEW_CHANNEL: u64 = 1 << 10;
    pub const SEND_MESSAGES: u64 = 1 << 11;
    pub const SEND_TTS_MESSAGES: u64 = 1 << 12;
    pub const MANAGE_MESSAGES: u64 = 1 << 13;
    pub const EMBED_LINKS: u64 = 1 << 14;
    pub const ATTACH_FILES: u64 = 1 << 15;
    pub const READ_MESSAGE_HISTORY: u64 = 1 << 16;
    pub const MENTION_EVERYONE: u64 = 1 << 17;
    pub const USE_EXTERNAL_EMOJIS: u64 = 1 << 18;
    pub const CONNECT: u64 = 1 << 20;
    pub const SPEAK: u64 = 1 << 21;
    pub const MUTE_MEMBERS: u64 = 1 << 22;
    pub const DEAFEN_MEMBERS: u64 = 1 << 23;
    pub const MOVE_MEMBERS: u64 = 1 << 24;
    pub const USE_VAD: u64 = 1 << 25;
    pub const CHANGE_NICKNAME: u64 = 1 << 26;
    pub const MANAGE_NICKNAMES: u64 = 1 << 27;
    pub const MANAGE_ROLES: u64 = 1 << 28;
    pub const MANAGE_WEBHOOKS: u64 = 1 << 29;
    pub const MANAGE_EXPRESSIONS: u64 = 1 << 30;
    pub const USE_EXTERNAL_STICKERS: u64 = 1 << 37;
    pub const MODERATE_MEMBERS: u64 = 1 << 40;
    pub const CREATE_EXPRESSIONS: u64 = 1 << 43;
    pub const PIN_MESSAGES: u64 = 1 << 51;
    pub const BYPASS_SLOWMODE: u64 = 1 << 52;
    pub const UPDATE_RTC_REGION: u64 = 1 << 53;
    pub const VIEW_CHANNEL_MEMBERS: u64 = 1 << 54;

    /// Micup 3B oda sahnesini yönetme izni (`1 << 62`).
    ///
    /// Bu izin TV'de pano/video göstermek ve sunucu NPC karakteri ayarlamak için gereklidir.
    pub const MANAGE_ROOM_STAGE: u64 = 1 << 62;

    pub const fn new(bits: u64) -> Self {
        Self(bits)
    }

    pub const fn contains(&self, permission: u64) -> bool {
        (self.0 & permission) == permission
    }

    pub fn insert(&mut self, permission: u64) {
        self.0 |= permission;
    }

    pub fn remove(&mut self, permission: u64) {
        self.0 &= !permission;
    }

    pub fn is_admin(&self) -> bool {
        self.contains(Self::ADMINISTRATOR)
    }

    pub fn can_manage_stage(&self) -> bool {
        self.is_admin() || self.contains(Self::MANAGE_ROOM_STAGE)
    }

    /// Micup platformu için bot davet bağlantısı oluşturur.
    pub fn bot_invite_url(client_id: u64, permissions: u64) -> String {
        format!("https://micup.gg/oauth2/authorize?client_id={client_id}&scope=bot&permissions={permissions}")
    }
}
