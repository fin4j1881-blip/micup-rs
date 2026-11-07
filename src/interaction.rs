//! Etkileşimler (Slash Komutları, Buton Tıklamaları, Seçim Menüleri, Modallar).

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::components::Component;
use crate::embed::Embed;
use crate::models::{Id, Member, User};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InteractionType {
    Ping,
    ApplicationCommand,
    MessageComponent,
    ApplicationCommandAutocomplete,
    ModalSubmit,
    Unknown(u64),
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
        let v = u64::deserialize(deserializer)?;
        Ok(match v {
            1 => Self::Ping,
            2 => Self::ApplicationCommand,
            3 => Self::MessageComponent,
            4 => Self::ApplicationCommandAutocomplete,
            5 => Self::ModalSubmit,
            other => Self::Unknown(other),
        })
    }
}

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
        let v = u64::deserialize(deserializer)?;
        Ok(match v {
            1 => Self::Pong,
            4 => Self::ChannelMessageWithSource,
            5 => Self::DeferredChannelMessageWithSource,
            6 => Self::DeferredUpdateMessage,
            7 => Self::UpdateMessage,
            8 => Self::ApplicationCommandAutocompleteResult,
            9 => Self::Modal,
            other => Self::Unknown(other),
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandInteractionDataOption {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: u64,
    #[serde(default)]
    pub value: Option<serde_json::Value>,
    #[serde(default)]
    pub options: Vec<CommandInteractionDataOption>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct InteractionData {
    pub id: Option<Id>,
    pub name: Option<String>,
    pub custom_id: Option<String>,
    pub component_type: Option<u64>,
    #[serde(default)]
    pub values: Vec<String>,
    #[serde(default)]
    pub options: Vec<CommandInteractionDataOption>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Interaction {
    pub id: Id,
    pub application_id: Id,
    #[serde(rename = "type")]
    pub kind: InteractionType,
    #[serde(default)]
    pub data: Option<InteractionData>,
    #[serde(default)]
    pub guild_id: Option<Id>,
    #[serde(default)]
    pub channel_id: Option<Id>,
    #[serde(default)]
    pub member: Option<Member>,
    #[serde(default)]
    pub user: Option<User>,
    pub token: String,
    pub version: u64,
}

impl Interaction {
    pub fn author(&self) -> Option<&User> {
        self.member
            .as_ref()
            .and_then(|m| m.user.as_ref())
            .or(self.user.as_ref())
    }

    pub fn author_id(&self) -> Option<u64> {
        self.author().map(|u| u.id.get())
    }

    pub fn guild_id_u64(&self) -> Option<u64> {
        self.guild_id.map(|g| g.get())
    }

    pub fn channel_id_u64(&self) -> Option<u64> {
        self.channel_id.map(|c| c.get())
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct InteractionResponseMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub embeds: Vec<Embed>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub components: Vec<Component>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u64>, // 64: EPHEMERAL
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

    pub fn component(mut self, component: Component) -> Self {
        self.components.push(component);
        self
    }

    pub fn ephemeral(mut self, ephemeral: bool) -> Self {
        if ephemeral {
            self.flags = Some(64);
        } else {
            self.flags = None;
        }
        self
    }
}

/// Modal açma yanıtı verisi.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModalResponseData {
    pub custom_id: String,
    pub title: String,
    pub components: Vec<Component>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InteractionResponseData {
    Message(InteractionResponseMessage),
    Modal(ModalResponseData),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InteractionResponse {
    #[serde(rename = "type")]
    pub kind: InteractionResponseType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<InteractionResponseData>,
}

impl InteractionResponse {
    /// 3 saniyelik Discord/Micup etkileşim zaman aşımını aşmak için anında defer eder.
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

    /// Doğrudan yanıt mesajı.
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
}
