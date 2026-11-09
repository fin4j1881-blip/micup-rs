//! Discord ve Micup API kısıtlamalarına tam uyumlu, otomatik sanitasyonlu ve CDN entegrasyonlu Embed oluşturucu.

use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize};

/// Eksik veya null dönebilecek alanları türün varsayılan değerine eşleyen toleranslı deserializer.
pub fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

/// CDN URL'lerini `media.micup.gg` tabanına normalize eder.
pub fn normalize_embed_url(url: &str) -> String {
    let trimmed = url.trim();
    if let Some(rest) = trimmed.strip_prefix("https://cdn.discordapp.com") {
        format!("https://media.micup.gg{rest}")
    } else if let Some(rest) = trimmed.strip_prefix("http://cdn.discordapp.com") {
        format!("https://media.micup.gg{rest}")
    } else {
        trimmed.to_string()
    }
}

fn sanitize_embed_field_str(s: &str, max_len: usize) -> String {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return "-".to_string();
    }
    let count = trimmed.chars().count();
    if count <= max_len {
        return trimmed.to_string();
    }
    let mut out: String = trimmed.chars().take(max_len.saturating_sub(3)).collect();
    out.push_str("...");
    out
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct EmbedField {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub value: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub inline: bool,
}

impl EmbedField {
    pub fn new(name: impl Into<String>, value: impl Into<String>, inline: bool) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            inline,
        }
    }
}

impl Serialize for EmbedField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Discord / Micup kuralı: name maks 256, value maks 1024 karakter
        let safe_name = sanitize_embed_field_str(&self.name, 256);
        let safe_value = sanitize_embed_field_str(&self.value, 1024);
        let mut state = serializer.serialize_struct("EmbedField", 3)?;
        state.serialize_field("name", &safe_name)?;
        state.serialize_field("value", &safe_value)?;
        state.serialize_field("inline", &self.inline)?;
        state.end()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct EmbedImage {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub proxy_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub width: Option<u32>,
}

impl EmbedImage {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: normalize_embed_url(&url.into()),
            proxy_url: None,
            height: None,
            width: None,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct EmbedFooter {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub proxy_icon_url: Option<String>,
}

impl EmbedFooter {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            icon_url: None,
            proxy_icon_url: None,
        }
    }

    pub fn with_icon(text: impl Into<String>, icon_url: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            icon_url: Some(normalize_embed_url(&icon_url.into())),
            proxy_icon_url: None,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct EmbedAuthor {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub proxy_icon_url: Option<String>,
}

impl EmbedAuthor {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            url: None,
            icon_url: None,
            proxy_icon_url: None,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Embed {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub color: Option<u32>,
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        default,
        deserialize_with = "deserialize_null_default"
    )]
    pub fields: Vec<EmbedField>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub image: Option<EmbedImage>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub thumbnail: Option<EmbedImage>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub footer: Option<EmbedFooter>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub author: Option<EmbedAuthor>,
}

/// Çift katmanlı API uyumluluğu için `Embed` ile eş anlamlı EmbedBuilder tür tanımı.
pub type EmbedBuilder = Embed;

impl Embed {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> Self {
        Self::default()
    }

    pub fn build(self) -> Self {
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn timestamp(mut self, ts: impl Into<String>) -> Self {
        self.timestamp = Some(ts.into());
        self
    }

    pub fn color(mut self, color: u32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn field(
        mut self,
        name: impl Into<String>,
        value: impl Into<String>,
        inline: bool,
    ) -> Self {
        self.fields.push(EmbedField {
            name: name.into(),
            value: value.into(),
            inline,
        });
        self
    }

    pub fn fields(mut self, fields: impl IntoIterator<Item = EmbedField>) -> Self {
        self.fields.extend(fields);
        self
    }

    pub fn image(mut self, url: impl Into<String>) -> Self {
        let u = normalize_embed_url(&url.into());
        self.image = Some(EmbedImage {
            url: u,
            proxy_url: None,
            height: None,
            width: None,
        });
        self
    }

    pub fn thumbnail(mut self, url: impl Into<String>) -> Self {
        let u = normalize_embed_url(&url.into());
        self.thumbnail = Some(EmbedImage {
            url: u,
            proxy_url: None,
            height: None,
            width: None,
        });
        self
    }

    pub fn footer(mut self, text: impl Into<String>, icon_url: Option<String>) -> Self {
        let icon = icon_url.map(|u| normalize_embed_url(&u));
        self.footer = Some(EmbedFooter {
            text: text.into(),
            icon_url: icon,
            proxy_icon_url: None,
        });
        self
    }

    pub fn author(
        mut self,
        name: impl Into<String>,
        url: Option<String>,
        icon_url: Option<String>,
    ) -> Self {
        let icon = icon_url.map(|u| normalize_embed_url(&u));
        self.author = Some(EmbedAuthor {
            name: name.into(),
            url,
            icon_url: icon,
            proxy_icon_url: None,
        });
        self
    }

    /// Görsel olarak bir Micup ek dosya (attachment) CDN bağlantısı ekler.
    pub fn attachment_image(
        mut self,
        channel_id: u64,
        attachment_id: u64,
        filename: impl AsRef<str>,
    ) -> Self {
        let url = crate::cdn::attachment_url(channel_id, attachment_id, filename.as_ref());
        self.image = Some(EmbedImage {
            url,
            proxy_url: None,
            height: None,
            width: None,
        });
        self
    }

    /// Küçük resim (thumbnail) olarak bir Micup ek dosya (attachment) CDN bağlantısı ekler.
    pub fn attachment_thumbnail(
        mut self,
        channel_id: u64,
        attachment_id: u64,
        filename: impl AsRef<str>,
    ) -> Self {
        let url = crate::cdn::attachment_url(channel_id, attachment_id, filename.as_ref());
        self.thumbnail = Some(EmbedImage {
            url,
            proxy_url: None,
            height: None,
            width: None,
        });
        self
    }

    /// Küçük resim (thumbnail) olarak bir kullanıcı avatarı CDN bağlantısı ekler.
    pub fn avatar_thumbnail(
        mut self,
        user_id: u64,
        avatar_hash: Option<&str>,
        discriminator_modulo: u64,
    ) -> Self {
        let url = crate::cdn::avatar_url(user_id, avatar_hash, discriminator_modulo);
        self.thumbnail = Some(EmbedImage {
            url,
            proxy_url: None,
            height: None,
            width: None,
        });
        self
    }

    /// Küçük resim (thumbnail) olarak bir sunucu simgesi (icon) CDN bağlantısı ekler.
    pub fn icon_thumbnail(mut self, guild_id: u64, icon_hash: impl AsRef<str>) -> Self {
        let url = crate::cdn::guild_icon_url(guild_id, icon_hash.as_ref());
        self.thumbnail = Some(EmbedImage {
            url,
            proxy_url: None,
            height: None,
            width: None,
        });
        self
    }

    /// Embed içindeki görsel veya küçük resim bağlantısının süresinin dolup dolmadığını denetler.
    pub fn is_attachment_expired(&self) -> bool {
        if let Some(ref img) = self.image {
            if crate::cdn::is_attachment_url_expired(&img.url) {
                return true;
            }
        }
        if let Some(ref thumb) = self.thumbnail {
            if crate::cdn::is_attachment_url_expired(&thumb.url) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embed_builder_chaining_and_build() {
        let embed = EmbedBuilder::new()
            .title("Başlık")
            .description("Açıklama")
            .color(0x7c5cff)
            .url("https://micup.gg")
            .timestamp("2026-11-09T00:00:00Z")
            .field("Alan 1", "Değer 1", true)
            .fields(vec![EmbedField::new("Alan 2", "Değer 2", false)])
            .image("https://cdn.discordapp.com/attachments/1/2/test.png")
            .thumbnail("https://cdn.discordapp.com/avatars/1/abc.png")
            .footer(
                "Altbilgi",
                Some("https://cdn.discordapp.com/icons/1/ic.png".into()),
            )
            .author("Yazar", Some("https://micup.gg".into()), None)
            .build();

        assert_eq!(embed.title.as_deref(), Some("Başlık"));
        assert_eq!(embed.description.as_deref(), Some("Açıklama"));
        assert_eq!(embed.color, Some(0x7c5cff));
        assert_eq!(embed.fields.len(), 2);
        assert_eq!(
            embed.image.as_ref().map(|i| i.url.as_str()),
            Some("https://media.micup.gg/attachments/1/2/test.png")
        );
        assert_eq!(
            embed.thumbnail.as_ref().map(|t| t.url.as_str()),
            Some("https://media.micup.gg/avatars/1/abc.png")
        );
        assert_eq!(
            embed.footer.as_ref().and_then(|f| f.icon_url.as_deref()),
            Some("https://media.micup.gg/icons/1/ic.png")
        );
    }

    #[test]
    fn test_tolerant_null_deserialization() {
        let json_str = r#"{
            "title": null,
            "description": null,
            "url": null,
            "timestamp": null,
            "color": null,
            "fields": null,
            "image": null,
            "thumbnail": null,
            "footer": null,
            "author": null
        }"#;

        let embed: Embed =
            serde_json::from_str(json_str).expect("Tüm alanlar null iken başarılı ayrıştırılmalı");
        assert!(embed.title.is_none());
        assert!(embed.description.is_none());
        assert!(embed.fields.is_empty());
        assert!(embed.image.is_none());
        assert!(embed.thumbnail.is_none());
        assert!(embed.footer.is_none());
    }

    #[test]
    fn test_tolerant_field_null_deserialization() {
        let json_str = r#"{
            "name": null,
            "value": null,
            "inline": null
        }"#;

        let field: EmbedField =
            serde_json::from_str(json_str).expect("EmbedField null alanları tolere etmeli");
        assert_eq!(field.name, "");
        assert_eq!(field.value, "");
        assert!(!field.inline);
    }

    #[test]
    fn test_cdn_helpers_and_expiration() {
        let embed = Embed::new()
            .attachment_image(10, 20, "resim.png")
            .avatar_thumbnail(12345, Some("hash123"), 0);

        assert_eq!(
            embed.image.as_ref().unwrap().url,
            "https://media.micup.gg/attachments/10/20/resim.png"
        );
        assert_eq!(
            embed.thumbnail.as_ref().unwrap().url,
            "https://media.micup.gg/avatars/12345/hash123.png?size=4096"
        );
        assert!(!embed.is_attachment_expired());
    }
}
