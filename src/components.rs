//! Butonlar, Seçim Menüleri ve Modal Metin Girişleri (Components).
//!
//! Micup API kuralı: `disabled: false` veya boş seçenek dizileri `Invalid form body`
//! hatasına neden olur. `Component` serileştiricimiz bu hataları otomatik önler.

use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};

/// Serde için `null` gelen alanları varsayılan (`Default`) değerine dönüştüren yardımcı işlev.
fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

impl Default for ComponentType {
    fn default() -> Self {
        Self::ActionRow
    }
}

impl Serialize for ComponentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for ComponentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ComponentTypeVisitor;

        impl<'de> serde::de::Visitor<'de> for ComponentTypeVisitor {
            type Value = ComponentType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("1-8 arası bir bileşen türü tamsayısı veya tür adı")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    1 => Ok(ComponentType::ActionRow),
                    2 => Ok(ComponentType::Button),
                    3 => Ok(ComponentType::StringSelect),
                    4 => Ok(ComponentType::TextInput),
                    5 => Ok(ComponentType::UserSelect),
                    6 => Ok(ComponentType::RoleSelect),
                    7 => Ok(ComponentType::MentionableSelect),
                    8 => Ok(ComponentType::ChannelSelect),
                    _ => Ok(ComponentType::ActionRow),
                }
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_u64(v as u64)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v.to_ascii_lowercase().as_str() {
                    "actionrow" | "action_row" | "1" => Ok(ComponentType::ActionRow),
                    "button" | "2" => Ok(ComponentType::Button),
                    "stringselect" | "string_select" | "3" => Ok(ComponentType::StringSelect),
                    "textinput" | "text_input" | "4" => Ok(ComponentType::TextInput),
                    "userselect" | "user_select" | "5" => Ok(ComponentType::UserSelect),
                    "roleselect" | "role_select" | "6" => Ok(ComponentType::RoleSelect),
                    "mentionableselect" | "mentionable_select" | "7" => {
                        Ok(ComponentType::MentionableSelect)
                    }
                    "channelselect" | "channel_select" | "8" => Ok(ComponentType::ChannelSelect),
                    _ => Ok(ComponentType::ActionRow),
                }
            }
        }

        deserializer.deserialize_any(ComponentTypeVisitor)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ButtonStyle {
    Primary = 1,
    Secondary = 2,
    Success = 3,
    Danger = 4,
    Link = 5,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self::Primary
    }
}

impl Serialize for ButtonStyle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for ButtonStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ButtonStyleVisitor;

        impl<'de> serde::de::Visitor<'de> for ButtonStyleVisitor {
            type Value = ButtonStyle;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("1-5 arası bir tamsayı veya stil adı")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    1 => Ok(ButtonStyle::Primary),
                    2 => Ok(ButtonStyle::Secondary),
                    3 => Ok(ButtonStyle::Success),
                    4 => Ok(ButtonStyle::Danger),
                    5 => Ok(ButtonStyle::Link),
                    _ => Ok(ButtonStyle::Primary),
                }
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_u64(v as u64)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v.to_ascii_lowercase().as_str() {
                    "primary" | "1" => Ok(ButtonStyle::Primary),
                    "secondary" | "2" => Ok(ButtonStyle::Secondary),
                    "success" | "3" => Ok(ButtonStyle::Success),
                    "danger" | "4" => Ok(ButtonStyle::Danger),
                    "link" | "5" => Ok(ButtonStyle::Link),
                    _ => Ok(ButtonStyle::Primary),
                }
            }
        }

        deserializer.deserialize_any(ButtonStyleVisitor)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TextInputStyle {
    Short = 1,
    Paragraph = 2,
}

impl Default for TextInputStyle {
    fn default() -> Self {
        Self::Short
    }
}

impl Serialize for TextInputStyle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for TextInputStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TextInputStyleVisitor;

        impl<'de> serde::de::Visitor<'de> for TextInputStyleVisitor {
            type Value = TextInputStyle;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("1-2 arası bir tamsayı veya metin girişi stil adı")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    1 => Ok(TextInputStyle::Short),
                    2 => Ok(TextInputStyle::Paragraph),
                    _ => Ok(TextInputStyle::Short),
                }
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_u64(v as u64)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v.to_ascii_lowercase().as_str() {
                    "short" | "1" => Ok(TextInputStyle::Short),
                    "paragraph" | "2" => Ok(TextInputStyle::Paragraph),
                    _ => Ok(TextInputStyle::Short),
                }
            }
        }

        deserializer.deserialize_any(TextInputStyleVisitor)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SelectOption {
    #[serde(deserialize_with = "deserialize_null_default")]
    pub label: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
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

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn default(mut self, default: bool) -> Self {
        self.default = Some(default);
        self
    }
}

/// Etkileşim butonu yapısı.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Button {
    pub style: ButtonStyle,
    pub label: Option<String>,
    pub custom_id: Option<String>,
    pub url: Option<String>,
    pub disabled: bool,
}

impl Button {
    pub fn new(style: ButtonStyle) -> Self {
        Self {
            style,
            label: None,
            custom_id: None,
            url: None,
            disabled: false,
        }
    }

    pub fn primary(label: impl Into<String>, custom_id: impl Into<String>) -> Self {
        Self {
            style: ButtonStyle::Primary,
            label: Some(label.into()),
            custom_id: Some(custom_id.into()),
            url: None,
            disabled: false,
        }
    }

    pub fn secondary(label: impl Into<String>, custom_id: impl Into<String>) -> Self {
        Self {
            style: ButtonStyle::Secondary,
            label: Some(label.into()),
            custom_id: Some(custom_id.into()),
            url: None,
            disabled: false,
        }
    }

    pub fn success(label: impl Into<String>, custom_id: impl Into<String>) -> Self {
        Self {
            style: ButtonStyle::Success,
            label: Some(label.into()),
            custom_id: Some(custom_id.into()),
            url: None,
            disabled: false,
        }
    }

    pub fn danger(label: impl Into<String>, custom_id: impl Into<String>) -> Self {
        Self {
            style: ButtonStyle::Danger,
            label: Some(label.into()),
            custom_id: Some(custom_id.into()),
            url: None,
            disabled: false,
        }
    }

    pub fn link(label: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            style: ButtonStyle::Link,
            label: Some(label.into()),
            custom_id: None,
            url: Some(url.into()),
            disabled: false,
        }
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn custom_id(mut self, custom_id: impl Into<String>) -> Self {
        self.custom_id = Some(custom_id.into());
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn build(self) -> Component {
        Component::from(self)
    }

    pub fn into_component(self) -> Component {
        self.build()
    }

    pub fn into_row(self) -> ComponentRow {
        ComponentRow::new().add_button(self)
    }

    pub fn into_action_row(self) -> Component {
        Component::action_row(vec![self.build()])
    }
}

impl Serialize for Button {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut count = 2;
        if self.label.is_some() {
            count += 1;
        }
        if self.custom_id.is_some() {
            count += 1;
        }
        if self.url.is_some() {
            count += 1;
        }
        if self.disabled {
            count += 1;
        }

        let mut state = serializer.serialize_struct("Button", count)?;
        state.serialize_field("type", &(ComponentType::Button as u8))?;
        state.serialize_field("style", &(self.style as u8))?;
        if let Some(lbl) = &self.label {
            state.serialize_field("label", lbl)?;
        }
        if let Some(cid) = &self.custom_id {
            state.serialize_field("custom_id", cid)?;
        }
        if let Some(u) = &self.url {
            state.serialize_field("url", u)?;
        }
        if self.disabled {
            state.serialize_field("disabled", &true)?;
        }
        state.end()
    }
}

#[derive(Deserialize)]
struct ButtonHelper {
    #[serde(rename = "type", default)]
    _type: Option<serde_json::Value>,
    #[serde(default)]
    style: Option<ButtonStyle>,
    #[serde(default)]
    label: Option<String>,
    #[serde(default)]
    custom_id: Option<String>,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    disabled: Option<bool>,
}

impl<'de> Deserialize<'de> for Button {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let h = ButtonHelper::deserialize(deserializer)?;
        Ok(Self {
            style: h.style.unwrap_or(ButtonStyle::Primary),
            label: h.label,
            custom_id: h.custom_id,
            url: h.url,
            disabled: h.disabled.unwrap_or(false),
        })
    }
}

/// Seçim menüsü bileşeni.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SelectMenu {
    pub custom_id: String,
    pub placeholder: Option<String>,
    pub min_values: Option<u64>,
    pub max_values: Option<u64>,
    pub options: Vec<SelectOption>,
    pub disabled: bool,
}

impl SelectMenu {
    pub fn new(custom_id: impl Into<String>) -> Self {
        Self {
            custom_id: custom_id.into(),
            placeholder: None,
            min_values: None,
            max_values: None,
            options: Vec::new(),
            disabled: false,
        }
    }

    pub fn string(custom_id: impl Into<String>, options: Vec<SelectOption>) -> Self {
        Self {
            custom_id: custom_id.into(),
            placeholder: None,
            min_values: None,
            max_values: None,
            options,
            disabled: false,
        }
    }

    pub fn custom_id(mut self, custom_id: impl Into<String>) -> Self {
        self.custom_id = custom_id.into();
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn min_values(mut self, min: u64) -> Self {
        self.min_values = Some(min);
        self
    }

    pub fn max_values(mut self, max: u64) -> Self {
        self.max_values = Some(max);
        self
    }

    pub fn options(mut self, options: Vec<SelectOption>) -> Self {
        self.options = options;
        self
    }

    pub fn option(mut self, option: SelectOption) -> Self {
        self.options.push(option);
        self
    }

    pub fn add_option(
        mut self,
        label: impl Into<String>,
        value: impl Into<String>,
        description: Option<impl Into<String>>,
    ) -> Self {
        let mut opt = SelectOption::new(label, value);
        if let Some(desc) = description {
            opt = opt.description(desc);
        }
        self.options.push(opt);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn build(self) -> Component {
        Component::from(self)
    }

    pub fn into_component(self) -> Component {
        self.build()
    }

    pub fn into_row(self) -> ComponentRow {
        ComponentRow::new().add_select_menu(self)
    }

    pub fn into_action_row(self) -> Component {
        Component::action_row(vec![self.build()])
    }
}

impl Serialize for SelectMenu {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut count = 2;
        if !self.options.is_empty() {
            count += 1;
        }
        if self.placeholder.is_some() {
            count += 1;
        }
        if self.min_values.is_some() {
            count += 1;
        }
        if self.max_values.is_some() {
            count += 1;
        }
        if self.disabled {
            count += 1;
        }

        let mut state = serializer.serialize_struct("SelectMenu", count)?;
        state.serialize_field("type", &(ComponentType::StringSelect as u8))?;
        state.serialize_field("custom_id", &self.custom_id)?;
        if !self.options.is_empty() {
            state.serialize_field("options", &self.options)?;
        }
        if let Some(p) = &self.placeholder {
            state.serialize_field("placeholder", p)?;
        }
        if let Some(min) = self.min_values {
            state.serialize_field("min_values", &min)?;
        }
        if let Some(max) = self.max_values {
            state.serialize_field("max_values", &max)?;
        }
        if self.disabled {
            state.serialize_field("disabled", &true)?;
        }
        state.end()
    }
}

#[derive(Deserialize)]
struct SelectMenuHelper {
    #[serde(rename = "type", default)]
    _type: Option<serde_json::Value>,
    #[serde(default)]
    custom_id: Option<String>,
    #[serde(default)]
    placeholder: Option<String>,
    #[serde(default)]
    min_values: Option<u64>,
    #[serde(default)]
    max_values: Option<u64>,
    #[serde(default)]
    options: Option<Vec<SelectOption>>,
    #[serde(default)]
    disabled: Option<bool>,
}

impl<'de> Deserialize<'de> for SelectMenu {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let h = SelectMenuHelper::deserialize(deserializer)?;
        Ok(Self {
            custom_id: h.custom_id.unwrap_or_default(),
            placeholder: h.placeholder,
            min_values: h.min_values,
            max_values: h.max_values,
            options: h.options.unwrap_or_default(),
            disabled: h.disabled.unwrap_or(false),
        })
    }
}

/// Bileşen satırı yapısı (`ActionRow`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ComponentRow {
    pub components: Vec<Component>,
}

pub type ActionRow = ComponentRow;

impl ComponentRow {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub fn with_components(components: Vec<Component>) -> Self {
        Self { components }
    }

    pub fn add_component(mut self, component: impl Into<Component>) -> Self {
        self.components.push(component.into());
        self
    }

    pub fn add_button(mut self, button: Button) -> Self {
        self.components.push(Component::from(button));
        self
    }

    pub fn add_select_menu(mut self, menu: SelectMenu) -> Self {
        self.components.push(Component::from(menu));
        self
    }

    pub fn components(mut self, components: Vec<Component>) -> Self {
        self.components = components;
        self
    }

    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    pub fn len(&self) -> usize {
        self.components.len()
    }

    pub fn build(self) -> Component {
        Component::from(self)
    }

    pub fn into_component(self) -> Component {
        self.build()
    }
}

impl Serialize for ComponentRow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("ComponentRow", 2)?;
        state.serialize_field("type", &(ComponentType::ActionRow as u8))?;
        state.serialize_field("components", &self.components)?;
        state.end()
    }
}

#[derive(Deserialize)]
struct ComponentRowHelper {
    #[serde(rename = "type", default)]
    _type: Option<serde_json::Value>,
    #[serde(default)]
    components: Option<Vec<Component>>,
}

impl<'de> Deserialize<'de> for ComponentRow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let h = ComponentRowHelper::deserialize(deserializer)?;
        Ok(Self {
            components: h.components.unwrap_or_default(),
        })
    }
}

/// Modal (açılır form) yapısı.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Modal {
    #[serde(deserialize_with = "deserialize_null_default")]
    pub custom_id: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub title: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub components: Vec<Component>,
}

impl Modal {
    pub fn new(custom_id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            custom_id: custom_id.into(),
            title: title.into(),
            components: Vec::new(),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn custom_id(mut self, custom_id: impl Into<String>) -> Self {
        self.custom_id = custom_id.into();
        self
    }

    pub fn component(mut self, component: impl Into<Component>) -> Self {
        self.components.push(component.into());
        self
    }

    pub fn add_component(mut self, component: impl Into<Component>) -> Self {
        self.components.push(component.into());
        self
    }

    pub fn add_row(mut self, row: impl Into<Component>) -> Self {
        self.components.push(row.into());
        self
    }

    pub fn components(mut self, components: Vec<Component>) -> Self {
        self.components = components;
        self
    }

    pub fn text_input(
        mut self,
        custom_id: impl Into<String>,
        style: TextInputStyle,
        label: impl Into<String>,
    ) -> Self {
        let input = Component::text_input(custom_id, style, label);
        self.components.push(Component::action_row(vec![input]));
        self
    }

    pub fn add_text_input(
        self,
        custom_id: impl Into<String>,
        style: TextInputStyle,
        label: impl Into<String>,
    ) -> Self {
        self.text_input(custom_id, style, label)
    }

    pub fn into_response(self) -> crate::interaction::InteractionResponse {
        crate::interaction::InteractionResponse::modal(self.custom_id, self.title, self.components)
    }
}

/// Metin giriş kutusu bileşeni.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TextInput {
    pub custom_id: String,
    pub style: TextInputStyle,
    pub label: String,
    pub min_length: Option<u64>,
    pub max_length: Option<u64>,
    pub required: Option<bool>,
    pub value: Option<String>,
    pub placeholder: Option<String>,
}

impl TextInput {
    pub fn new(
        custom_id: impl Into<String>,
        style: TextInputStyle,
        label: impl Into<String>,
    ) -> Self {
        Self {
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

    pub fn short(custom_id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(custom_id, TextInputStyle::Short, label)
    }

    pub fn paragraph(custom_id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(custom_id, TextInputStyle::Paragraph, label)
    }

    pub fn custom_id(mut self, id: impl Into<String>) -> Self {
        self.custom_id = id.into();
        self
    }

    pub fn style(mut self, style: TextInputStyle) -> Self {
        self.style = style;
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn min_length(mut self, min: u64) -> Self {
        self.min_length = Some(min);
        self
    }

    pub fn max_length(mut self, max: u64) -> Self {
        self.max_length = Some(max);
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = Some(required);
        self
    }

    pub fn value(mut self, val: impl Into<String>) -> Self {
        self.value = Some(val.into());
        self
    }

    pub fn placeholder(mut self, ph: impl Into<String>) -> Self {
        self.placeholder = Some(ph.into());
        self
    }

    pub fn build(self) -> Component {
        Component::from(self)
    }

    pub fn into_component(self) -> Component {
        self.build()
    }

    pub fn into_row(self) -> ComponentRow {
        ComponentRow::new().add_component(self)
    }

    pub fn into_action_row(self) -> Component {
        Component::action_row(vec![self.build()])
    }
}

impl Serialize for TextInput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut count = 3;
        if self.min_length.is_some() {
            count += 1;
        }
        if self.max_length.is_some() {
            count += 1;
        }
        if self.required.is_some() {
            count += 1;
        }
        if self.value.is_some() {
            count += 1;
        }
        if self.placeholder.is_some() {
            count += 1;
        }

        let mut state = serializer.serialize_struct("TextInput", count)?;
        state.serialize_field("type", &(ComponentType::TextInput as u8))?;
        state.serialize_field("custom_id", &self.custom_id)?;
        state.serialize_field("style", &(self.style as u8))?;
        state.serialize_field("label", &self.label)?;
        if let Some(min) = self.min_length {
            state.serialize_field("min_length", &min)?;
        }
        if let Some(max) = self.max_length {
            state.serialize_field("max_length", &max)?;
        }
        if let Some(req) = self.required {
            state.serialize_field("required", &req)?;
        }
        if let Some(val) = &self.value {
            state.serialize_field("value", val)?;
        }
        if let Some(ph) = &self.placeholder {
            state.serialize_field("placeholder", ph)?;
        }
        state.end()
    }
}

#[derive(Deserialize)]
struct TextInputHelper {
    #[serde(rename = "type", default)]
    _type: Option<serde_json::Value>,
    #[serde(default)]
    custom_id: Option<String>,
    #[serde(default)]
    style: Option<TextInputStyle>,
    #[serde(default)]
    label: Option<String>,
    #[serde(default)]
    min_length: Option<u64>,
    #[serde(default)]
    max_length: Option<u64>,
    #[serde(default)]
    required: Option<bool>,
    #[serde(default)]
    value: Option<String>,
    #[serde(default)]
    placeholder: Option<String>,
}

impl<'de> Deserialize<'de> for TextInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let h = TextInputHelper::deserialize(deserializer)?;
        Ok(Self {
            custom_id: h.custom_id.unwrap_or_default(),
            style: h.style.unwrap_or(TextInputStyle::Short),
            label: h.label.unwrap_or_default(),
            min_length: h.min_length,
            max_length: h.max_length,
            required: h.required,
            value: h.value,
            placeholder: h.placeholder,
        })
    }
}

impl From<Button> for Component {
    fn from(b: Button) -> Self {
        Self::Button {
            style: b.style,
            label: b.label,
            custom_id: b.custom_id,
            url: b.url,
            disabled: b.disabled,
        }
    }
}

impl From<SelectMenu> for Component {
    fn from(s: SelectMenu) -> Self {
        Self::StringSelect {
            custom_id: s.custom_id,
            placeholder: s.placeholder,
            min_values: s.min_values,
            max_values: s.max_values,
            options: s.options,
            disabled: s.disabled,
        }
    }
}

impl From<ComponentRow> for Component {
    fn from(r: ComponentRow) -> Self {
        Self::ActionRow {
            components: r.components,
        }
    }
}

impl From<Vec<Component>> for ComponentRow {
    fn from(components: Vec<Component>) -> Self {
        Self { components }
    }
}

impl From<TextInput> for Component {
    fn from(t: TextInput) -> Self {
        Self::TextInput {
            custom_id: t.custom_id,
            style: t.style,
            label: t.label,
            min_length: t.min_length,
            max_length: t.max_length,
            required: t.required,
            value: t.value,
            placeholder: t.placeholder,
        }
    }
}

impl From<Button> for ComponentRow {
    fn from(b: Button) -> Self {
        Self::new().add_button(b)
    }
}

impl From<SelectMenu> for ComponentRow {
    fn from(s: SelectMenu) -> Self {
        Self::new().add_select_menu(s)
    }
}

impl From<TextInput> for ComponentRow {
    fn from(t: TextInput) -> Self {
        Self::new().add_component(t)
    }
}

impl From<Modal> for crate::interaction::ModalResponseData {
    fn from(m: Modal) -> Self {
        Self {
            custom_id: m.custom_id,
            title: m.title,
            components: m.components,
        }
    }
}

impl From<Modal> for crate::interaction::InteractionResponse {
    fn from(m: Modal) -> Self {
        m.into_response()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Component {
    ActionRow {
        components: Vec<Component>,
    },
    Button {
        style: ButtonStyle,
        label: Option<String>,
        custom_id: Option<String>,
        url: Option<String>,
        disabled: bool,
    },
    StringSelect {
        custom_id: String,
        placeholder: Option<String>,
        min_values: Option<u64>,
        max_values: Option<u64>,
        options: Vec<SelectOption>,
        disabled: bool,
    },
    TextInput {
        custom_id: String,
        style: TextInputStyle,
        label: String,
        min_length: Option<u64>,
        max_length: Option<u64>,
        required: Option<bool>,
        value: Option<String>,
        placeholder: Option<String>,
    },
}

impl Default for Component {
    fn default() -> Self {
        Self::ActionRow {
            components: Vec::new(),
        }
    }
}

#[derive(Deserialize)]
struct ComponentHelper {
    #[serde(rename = "type", default)]
    r#type: Option<serde_json::Value>,
    #[serde(default)]
    components: Option<Vec<Component>>,
    #[serde(default)]
    style: Option<serde_json::Value>,
    #[serde(default)]
    label: Option<String>,
    #[serde(default)]
    custom_id: Option<String>,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    disabled: Option<bool>,
    #[serde(default)]
    placeholder: Option<String>,
    #[serde(default)]
    min_values: Option<u64>,
    #[serde(default)]
    max_values: Option<u64>,
    #[serde(default)]
    options: Option<Vec<SelectOption>>,
    #[serde(default)]
    min_length: Option<u64>,
    #[serde(default)]
    max_length: Option<u64>,
    #[serde(default)]
    required: Option<bool>,
    #[serde(default)]
    value: Option<String>,
}

impl<'de> Deserialize<'de> for Component {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let h = ComponentHelper::deserialize(deserializer)?;
        let type_num = match &h.r#type {
            Some(serde_json::Value::Number(n)) => n.as_u64().unwrap_or(1),
            Some(serde_json::Value::String(s)) => match s.to_ascii_lowercase().as_str() {
                "1" | "actionrow" | "action_row" => 1,
                "2" | "button" => 2,
                "3" | "stringselect" | "string_select" => 3,
                "4" | "textinput" | "text_input" => 4,
                "5" | "userselect" | "user_select" => 3,
                "6" | "roleselect" | "role_select" => 3,
                "7" | "mentionableselect" | "mentionable_select" => 3,
                "8" | "channelselect" | "channel_select" => 3,
                _ => 1,
            },
            _ => {
                if h.components.is_some() {
                    1
                } else if h.options.is_some() {
                    3
                } else if h.min_length.is_some() || h.max_length.is_some() || h.required.is_some() {
                    4
                } else {
                    2
                }
            }
        };

        match type_num {
            1 => Ok(Component::ActionRow {
                components: h.components.unwrap_or_default(),
            }),
            2 => {
                let style = match h.style {
                    Some(serde_json::Value::Number(n)) => match n.as_u64().unwrap_or(1) {
                        1 => ButtonStyle::Primary,
                        2 => ButtonStyle::Secondary,
                        3 => ButtonStyle::Success,
                        4 => ButtonStyle::Danger,
                        5 => ButtonStyle::Link,
                        _ => ButtonStyle::Primary,
                    },
                    Some(serde_json::Value::String(s)) => match s.to_ascii_lowercase().as_str() {
                        "primary" | "1" => ButtonStyle::Primary,
                        "secondary" | "2" => ButtonStyle::Secondary,
                        "success" | "3" => ButtonStyle::Success,
                        "danger" | "4" => ButtonStyle::Danger,
                        "link" | "5" => ButtonStyle::Link,
                        _ => ButtonStyle::Primary,
                    },
                    _ => ButtonStyle::Primary,
                };
                Ok(Component::Button {
                    style,
                    label: h.label,
                    custom_id: h.custom_id,
                    url: h.url,
                    disabled: h.disabled.unwrap_or(false),
                })
            }
            3 | 5 | 6 | 7 | 8 => Ok(Component::StringSelect {
                custom_id: h.custom_id.unwrap_or_default(),
                placeholder: h.placeholder,
                min_values: h.min_values,
                max_values: h.max_values,
                options: h.options.unwrap_or_default(),
                disabled: h.disabled.unwrap_or(false),
            }),
            4 => {
                let style = match h.style {
                    Some(serde_json::Value::Number(n)) => match n.as_u64().unwrap_or(1) {
                        1 => TextInputStyle::Short,
                        2 => TextInputStyle::Paragraph,
                        _ => TextInputStyle::Short,
                    },
                    Some(serde_json::Value::String(s)) => match s.to_ascii_lowercase().as_str() {
                        "short" | "1" => TextInputStyle::Short,
                        "paragraph" | "2" => TextInputStyle::Paragraph,
                        _ => TextInputStyle::Short,
                    },
                    _ => TextInputStyle::Short,
                };
                Ok(Component::TextInput {
                    custom_id: h.custom_id.unwrap_or_default(),
                    style,
                    label: h.label.unwrap_or_default(),
                    min_length: h.min_length,
                    max_length: h.max_length,
                    required: h.required,
                    value: h.value,
                    placeholder: h.placeholder,
                })
            }
            _ => Ok(Component::ActionRow {
                components: h.components.unwrap_or_default(),
            }),
        }
    }
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
                let mut count = 2;
                if !options.is_empty() {
                    count += 1;
                }
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
                if !options.is_empty() {
                    state.serialize_field("options", options)?;
                }
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

    pub fn primary(label: impl Into<String>, custom_id: impl Into<String>) -> Self {
        Button::primary(label, custom_id).build()
    }

    pub fn secondary(label: impl Into<String>, custom_id: impl Into<String>) -> Self {
        Button::secondary(label, custom_id).build()
    }

    pub fn success(label: impl Into<String>, custom_id: impl Into<String>) -> Self {
        Button::success(label, custom_id).build()
    }

    pub fn danger(label: impl Into<String>, custom_id: impl Into<String>) -> Self {
        Button::danger(label, custom_id).build()
    }

    pub fn link(label: impl Into<String>, url: impl Into<String>) -> Self {
        Button::link(label, url).build()
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

    pub fn custom_id(mut self, cid: impl Into<String>) -> Self {
        let val = cid.into();
        match &mut self {
            Self::Button { custom_id, .. } => *custom_id = Some(val),
            Self::StringSelect { custom_id, .. } => *custom_id = val,
            Self::TextInput { custom_id, .. } => *custom_id = val,
            _ => {}
        }
        self
    }

    pub fn label(mut self, lbl: impl Into<String>) -> Self {
        let val = lbl.into();
        match &mut self {
            Self::Button { label, .. } => *label = Some(val),
            Self::TextInput { label, .. } => *label = val,
            _ => {}
        }
        self
    }

    pub fn url(mut self, u: impl Into<String>) -> Self {
        if let Self::Button { url, .. } = &mut self {
            *url = Some(u.into());
        }
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        match &mut self {
            Self::Button { disabled: d, .. } => *d = disabled,
            Self::StringSelect { disabled: d, .. } => *d = disabled,
            _ => {}
        }
        self
    }

    pub fn min_length(mut self, min: u64) -> Self {
        if let Self::TextInput { min_length, .. } = &mut self {
            *min_length = Some(min);
        }
        self
    }

    pub fn max_length(mut self, max: u64) -> Self {
        if let Self::TextInput { max_length, .. } = &mut self {
            *max_length = Some(max);
        }
        self
    }

    pub fn required(mut self, req: bool) -> Self {
        if let Self::TextInput { required, .. } = &mut self {
            *required = Some(req);
        }
        self
    }

    pub fn value(mut self, val: impl Into<String>) -> Self {
        if let Self::TextInput { value, .. } = &mut self {
            *value = Some(val.into());
        }
        self
    }

    pub fn placeholder(mut self, ph: impl Into<String>) -> Self {
        match &mut self {
            Self::TextInput { placeholder, .. } => *placeholder = Some(ph.into()),
            Self::StringSelect { placeholder, .. } => *placeholder = Some(ph.into()),
            _ => {}
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_builders_and_serialization() {
        let btn = Button::primary("Tıkla", "btn_click").disabled(true);
        let json = serde_json::to_value(&btn).unwrap();
        assert_eq!(json["type"], 2);
        assert_eq!(json["style"], 1);
        assert_eq!(json["label"], "Tıkla");
        assert_eq!(json["custom_id"], "btn_click");
        assert_eq!(json["disabled"], true);

        let btn_active = Button::success("Aktif", "btn_ok");
        let json_act = serde_json::to_value(&btn_active).unwrap();
        assert!(json_act.get("disabled").is_none());

        let link_btn = Button::link("Site", "https://micup.gg");
        let json_link = serde_json::to_value(&link_btn).unwrap();
        assert_eq!(json_link["style"], 5);
        assert_eq!(json_link["url"], "https://micup.gg");
    }

    #[test]
    fn test_button_deserialization() {
        let raw = r#"{"type": 2, "style": 3, "label": "Onay", "custom_id": "btn_ack"}"#;
        let btn: Button = serde_json::from_str(raw).unwrap();
        assert_eq!(btn.style, ButtonStyle::Success);
        assert_eq!(btn.label.as_deref(), Some("Onay"));
        assert!(!btn.disabled);
    }

    #[test]
    fn test_select_menu_builders_and_serialization() {
        let menu = SelectMenu::new("menu_1")
            .placeholder("Seçim yapın")
            .min_values(1)
            .max_values(2)
            .add_option("A", "opt_a", Some("Açıklama A"))
            .add_option("B", "opt_b", None::<String>);

        let json = serde_json::to_value(&menu).unwrap();
        assert_eq!(json["type"], 3);
        assert_eq!(json["custom_id"], "menu_1");
        assert_eq!(json["placeholder"], "Seçim yapın");
        assert_eq!(json["min_values"], 1);
        assert_eq!(json["max_values"], 2);
        assert_eq!(json["options"].as_array().unwrap().len(), 2);
        assert!(json.get("disabled").is_none());

        let deserialized: SelectMenu = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.custom_id, "menu_1");
        assert_eq!(deserialized.options.len(), 2);
    }

    #[test]
    fn test_component_row_builders_and_serialization() {
        let row = ComponentRow::new()
            .add_button(Button::primary("1", "b1"))
            .add_button(Button::danger("2", "b2"));

        let json = serde_json::to_value(&row).unwrap();
        assert_eq!(json["type"], 1);
        assert_eq!(json["components"].as_array().unwrap().len(), 2);

        let deserialized: ComponentRow = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.components.len(), 2);
    }

    #[test]
    fn test_modal_builders_and_serialization() {
        let modal = Modal::new("modal_feedback", "Geri Bildirim Formu").text_input(
            "txt_1",
            TextInputStyle::Paragraph,
            "Açıklamanız",
        );

        let json = serde_json::to_value(&modal).unwrap();
        assert_eq!(json["custom_id"], "modal_feedback");
        assert_eq!(json["title"], "Geri Bildirim Formu");
        assert_eq!(json["components"].as_array().unwrap().len(), 1);

        let deserialized: Modal = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.custom_id, "modal_feedback");
        assert_eq!(deserialized.title, "Geri Bildirim Formu");
        assert_eq!(deserialized.components.len(), 1);
    }

    #[test]
    fn test_tolerant_null_and_missing_deserialization() {
        let raw_btn =
            r#"{"type": null, "style": null, "label": null, "custom_id": null, "disabled": null}"#;
        let btn: Button = serde_json::from_str(raw_btn).unwrap();
        assert_eq!(btn.style, ButtonStyle::Primary);
        assert_eq!(btn.label, None);
        assert!(!btn.disabled);

        let raw_modal = r#"{"custom_id": null, "title": null, "components": null}"#;
        let modal: Modal = serde_json::from_str(raw_modal).unwrap();
        assert_eq!(modal.custom_id, "");
        assert_eq!(modal.title, "");
        assert!(modal.components.is_empty());

        let raw_opt = r#"{"label": null, "value": "v1", "description": null, "default": null}"#;
        let opt: SelectOption = serde_json::from_str(raw_opt).unwrap();
        assert_eq!(opt.label, "");
        assert_eq!(opt.value, "v1");
        assert_eq!(opt.description, None);
    }

    #[test]
    fn test_component_macro_compatible_builders() {
        let c1 = Component::primary("Test", "btn_test");
        let c2 = Component::link("Site", "https://micup.gg");
        if let Component::Button {
            style,
            label,
            custom_id,
            ..
        } = c1
        {
            assert_eq!(style, ButtonStyle::Primary);
            assert_eq!(label.as_deref(), Some("Test"));
            assert_eq!(custom_id.as_deref(), Some("btn_test"));
        } else {
            panic!("Beklenen Button bileşeni");
        }

        if let Component::Button { style, url, .. } = c2 {
            assert_eq!(style, ButtonStyle::Link);
            assert_eq!(url.as_deref(), Some("https://micup.gg"));
        } else {
            panic!("Beklenen Link Button bileşeni");
        }
    }
}
