//! Micup.gg resmi medya CDN adresi (`https://media.micup.gg`) yardımcıları.

use crate::models::constants::CDN_BASE;

/// Ek dosya bağlantılarının varsayılan geçerlilik süresi: 24 saat (86.400 saniye).
pub const ATTACHMENT_TIMEOUT_SECS: u64 = 86_400;

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

/// Sunucu simgesi (icon) bağlantısı için kısayol.
pub fn icon_url(guild_id: u64, icon_hash: &str) -> String {
    guild_icon_url(guild_id, icon_hash)
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

/// Ek dosya (Attachment) CDN bağlantısını üretir (`attachments/{channel_id}/{attachment_id}/{filename}`).
pub fn attachment_url(channel_id: u64, attachment_id: u64, filename: &str) -> String {
    let clean = filename.trim();
    format!("{CDN_BASE}/attachments/{channel_id}/{attachment_id}/{clean}")
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

/// İmzalı ek dosya bağlantısından son kullanma Unix zaman damgasını (saniye) güvenli bir şekilde ayrıştırır.
///
/// `ex` (son kullanma tarihi, hex Unix timestamp) veya `is` (oluşturulma tarihi + 86.400 sn)
/// parametresi okunur. URL'de geçerli bir zaman damgası bulunamazsa `None` döner.
pub fn attachment_expiration_timestamp(url: &str) -> Option<u64> {
    let query_part = if let Some((_, q)) = url.split_once('?') {
        q
    } else if !url.contains('/') && url.contains('=') {
        url
    } else {
        return None;
    };
    let query = query_part.split('#').next().unwrap_or(query_part);

    let mut ex_ts = None;
    let mut is_ts = None;

    for param in query.split('&') {
        if let Some((key, val)) = param.split_once('=') {
            let clean_val = val.trim();
            if key == "ex" {
                let hex_str = clean_val
                    .strip_prefix("0x")
                    .or_else(|| clean_val.strip_prefix("0X"))
                    .unwrap_or(clean_val);
                if let Ok(ts) = u64::from_str_radix(hex_str, 16) {
                    ex_ts = Some(ts);
                }
            } else if key == "is" {
                let hex_str = clean_val
                    .strip_prefix("0x")
                    .or_else(|| clean_val.strip_prefix("0X"))
                    .unwrap_or(clean_val);
                if let Ok(ts) = u64::from_str_radix(hex_str, 16) {
                    is_ts = Some(ts);
                }
            }
        }
    }

    if let Some(ts) = ex_ts {
        Some(ts)
    } else {
        is_ts.map(|issued| issued.saturating_add(ATTACHMENT_TIMEOUT_SECS))
    }
}

/// İmzalı ek dosya bağlantısının süresinin dolup dolmadığını denetler (`ex` veya `is` parametresi).
///
/// Micup ve Fluxer platformunda ek dosya bağlantıları 24 saat sonra zaman aşımına uğrar.
/// `ex` (son kullanma tarihi, hex Unix timestamp) veya `is` (oluşturulma tarihi + 86400 sn)
/// parametresi Unix epoch saniyesi ile karşılaştırılır. URL geçersiz veya zaman damgasızsa `false` döner.
pub fn is_attachment_url_expired(url: &str) -> bool {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    match attachment_expiration_timestamp(url) {
        Some(exp) => now >= exp,
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_urls_use_micup_cdn() {
        assert!(avatar_url(1, Some("abc"), 0).starts_with("https://media.micup.gg"));
        assert!(banner_url(1, "abc").starts_with("https://media.micup.gg"));
        assert!(icon_url(1, "abc").starts_with("https://media.micup.gg"));
        assert!(attachment_url(1, 2, "file.png").starts_with("https://media.micup.gg"));
        assert!(guild_banner_url(1, "abc").starts_with("https://media.micup.gg"));
        assert!(splash_url(1, "abc").starts_with("https://media.micup.gg"));
        assert!(member_banner_url(1, 2, "abc").starts_with("https://media.micup.gg"));
        assert!(emoji_url(1, false).starts_with("https://media.micup.gg"));
        assert!(emoji_url(1, true).starts_with("https://media.micup.gg"));
        assert!(sticker_url(1).starts_with("https://media.micup.gg"));
        assert!(branding_url(1, "abc").starts_with("https://media.micup.gg"));
    }

    #[test]
    fn test_default_avatar_urls() {
        for i in 0..6 {
            assert_eq!(
                default_avatar_url(i),
                format!("https://media.micup.gg/embed/avatars/{i}.png")
            );
        }
        // Modulo 6 döngüsü (6 -> 0, 7 -> 1, vb.)
        assert_eq!(
            default_avatar_url(6),
            "https://media.micup.gg/embed/avatars/0.png"
        );
        assert_eq!(
            default_avatar_url(11),
            "https://media.micup.gg/embed/avatars/5.png"
        );

        // avatar_url varsayılan avatar durumunda default_avatar_url'e başvurur
        assert_eq!(
            avatar_url(42, None, 3),
            "https://media.micup.gg/embed/avatars/3.png"
        );
        assert_eq!(
            avatar_url(42, None, 9),
            "https://media.micup.gg/embed/avatars/3.png"
        );
    }

    #[test]
    fn test_animated_media_urls() {
        let gif_avatar = avatar_url(1, Some("a_anim123"), 0);
        assert!(gif_avatar.ends_with(".gif?size=4096"));

        let static_avatar = avatar_url(1, Some("stat123"), 0);
        assert!(static_avatar.ends_with(".png?size=4096"));

        let gif_emoji = emoji_url(123, true);
        assert!(gif_emoji.ends_with(".gif"));

        let static_emoji = emoji_url(123, false);
        assert!(static_emoji.ends_with(".png"));
    }

    #[test]
    fn test_attachment_url_expiration() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Henüz süresi dolmamış
        let future_ex = format!(
            "https://media.micup.gg/attachments/1/2/file.png?ex={:x}",
            now + 3600
        );
        assert!(!is_attachment_url_expired(&future_ex));

        // Süresi dolmuş
        let expired_ex = format!(
            "https://media.micup.gg/attachments/1/2/file.png?ex={:x}",
            now - 10
        );
        assert!(is_attachment_url_expired(&expired_ex));

        // 24 saat kuralı (is parametresi)
        let recent_issued = format!(
            "https://media.micup.gg/attachments/1/2/file.png?is={:x}",
            now - 3600
        );
        assert!(!is_attachment_url_expired(&recent_issued));

        let expired_issued = format!(
            "https://media.micup.gg/attachments/1/2/file.png?is={:x}",
            now - 86_500
        );
        assert!(is_attachment_url_expired(&expired_issued));
    }

    #[test]
    fn test_attachment_expiration_timestamp_parsing() {
        let ts = attachment_expiration_timestamp(
            "https://media.micup.gg/attachments/1/2/file.png?ex=65e1234a",
        );
        assert_eq!(ts, Some(0x65e1234a));

        let ts_with_0x = attachment_expiration_timestamp(
            "https://media.micup.gg/attachments/1/2/file.png?ex=0x65e1234a",
        );
        assert_eq!(ts_with_0x, Some(0x65e1234a));

        let ts_is = attachment_expiration_timestamp(
            "https://media.micup.gg/attachments/1/2/file.png?is=65e1234a",
        );
        assert_eq!(ts_is, Some(0x65e1234a + ATTACHMENT_TIMEOUT_SECS));

        let ts_none =
            attachment_expiration_timestamp("https://media.micup.gg/attachments/1/2/file.png");
        assert_eq!(ts_none, None);
    }

    #[test]
    fn test_attachment_url_malformed_and_edge_cases() {
        assert!(!is_attachment_url_expired(
            "https://media.micup.gg/attachments/1/2/file.png"
        ));
        assert!(!is_attachment_url_expired(
            "https://media.micup.gg/attachments/1/2/file.png?index=123"
        ));
        assert!(!is_attachment_url_expired(
            "https://media.micup.gg/attachments/1/2/file.png?ex=invalid_hex"
        ));
        assert!(!is_attachment_url_expired(""));
    }
}
