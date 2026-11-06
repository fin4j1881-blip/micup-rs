use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::models::Id;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandType {
    ChatInput,
    User,
    Message,
    Unknown(u64),
}

impl Serialize for CommandType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ChatInput => serializer.serialize_u8(1),
            Self::User => serializer.serialize_u8(2),
            Self::Message => serializer.serialize_u8(3),
            Self::Unknown(v) => serializer.serialize_u64(*v),
        }
    }
}

impl<'de> Deserialize<'de> for CommandType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = u64::deserialize(deserializer)?;
        Ok(match v {
            1 => Self::ChatInput,
            2 => Self::User,
            3 => Self::Message,
            other => Self::Unknown(other),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandOptionType {
    SubCommand,
    SubCommandGroup,
    String,
    Integer,
    Boolean,
    User,
    Channel,
    Role,
    Mentionable,
    Number,
    Attachment,
    Unknown(u64),
}

impl Serialize for CommandOptionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let val = match self {
            Self::SubCommand => 1,
            Self::SubCommandGroup => 2,
            Self::String => 3,
            Self::Integer => 4,
            Self::Boolean => 5,
            Self::User => 6,
            Self::Channel => 7,
            Self::Role => 8,
            Self::Mentionable => 9,
            Self::Number => 10,
            Self::Attachment => 11,
            Self::Unknown(v) => *v,
        };
        serializer.serialize_u64(val)
    }
}

impl<'de> Deserialize<'de> for CommandOptionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = u64::deserialize(deserializer)?;
        Ok(match v {
            1 => Self::SubCommand,
            2 => Self::SubCommandGroup,
            3 => Self::String,
            4 => Self::Integer,
            5 => Self::Boolean,
            6 => Self::User,
            7 => Self::Channel,
            8 => Self::Role,
            9 => Self::Mentionable,
            10 => Self::Number,
            11 => Self::Attachment,
            other => Self::Unknown(other),
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandOptionChoice {
    pub name: String,
    pub value: serde_json::Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandOption {
    #[serde(rename = "type")]
    pub kind: CommandOptionType,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub required: bool,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub choices: Vec<CommandOptionChoice>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub options: Vec<CommandOption>,
}

impl CommandOption {
    pub fn new(kind: CommandOptionType, name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            kind,
            name: name.into(),
            description: description.into(),
            required: false,
            choices: Vec::new(),
            options: Vec::new(),
        }
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn choice(mut self, name: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        self.choices.push(CommandOptionChoice {
            name: name.into(),
            value: value.into(),
        });
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateCommand {
    pub name: String,
    pub description: String,
    #[serde(rename = "type", default = "default_command_type")]
    pub kind: CommandType,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub options: Vec<CommandOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_member_permissions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm_permission: Option<bool>,
}

fn default_command_type() -> CommandType {
    CommandType::ChatInput
}

impl CreateCommand {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            kind: CommandType::ChatInput,
            options: Vec::new(),
            default_member_permissions: None,
            dm_permission: None,
        }
    }

    pub fn option(mut self, option: CommandOption) -> Self {
        self.options.push(option);
        self
    }

    pub fn dm_permission(mut self, allowed: bool) -> Self {
        self.dm_permission = Some(allowed);
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Command {
    pub id: Id,
    pub application_id: Id,
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub kind: CommandType,
    #[serde(default)]
    pub options: Vec<CommandOption>,
    #[serde(default)]
    pub guild_id: Option<Id>,
}
