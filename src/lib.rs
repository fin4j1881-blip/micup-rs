//! Micup.gg / Fluxer platformu için resmi olmayan, Micup API ve CDN uyumlu Rust kütüphanesi.
//!
//! # Neden micup-rs?
//! Standart Discord kütüphaneleri (serenity, twilight vb.) Micup üzerinde kullanılırken:
//! - CDN istekleri `cdn.discordapp.com`'a gider ve görseller kırılır.
//! - Micup'ın dönmediği `preferred_locale` ve takviye çubuğu gibi alanlar yüzünden `Guild` modelleri çöker.
//! - Boş buton veya menü dizilerinde API `Invalid form body` hatası verir.
//! - Embed alanlarında 1024 karakter ve 256 başlık sınırı manuel yönetilmezse hata alınır.
//!
//! `micup-rs`, tüm bu platform uyumsuzluklarını dahili olarak çözer.

pub mod cdn;
pub mod client;
pub mod embed;
pub mod error;
pub mod gateway;
pub mod models;

pub use client::Client;
pub use embed::{Embed, EmbedField};
pub use error::MicupError;
pub use gateway::Shard;
pub use models::{Channel, Guild, Id, Member, Role, User};

pub mod prelude {
    pub use crate::cdn;
    pub use crate::client::Client;
    pub use crate::embed::Embed;
    pub use crate::error::MicupError;
    pub use crate::gateway::Shard;
    pub use crate::models::{Channel, Guild, Id, Member, Role, User};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdn_urls() {
        let avatar = cdn::avatar_url(1566954479021461510, Some("a313e966"), 0);
        assert_eq!(
            avatar,
            "https://media.micup.gg/avatars/1566954479021461510/a313e966.png?size=4096"
        );

        let default_avatar = cdn::avatar_url(123, None, 2);
        assert_eq!(default_avatar, "https://media.micup.gg/embed/avatars/2.png");

        let icon = cdn::guild_icon_url(1567986170364166147, "cccae7b5");
        assert_eq!(
            icon,
            "https://media.micup.gg/icons/1567986170364166147/cccae7b5.png?size=4096"
        );
    }

    #[test]
    fn test_embed_field_sanitizer() {
        let long_val = "x".repeat(1500);
        let embed = Embed::new().field("Başlık", long_val, false);
        let json = serde_json::to_value(&embed).unwrap();
        let field_val = json["fields"][0]["value"].as_str().unwrap();
        assert_eq!(field_val.chars().count(), 1024);
        assert!(field_val.ends_with("..."));
    }

    #[test]
    fn test_guild_deserialization_with_missing_fields() {
        // Micup'ın döndüğü gerçek payload
        let raw = r#"{
            "id": "1567986170364166147",
            "name": "Finmos Development",
            "icon": "cccae7b5",
            "owner_id": "1566954479021461510",
            "roles": [],
            "emojis": [],
            "features": []
        }"#;

        let guild: Guild = serde_json::from_str(raw).expect("Micup Guild parse edilmeli");
        assert_eq!(guild.id.get(), 1567986170364166147);
        assert_eq!(guild.name, "Finmos Development");
        assert!(guild.preferred_locale.is_none());
        assert!(guild.premium_progress_bar_enabled.is_none());
    }
}
