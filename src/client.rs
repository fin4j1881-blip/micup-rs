use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, RETRY_AFTER, USER_AGENT};
use serde::Serialize;
use serde_json::Value;
use std::time::Duration;

use crate::components::Component;
use crate::embed::Embed;
use crate::error::MicupError;
use crate::models::constants::API_BASE;
use crate::models::{
    Channel, Emoji, FluxerDiscovery, GatewayBotInfo, Guild, Id, Invite, Member, Message, Role,
    User, Webhook,
};
use crate::stage::{SetVoiceRoomStage, VoiceRoomEffect, VoiceRoomStage, VoiceRoomStageResponse};

fn url_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

pub type RestClient = Client;
pub type MicupClient = Client;
pub type Error = MicupError;

#[derive(Clone)]
pub struct Client {
    token: String,
    http: reqwest::Client,
    base_url: String,
    max_retries: u32,
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
        headers.insert(USER_AGENT, HeaderValue::from_static("micup-rs/0.2.0"));

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(15))
            .build()?;

        Ok(Self {
            token,
            http,
            base_url: API_BASE.to_string(),
            max_retries: 3,
        })
    }

    /// Hız sınırı (429 Rate Limit) durumunda otomatik tekrar deneme sayısını ayarlar.
    pub fn with_max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }

    /// Hız sınırı durumunda yapılandırılmış maksimum tekrar deneme sayısını döner.
    pub fn max_retries(&self) -> u32 {
        self.max_retries
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    // =========================================================================
    // Keşif ve Ağ Geçidi Bilgisi (Discovery & Gateway Info)
    // =========================================================================

    /// Micup platform keşif yapılandırmasını getirir (`GET /.well-known/fluxer`).
    pub async fn get_discovery(&self) -> Result<FluxerDiscovery, MicupError> {
        let resp = self
            .execute_with_retry(|| self.http.get("https://micup.gg/.well-known/fluxer"))
            .await?;
        Self::handle_response(resp).await
    }

    /// Bot için önerilen ağ geçidi adresini ve shard limitlerini çeker (`GET /gateway/bot`).
    pub async fn get_gateway_bot(&self) -> Result<GatewayBotInfo, MicupError> {
        self.get("/gateway/bot").await
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

    /// Botun kendi profilini (kullanıcı adı, avatar, afiş) günceller (`PATCH /users/@me`).
    ///
    /// Micup kuralı: Bot adını saatte en fazla iki kez değiştirebilir.
    pub async fn edit_current_user(
        &self,
        username: Option<&str>,
        avatar: Option<&str>,
        banner: Option<&str>,
    ) -> Result<User, MicupError> {
        #[derive(Serialize)]
        struct Payload<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            username: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            avatar: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            banner: Option<&'a str>,
        }
        self.patch(
            "/users/@me",
            &Payload {
                username,
                avatar,
                banner,
            },
        )
        .await
    }

    // =========================================================================
    // Sunucular (Guilds) ve Kanallar
    // =========================================================================

    /// Bir sunucunun detaylı bilgilerini çeker (`GET /guilds/{id}`).
    pub async fn get_guild(&self, guild_id: u64) -> Result<Guild, MicupError> {
        self.get(&format!("/guilds/{guild_id}")).await
    }

    /// Sunucudaki kanalları listeler (`GET /guilds/{id}/channels`).
    pub async fn get_guild_channels(&self, guild_id: u64) -> Result<Vec<Channel>, MicupError> {
        self.get(&format!("/guilds/{guild_id}/channels")).await
    }

    /// Belirtilen kanalı getirir (`GET /channels/{id}`).
    pub async fn get_channel(&self, channel_id: u64) -> Result<Channel, MicupError> {
        self.get(&format!("/channels/{channel_id}")).await
    }

    /// Kanalı siler (`DELETE /channels/{id}`).
    pub async fn delete_channel(&self, channel_id: u64) -> Result<Channel, MicupError> {
        self.delete_json(&format!("/channels/{channel_id}")).await
    }

    /// Kanala özel davet oluşturur (`POST /channels/{id}/invites`).
    pub async fn create_channel_invite(
        &self,
        channel_id: u64,
        max_age_seconds: Option<u64>,
        max_uses: Option<u64>,
    ) -> Result<Invite, MicupError> {
        #[derive(Serialize)]
        struct InvitePayload {
            #[serde(skip_serializing_if = "Option::is_none")]
            max_age: Option<u64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            max_uses: Option<u64>,
        }
        self.post(
            &format!("/channels/{channel_id}/invites"),
            &InvitePayload {
                max_age: max_age_seconds,
                max_uses,
            },
        )
        .await
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
    // Roller (Roles)
    // =========================================================================

    /// Sunucudaki rolleri listeler (`GET /guilds/{id}/roles`).
    pub async fn get_guild_roles(&self, guild_id: u64) -> Result<Vec<Role>, MicupError> {
        self.get(&format!("/guilds/{guild_id}/roles")).await
    }

    /// Sunucuda yeni rol oluşturur (`POST /guilds/{id}/roles`).
    pub async fn create_role(
        &self,
        guild_id: u64,
        name: &str,
        color: Option<u64>,
        hoist: bool,
        mentionable: bool,
    ) -> Result<Role, MicupError> {
        #[derive(Serialize)]
        struct CreateRolePayload<'a> {
            name: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            color: Option<u64>,
            hoist: bool,
            mentionable: bool,
        }
        self.post(
            &format!("/guilds/{guild_id}/roles"),
            &CreateRolePayload {
                name,
                color,
                hoist,
                mentionable,
            },
        )
        .await
    }

    /// Sunucudan rol siler (`DELETE /guilds/{id}/roles/{role_id}`).
    pub async fn delete_role(&self, guild_id: u64, role_id: u64) -> Result<(), MicupError> {
        self.delete(&format!("/guilds/{guild_id}/roles/{role_id}"))
            .await
    }

    // =========================================================================
    // Emojiler (Emojis)
    // =========================================================================

    /// Sunucu emojilerini listeler (`GET /guilds/{id}/emojis`).
    pub async fn get_guild_emojis(&self, guild_id: u64) -> Result<Vec<Emoji>, MicupError> {
        self.get(&format!("/guilds/{guild_id}/emojis")).await
    }

    /// Sunucudan belirli bir emojiyi çeker (`GET /guilds/{id}/emojis/{emoji_id}`).
    pub async fn get_guild_emoji(&self, guild_id: u64, emoji_id: u64) -> Result<Emoji, MicupError> {
        self.get(&format!("/guilds/{guild_id}/emojis/{emoji_id}"))
            .await
    }

    /// Sunucudan emoji siler (`DELETE /guilds/{id}/emojis/{emoji_id}`).
    pub async fn delete_guild_emoji(&self, guild_id: u64, emoji_id: u64) -> Result<(), MicupError> {
        self.delete(&format!("/guilds/{guild_id}/emojis/{emoji_id}"))
            .await
    }

    // =========================================================================
    // Üyeler ve Moderasyon (Members & Moderation)
    // =========================================================================

    /// Sunucu üyesini çeker (`GET /guilds/{id}/members/{user_id}`).
    pub async fn get_member(&self, guild_id: u64, user_id: u64) -> Result<Member, MicupError> {
        self.get(&format!("/guilds/{guild_id}/members/{user_id}"))
            .await
    }

    /// Sunucu üyelerini ada göre arar (`GET /guilds/{id}/members/search`).
    ///
    /// Micup'ta botlar sunucu başına saniyede 1 arama yapabilir.
    pub async fn search_guild_members(
        &self,
        guild_id: u64,
        query: &str,
        limit: Option<u64>,
    ) -> Result<Vec<Member>, MicupError> {
        let lim = limit.unwrap_or(20).min(100);
        let encoded = url_encode(query);
        self.get(&format!(
            "/guilds/{guild_id}/members/search?query={encoded}&limit={lim}"
        ))
        .await
    }

    /// Sunucudan üye atar (Kick: `DELETE /guilds/{id}/members/{user_id}`).
    pub async fn kick(
        &self,
        guild_id: u64,
        user_id: u64,
        reason: Option<&str>,
    ) -> Result<(), MicupError> {
        let path = format!("/guilds/{guild_id}/members/{user_id}");
        self.delete_with_reason(&path, reason).await
    }

    /// Sunucudan üyeyi yasaklar (Ban: `PUT /guilds/{id}/bans/{user_id}`).
    pub async fn ban(
        &self,
        guild_id: u64,
        user_id: u64,
        reason: Option<&str>,
    ) -> Result<(), MicupError> {
        #[derive(Serialize)]
        struct BanPayload<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            reason: Option<&'a str>,
        }
        self.put(
            &format!("/guilds/{guild_id}/bans/{user_id}"),
            &BanPayload { reason },
        )
        .await
    }

    /// Kullanıcının yasağını kaldırır (Unban: `DELETE /guilds/{id}/bans/{user_id}`).
    pub async fn unban(&self, guild_id: u64, user_id: u64) -> Result<(), MicupError> {
        self.delete(&format!("/guilds/{guild_id}/bans/{user_id}"))
            .await
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
    pub async fn add_role(
        &self,
        guild_id: u64,
        user_id: u64,
        role_id: u64,
    ) -> Result<(), MicupError> {
        self.put_empty(&format!(
            "/guilds/{guild_id}/members/{user_id}/roles/{role_id}"
        ))
        .await
    }

    /// Üyeden rol alır (`DELETE /guilds/{id}/members/{user_id}/roles/{role_id}`).
    pub async fn remove_role(
        &self,
        guild_id: u64,
        user_id: u64,
        role_id: u64,
    ) -> Result<(), MicupError> {
        self.delete(&format!(
            "/guilds/{guild_id}/members/{user_id}/roles/{role_id}"
        ))
        .await
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
    ) -> Result<Message, MicupError> {
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
        self.post(&format!("/channels/{channel_id}/messages"), &p)
            .await
    }

    /// Dosya ekleriyle birlikte mesaj gönderir (`multipart/form-data`).
    pub async fn send_message_with_files(
        &self,
        channel_id: u64,
        content: Option<&str>,
        embeds: Vec<Embed>,
        components: Vec<Component>,
        files: Vec<(&str, Vec<u8>)>,
    ) -> Result<Message, MicupError> {
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
        let payload_json = serde_json::to_string(&p)?;

        let build_form = || {
            let mut form = reqwest::multipart::Form::new();
            form = form.text("payload_json", payload_json.clone());
            for (i, (filename, bytes)) in files.iter().enumerate() {
                let part =
                    reqwest::multipart::Part::bytes(bytes.clone()).file_name(filename.to_string());
                form = form.part(format!("files[{i}]"), part);
            }
            form
        };

        let url = format!("{}/channels/{channel_id}/messages", self.base_url);
        let resp = self
            .execute_with_retry(|| self.http.post(&url).multipart(build_form()))
            .await?;
        Self::handle_response(resp).await
    }

    /// Mesaj detayını çeker (`GET /channels/{id}/messages/{message_id}`).
    pub async fn get_message(
        &self,
        channel_id: u64,
        message_id: u64,
    ) -> Result<Message, MicupError> {
        self.get(&format!("/channels/{channel_id}/messages/{message_id}"))
            .await
    }

    /// Kanaldaki son mesajları listeler (`GET /channels/{id}/messages?limit={limit}`).
    pub async fn get_channel_messages(
        &self,
        channel_id: u64,
        limit: Option<u64>,
    ) -> Result<Vec<Message>, MicupError> {
        let lim = limit.unwrap_or(50).min(100);
        self.get(&format!("/channels/{channel_id}/messages?limit={lim}"))
            .await
    }

    /// Mesajı düzenler (`PATCH /channels/{id}/messages/{message_id}`).
    pub async fn edit_message(
        &self,
        channel_id: u64,
        message_id: u64,
        content: Option<&str>,
        embeds: Vec<Embed>,
        components: Vec<Component>,
    ) -> Result<Message, MicupError> {
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
        self.patch(&format!("/channels/{channel_id}/messages/{message_id}"), &p)
            .await
    }

    /// Mesaj siler (`DELETE /channels/{id}/messages/{message_id}`).
    pub async fn delete_message(&self, channel_id: u64, message_id: u64) -> Result<(), MicupError> {
        self.delete(&format!("/channels/{channel_id}/messages/{message_id}"))
            .await
    }

    /// Toplu mesaj siler (Bulk delete: `POST /channels/{id}/messages/bulk-delete`).
    pub async fn bulk_delete_messages(
        &self,
        channel_id: u64,
        message_ids: &[u64],
    ) -> Result<(), MicupError> {
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

    /// Bir kanalda botun yazıyor olduğunu tetikler (`POST /channels/{id}/typing`).
    pub async fn trigger_typing(&self, channel_id: u64) -> Result<(), MicupError> {
        self.post_empty(&format!("/channels/{channel_id}/typing"))
            .await
    }

    // =========================================================================
    // Webhook İşlemleri
    // =========================================================================

    /// Webhook üzerinden mesaj gönderir (`POST /webhooks/{id}/{token}`).
    pub async fn execute_webhook(
        &self,
        webhook_id: u64,
        webhook_token: &str,
        content: Option<&str>,
        embeds: Vec<Embed>,
        components: Vec<Component>,
        wait: bool,
    ) -> Result<Option<Message>, MicupError> {
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
        let wait_param = if wait { "?wait=true" } else { "" };
        let url = format!("/webhooks/{webhook_id}/{webhook_token}{wait_param}");
        if wait {
            let msg: Message = self.post(&url, &p).await?;
            Ok(Some(msg))
        } else {
            let _: Value = self.post(&url, &p).await?;
            Ok(None)
        }
    }

    /// Belirtilen kanaldaki webhook'ları çeker (`GET /channels/{id}/webhooks`).
    pub async fn get_channel_webhooks(&self, channel_id: u64) -> Result<Vec<Webhook>, MicupError> {
        self.get(&format!("/channels/{channel_id}/webhooks")).await
    }

    /// Yeni bir webhook oluşturur (`POST /channels/{id}/webhooks`).
    pub async fn create_webhook(
        &self,
        channel_id: u64,
        name: &str,
        avatar: Option<&str>,
    ) -> Result<Webhook, MicupError> {
        #[derive(Serialize)]
        struct CreateWebhookPayload<'a> {
            name: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            avatar: Option<&'a str>,
        }
        self.post(
            &format!("/channels/{channel_id}/webhooks"),
            &CreateWebhookPayload { name, avatar },
        )
        .await
    }

    /// Bir webhook'u siler (`DELETE /webhooks/{id}`).
    pub async fn delete_webhook(&self, webhook_id: u64) -> Result<(), MicupError> {
        self.delete(&format!("/webhooks/{webhook_id}")).await
    }

    // =========================================================================
    // Tepkiler (Reactions)
    // =========================================================================

    /// Mesaja tepki ekler (`PUT /channels/{id}/messages/{message_id}/reactions/{emoji}/@me`).
    pub async fn create_reaction(
        &self,
        channel_id: u64,
        message_id: u64,
        emoji: &str,
    ) -> Result<(), MicupError> {
        let enc = url_encode(emoji);
        self.put_empty(&format!(
            "/channels/{channel_id}/messages/{message_id}/reactions/{enc}/@me"
        ))
        .await
    }

    /// Mesaja tepki ekler (`PUT /channels/{id}/messages/{message_id}/reactions/{emoji}/@me`).
    pub async fn add_reaction(
        &self,
        channel_id: u64,
        message_id: u64,
        emoji: &str,
    ) -> Result<(), MicupError> {
        self.create_reaction(channel_id, message_id, emoji).await
    }

    /// Kendi tepkisini siler (`DELETE /channels/{id}/messages/{message_id}/reactions/{emoji}/@me`).
    pub async fn delete_own_reaction(
        &self,
        channel_id: u64,
        message_id: u64,
        emoji: &str,
    ) -> Result<(), MicupError> {
        let enc = url_encode(emoji);
        self.delete(&format!(
            "/channels/{channel_id}/messages/{message_id}/reactions/{enc}/@me"
        ))
        .await
    }

    /// Kendi tepkisini siler (`DELETE /channels/{id}/messages/{message_id}/reactions/{emoji}/@me`).
    pub async fn remove_reaction(
        &self,
        channel_id: u64,
        message_id: u64,
        emoji: &str,
    ) -> Result<(), MicupError> {
        self.delete_own_reaction(channel_id, message_id, emoji)
            .await
    }

    /// Bir kullanıcının tepkisini siler (`DELETE /channels/{id}/messages/{message_id}/reactions/{emoji}/{user_id}`).
    pub async fn delete_user_reaction(
        &self,
        channel_id: u64,
        message_id: u64,
        emoji: &str,
        user_id: u64,
    ) -> Result<(), MicupError> {
        let enc = url_encode(emoji);
        self.delete(&format!(
            "/channels/{channel_id}/messages/{message_id}/reactions/{enc}/{user_id}"
        ))
        .await
    }

    /// Bir kullanıcının tepkisini siler (`DELETE /channels/{id}/messages/{message_id}/reactions/{emoji}/{user_id}`).
    pub async fn remove_user_reaction(
        &self,
        channel_id: u64,
        message_id: u64,
        emoji: &str,
        user_id: u64,
    ) -> Result<(), MicupError> {
        self.delete_user_reaction(channel_id, message_id, emoji, user_id)
            .await
    }

    /// Bir tepkiyi veren kullanıcıları listeler (`GET /channels/{id}/messages/{message_id}/reactions/{emoji}`).
    pub async fn get_reactions(
        &self,
        channel_id: u64,
        message_id: u64,
        emoji: &str,
        limit: Option<u64>,
    ) -> Result<Vec<User>, MicupError> {
        let enc = url_encode(emoji);
        let lim = limit.unwrap_or(25).min(100);
        self.get(&format!(
            "/channels/{channel_id}/messages/{message_id}/reactions/{enc}?limit={lim}"
        ))
        .await
    }

    // =========================================================================
    // Sabitleme (Pins)
    // =========================================================================

    /// Mesajı kanalda sabitler (`PUT /channels/{id}/pins/{message_id}`).
    pub async fn pin_message(&self, channel_id: u64, message_id: u64) -> Result<(), MicupError> {
        self.put_empty(&format!("/channels/{channel_id}/pins/{message_id}"))
            .await
    }

    /// Sabitlenmiş mesajı kaldırır (`DELETE /channels/{id}/pins/{message_id}`).
    pub async fn unpin_message(&self, channel_id: u64, message_id: u64) -> Result<(), MicupError> {
        self.delete(&format!("/channels/{channel_id}/pins/{message_id}"))
            .await
    }

    /// Sabitlenmiş mesajları listeler (`GET /channels/{id}/pins`).
    pub async fn get_pinned_messages(&self, channel_id: u64) -> Result<Vec<Message>, MicupError> {
        self.get(&format!("/channels/{channel_id}/pins")).await
    }

    // =========================================================================
    // 3B Oda Sahnesi (3D Voice Room Stage)
    // =========================================================================

    /// Bir sesli kanalın 3B oda sahnesini getirir (`GET /channels/{id}/voice-room/stage`).
    ///
    /// Kanalda kimse yoksa veya sahne kurulmamışsa `Ok(None)` döner.
    pub async fn get_voice_room_stage(
        &self,
        channel_id: u64,
    ) -> Result<Option<VoiceRoomStage>, MicupError> {
        let resp: VoiceRoomStageResponse = self
            .get(&format!("/channels/{channel_id}/voice-room/stage"))
            .await?;
        Ok(resp.stage)
    }

    /// Bir sesli kanalın 3B oda sahnesini kurar veya günceller (`PUT /channels/{id}/voice-room/stage`).
    ///
    /// Bot odada NPC karakteri olarak görünür, TV panosu veya YouTube videosu gösterir.
    pub async fn set_voice_room_stage(
        &self,
        channel_id: u64,
        stage: &SetVoiceRoomStage,
    ) -> Result<VoiceRoomStage, MicupError> {
        self.put(&format!("/channels/{channel_id}/voice-room/stage"), stage)
            .await
    }

    /// 3B oda sahnesini temizler / kaldırır (`DELETE /channels/{id}/voice-room/stage`).
    pub async fn clear_voice_room_stage(&self, channel_id: u64) -> Result<(), MicupError> {
        self.delete(&format!("/channels/{channel_id}/voice-room/stage"))
            .await
    }

    /// 3B odadaki herkese tek seferlik efekt / an gönderir (`POST /channels/{id}/voice-room/stage/effects`).
    ///
    /// Konfeti, spot ışığı (`Spotlight`) veya sunucu karakter hareketi (`Wave`, `Dance`, `Jump`, `Emoji`).
    pub async fn send_voice_room_effect(
        &self,
        channel_id: u64,
        effect: &VoiceRoomEffect,
    ) -> Result<(), MicupError> {
        let url = format!(
            "{}/channels/{channel_id}/voice-room/stage/effects",
            self.base_url
        );
        let resp = self
            .execute_with_retry(|| self.http.post(&url).json(effect))
            .await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            if status == 429 {
                return Err(MicupError::RateLimited(format!(
                    "429 Too Many Requests: {body}"
                )));
            }
            return Err(MicupError::Api {
                status,
                message: body,
            });
        }
        Ok(())
    }

    // =========================================================================
    // Slash Komutları (Application Commands)
    // =========================================================================

    /// Global slash komutlarını listeler (`GET /applications/{app_id}/commands`).
    pub async fn get_global_commands(
        &self,
        application_id: u64,
    ) -> Result<Vec<crate::commands::Command>, MicupError> {
        self.get(&format!("/applications/{application_id}/commands"))
            .await
    }

    /// Global slash komutlarını toplu olarak günceller / kaydeder (`PUT /applications/{app_id}/commands`).
    pub async fn set_global_commands(
        &self,
        application_id: u64,
        commands: &[crate::commands::CreateCommand],
    ) -> Result<Vec<crate::commands::Command>, MicupError> {
        self.put(
            &format!("/applications/{application_id}/commands"),
            &commands,
        )
        .await
    }

    /// Sunucuya özel slash komutlarını listeler (`GET /applications/{app_id}/guilds/{guild_id}/commands`).
    pub async fn get_guild_commands(
        &self,
        application_id: u64,
        guild_id: u64,
    ) -> Result<Vec<crate::commands::Command>, MicupError> {
        self.get(&format!(
            "/applications/{application_id}/guilds/{guild_id}/commands"
        ))
        .await
    }

    /// Sunucuya özel slash komutlarını toplu günceller (`PUT /applications/{app_id}/guilds/{guild_id}/commands`).
    pub async fn set_guild_commands(
        &self,
        application_id: u64,
        guild_id: u64,
        commands: &[crate::commands::CreateCommand],
    ) -> Result<Vec<crate::commands::Command>, MicupError> {
        self.put(
            &format!("/applications/{application_id}/guilds/{guild_id}/commands"),
            &commands,
        )
        .await
    }

    // =========================================================================
    // Etkileşim Yanıtları (Interaction Responses)
    // =========================================================================

    /// Gelen etkileşime (Slash komut veya buton) ilk yanıtı verir (`POST /interactions/{id}/{token}/callback`).
    pub async fn create_interaction_response(
        &self,
        interaction_id: u64,
        interaction_token: &str,
        response: &crate::interaction::InteractionResponse,
    ) -> Result<(), MicupError> {
        let url = format!(
            "{}/interactions/{interaction_id}/{interaction_token}/callback",
            self.base_url
        );
        let resp = self
            .execute_with_retry(|| self.http.post(&url).json(response))
            .await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            if status == 429 {
                return Err(MicupError::RateLimited(format!(
                    "429 Too Many Requests: {body}"
                )));
            }
            return Err(MicupError::Api {
                status,
                message: body,
            });
        }
        Ok(())
    }

    /// Defer edilmiş etkileşime sonradan takip yanıtı (followup) gönderir.
    pub async fn create_followup_message(
        &self,
        application_id: u64,
        interaction_token: &str,
        message: &crate::interaction::InteractionResponseMessage,
    ) -> Result<Value, MicupError> {
        self.post(
            &format!("/webhooks/{application_id}/{interaction_token}"),
            message,
        )
        .await
    }

    // =========================================================================
    // Denetim Kaydı, Yasaklar ve Davetler (Audit Logs, Bans, Invites)
    // =========================================================================

    /// Sunucu denetim kaydını çeker (`GET /guilds/{id}/audit-logs`).
    pub async fn get_audit_logs(
        &self,
        guild_id: u64,
        limit: Option<u64>,
    ) -> Result<crate::models::AuditLog, MicupError> {
        let lim = limit.unwrap_or(50);
        self.get(&format!("/guilds/{guild_id}/audit-logs?limit={lim}"))
            .await
    }

    /// Sunucudaki yasaklı kullanıcıları listeler (`GET /guilds/{id}/bans`).
    pub async fn get_bans(&self, guild_id: u64) -> Result<Vec<crate::models::Ban>, MicupError> {
        self.get(&format!("/guilds/{guild_id}/bans")).await
    }

    /// Sunucudaki davetleri listeler (`GET /guilds/{id}/invites`).
    pub async fn get_guild_invites(
        &self,
        guild_id: u64,
    ) -> Result<Vec<crate::models::Invite>, MicupError> {
        self.get(&format!("/guilds/{guild_id}/invites")).await
    }

    // =========================================================================
    // Dahili HTTP yardımcıları (429 Hız Sınırı Korumalı)
    // =========================================================================

    async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, MicupError> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self.execute_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    async fn post<P: Serialize, T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        payload: &P,
    ) -> Result<T, MicupError> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .execute_with_retry(|| self.http.post(&url).json(payload))
            .await?;
        Self::handle_response(resp).await
    }

    async fn patch<P: Serialize, T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        payload: &P,
    ) -> Result<T, MicupError> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .execute_with_retry(|| self.http.patch(&url).json(payload))
            .await?;
        Self::handle_response(resp).await
    }

    async fn put<P: Serialize, T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        payload: &P,
    ) -> Result<T, MicupError> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .execute_with_retry(|| self.http.put(&url).json(payload))
            .await?;
        Self::handle_response(resp).await
    }

    async fn put_empty(&self, path: &str) -> Result<(), MicupError> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .execute_with_retry(|| self.http.put(&url).header("Content-Length", 0))
            .await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            if status == 429 {
                return Err(MicupError::RateLimited(format!(
                    "429 Too Many Requests: {body}"
                )));
            }
            return Err(MicupError::Api {
                status,
                message: body,
            });
        }
        Ok(())
    }

    async fn post_empty(&self, path: &str) -> Result<(), MicupError> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .execute_with_retry(|| self.http.post(&url).header("Content-Length", 0))
            .await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            if status == 429 {
                return Err(MicupError::RateLimited(format!(
                    "429 Too Many Requests: {body}"
                )));
            }
            return Err(MicupError::Api {
                status,
                message: body,
            });
        }
        Ok(())
    }

    async fn delete(&self, path: &str) -> Result<(), MicupError> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self.execute_with_retry(|| self.http.delete(&url)).await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            if status == 429 {
                return Err(MicupError::RateLimited(format!(
                    "429 Too Many Requests: {body}"
                )));
            }
            return Err(MicupError::Api {
                status,
                message: body,
            });
        }
        Ok(())
    }

    async fn delete_json<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T, MicupError> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self.execute_with_retry(|| self.http.delete(&url)).await?;
        Self::handle_response(resp).await
    }

    async fn delete_with_reason(&self, path: &str, reason: Option<&str>) -> Result<(), MicupError> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .execute_with_retry(|| {
                let mut req = self.http.delete(&url);
                if let Some(r) = reason {
                    req = req.header("X-Audit-Log-Reason", r);
                }
                req
            })
            .await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            if status == 429 {
                return Err(MicupError::RateLimited(format!(
                    "429 Too Many Requests: {body}"
                )));
            }
            return Err(MicupError::Api {
                status,
                message: body,
            });
        }
        Ok(())
    }

    /// 429 yanıtında `Retry-After` başlığını asenkron bekleme süresine dönüştürür.
    pub(crate) fn parse_retry_after(resp: &reqwest::Response) -> Duration {
        if let Some(header_val) = resp.headers().get(RETRY_AFTER) {
            if let Ok(s) = header_val.to_str() {
                if let Ok(secs) = s.trim().parse::<f64>() {
                    if secs > 0.0 && secs.is_finite() {
                        return Duration::from_millis((secs * 1000.0).ceil() as u64);
                    }
                }
            }
        }
        // Başlık bulunamazsa veya geçersizse varsayılan 1 saniye bekle
        Duration::from_secs(1)
    }

    async fn execute_with_retry<F>(&self, make_req: F) -> Result<reqwest::Response, MicupError>
    where
        F: Fn() -> reqwest::RequestBuilder,
    {
        let mut retries = 0;
        loop {
            let resp = make_req().send().await?;
            let status = resp.status();
            if status.as_u16() == 429 {
                if retries < self.max_retries {
                    let duration = Self::parse_retry_after(&resp);
                    let wait_secs = duration.as_secs_f64();
                    tracing::warn!(
                        retry_after_secs = wait_secs,
                        retries = retries + 1,
                        max_retries = self.max_retries,
                        "Micup API 429 Hız Sınırı: bekleniyor ve tekrar denenecek..."
                    );
                    tokio::time::sleep(duration).await;
                    retries += 1;
                    continue;
                } else {
                    let body = resp.text().await.unwrap_or_default();
                    let msg = if body.is_empty() {
                        "429 Too Many Requests (maksimum yeniden deneme sayısına ulaşıldı)"
                            .to_string()
                    } else {
                        format!("429 Too Many Requests: {body}")
                    };
                    return Err(MicupError::RateLimited(msg));
                }
            }
            return Ok(resp);
        }
    }

    async fn handle_response<T: serde::de::DeserializeOwned>(
        resp: reqwest::Response,
    ) -> Result<T, MicupError> {
        let status = resp.status();
        if status.as_u16() == 429 {
            let body = resp.text().await.unwrap_or_default();
            return Err(MicupError::RateLimited(format!(
                "429 Too Many Requests: {body}"
            )));
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_builder_and_retries() {
        let client = Client::new("test_token").unwrap();
        assert_eq!(client.max_retries(), 3);
        let client = client.with_max_retries(5);
        assert_eq!(client.max_retries(), 5);
        assert_eq!(client.token(), "test_token");
    }

    #[tokio::test]
    async fn test_rate_limit_retry_success() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();

        tokio::spawn(async move {
            // İlk istek 429 döner
            if let Ok((mut socket, _)) = listener.accept().await {
                let mut buf = [0u8; 1024];
                let _ = socket.read(&mut buf).await;
                let response = "HTTP/1.1 429 Too Many Requests\r\nRetry-After: 0.02\r\nContent-Length: 12\r\n\r\nrate limited";
                let _ = socket.write_all(response.as_bytes()).await;
            }
            // İkinci istek 200 OK döner
            if let Ok((mut socket, _)) = listener.accept().await {
                let mut buf = [0u8; 1024];
                let _ = socket.read(&mut buf).await;
                let body =
                    r#"{"id":"100","username":"retry_bot","discriminator":"0001","avatar":null}"#;
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(),
                    body
                );
                let _ = socket.write_all(response.as_bytes()).await;
            }
        });

        let mut client = Client::new("test_token").unwrap();
        client.base_url = format!("http://127.0.0.1:{port}");
        let client = client.with_max_retries(2);

        let user: User = client
            .get("/users/@me")
            .await
            .expect("Yeniden deneme ile başarılı olmalı");
        assert_eq!(user.username, "retry_bot");
    }

    #[tokio::test]
    async fn test_rate_limit_retry_and_exceeded() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();

        tokio::spawn(async move {
            for _ in 0..3 {
                if let Ok((mut socket, _)) = listener.accept().await {
                    let mut buf = [0u8; 1024];
                    let _ = socket.read(&mut buf).await;
                    let response = "HTTP/1.1 429 Too Many Requests\r\nRetry-After: 0.01\r\nContent-Length: 12\r\n\r\nrate limited";
                    let _ = socket.write_all(response.as_bytes()).await;
                }
            }
        });

        let mut client = Client::new("test_token").unwrap();
        client.base_url = format!("http://127.0.0.1:{port}");
        let client = client.with_max_retries(1);

        let res: Result<User, MicupError> = client.get("/users/@me").await;
        assert!(res.is_err());
        match res.unwrap_err() {
            MicupError::RateLimited(msg) => {
                assert!(msg.contains("429"));
            }
            other => panic!("Beklenen RateLimited hatası, alınan: {:?}", other),
        }
    }
}
