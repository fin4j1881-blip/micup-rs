//! Micup ve Fluxer izin bitleri (Permissions).
//!
//! Micup, standart Discord/Fluxer izinlerinin yanı sıra `MANAGE_ROOM_STAGE` (`1 << 62`),
//! `PIN_MESSAGES` (`1 << 51`), `BYPASS_SLOWMODE` (`1 << 52`), `UPDATE_RTC_REGION` (`1 << 53`)
//! ve `VIEW_CHANNEL_MEMBERS` (`1 << 54`) gibi özel platform izinlerine sahiptir.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

/// Kanal veya kategori üzerinde belirli bir rol ya da üye için izin geçersiz kılma (overwrite) tanımı.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PermissionOverwrite {
    /// Hedef rol veya kullanıcı kimliği.
    pub id: u64,
    /// İzin verilen bitler.
    pub allow: Permissions,
    /// Reddedilen bitler.
    pub deny: Permissions,
}

impl PermissionOverwrite {
    pub const fn new(id: u64, allow: Permissions, deny: Permissions) -> Self {
        Self { id, allow, deny }
    }
}

/// Micup ve Fluxer platformunda kullanılan 64-bit izin bayrakları yapısı.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Permissions(pub u64);

impl Permissions {
    pub const NONE: Self = Self(0);
    pub const ALL: Self = Self(u64::MAX);

    // Standart platform izinleri
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
    pub const VIEW_GUILD_INSIGHTS: u64 = 1 << 19;
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
    pub const MANAGE_EMOJIS_AND_STICKERS: u64 = Self::MANAGE_EXPRESSIONS;
    pub const REQUEST_TO_SPEAK: u64 = 1 << 31;
    pub const MANAGE_EVENTS: u64 = 1 << 32;
    pub const MANAGE_THREADS: u64 = 1 << 33;
    pub const CREATE_PUBLIC_THREADS: u64 = 1 << 34;
    pub const CREATE_PRIVATE_THREADS: u64 = 1 << 35;
    pub const USE_EXTERNAL_STICKERS: u64 = 1 << 37;
    pub const SEND_MESSAGES_IN_THREADS: u64 = 1 << 38;
    pub const USE_EMBEDDED_ACTIVITIES: u64 = 1 << 39;
    pub const MODERATE_MEMBERS: u64 = 1 << 40;
    pub const VIEW_CREATOR_MONETIZATION_ANALYTICS: u64 = 1 << 41;
    pub const USE_SOUNDBOARD: u64 = 1 << 42;
    pub const CREATE_EXPRESSIONS: u64 = 1 << 43;
    pub const USE_EXTERNAL_SOUNDS: u64 = 1 << 44;
    pub const SEND_VOICE_MESSAGES: u64 = 1 << 45;
    pub const SEND_POLLS: u64 = 1 << 49;
    pub const USE_EXTERNAL_APPS: u64 = 1 << 50;

    // Micup platformuna özgü özel izin bitleri
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

    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn all() -> Self {
        Self(u64::MAX)
    }

    pub const fn bits(&self) -> u64 {
        self.0
    }

    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Belirtilen iznin bit maskesinde doğrudan yer alıp almadığını kontrol eder.
    pub const fn contains(&self, permission: u64) -> bool {
        (self.0 & permission) == permission
    }

    /// Hiyerarşik yetki doğrulama: ADMINISTRATOR izni varsa veya doğrudan istenen izin(ler) mevcutsa true döner.
    pub const fn has_permission(&self, permission: u64) -> bool {
        self.is_admin() || self.contains(permission)
    }

    /// Birden fazla iznin tamamının karşılanıp karşılanmadığını doğrular.
    pub const fn has_permissions(&self, permissions: u64) -> bool {
        self.is_admin() || self.contains(permissions)
    }

    pub fn insert(&mut self, permission: u64) {
        self.0 |= permission;
    }

    pub fn remove(&mut self, permission: u64) {
        self.0 &= !permission;
    }

    pub fn toggle(&mut self, permission: u64) {
        self.0 ^= permission;
    }

    pub const fn is_admin(&self) -> bool {
        self.contains(Self::ADMINISTRATOR)
    }

    pub const fn can_manage_stage(&self) -> bool {
        self.is_admin() || self.contains(Self::MANAGE_ROOM_STAGE)
    }

    /// Verilen rollere ait izinleri birleştirerek kullanıcının temel sunucu izinlerini hesaplar.
    pub fn compute_base_permissions<I: IntoIterator<Item = Permissions>>(roles: I) -> Self {
        let mut combined = Self::empty();
        for r in roles {
            combined.insert(r.0);
        }
        combined
    }

    /// Kanal bazlı izin geçersiz kılmalarını (overwrites) uygulayarak nihai izinleri hesaplar.
    ///
    /// Discord/Micup izin hiyerarşisi:
    /// 1. Sunucu sahibi (`is_owner`) ise tüm izinlere sahiptir.
    /// 2. `ADMINISTRATOR` iznine sahipse kanal kısıtlamaları uygulanmaz, tam yetki döner.
    /// 3. `@everyone` rol overwrite'ı uygulanır: `(base & !deny) | allow`.
    /// 4. Kullanıcının sahip olduğu rollerin overwrite'ları toplanıp uygulanır: `(perms & !role_deny) | role_allow`.
    /// 5. Kullanıcıya özel üye overwrite'ı varsa en son uygulanır: `(perms & !member_deny) | member_allow`.
    pub fn compute_channel_permissions(
        base_permissions: Permissions,
        is_owner: bool,
        everyone_overwrite: Option<&PermissionOverwrite>,
        role_overwrites: &[PermissionOverwrite],
        member_overwrite: Option<&PermissionOverwrite>,
    ) -> Permissions {
        if is_owner || base_permissions.is_admin() {
            return Self::all();
        }

        let mut perms = base_permissions.0;

        // 1. @everyone overwrite
        if let Some(ow) = everyone_overwrite {
            perms &= !ow.deny.0;
            perms |= ow.allow.0;
        }

        // 2. Rol overwrite'ları
        let mut roles_deny = 0u64;
        let mut roles_allow = 0u64;
        for ow in role_overwrites {
            roles_deny |= ow.deny.0;
            roles_allow |= ow.allow.0;
        }
        perms &= !roles_deny;
        perms |= roles_allow;

        // 3. Üye özel overwrite'ı
        if let Some(ow) = member_overwrite {
            perms &= !ow.deny.0;
            perms |= ow.allow.0;
        }

        Permissions(perms)
    }

    /// Micup platformu için bot davet bağlantısı oluşturur.
    pub fn bot_invite_url(client_id: u64, permissions: u64) -> String {
        format!("https://micup.gg/oauth2/authorize?client_id={client_id}&scope=bot&permissions={permissions}")
    }
}

// Operatör Aşırı Yüklemeleri (Bitwise traits)

impl BitOr for Permissions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOr<u64> for Permissions {
    type Output = Self;
    fn bitor(self, rhs: u64) -> Self::Output {
        Self(self.0 | rhs)
    }
}

impl BitOrAssign for Permissions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitOrAssign<u64> for Permissions {
    fn bitor_assign(&mut self, rhs: u64) {
        self.0 |= rhs;
    }
}

impl BitAnd for Permissions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAnd<u64> for Permissions {
    type Output = Self;
    fn bitand(self, rhs: u64) -> Self::Output {
        Self(self.0 & rhs)
    }
}

impl BitAndAssign for Permissions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitAndAssign<u64> for Permissions {
    fn bitand_assign(&mut self, rhs: u64) {
        self.0 &= rhs;
    }
}

impl BitXor for Permissions {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXor<u64> for Permissions {
    type Output = Self;
    fn bitxor(self, rhs: u64) -> Self::Output {
        Self(self.0 ^ rhs)
    }
}

impl BitXorAssign for Permissions {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl BitXorAssign<u64> for Permissions {
    fn bitxor_assign(&mut self, rhs: u64) {
        self.0 ^= rhs;
    }
}

impl Not for Permissions {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl From<u64> for Permissions {
    fn from(bits: u64) -> Self {
        Self(bits)
    }
}

impl From<Permissions> for u64 {
    fn from(perm: Permissions) -> Self {
        perm.0
    }
}

impl fmt::Display for Permissions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Toleranslı Serde Desteği (String ve integer JSON tipleri)

impl Serialize for Permissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Permissions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PermissionsVisitor;

        impl<'de> serde::de::Visitor<'de> for PermissionsVisitor {
            type Value = Permissions;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("bir tamsayı veya string formatında izin değeri")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
                Ok(Permissions(v))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                u64::try_from(v)
                    .map(Permissions)
                    .map_err(serde::de::Error::custom)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.parse::<u64>()
                    .map(Permissions)
                    .map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(PermissionsVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_micup_custom_permission_bits() {
        assert_eq!(Permissions::PIN_MESSAGES, 1 << 51);
        assert_eq!(Permissions::BYPASS_SLOWMODE, 1 << 52);
        assert_eq!(Permissions::UPDATE_RTC_REGION, 1 << 53);
        assert_eq!(Permissions::VIEW_CHANNEL_MEMBERS, 1 << 54);
        assert_eq!(Permissions::MANAGE_ROOM_STAGE, 1 << 62);

        // Bitlerin birbirinden bağımsız ve çakışmasız olduğunu doğrula
        let custom_perms = [
            Permissions::PIN_MESSAGES,
            Permissions::BYPASS_SLOWMODE,
            Permissions::UPDATE_RTC_REGION,
            Permissions::VIEW_CHANNEL_MEMBERS,
            Permissions::MANAGE_ROOM_STAGE,
        ];
        for (i, &p1) in custom_perms.iter().enumerate() {
            assert_ne!(p1, 0);
            for (j, &p2) in custom_perms.iter().enumerate() {
                if i != j {
                    assert_eq!(p1 & p2, 0, "İzin bitleri çakışmamalıdır");
                }
            }
        }
    }

    #[test]
    fn test_permission_contains_and_combinations() {
        let mut perms = Permissions::new(0);
        assert!(!perms.contains(Permissions::PIN_MESSAGES));
        assert!(!perms.contains(Permissions::MANAGE_ROOM_STAGE));

        perms.insert(Permissions::PIN_MESSAGES);
        assert!(perms.contains(Permissions::PIN_MESSAGES));
        assert!(!perms.contains(Permissions::BYPASS_SLOWMODE));

        perms.insert(Permissions::BYPASS_SLOWMODE | Permissions::VIEW_CHANNEL_MEMBERS);
        assert!(perms.contains(Permissions::PIN_MESSAGES));
        assert!(perms.contains(Permissions::BYPASS_SLOWMODE));
        assert!(perms.contains(Permissions::VIEW_CHANNEL_MEMBERS));
        assert!(!perms.contains(Permissions::UPDATE_RTC_REGION));

        // Çoklu izin kontrolü
        assert!(perms.contains(Permissions::PIN_MESSAGES | Permissions::BYPASS_SLOWMODE));
        assert!(!perms.contains(Permissions::PIN_MESSAGES | Permissions::UPDATE_RTC_REGION));

        // Kaldırma (remove) işlemi
        perms.remove(Permissions::PIN_MESSAGES);
        assert!(!perms.contains(Permissions::PIN_MESSAGES));
        assert!(perms.contains(Permissions::BYPASS_SLOWMODE));
    }

    #[test]
    fn test_stage_management_and_admin() {
        let normal_user = Permissions::new(Permissions::SEND_MESSAGES | Permissions::VIEW_CHANNEL);
        assert!(!normal_user.can_manage_stage());
        assert!(!normal_user.is_admin());

        let stage_manager = Permissions::new(Permissions::MANAGE_ROOM_STAGE);
        assert!(stage_manager.can_manage_stage());
        assert!(!stage_manager.is_admin());

        let admin = Permissions::new(Permissions::ADMINISTRATOR);
        assert!(admin.is_admin());
        assert!(admin.can_manage_stage());

        let all_custom = Permissions::new(
            Permissions::PIN_MESSAGES
                | Permissions::BYPASS_SLOWMODE
                | Permissions::UPDATE_RTC_REGION
                | Permissions::VIEW_CHANNEL_MEMBERS
                | Permissions::MANAGE_ROOM_STAGE,
        );
        assert!(all_custom.can_manage_stage());
        assert!(all_custom.contains(Permissions::PIN_MESSAGES));
        assert!(all_custom.contains(Permissions::BYPASS_SLOWMODE));
        assert!(all_custom.contains(Permissions::UPDATE_RTC_REGION));
        assert!(all_custom.contains(Permissions::VIEW_CHANNEL_MEMBERS));
        assert!(all_custom.contains(Permissions::MANAGE_ROOM_STAGE));
    }

    #[test]
    fn test_has_permission_hierarchical_check() {
        let admin = Permissions::new(Permissions::ADMINISTRATOR);
        // Administrator yetkisi bit maskesinde olmasa bile tüm izinler için has_permission = true dönmelidir
        assert!(admin.has_permission(Permissions::MANAGE_ROOM_STAGE));
        assert!(admin.has_permission(Permissions::PIN_MESSAGES));
        assert!(admin.has_permission(Permissions::BAN_MEMBERS));
        assert!(admin.has_permissions(Permissions::SEND_MESSAGES | Permissions::VIEW_CHANNEL));

        let user = Permissions::new(Permissions::SEND_MESSAGES | Permissions::PIN_MESSAGES);
        assert!(user.has_permission(Permissions::SEND_MESSAGES));
        assert!(user.has_permission(Permissions::PIN_MESSAGES));
        assert!(!user.has_permission(Permissions::BAN_MEMBERS));
        assert!(!user.has_permission(Permissions::MANAGE_ROOM_STAGE));
        assert!(user.has_permissions(Permissions::SEND_MESSAGES | Permissions::PIN_MESSAGES));
        assert!(!user.has_permissions(Permissions::SEND_MESSAGES | Permissions::BAN_MEMBERS));
    }

    #[test]
    fn test_role_and_user_permission_computation() {
        // Rol birleştirme testi
        let role1 = Permissions::new(Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES);
        let role2 = Permissions::new(Permissions::PIN_MESSAGES | Permissions::BYPASS_SLOWMODE);
        let base = Permissions::compute_base_permissions([role1, role2]);

        assert!(base.contains(Permissions::VIEW_CHANNEL));
        assert!(base.contains(Permissions::SEND_MESSAGES));
        assert!(base.contains(Permissions::PIN_MESSAGES));
        assert!(base.contains(Permissions::BYPASS_SLOWMODE));
        assert!(!base.contains(Permissions::MANAGE_MESSAGES));

        // Kanal izin geçersiz kılma (overwrite) testi
        let everyone_ow = PermissionOverwrite::new(
            0,
            Permissions::empty(),
            Permissions::new(Permissions::SEND_MESSAGES), // @everyone mesaj gönderemez
        );

        let role_ow = PermissionOverwrite::new(
            10,
            Permissions::new(Permissions::SEND_MESSAGES), // rol1 mesaj gönderebilir
            Permissions::empty(),
        );

        let member_ow = PermissionOverwrite::new(
            100,
            Permissions::empty(),
            Permissions::new(Permissions::PIN_MESSAGES), // üyenin pinleme yetkisi kanalda engellendi
        );

        let channel_perms = Permissions::compute_channel_permissions(
            base,
            false,
            Some(&everyone_ow),
            &[role_ow],
            Some(&member_ow),
        );

        assert!(channel_perms.contains(Permissions::VIEW_CHANNEL));
        assert!(channel_perms.contains(Permissions::SEND_MESSAGES));
        assert!(!channel_perms.contains(Permissions::PIN_MESSAGES)); // member overwrite ile engellendi
        assert!(channel_perms.contains(Permissions::BYPASS_SLOWMODE));

        // Sunucu sahibi kontrolü
        let owner_perms = Permissions::compute_channel_permissions(
            Permissions::empty(),
            true,
            Some(&everyone_ow),
            &[],
            None,
        );
        assert_eq!(owner_perms, Permissions::all());
    }

    #[test]
    fn test_bitwise_operations() {
        let p1 = Permissions::new(Permissions::SEND_MESSAGES);
        let p2 = Permissions::new(Permissions::ATTACH_FILES);
        let combined = p1 | p2;
        assert_eq!(
            combined.0,
            Permissions::SEND_MESSAGES | Permissions::ATTACH_FILES
        );

        let intersected = combined & p1;
        assert_eq!(intersected, p1);

        let mut toggled = Permissions::new(Permissions::SEND_MESSAGES);
        toggled.toggle(Permissions::SEND_MESSAGES);
        assert!(toggled.is_empty());
    }

    #[test]
    fn test_serde_serialization() {
        let perms = Permissions::new(Permissions::PIN_MESSAGES | Permissions::VIEW_CHANNEL);
        let json_str = serde_json::to_string(&perms).unwrap();
        // String formatında serialize edildiğini doğrula
        assert_eq!(json_str, format!("\"{}\"", perms.0));

        // Hem string hem integer formatından deserialize edilebilmeli
        let from_str: Permissions = serde_json::from_str(&json_str).unwrap();
        assert_eq!(from_str, perms);

        let from_num: Permissions = serde_json::from_str(&perms.0.to_string()).unwrap();
        assert_eq!(from_num, perms);
    }

    #[test]
    fn test_bot_invite_url() {
        let expected_perms = Permissions::MANAGE_ROOM_STAGE | Permissions::SEND_MESSAGES;
        let url = Permissions::bot_invite_url(123456789, expected_perms);
        assert!(url.contains("client_id=123456789"));
        assert!(url.contains("scope=bot"));
        assert!(url.contains(&format!("permissions={expected_perms}")));
    }
}
