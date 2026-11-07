use serde::{Deserialize, Serialize};

use crate::embed::Embed;
use crate::models::{Id, User};

/// Bir kanal mesajı. Micup'ın göndermediği alanlar varsayılan değerlere eşlenir.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: Id,
    #[serde(default)]
    pub channel_id: Option<Id>,
    #[serde(default)]
    pub guild_id: Option<Id>,
    #[serde(default)]
    pub author: Option<User>,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub timestamp: Option<String>,
    #[serde(default)]
    pub edited_timestamp: Option<String>,
    #[serde(default)]
    pub pinned: bool,
    #[serde(default)]
    pub mention_everyone: bool,
    #[serde(default)]
    pub embeds: Vec<Embed>,
    #[serde(default)]
    pub attachments: Vec<Attachment>,
    /// Bileşenler ham JSON olarak tutulur; gönderim için `Component` kullanın.
    #[serde(default)]
    pub components: Vec<serde_json::Value>,
    #[serde(default)]
    pub reactions: Vec<Reaction>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Reaction {
    pub count: u64,
    #[serde(default)]
    pub me: bool,
    pub emoji: Emoji,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Attachment {
    pub id: Id,
    pub filename: String,
    pub size: u64,
    pub url: String,
    pub proxy_url: Option<String>,
    pub content_type: Option<String>,
    pub height: Option<u64>,
    pub width: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Emoji {
    pub id: Option<Id>,
    pub name: Option<String>,
    #[serde(default)]
    pub animated: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Invite {
    pub code: String,
    #[serde(default)]
    pub guild_id: Option<Id>,
    #[serde(default)]
    pub channel_id: Option<Id>,
    #[serde(default)]
    pub inviter: Option<User>,
    #[serde(default)]
    pub approximate_member_count: Option<u64>,
    #[serde(default)]
    pub approximate_presence_count: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Webhook {
    pub id: Id,
    #[serde(rename = "type")]
    pub kind: u64,
    #[serde(default)]
    pub guild_id: Option<Id>,
    #[serde(default)]
    pub channel_id: Option<Id>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default)]
    pub token: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub id: Id,
    pub user_id: Option<Id>,
    pub target_id: Option<String>,
    pub action_type: u64,
    #[serde(default)]
    pub reason: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditLog {
    #[serde(default)]
    pub audit_log_entries: Vec<AuditLogEntry>,
    #[serde(default)]
    pub users: Vec<User>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ban {
    pub user: User,
    #[serde(default)]
    pub reason: Option<String>,
}
