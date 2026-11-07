//! Butonlar, Seçim Menüleri ve Modal Metin Girişleri (Components).
//!
//! Micup API kuralı: `disabled: false` veya boş seçenek dizileri `Invalid form body`
//! hatasına neden olur. `Component` serileştiricimiz bu hataları otomatik önler.

use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum ComponentType {
    ActionRow = 1,
    Button = 2,
    StringSelect = 3,
    TextInput = 4,
    UserSelect = 5,
    RoleSelect = 6,
    MentionableSelect = 7,
    ChannelSelect = 8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum ButtonStyle {
    Primary = 1,
    Secondary = 2,
    Success = 3,
    Danger = 4,
    Link = 5,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum TextInputStyle {
    Short = 1,
    Paragraph = 2,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SelectOption {
    pub label: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

impl SelectOption {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            description: None,
            default: None,
        }
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Component {
    #[serde(rename = "1")]
    ActionRow { components: Vec<Component> },
    #[serde(rename = "2")]
    Button {
        style: ButtonStyle,
        label: Option<String>,
        custom_id: Option<String>,
        url: Option<String>,
        #[serde(default)]
        disabled: bool,
    },
    #[serde(rename = "3")]
    StringSelect {
        custom_id: String,
        placeholder: Option<String>,
        min_values: Option<u64>,
        max_values: Option<u64>,
        options: Vec<SelectOption>,
        #[serde(default)]
        disabled: bool,
    },
    #[serde(rename = "4")]
    TextInput {
        custom_id: String,
        style: TextInputStyle,
        label: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        min_length: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max_length: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        required: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        placeholder: Option<String>,
    },
}

impl Serialize for Component {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::ActionRow { components } => {
                let mut state = serializer.serialize_struct("ActionRow", 2)?;
                state.serialize_field("type", &(ComponentType::ActionRow as u8))?;
                state.serialize_field("components", components)?;
                state.end()
            }
            Self::Button {
                style,
                label,
                custom_id,
                url,
                disabled,
            } => {
                let mut count = 2;
                if label.is_some() {
                    count += 1;
                }
                if custom_id.is_some() {
                    count += 1;
                }
                if url.is_some() {
                    count += 1;
                }
                if *disabled {
                    count += 1;
                }

                let mut state = serializer.serialize_struct("Button", count)?;
                state.serialize_field("type", &(ComponentType::Button as u8))?;
                state.serialize_field("style", &(*style as u8))?;
                if let Some(lbl) = label {
                    state.serialize_field("label", lbl)?;
                }
                if let Some(cid) = custom_id {
                    state.serialize_field("custom_id", cid)?;
                }
                if let Some(u) = url {
                    state.serialize_field("url", u)?;
                }
                // Micup Fluxer: Sadece disabled: true ise gönder, false ise atla!
                if *disabled {
                    state.serialize_field("disabled", &true)?;
                }
                state.end()
            }
            Self::StringSelect {
                custom_id,
                placeholder,
                min_values,
                max_values,
                options,
                disabled,
            } => {
                let mut count = 3;
                if placeholder.is_some() {
                    count += 1;
                }
                if min_values.is_some() {
                    count += 1;
                }
                if max_values.is_some() {
                    count += 1;
                }
                if *disabled {
                    count += 1;
                }

                let mut state = serializer.serialize_struct("StringSelect", count)?;
                state.serialize_field("type", &(ComponentType::StringSelect as u8))?;
                state.serialize_field("custom_id", custom_id)?;
                state.serialize_field("options", options)?;
                if let Some(p) = placeholder {
                    state.serialize_field("placeholder", p)?;
                }
                if let Some(min) = min_values {
                    state.serialize_field("min_values", min)?;
                }
                if let Some(max) = max_values {
                    state.serialize_field("max_values", max)?;
                }
                if *disabled {
                    state.serialize_field("disabled", &true)?;
                }
                state.end()
            }
            Self::TextInput {
                custom_id,
                style,
                label,
                min_length,
                max_length,
                required,
                value,
                placeholder,
            } => {
                let mut count = 3;
                if min_length.is_some() {
                    count += 1;
                }
                if max_length.is_some() {
                    count += 1;
                }
                if required.is_some() {
                    count += 1;
                }
                if value.is_some() {
                    count += 1;
                }
                if placeholder.is_some() {
                    count += 1;
                }

                let mut state = serializer.serialize_struct("TextInput", count)?;
                state.serialize_field("type", &(ComponentType::TextInput as u8))?;
                state.serialize_field("custom_id", custom_id)?;
                state.serialize_field("style", &(*style as u8))?;
                state.serialize_field("label", label)?;
                if let Some(min) = min_length {
                    state.serialize_field("min_length", min)?;
                }
                if let Some(max) = max_length {
                    state.serialize_field("max_length", max)?;
                }
                if let Some(req) = required {
                    state.serialize_field("required", req)?;
                }
                if let Some(val) = value {
                    state.serialize_field("value", val)?;
                }
                if let Some(ph) = placeholder {
                    state.serialize_field("placeholder", ph)?;
                }
                state.end()
            }
        }
    }
}

impl Component {
    pub fn action_row(components: Vec<Component>) -> Self {
        Self::ActionRow { components }
    }

    pub fn button(
        style: ButtonStyle,
        label: impl Into<String>,
        custom_id: impl Into<String>,
    ) -> Self {
        Self::Button {
            style,
            label: Some(label.into()),
            custom_id: Some(custom_id.into()),
            url: None,
            disabled: false,
        }
    }

    pub fn link_button(label: impl Into<String>, url: impl Into<String>) -> Self {
        Self::Button {
            style: ButtonStyle::Link,
            label: Some(label.into()),
            custom_id: None,
            url: Some(url.into()),
            disabled: false,
        }
    }

    pub fn string_select(
        custom_id: impl Into<String>,
        placeholder: Option<impl Into<String>>,
        options: Vec<SelectOption>,
    ) -> Self {
        Self::StringSelect {
            custom_id: custom_id.into(),
            placeholder: placeholder.map(Into::into),
            min_values: None,
            max_values: None,
            options,
            disabled: false,
        }
    }

    pub fn text_input(
        custom_id: impl Into<String>,
        style: TextInputStyle,
        label: impl Into<String>,
    ) -> Self {
        Self::TextInput {
            custom_id: custom_id.into(),
            style,
            label: label.into(),
            min_length: None,
            max_length: None,
            required: None,
            value: None,
            placeholder: None,
        }
    }
}
