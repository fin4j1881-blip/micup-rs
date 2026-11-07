//! Micup.gg resmi medya CDN adresi (`https://media.micup.gg`) yardımcıları.

use crate::models::constants::CDN_BASE;

pub fn media_url(category: &str, id: u64, hash: &str, size: Option<u32>) -> String {
    let clean = hash.trim();
    let ext = if clean.starts_with("a_") {
        "gif"
    } else {
        "png"
    };
    let size_param = size.map(|s| format!("?size={s}")).unwrap_or_default();
    format!("{CDN_BASE}/{category}/{id}/{clean}.{ext}{size_param}")
}

/// Kullanıcı avatarı bağlantısını üretir.
///
/// Özel avatar yoksa Micup'ın 6 varsayılan avatarından biri (`0-5.png`) döner.
pub fn avatar_url(user_id: u64, avatar_hash: Option<&str>, discriminator_modulo: u64) -> String {
    if let Some(hash) = avatar_hash {
        media_url("avatars", user_id, hash, Some(4096))
    } else {
        default_avatar_url(discriminator_modulo)
    }
}

/// Varsayılan Micup avatar URL'si (`embed/avatars/0.png` .. `embed/avatars/5.png`).
pub fn default_avatar_url(index: u64) -> String {
    format!("{CDN_BASE}/embed/avatars/{}.png", index % 6)
}

pub fn guild_icon_url(guild_id: u64, icon_hash: &str) -> String {
    media_url("icons", guild_id, icon_hash, Some(4096))
}

pub fn guild_banner_url(guild_id: u64, banner_hash: &str) -> String {
    media_url("banners", guild_id, banner_hash, Some(4096))
}

pub fn splash_url(guild_id: u64, splash_hash: &str) -> String {
    media_url("splashes", guild_id, splash_hash, Some(4096))
}

pub fn banner_url(user_id: u64, banner_hash: &str) -> String {
    media_url("banners", user_id, banner_hash, Some(4096))
}

pub fn member_banner_url(guild_id: u64, user_id: u64, banner_hash: &str) -> String {
    let clean = banner_hash.trim();
    let ext = if clean.starts_with("a_") {
        "gif"
    } else {
        "png"
    };
    format!("{CDN_BASE}/guilds/{guild_id}/users/{user_id}/banners/{clean}.{ext}?size=4096")
}

/// Özel emoji CDN bağlantısı.
pub fn emoji_url(emoji_id: u64, animated: bool) -> String {
    let ext = if animated { "gif" } else { "png" };
    format!("{CDN_BASE}/emojis/{emoji_id}.{ext}")
}

/// Çıkartma (Sticker) CDN bağlantısı.
pub fn sticker_url(sticker_id: u64) -> String {
    format!("{CDN_BASE}/stickers/{sticker_id}.png")
}

/// Micup marka görselleri CDN bağlantısı (`branding/{category}/{hash}.png`).
pub fn branding_url(category: u64, hash: &str) -> String {
    format!("{CDN_BASE}/branding/{category}/{hash}.png")
}
