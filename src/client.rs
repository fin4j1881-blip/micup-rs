use std::time::Duration;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};
use serde::Serialize;
use serde_json::Value;

use crate::components::Component;
use crate::embed::Embed;
use crate::error::MicupError;
use crate::models::constants::API_BASE;
use crate::models::{Channel, Guild, Id, Member, Role, User};

#[derive(Clone)]
pub struct Client {
    token: String,
    http: reqwest::Client,
    base_url: String,
}

impl Client {
    pub fn new(token: impl Into<String>) -> Result<Self, MicupError> {
        let token = token.into();
        let mut headers = HeaderMap::new();
        let auth_val = format!("Bot {token}");
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&auth_val).map_err(|e| MicupError::Other(e.to_string()))?,
        );
        headers.insert(USER_AGENT, HeaderValue::from_static("micup-rs/0.1.0"));

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(15))
            .build()?;

        Ok(Self {
            token,
            http,
            base_url: API_BASE.to_string(),
        })
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    // =========================================================================
    // Kullanıcılar (Users)
    // =========================================================================

    /// Mevcut bot kullanıcısını getirir (`GET /users/@me`).
    pub async fn current_user(&self) -> Result<User, MicupError> {
        self.get("/users/@me").await
    }

    /// Kullanıcı profili getirir (`GET /users/{id}`).
    pub async fn get_user(&self, user_id: u64) -> Result<User, MicupError> {
        self.get(&format!("/users/{user_id}")).await
    }

    // =========================================================================
    // Sunucular (Guilds)
    // =========================================================================

    /// Bir sunucunun detaylı bilgilerini çeker (`GET /guilds/{id}`).
    pub async fn get_guild(&self, guild_id: u64) -> Result<Guild, MicupError> {
        self.get(&format!("/guilds/{guild_id}")).await
    }

    /// Sunucudaki kanalları listeler (`GET /guilds/{id}/channels`).
    pub async fn get_guild_channels(&self, guild_id: u64) -> Result<Vec<Channel>, MicupError> {
        self.get(&format!("/guilds/{guild_id}/channels")).await
    }

    /// Sunucudaki rolleri listeler (`GET /guilds/{id}/roles`).
    pub async fn get_guild_roles(&self, guild_id: u64) -> Result<Vec<Role>, MicupError> {
        self.get(&format!("/guilds/{guild_id}/roles")).await
    }

    /// Yeni kanal oluşturur (`POST /guilds/{id}/channels`).
    pub async fn create_channel(
        &self,
        guild_id: u64,
        name: &str,
        kind: u64, // 0: Text, 2: Voice, 4: Category
        parent_id: Option<u64>,
    ) -> Result<Channel, MicupError> {
        #[derive(Serialize)]
        struct CreateChannelPayload<'a> {
            name: &'a str,
            #[serde(rename = "type")]
            kind: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            parent_id: Option<Id>,
        }
        let p = CreateChannelPayload {
            name,
            kind,
            parent_id: parent_id.map(Id),
        };
        self.post(&format!("/guilds/{guild_id}/channels"), &p).await
    }

    // =========================================================================
    // Üyeler ve Moderasyon (Members & Moderation)
    // =========================================================================

    /// Sunucu üyesini çeker (`GET /guilds/{id}/members/{user_id}`).
    pub async fn get_member(&self, guild_id: u64, user_id: u64) -> Result<Member, MicupError> {
        self.get(&format!("/guilds/{guild_id}/members/{user_id}")).await
    }

    /// Sunucudan üye atar (Kick: `DELETE /guilds/{id}/members/{user_id}`).
    pub async fn kick(&self, guild_id: u64, user_id: u64, reason: Option<&str>) -> Result<(), MicupError> {
        let path = format!("/guilds/{guild_id}/members/{user_id}");
        self.delete_with_reason(&path, reason).await
    }

    /// Sunucudan üyeyi yasaklar (Ban: `PUT /guilds/{id}/bans/{user_id}`).
    pub async fn ban(&self, guild_id: u64, user_id: u64, reason: Option<&str>) -> Result<(), MicupError> {
        #[derive(Serialize)]
        struct BanPayload<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            reason: Option<&'a str>,
        }
        self.put(&format!("/guilds/{guild_id}/bans/{user_id}"), &BanPayload { reason }).await
    }

    /// Kullanıcının yasağını kaldırır (Unban: `DELETE /guilds/{id}/bans/{user_id}`).
    pub async fn unban(&self, guild_id: u64, user_id: u64) -> Result<(), MicupError> {
        self.delete(&format!("/guilds/{guild_id}/bans/{user_id}")).await
    }

    /// Kullanıcıya zaman aşımı / susturma uygular (Timeout: `PATCH /guilds/{id}/members/{user_id}`).
    pub async fn set_timeout(
        &self,
        guild_id: u64,
        user_id: u64,
        until_iso8601: Option<&str>,
    ) -> Result<Member, MicupError> {
        #[derive(Serialize)]
        struct Payload<'a> {
            communication_disabled_until: Option<&'a str>,
        }
        self.patch(
            &format!("/guilds/{guild_id}/members/{user_id}"),
            &Payload {
                communication_disabled_until: until_iso8601,
            },
        )
        .await
    }

    /// Üyeye rol verir (`PUT /guilds/{id}/members/{user_id}/roles/{role_id}`).
    pub async fn add_role(&self, guild_id: u64, user_id: u64, role_id: u64) -> Result<(), MicupError> {
        self.put_empty(&format!("/guilds/{guild_id}/members/{user_id}/roles/{role_id}")).await
    }

    /// Üyeden rol alır (`DELETE /guilds/{id}/members/{user_id}/roles/{role_id}`).
    pub async fn remove_role(&self, guild_id: u64, user_id: u64, role_id: u64) -> Result<(), MicupError> {
        self.delete(&format!("/guilds/{guild_id}/members/{user_id}/roles/{role_id}")).await
    }

    // =========================================================================
    // Mesajlar (Messages)
    // =========================================================================

    /// Bir kanala metin, Embed ve Buton/Menü bileşenleriyle mesaj gönderir.
    pub async fn send_message(
        &self,
        channel_id: u64,
        content: Option<&str>,
        embeds: Vec<Embed>,
        components: Vec<Component>,
    ) -> Result<Value, MicupError> {
        #[derive(Serialize)]
        struct Payload<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            content: Option<&'a str>,
            #[serde(skip_serializing_if = "Vec::is_empty")]
            embeds: Vec<Embed>,
            #[serde(skip_serializing_if = "Vec::is_empty")]
            components: Vec<Component>,
        }
        let p = Payload {
            content,
            embeds,
            components,
        };
        self.post(&format!("/channels/{channel_id}/messages"), &p).await
    }

    /// Mesaj siler (`DELETE /channels/{id}/messages/{message_id}`).
    pub async fn delete_message(&self, channel_id: u64, message_id: u64) -> Result<(), MicupError> {
        self.delete(&format!("/channels/{channel_id}/messages/{message_id}")).await
    }

    /// Toplu mesaj siler (Bulk delete: `POST /channels/{id}/messages/bulk-delete`).
    pub async fn bulk_delete_messages(&self, channel_id: u64, message_ids: &[u64]) -> Result<(), MicupError> {
        #[derive(Serialize)]
        struct Payload<'a> {
            messages: &'a [Id],
        }
        let ids: Vec<Id> = message_ids.iter().map(|id| Id(*id)).collect();
        let _: Value = self
            .post(
                &format!("/channels/{channel_id}/messages/bulk-delete"),
                &Payload { messages: &ids },
            )
            .await?;
        Ok(())
    }

    // =========================================================================
    // Dahili HTTP yardımcıları
    // =========================================================================

    async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, MicupError> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    async fn post<P: Serialize, T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        payload: &P,
    ) -> Result<T, MicupError> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self.http.post(&url).json(payload).send().await?;
        Self::handle_response(resp).await
    }

    async fn patch<P: Serialize, T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        payload: &P,
    ) -> Result<T, MicupError> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self.http.patch(&url).json(payload).send().await?;
        Self::handle_response(resp).await
    }

    async fn put<P: Serialize, T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        payload: &P,
    ) -> Result<T, MicupError> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self.http.put(&url).json(payload).send().await?;
        Self::handle_response(resp).await
    }

    async fn put_empty(&self, path: &str) -> Result<(), MicupError> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self.http.put(&url).header("Content-Length", 0).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(MicupError::Api { status, message: body });
        }
        Ok(())
    }

    async fn delete(&self, path: &str) -> Result<(), MicupError> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self.http.delete(&url).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(MicupError::Api { status, message: body });
        }
        Ok(())
    }

    async fn delete_with_reason(&self, path: &str, reason: Option<&str>) -> Result<(), MicupError> {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.http.delete(&url);
        if let Some(r) = reason {
            req = req.header("X-Audit-Log-Reason", r);
        }
        let resp = req.send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(MicupError::Api { status, message: body });
        }
        Ok(())
    }

    async fn handle_response<T: serde::de::DeserializeOwned>(
        resp: reqwest::Response,
    ) -> Result<T, MicupError> {
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(MicupError::Api {
                status: status.as_u16(),
                message: body,
            });
        }
        let parsed = resp.json::<T>().await?;
        Ok(parsed)
    }
}
