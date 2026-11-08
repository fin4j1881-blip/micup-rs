//! Micup Gateway Niyetleri (Intents).
//!
//! Bit hesaplamalarıyla uğraşmadan hazır niyet kombinasyonları sunar.

use std::ops::{BitOr, BitOrAssign};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Intents(pub u64);

impl Intents {
    pub const GUILDS: u64 = 1 << 0;
    pub const GUILD_MEMBERS: u64 = 1 << 1;
    pub const GUILD_MODERATION: u64 = 1 << 2;
    pub const GUILD_EMOJIS_AND_STICKERS: u64 = 1 << 3;
    pub const GUILD_INTEGRATIONS: u64 = 1 << 4;
    pub const GUILD_WEBHOOKS: u64 = 1 << 5;
    pub const GUILD_INVITES: u64 = 1 << 6;
    pub const GUILD_VOICE_STATES: u64 = 1 << 7;
    pub const GUILD_PRESENCES: u64 = 1 << 8;
    pub const GUILD_MESSAGES: u64 = 1 << 9;
    pub const GUILD_MESSAGE_REACTIONS: u64 = 1 << 10;
    pub const GUILD_MESSAGE_TYPING: u64 = 1 << 11;
    pub const DIRECT_MESSAGES: u64 = 1 << 12;
    pub const DIRECT_MESSAGE_REACTIONS: u64 = 1 << 13;
    pub const DIRECT_MESSAGE_TYPING: u64 = 1 << 14;
    pub const MESSAGE_CONTENT: u64 = 1 << 15;
    pub const GUILD_SCHEDULED_EVENTS: u64 = 1 << 16;
    pub const AUTO_MODERATION_CONFIGURATION: u64 = 1 << 20;
    pub const AUTO_MODERATION_EXECUTION: u64 = 1 << 21;

    pub const fn new(bits: u64) -> Self {
        Self(bits)
    }

    /// Çoğu bot için yeterli olan standart niyetler (`GUILDS | GUILD_MESSAGES | MESSAGE_CONTENT`).
    pub const fn default_intents() -> Self {
        Self(Self::GUILDS | Self::GUILD_MESSAGES | Self::MESSAGE_CONTENT)
    }

    /// Tüm standart niyetleri etkinleştirir.
    pub const fn all() -> Self {
        Self(
            Self::GUILDS
                | Self::GUILD_MEMBERS
                | Self::GUILD_MODERATION
                | Self::GUILD_EMOJIS_AND_STICKERS
                | Self::GUILD_INTEGRATIONS
                | Self::GUILD_WEBHOOKS
                | Self::GUILD_INVITES
                | Self::GUILD_VOICE_STATES
                | Self::GUILD_PRESENCES
                | Self::GUILD_MESSAGES
                | Self::GUILD_MESSAGE_REACTIONS
                | Self::GUILD_MESSAGE_TYPING
                | Self::DIRECT_MESSAGES
                | Self::DIRECT_MESSAGE_REACTIONS
                | Self::DIRECT_MESSAGE_TYPING
                | Self::MESSAGE_CONTENT
                | Self::GUILD_SCHEDULED_EVENTS
                | Self::AUTO_MODERATION_CONFIGURATION
                | Self::AUTO_MODERATION_EXECUTION,
        )
    }

    pub const fn bits(self) -> u64 {
        self.0
    }
}

impl BitOr for Intents {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOr<u64> for Intents {
    type Output = Self;
    fn bitor(self, rhs: u64) -> Self::Output {
        Self(self.0 | rhs)
    }
}

impl BitOrAssign<u64> for Intents {
    fn bitor_assign(&mut self, rhs: u64) {
        self.0 |= rhs;
    }
}
