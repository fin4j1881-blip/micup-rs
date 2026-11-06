//! Discord ve Micup API kısıtlamalarına tam uyumlu, otomatik sanitasyonlu Embed oluşturucu.

use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EmbedImage {
    pub url: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EmbedFooter {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Embed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<u32>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub fields: Vec<EmbedField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<EmbedImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<EmbedImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<EmbedFooter>,
}

impl Embed {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn color(mut self, color: u32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn field(mut self, name: impl Into<String>, value: impl Into<String>, inline: bool) -> Self {
        self.fields.push(EmbedField {
            name: name.into(),
            value: value.into(),
            inline,
        });
        self
    }

    pub fn image(mut self, url: impl Into<String>) -> Self {
        self.image = Some(EmbedImage { url: url.into() });
        self
    }

    pub fn thumbnail(mut self, url: impl Into<String>) -> Self {
        self.thumbnail = Some(EmbedImage { url: url.into() });
        self
    }

    pub fn footer(mut self, text: impl Into<String>, icon_url: Option<String>) -> Self {
        self.footer = Some(EmbedFooter {
            text: text.into(),
            icon_url,
        });
        self
    }
}
