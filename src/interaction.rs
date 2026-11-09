//! Etkileşimler (Slash Komutları, Buton Tıklamaları, Seçim Menüleri, Modallar).

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::client::Client;
use crate::components::Component;
use crate::embed::Embed;
use crate::error::MicupError;
use crate::models::{deserialize_null_default, Id, Member, Message, User};

/// Etkileşim türü (Ping, Slash Komut, Buton/Seçim Menüsü, Autocomplete, Modal).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InteractionType {
    Ping,
    ApplicationCommand,
    MessageComponent,
    ApplicationCommandAutocomplete,
    ModalSubmit,
    Unknown(u64),
}

impl Default for InteractionType {
    fn default() -> Self {
        Self::Unknown(0)
    }
}

impl Serialize for InteractionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let val = match self {
            Self::Ping => 1,
            Self::ApplicationCommand => 2,
            Self::MessageComponent => 3,
            Self::ApplicationCommandAutocomplete => 4,
            Self::ModalSubmit => 5,
            Self::Unknown(v) => *v,
        };
        serializer.serialize_u64(val)
    }
}

impl<'de> Deserialize<'de> for InteractionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct InteractionTypeVisitor;

        impl<'de> serde::de::Visitor<'de> for InteractionTypeVisitor {
            type Value = InteractionType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("1-5 arası bir tamsayı veya etkileşim türü adı")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(match v {
                    1 => InteractionType::Ping,
                    2 => InteractionType::ApplicationCommand,
                    3 => InteractionType::MessageComponent,
                    4 => InteractionType::ApplicationCommandAutocomplete,
                    5 => InteractionType::ModalSubmit,
                    other => InteractionType::Unknown(other),
                })
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
                if let Ok(num) = v.parse::<u64>() {
                    return self.visit_u64(num);
                }
                match v.to_ascii_uppercase().as_str() {
                    "PING" => Ok(InteractionType::Ping),
                    "APPLICATION_COMMAND" | "APPLICATIONCOMMAND" | "COMMAND" => {
                        Ok(InteractionType::ApplicationCommand)
                    }
                    "MESSAGE_COMPONENT" | "MESSAGECOMPONENT" | "COMPONENT" => {
                        Ok(InteractionType::MessageComponent)
                    }
                    "APPLICATION_COMMAND_AUTOCOMPLETE" | "AUTOCOMPLETE" => {
                        Ok(InteractionType::ApplicationCommandAutocomplete)
                    }
                    "MODAL_SUBMIT" | "MODALSUBMIT" => Ok(InteractionType::ModalSubmit),
                    _ => Ok(InteractionType::Unknown(0)),
                }
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(InteractionType::Unknown(0))
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(InteractionType::Unknown(0))
            }
        }

        deserializer.deserialize_any(InteractionTypeVisitor)
    }
}

/// Etkileşime verilecek yanıt türü.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InteractionResponseType {
    Pong,
    ChannelMessageWithSource,
    DeferredChannelMessageWithSource,
    DeferredUpdateMessage,
    UpdateMessage,
    ApplicationCommandAutocompleteResult,
    Modal,
    Unknown(u64),
}

impl Default for InteractionResponseType {
    fn default() -> Self {
        Self::Unknown(0)
    }
}

impl Serialize for InteractionResponseType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let val = match self {
            Self::Pong => 1,
            Self::ChannelMessageWithSource => 4,
            Self::DeferredChannelMessageWithSource => 5,
            Self::DeferredUpdateMessage => 6,
            Self::UpdateMessage => 7,
            Self::ApplicationCommandAutocompleteResult => 8,
            Self::Modal => 9,
            Self::Unknown(v) => *v,
        };
        serializer.serialize_u64(val)
    }
}

impl<'de> Deserialize<'de> for InteractionResponseType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct InteractionResponseTypeVisitor;

        impl<'de> serde::de::Visitor<'de> for InteractionResponseTypeVisitor {
            type Value = InteractionResponseType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("1-9 arası bir yanıt türü tamsayısı veya tür adı")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(match v {
                    1 => InteractionResponseType::Pong,
                    4 => InteractionResponseType::ChannelMessageWithSource,
                    5 => InteractionResponseType::DeferredChannelMessageWithSource,
                    6 => InteractionResponseType::DeferredUpdateMessage,
                    7 => InteractionResponseType::UpdateMessage,
                    8 => InteractionResponseType::ApplicationCommandAutocompleteResult,
                    9 => InteractionResponseType::Modal,
                    other => InteractionResponseType::Unknown(other),
                })
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
                if let Ok(num) = v.parse::<u64>() {
                    return self.visit_u64(num);
                }
                match v.to_ascii_uppercase().as_str() {
                    "PONG" => Ok(InteractionResponseType::Pong),
                    "CHANNEL_MESSAGE_WITH_SOURCE" | "CHANNELMESSAGEWITHSOURCE" => {
                        Ok(InteractionResponseType::ChannelMessageWithSource)
                    }
                    "DEFERRED_CHANNEL_MESSAGE_WITH_SOURCE" | "DEFERREDCHANNELMESSAGEWITHSOURCE" => {
                        Ok(InteractionResponseType::DeferredChannelMessageWithSource)
                    }
                    "DEFERRED_UPDATE_MESSAGE" | "DEFERREDUPDATEMESSAGE" => {
                        Ok(InteractionResponseType::DeferredUpdateMessage)
                    }
                    "UPDATE_MESSAGE" | "UPDATEMESSAGE" => {
                        Ok(InteractionResponseType::UpdateMessage)
                    }
                    "APPLICATION_COMMAND_AUTOCOMPLETE_RESULT" | "AUTOCOMPLETE" => {
                        Ok(InteractionResponseType::ApplicationCommandAutocompleteResult)
                    }
                    "MODAL" => Ok(InteractionResponseType::Modal),
                    _ => Ok(InteractionResponseType::Unknown(0)),
                }
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(InteractionResponseType::Unknown(0))
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(InteractionResponseType::Unknown(0))
            }
        }

        deserializer.deserialize_any(InteractionResponseTypeVisitor)
    }
}

/// Slash komut parametresi veya alt komut verisi.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CommandInteractionDataOption {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub name: String,
    #[serde(
        rename = "type",
        default,
        deserialize_with = "deserialize_null_default"
    )]
    pub kind: u64,
    #[serde(default)]
    pub value: Option<serde_json::Value>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub options: Vec<CommandInteractionDataOption>,
    #[serde(default)]
    pub focused: Option<bool>,
}

impl CommandInteractionDataOption {
    pub fn as_str(&self) -> Option<&str> {
        self.value.as_ref().and_then(|v| v.as_str())
    }

    pub fn as_i64(&self) -> Option<i64> {
        self.value.as_ref().and_then(|v| v.as_i64())
    }

    pub fn as_u64(&self) -> Option<u64> {
        self.value.as_ref().and_then(|v| v.as_u64())
    }

    pub fn as_bool(&self) -> Option<bool> {
        self.value.as_ref().and_then(|v| v.as_bool())
    }

    pub fn as_f64(&self) -> Option<f64> {
        self.value.as_ref().and_then(|v| v.as_f64())
    }
}

/// Etkileşimle birlikte gelen veri yükü.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct InteractionData {
    #[serde(default)]
    pub id: Option<Id>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "type", default)]
    pub kind: Option<u64>,
    #[serde(default)]
    pub custom_id: Option<String>,
    #[serde(default)]
    pub component_type: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub values: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub options: Vec<CommandInteractionDataOption>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub components: Vec<Component>,
    #[serde(default)]
    pub target_id: Option<Id>,
}

impl InteractionData {
    /// Slash komut seçenekleri arasından belirli ada sahip seçeneği bulur.
    pub fn get_option(&self, name: &str) -> Option<&CommandInteractionDataOption> {
        self.options.iter().find(|opt| opt.name == name)
    }

    /// Modal gönderiminde (ModalSubmit) belirli `custom_id`'ye sahip bileşenin metin değerini bulur.
    pub fn get_modal_value(&self, custom_id: &str) -> Option<&str> {
        fn find_in_components<'a>(comps: &'a [Component], target_id: &str) -> Option<&'a str> {
            for c in comps {
                match c {
                    Component::ActionRow { components } => {
                        if let Some(val) = find_in_components(components, target_id) {
                            return Some(val);
                        }
                    }
                    Component::TextInput {
                        custom_id: id,
                        value: Some(val),
                        ..
                    } if id == target_id => {
                        return Some(val.as_str());
                    }
                    _ => {}
                }
            }
            None
        }
        find_in_components(&self.components, custom_id)
    }
}

/// Ağ geçidinden veya webhook'tan gelen etkileşim nesnesi.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Interaction {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub id: Id,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub application_id: Id,
    #[serde(
        rename = "type",
        default,
        deserialize_with = "deserialize_null_default"
    )]
    pub kind: InteractionType,
    #[serde(default)]
    pub data: Option<Box<InteractionData>>,
    #[serde(default)]
    pub guild_id: Option<Id>,
    #[serde(default)]
    pub channel_id: Option<Id>,
    #[serde(default)]
    pub member: Option<Box<Member>>,
    #[serde(default)]
    pub user: Option<User>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub token: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub version: u64,
    #[serde(default)]
    pub message: Option<Box<Message>>,
    #[serde(default)]
    pub app_permissions: Option<String>,
    #[serde(default)]
    pub locale: Option<String>,
    #[serde(default)]
    pub guild_locale: Option<String>,
}

impl Interaction {
    /// Etkileşimi tetikleyen kullanıcıyı (sunucu üyesi veya direkt kullanıcı) döner.
    pub fn author(&self) -> Option<&User> {
        self.member
            .as_ref()
            .and_then(|m| m.user.as_ref())
            .or(self.user.as_ref())
    }

    /// Etkileşimi tetikleyen kullanıcının ID'sini döner.
    pub fn author_id(&self) -> Option<u64> {
        self.author().map(|u| u.id.get())
    }

    /// Sunucu ID'sini ham `u64` olarak döner.
    pub fn guild_id_u64(&self) -> Option<u64> {
        self.guild_id.map(|g| g.get())
    }

    /// Kanal ID'sini ham `u64` olarak döner.
    pub fn channel_id_u64(&self) -> Option<u64> {
        self.channel_id.map(|c| c.get())
    }

    pub fn is_command(&self) -> bool {
        matches!(self.kind, InteractionType::ApplicationCommand)
    }

    pub fn is_component(&self) -> bool {
        matches!(self.kind, InteractionType::MessageComponent)
    }

    pub fn is_modal_submit(&self) -> bool {
        matches!(self.kind, InteractionType::ModalSubmit)
    }

    pub fn is_autocomplete(&self) -> bool {
        matches!(self.kind, InteractionType::ApplicationCommandAutocomplete)
    }

    pub fn is_ping(&self) -> bool {
        matches!(self.kind, InteractionType::Ping)
    }

    /// Slash komut adını döner.
    pub fn command_name(&self) -> Option<&str> {
        self.data.as_ref().and_then(|d| d.name.as_deref())
    }

    /// Bileşen veya modal `custom_id` değerini döner.
    pub fn custom_id(&self) -> Option<&str> {
        self.data.as_ref().and_then(|d| d.custom_id.as_deref())
    }

    /// Slash komut seçenekleri arasından belirli ada sahip seçeneği döner.
    pub fn get_option(&self, name: &str) -> Option<&CommandInteractionDataOption> {
        self.data.as_ref().and_then(|d| d.get_option(name))
    }

    /// Slash komut seçeneğinin değerini döner.
    pub fn get_option_value(&self, name: &str) -> Option<&serde_json::Value> {
        self.get_option(name).and_then(|opt| opt.value.as_ref())
    }

    /// Slash komut seçeneğinin metin (string) değerini döner.
    pub fn get_option_string(&self, name: &str) -> Option<&str> {
        self.get_option_value(name).and_then(|v| v.as_str())
    }

    /// Modal gönderiminde girilen metin değerini `custom_id` ile bulur.
    pub fn get_modal_value(&self, custom_id: &str) -> Option<&str> {
        self.data
            .as_ref()
            .and_then(|d| d.get_modal_value(custom_id))
    }

    // =========================================================================
    // Hızlı Yanıt ve Asenkron REST Metotları
    // =========================================================================

    /// Doğrudan bir `InteractionResponse` nesnesi göndererek etkileşime yanıt verir (`POST /interactions/{id}/{token}/callback`).
    pub async fn create_response(
        &self,
        client: &Client,
        response: &InteractionResponse,
    ) -> Result<(), MicupError> {
        client
            .create_interaction_response(self.id.get(), &self.token, response)
            .await
    }

    /// Gelen etkileşime metin, embed veya yapılandırılmış mesaj ile hızlı yanıt verir (ChannelMessageWithSource).
    pub async fn reply(
        &self,
        client: &Client,
        message: impl Into<InteractionResponseMessage>,
    ) -> Result<(), MicupError> {
        let resp = InteractionResponse::message(message.into());
        self.create_response(client, &resp).await
    }

    /// Gelen etkileşime yalnızca komutu kullananın görebileceği (ephemeral / gizli) mesajla yanıt verir.
    pub async fn reply_ephemeral(
        &self,
        client: &Client,
        content: impl Into<String>,
    ) -> Result<(), MicupError> {
        let resp = InteractionResponse::message(
            InteractionResponseMessage::new()
                .content(content)
                .ephemeral(true),
        );
        self.create_response(client, &resp).await
    }

    /// Gelen etkileşime Embed içeren yanıt verir.
    pub async fn reply_embed(
        &self,
        client: &Client,
        embed: Embed,
        ephemeral: bool,
    ) -> Result<(), MicupError> {
        let resp = InteractionResponse::message(
            InteractionResponseMessage::new()
                .embed(embed)
                .ephemeral(ephemeral),
        );
        self.create_response(client, &resp).await
    }

    /// Gelen etkileşime birden fazla Embed içeren yanıt verir.
    pub async fn reply_embeds(
        &self,
        client: &Client,
        embeds: Vec<Embed>,
        ephemeral: bool,
    ) -> Result<(), MicupError> {
        let resp = InteractionResponse::message(
            InteractionResponseMessage::new()
                .embeds(embeds)
                .ephemeral(ephemeral),
        );
        self.create_response(client, &resp).await
    }

    /// Gelen etkileşime bileşenler (buton/menü) içeren yanıt verir.
    pub async fn reply_components(
        &self,
        client: &Client,
        content: Option<&str>,
        components: Vec<Component>,
        ephemeral: bool,
    ) -> Result<(), MicupError> {
        let mut msg = InteractionResponseMessage::new()
            .components(components)
            .ephemeral(ephemeral);
        if let Some(text) = content {
            msg = msg.content(text);
        }
        let resp = InteractionResponse::message(msg);
        self.create_response(client, &resp).await
    }

    /// 3 saniyelik zaman aşımını aşmak için etkileşimi hemen erteler (DeferredChannelMessageWithSource - "Düşünüyor...").
    pub async fn defer(&self, client: &Client, ephemeral: bool) -> Result<(), MicupError> {
        let resp = InteractionResponse::defer(ephemeral);
        self.create_response(client, &resp).await
    }

    /// Buton veya menü bileşen etkileşimini erteler (DeferredUpdateMessage - mesajda yükleniyor göstermez).
    pub async fn defer_update(&self, client: &Client) -> Result<(), MicupError> {
        let resp = InteractionResponse::defer_update();
        self.create_response(client, &resp).await
    }

    /// Kaynak mesajı günceller (UpdateMessage).
    pub async fn update_message(
        &self,
        client: &Client,
        message: impl Into<InteractionResponseMessage>,
    ) -> Result<(), MicupError> {
        let resp = InteractionResponse::update_message(message.into());
        self.create_response(client, &resp).await
    }

    /// Kaynak mesajı içerik, embed ve bileşen listesi belirterek günceller.
    pub async fn update_message_components(
        &self,
        client: &Client,
        content: Option<&str>,
        embeds: Vec<Embed>,
        components: Vec<Component>,
    ) -> Result<(), MicupError> {
        let mut msg = InteractionResponseMessage::new()
            .embeds(embeds)
            .components(components);
        if let Some(text) = content {
            msg = msg.content(text);
        }
        self.update_message(client, msg).await
    }

    /// Kullanıcıya modal popup formu açar (Modal).
    pub async fn modal(
        &self,
        client: &Client,
        custom_id: impl Into<String>,
        title: impl Into<String>,
        components: Vec<Component>,
    ) -> Result<(), MicupError> {
        let resp = InteractionResponse::modal(custom_id, title, components);
        self.create_response(client, &resp).await
    }

    /// `modal` metodunun diğer yaygın adlandırması.
    pub async fn reply_modal(
        &self,
        client: &Client,
        custom_id: impl Into<String>,
        title: impl Into<String>,
        components: Vec<Component>,
    ) -> Result<(), MicupError> {
        self.modal(client, custom_id, title, components).await
    }

    /// Autocomplete etkileşimine önerilen seçenekleri döner.
    pub async fn autocomplete(
        &self,
        client: &Client,
        choices: Vec<serde_json::Value>,
    ) -> Result<(), MicupError> {
        let resp = InteractionResponse::autocomplete(choices);
        self.create_response(client, &resp).await
    }

    /// Ping etkileşimine Pong yanıtı verir.
    pub async fn pong(&self, client: &Client) -> Result<(), MicupError> {
        let resp = InteractionResponse::pong();
        self.create_response(client, &resp).await
    }

    /// Ertelenmiş etkileşime sonradan takip yanıtı (follow-up) gönderir (`POST /webhooks/{application_id}/{token}`).
    pub async fn followup(
        &self,
        client: &Client,
        message: impl Into<InteractionResponseMessage>,
    ) -> Result<serde_json::Value, MicupError> {
        let msg = message.into();
        client
            .create_followup_message(self.application_id.get(), &self.token, &msg)
            .await
    }
}

/// Etkileşime verilecek yanıt mesajı yükü.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct InteractionResponseMessage {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub content: Option<String>,
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        default,
        deserialize_with = "deserialize_null_default"
    )]
    pub embeds: Vec<Embed>,
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        default,
        deserialize_with = "deserialize_null_default"
    )]
    pub components: Vec<Component>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub flags: Option<u64>, // 64: EPHEMERAL
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tts: Option<bool>,
}

impl InteractionResponseMessage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn content(mut self, text: impl Into<String>) -> Self {
        self.content = Some(text.into());
        self
    }

    pub fn embed(mut self, embed: Embed) -> Self {
        self.embeds.push(embed);
        self
    }

    pub fn embeds(mut self, embeds: Vec<Embed>) -> Self {
        self.embeds = embeds;
        self
    }

    pub fn component(mut self, component: Component) -> Self {
        self.components.push(component);
        self
    }

    pub fn components(mut self, components: Vec<Component>) -> Self {
        self.components = components;
        self
    }

    pub fn ephemeral(mut self, ephemeral: bool) -> Self {
        if ephemeral {
            self.flags = Some(self.flags.unwrap_or(0) | 64);
        } else if let Some(flags) = self.flags {
            let cleared = flags & !64;
            self.flags = if cleared == 0 { None } else { Some(cleared) };
        }
        self
    }

    pub fn tts(mut self, tts: bool) -> Self {
        self.tts = Some(tts);
        self
    }

    pub fn is_ephemeral(&self) -> bool {
        self.flags.map(|f| (f & 64) != 0).unwrap_or(false)
    }
}

impl From<String> for InteractionResponseMessage {
    fn from(text: String) -> Self {
        Self::new().content(text)
    }
}

impl From<&str> for InteractionResponseMessage {
    fn from(text: &str) -> Self {
        Self::new().content(text)
    }
}

impl From<&String> for InteractionResponseMessage {
    fn from(text: &String) -> Self {
        Self::new().content(text.as_str())
    }
}

impl From<Embed> for InteractionResponseMessage {
    fn from(embed: Embed) -> Self {
        Self::new().embed(embed)
    }
}

impl From<Vec<Embed>> for InteractionResponseMessage {
    fn from(embeds: Vec<Embed>) -> Self {
        Self::new().embeds(embeds)
    }
}

impl From<Component> for InteractionResponseMessage {
    fn from(component: Component) -> Self {
        Self::new().component(component)
    }
}

impl From<Vec<Component>> for InteractionResponseMessage {
    fn from(components: Vec<Component>) -> Self {
        Self::new().components(components)
    }
}

/// Modal açma yanıtı verisi.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ModalResponseData {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub custom_id: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub title: String,
    #[serde(
        default,
        deserialize_with = "deserialize_null_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub components: Vec<Component>,
}

/// Autocomplete yanıtı verisi.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AutocompleteResponseData {
    #[serde(
        default,
        deserialize_with = "deserialize_null_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub choices: Vec<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InteractionResponseData {
    Message(InteractionResponseMessage),
    Modal(ModalResponseData),
    Autocomplete(AutocompleteResponseData),
}

/// Etkileşim yanıtı (Interaction Response).
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct InteractionResponse {
    #[serde(
        rename = "type",
        default,
        deserialize_with = "deserialize_null_default"
    )]
    pub kind: InteractionResponseType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<InteractionResponseData>,
}

impl InteractionResponse {
    /// Pong yanıtı oluşturur.
    pub fn pong() -> Self {
        Self {
            kind: InteractionResponseType::Pong,
            data: None,
        }
    }

    /// 3 saniyelik etkileşim zaman aşımını aşmak için anında defer eder.
    pub fn defer(ephemeral: bool) -> Self {
        let flags = if ephemeral { Some(64) } else { None };
        Self {
            kind: InteractionResponseType::DeferredChannelMessageWithSource,
            data: Some(InteractionResponseData::Message(
                InteractionResponseMessage {
                    content: None,
                    embeds: Vec::new(),
                    components: Vec::new(),
                    flags,
                    tts: None,
                },
            )),
        }
    }

    /// Buton veya menü bileşen etkileşimini defer eder (yeni mesaj atmaz, butonu onaylar).
    pub fn defer_update() -> Self {
        Self {
            kind: InteractionResponseType::DeferredUpdateMessage,
            data: None,
        }
    }

    /// Doğrudan kaynak mesaj yanıtı (ChannelMessageWithSource).
    pub fn message(msg: InteractionResponseMessage) -> Self {
        Self {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(InteractionResponseData::Message(msg)),
        }
    }

    /// Mevcut bileşenin mesajını günceller.
    pub fn update_message(msg: InteractionResponseMessage) -> Self {
        Self {
            kind: InteractionResponseType::UpdateMessage,
            data: Some(InteractionResponseData::Message(msg)),
        }
    }

    /// Kullanıcıya modal popup formu açar.
    pub fn modal(
        custom_id: impl Into<String>,
        title: impl Into<String>,
        components: Vec<Component>,
    ) -> Self {
        Self {
            kind: InteractionResponseType::Modal,
            data: Some(InteractionResponseData::Modal(ModalResponseData {
                custom_id: custom_id.into(),
                title: title.into(),
                components,
            })),
        }
    }

    /// Autocomplete seçeneklerini döner.
    pub fn autocomplete(choices: Vec<serde_json::Value>) -> Self {
        Self {
            kind: InteractionResponseType::ApplicationCommandAutocompleteResult,
            data: Some(InteractionResponseData::Autocomplete(
                AutocompleteResponseData { choices },
            )),
        }
    }
}

impl From<InteractionResponseMessage> for InteractionResponse {
    fn from(msg: InteractionResponseMessage) -> Self {
        Self::message(msg)
    }
}

impl From<Embed> for InteractionResponse {
    fn from(embed: Embed) -> Self {
        Self::message(InteractionResponseMessage::from(embed))
    }
}

impl From<&str> for InteractionResponse {
    fn from(text: &str) -> Self {
        Self::message(InteractionResponseMessage::from(text))
    }
}

impl From<String> for InteractionResponse {
    fn from(text: String) -> Self {
        Self::message(InteractionResponseMessage::from(text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_interaction_type_serde() {
        assert_eq!(
            serde_json::from_str::<InteractionType>("2").unwrap(),
            InteractionType::ApplicationCommand
        );
        assert_eq!(
            serde_json::from_str::<InteractionType>("3").unwrap(),
            InteractionType::MessageComponent
        );
        assert_eq!(
            serde_json::from_str::<InteractionType>("5").unwrap(),
            InteractionType::ModalSubmit
        );
        assert_eq!(
            serde_json::from_str::<InteractionType>("\"APPLICATION_COMMAND\"").unwrap(),
            InteractionType::ApplicationCommand
        );
        assert_eq!(
            serde_json::to_string(&InteractionType::ModalSubmit).unwrap(),
            "5"
        );
    }

    #[test]
    fn test_interaction_response_type_serde() {
        assert_eq!(
            serde_json::from_str::<InteractionResponseType>("4").unwrap(),
            InteractionResponseType::ChannelMessageWithSource
        );
        assert_eq!(
            serde_json::from_str::<InteractionResponseType>("5").unwrap(),
            InteractionResponseType::DeferredChannelMessageWithSource
        );
        assert_eq!(
            serde_json::to_string(&InteractionResponseType::ChannelMessageWithSource).unwrap(),
            "4"
        );
    }

    #[test]
    fn test_command_interaction_payload_deserialization() {
        let payload = json!({
            "id": "111222333444",
            "application_id": "555666777888",
            "type": 2,
            "token": "test_interaction_token",
            "version": 1,
            "guild_id": "1234567890",
            "channel_id": "9876543210",
            "data": {
                "id": "100200300",
                "name": "ping",
                "type": 1,
                "options": [
                    {
                        "name": "target",
                        "type": 3,
                        "value": "user123"
                    }
                ]
            },
            "user": {
                "id": "444555666",
                "username": "tester"
            }
        });

        let interaction: Interaction = serde_json::from_value(payload).expect("deserialize failed");
        assert_eq!(interaction.id.get(), 111222333444);
        assert!(interaction.is_command());
        assert_eq!(interaction.command_name(), Some("ping"));
        assert_eq!(interaction.author_id(), Some(444555666));
        assert_eq!(interaction.guild_id_u64(), Some(1234567890));
        assert_eq!(interaction.channel_id_u64(), Some(9876543210));

        let opt = interaction.get_option("target").expect("option missing");
        assert_eq!(opt.as_str(), Some("user123"));
        assert_eq!(interaction.get_option_string("target"), Some("user123"));
    }

    #[test]
    fn test_message_component_payload_deserialization() {
        let payload = json!({
            "id": "111222333444",
            "application_id": "555666777888",
            "type": 3,
            "token": "btn_token",
            "data": {
                "custom_id": "btn_click_me",
                "component_type": 2
            }
        });

        let interaction: Interaction = serde_json::from_value(payload).expect("deserialize failed");
        assert!(interaction.is_component());
        assert_eq!(interaction.custom_id(), Some("btn_click_me"));
    }

    #[test]
    fn test_modal_submit_payload_and_value_lookup() {
        let payload = json!({
            "id": "111222333444",
            "application_id": "555666777888",
            "type": 5,
            "token": "modal_token",
            "data": {
                "custom_id": "feedback_modal",
                "components": [
                    {
                        "type": 1,
                        "components": [
                            {
                                "type": 4,
                                "custom_id": "feedback_text",
                                "value": "Micup is awesome!"
                            }
                        ]
                    }
                ]
            }
        });

        let interaction: Interaction = serde_json::from_value(payload).expect("deserialize failed");
        assert!(interaction.is_modal_submit());
        assert_eq!(interaction.custom_id(), Some("feedback_modal"));
        assert_eq!(
            interaction.get_modal_value("feedback_text"),
            Some("Micup is awesome!")
        );
    }

    #[test]
    fn test_response_ephemeral_and_source_serialization() {
        let resp = InteractionResponse::message(
            InteractionResponseMessage::new()
                .content("Gizli mesaj")
                .ephemeral(true),
        );

        let serialized = serde_json::to_value(&resp).expect("serialization failed");
        assert_eq!(serialized["type"], 4);
        assert_eq!(serialized["data"]["flags"], 64);
        assert_eq!(serialized["data"]["content"], "Gizli mesaj");

        let defer_resp = InteractionResponse::defer(true);
        let defer_json = serde_json::to_value(&defer_resp).expect("serialization failed");
        assert_eq!(defer_json["type"], 5);
        assert_eq!(defer_json["data"]["flags"], 64);

        let update_resp = InteractionResponse::defer_update();
        let update_json = serde_json::to_value(&update_resp).expect("serialization failed");
        assert_eq!(update_json["type"], 6);
        assert!(update_json.get("data").is_none());
    }

    #[test]
    fn test_tolerant_interaction_deserialization() {
        let payload = json!({
            "id": null,
            "application_id": null,
            "type": null,
            "token": null,
            "version": null,
            "guild_id": null,
            "channel_id": null,
            "member": null,
            "user": null,
            "data": {
                "id": null,
                "name": null,
                "type": null,
                "values": null,
                "options": null,
                "components": null
            }
        });

        let interaction: Interaction =
            serde_json::from_value(payload).expect("tolerant deserialize failed");
        assert_eq!(interaction.id.get(), 0);
        assert_eq!(interaction.application_id.get(), 0);
        assert_eq!(interaction.token, "");
        assert_eq!(interaction.version, 0);
        assert!(interaction.data.is_some());
        let data = interaction.data.unwrap();
        assert!(data.values.is_empty());
        assert!(data.options.is_empty());
        assert!(data.components.is_empty());
    }

    #[test]
    fn test_interaction_response_message_conversions() {
        let from_str: InteractionResponseMessage = "Hello".into();
        assert_eq!(from_str.content.as_deref(), Some("Hello"));

        let from_string: InteractionResponseMessage = String::from("World").into();
        assert_eq!(from_string.content.as_deref(), Some("World"));

        let embed = Embed::new().title("Test Embed");
        let from_embed: InteractionResponseMessage = embed.into();
        assert_eq!(from_embed.embeds.len(), 1);
        assert_eq!(from_embed.embeds[0].title.as_deref(), Some("Test Embed"));
    }
}
