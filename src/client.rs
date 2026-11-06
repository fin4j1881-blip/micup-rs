use std::time::Duration;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};
use serde::Serialize;
use serde_json::Value;

use crate::embed::Embed;
use crate::error::MicupError;
use crate::models::constants::API_BASE;
use crate::models::{Channel, Guild, Member, User};

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

    /// Mevcut bot kullanıcısını getirir.
    pub async fn current_user(&self) -> Result<User, MicupError> {
        self.get("/users/@me").await
    }

    /// Bir sunucunun detaylı bilgilerini çeker (eksik alanları güvenle doldurur).
    pub async fn get_guild(&self, guild_id: u64) -> Result<Guild, MicupError> {
        self.get(&format!("/guilds/{guild_id}")).await
    }

    /// Sunucudaki kanalları listeler.
    pub async fn get_guild_channels(&self, guild_id: u64) -> Result<Vec<Channel>, MicupError> {
        self.get(&format!("/guilds/{guild_id}/channels")).await
    }

    /// Bir kanala düz metin veya Embed mesajı gönderir.
    pub async fn send_message(
        &self,
        channel_id: u64,
        content: Option<&str>,
        embeds: Vec<Embed>,
    ) -> Result<Value, MicupError> {
        #[derive(Serialize)]
        struct Payload<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            content: Option<&'a str>,
            #[serde(skip_serializing_if = "Vec::is_empty")]
            embeds: Vec<Embed>,
        }
        let p = Payload { content, embeds };
        self.post(&format!("/channels/{channel_id}/messages"), &p).await
    }

    /// Sunucu üyesini çeker.
    pub async fn get_member(&self, guild_id: u64, user_id: u64) -> Result<Member, MicupError> {
        self.get(&format!("/guilds/{guild_id}/members/{user_id}")).await
    }

    /// Kullanıcıya zaman aşımı (Timeout) uygular.
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

    // --- Dahili HTTP yardımcıları ---
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
