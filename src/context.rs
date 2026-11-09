//! Bot Komut ve Olay Bağlamı (Context).
//!
//! Bot geliştiricilerinin mesaj gönderme, etkileşim yanıtlama, ses/kamera kontrolü
//! ve sahne efektlerini tek satırda gerçekleştirebilmesini sağlayan zengin bağlam yapısı.

use std::sync::Arc;

use crate::client::Client;
use crate::components::Component;
use crate::embed::Embed;
use crate::error::MicupError;
use crate::gateway::ShardSender;
use crate::interaction::{Interaction, InteractionResponse, InteractionResponseMessage};
use crate::models::Message;
use crate::stage::{SetVoiceRoomStage, StageBoard, VoiceRoomEffect, VoiceRoomStage};

#[derive(Clone)]
pub struct Context {
    pub client: Arc<Client>,
    pub shard_sender: Option<ShardSender>,
}

impl Context {
    pub fn new(client: Arc<Client>) -> Self {
        Self {
            client,
            shard_sender: None,
        }
    }

    pub fn with_shard_sender(mut self, sender: ShardSender) -> Self {
        self.shard_sender = Some(sender);
        self
    }

    // =========================================================================
    // Mesaj Gönderme ve Yönetimi (Message Sending & Management)
    // =========================================================================

    /// Bir kanala hızlıca düz metin mesajı gönderir.
    pub async fn say(
        &self,
        channel_id: u64,
        content: impl AsRef<str>,
    ) -> Result<Message, MicupError> {
        self.client
            .send_message(channel_id, Some(content.as_ref()), vec![], vec![])
            .await
    }

    /// Bir kanala Embed içeren mesaj gönderir.
    pub async fn send_embed(&self, channel_id: u64, embed: Embed) -> Result<Message, MicupError> {
        self.client
            .send_message(channel_id, None, vec![embed], vec![])
            .await
    }

    /// Bir kanala birden fazla Embed içeren mesaj gönderir.
    pub async fn send_embeds(
        &self,
        channel_id: u64,
        embeds: Vec<Embed>,
    ) -> Result<Message, MicupError> {
        self.client
            .send_message(channel_id, None, embeds, vec![])
            .await
    }

    /// Bir kanala buton veya menü içeren mesaj gönderir.
    pub async fn send_components(
        &self,
        channel_id: u64,
        content: Option<&str>,
        components: Vec<Component>,
    ) -> Result<Message, MicupError> {
        self.client
            .send_message(channel_id, content, vec![], components)
            .await
    }

    /// Bir kanala hem metin, hem Embed hem de bileşen içeren tam teşekküllü mesaj gönderir.
    pub async fn send_message(
        &self,
        channel_id: u64,
        content: Option<&str>,
        embeds: Vec<Embed>,
        components: Vec<Component>,
    ) -> Result<Message, MicupError> {
        self.client
            .send_message(channel_id, content, embeds, components)
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
        self.client
            .send_message_with_files(channel_id, content, embeds, components, files)
            .await
    }

    /// Bir kanaldaki mesajı düzenler.
    pub async fn edit(
        &self,
        channel_id: u64,
        message_id: u64,
        content: impl AsRef<str>,
    ) -> Result<Message, MicupError> {
        self.client
            .edit_message(
                channel_id,
                message_id,
                Some(content.as_ref()),
                vec![],
                vec![],
            )
            .await
    }

    /// Bir kanaldaki mesajı siler.
    pub async fn delete(&self, channel_id: u64, message_id: u64) -> Result<(), MicupError> {
        self.client.delete_message(channel_id, message_id).await
    }

    /// Bir mesaja tepki (emoji) ekler.
    pub async fn react(
        &self,
        channel_id: u64,
        message_id: u64,
        emoji: impl AsRef<str>,
    ) -> Result<(), MicupError> {
        self.client
            .create_reaction(channel_id, message_id, emoji.as_ref())
            .await
    }

    /// Bir kanalda botun yazıyor olduğunu belirtir (`POST /channels/{channel_id}/typing`).
    pub async fn typing(&self, channel_id: u64) -> Result<(), MicupError> {
        self.client.trigger_typing(channel_id).await
    }

    // =========================================================================
    // Etkileşim Yanıtları (Interaction Responses)
    // =========================================================================

    /// Gelen etkileşime hızlıca metin yanıtı verir.
    pub async fn reply(
        &self,
        interaction: &Interaction,
        content: impl Into<String>,
    ) -> Result<(), MicupError> {
        let resp = InteractionResponse::message(InteractionResponseMessage::new().content(content));
        self.client
            .create_interaction_response(interaction.id.get(), &interaction.token, &resp)
            .await
    }

    /// Gelen etkileşime sadece komutu kullananın görebileceği (ephemeral) yanıt verir.
    pub async fn reply_ephemeral(
        &self,
        interaction: &Interaction,
        content: impl Into<String>,
    ) -> Result<(), MicupError> {
        let resp = InteractionResponse::message(
            InteractionResponseMessage::new()
                .content(content)
                .ephemeral(true),
        );
        self.client
            .create_interaction_response(interaction.id.get(), &interaction.token, &resp)
            .await
    }

    /// Gelen etkileşime Embed içeren yanıt verir.
    pub async fn reply_embed(
        &self,
        interaction: &Interaction,
        embed: Embed,
        ephemeral: bool,
    ) -> Result<(), MicupError> {
        let resp = InteractionResponse::message(
            InteractionResponseMessage::new()
                .embed(embed)
                .ephemeral(ephemeral),
        );
        self.client
            .create_interaction_response(interaction.id.get(), &interaction.token, &resp)
            .await
    }

    /// Gelen etkileşime birden fazla Embed içeren yanıt verir.
    pub async fn reply_embeds(
        &self,
        interaction: &Interaction,
        embeds: Vec<Embed>,
        ephemeral: bool,
    ) -> Result<(), MicupError> {
        let resp = InteractionResponse::message(
            InteractionResponseMessage::new()
                .embeds(embeds)
                .ephemeral(ephemeral),
        );
        self.client
            .create_interaction_response(interaction.id.get(), &interaction.token, &resp)
            .await
    }

    /// Gelen etkileşime bileşenler (buton/menü) içeren yanıt verir.
    pub async fn reply_components(
        &self,
        interaction: &Interaction,
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
        self.client
            .create_interaction_response(interaction.id.get(), &interaction.token, &resp)
            .await
    }

    /// Etkileşime yanıt olarak modal (açılır form penceresi) gönderir.
    pub async fn reply_modal(
        &self,
        interaction: &Interaction,
        custom_id: impl Into<String>,
        title: impl Into<String>,
        components: Vec<Component>,
    ) -> Result<(), MicupError> {
        let resp = InteractionResponse::modal(custom_id, title, components);
        self.client
            .create_interaction_response(interaction.id.get(), &interaction.token, &resp)
            .await
    }

    /// 3 saniyelik zaman aşımını aşmak için etkileşimi hemen ertele (Düşünüyor... durumu).
    pub async fn defer(
        &self,
        interaction: &Interaction,
        ephemeral: bool,
    ) -> Result<(), MicupError> {
        let resp = InteractionResponse::defer(ephemeral);
        self.client
            .create_interaction_response(interaction.id.get(), &interaction.token, &resp)
            .await
    }

    /// Buton veya menü etkileşimini defer eder (mesajda yükleniyor göstermez, bileşeni onaylar).
    pub async fn defer_update(&self, interaction: &Interaction) -> Result<(), MicupError> {
        let resp = InteractionResponse::defer_update();
        self.client
            .create_interaction_response(interaction.id.get(), &interaction.token, &resp)
            .await
    }

    /// Buton veya menü etkileşiminin kaynak mesajını günceller.
    pub async fn update_message(
        &self,
        interaction: &Interaction,
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
        let resp = InteractionResponse::update_message(msg);
        self.client
            .create_interaction_response(interaction.id.get(), &interaction.token, &resp)
            .await
    }

    /// Ertelenmiş etkileşime takip mesajı (follow-up) gönderir.
    pub async fn followup(
        &self,
        interaction: &Interaction,
        content: impl Into<String>,
        ephemeral: bool,
    ) -> Result<serde_json::Value, MicupError> {
        let msg = InteractionResponseMessage::new()
            .content(content)
            .ephemeral(ephemeral);
        self.client
            .create_followup_message(interaction.application_id.get(), &interaction.token, &msg)
            .await
    }

    /// Ertelenmiş etkileşime Embed içeren takip mesajı (follow-up) gönderir.
    pub async fn followup_embed(
        &self,
        interaction: &Interaction,
        embed: Embed,
        ephemeral: bool,
    ) -> Result<serde_json::Value, MicupError> {
        let msg = InteractionResponseMessage::new()
            .embed(embed)
            .ephemeral(ephemeral);
        self.client
            .create_followup_message(interaction.application_id.get(), &interaction.token, &msg)
            .await
    }

    /// Ertelenmiş etkileşime bileşenler içeren takip mesajı (follow-up) gönderir.
    pub async fn followup_components(
        &self,
        interaction: &Interaction,
        content: Option<&str>,
        components: Vec<Component>,
        ephemeral: bool,
    ) -> Result<serde_json::Value, MicupError> {
        let mut msg = InteractionResponseMessage::new()
            .components(components)
            .ephemeral(ephemeral);
        if let Some(text) = content {
            msg = msg.content(text);
        }
        self.client
            .create_followup_message(interaction.application_id.get(), &interaction.token, &msg)
            .await
    }

    // =========================================================================
    // Ses ve Kamera Kontrolü (Voice & Camera Controls)
    // =========================================================================

    /// Ses kanalına bağlanır (`Opcode 4`).
    ///
    /// `self_video: true` verilirse Micup arayüzünde botun kamerası açılır.
    /// Bağlantı sağlandığında Gateway'den `VOICE_SERVER_UPDATE` olayı gelir.
    pub fn join_voice(
        &self,
        guild_id: u64,
        channel_id: u64,
        self_mute: bool,
        self_deaf: bool,
        self_video: bool,
    ) -> Result<(), MicupError> {
        if let Some(ref sender) = self.shard_sender {
            sender.join_voice_channel(guild_id, channel_id, self_mute, self_deaf, self_video)
        } else {
            Err(MicupError::Gateway(
                "Shard bağlantısı bulunamadı".to_string(),
            ))
        }
    }

    /// Ses kanalında kamerayı açar veya kapatır (`self_video`).
    pub fn set_camera(
        &self,
        guild_id: u64,
        channel_id: u64,
        enabled: bool,
    ) -> Result<(), MicupError> {
        self.join_voice(guild_id, channel_id, false, false, enabled)
    }

    /// Ses kanalından ayrılır.
    pub fn leave_voice(&self, guild_id: u64) -> Result<(), MicupError> {
        if let Some(ref sender) = self.shard_sender {
            sender.leave_voice_channel(guild_id)
        } else {
            Err(MicupError::Gateway(
                "Shard bağlantısı bulunamadı".to_string(),
            ))
        }
    }

    // =========================================================================
    // 3B Oda Sahnesi Kısayolları (Voice Room Stage Shortcuts)
    // =========================================================================

    /// 3B Odaya tek satırda TV panosu kurar.
    pub async fn set_stage_board(
        &self,
        channel_id: u64,
        title: &str,
        text: &str,
    ) -> Result<VoiceRoomStage, MicupError> {
        let board = StageBoard::new().title(title).text(text);
        let stage = SetVoiceRoomStage::new().board(board);
        self.client.set_voice_room_stage(channel_id, &stage).await
    }

    /// 3B Odaya konfeti yağdırır.
    pub async fn stage_confetti(&self, channel_id: u64) -> Result<(), MicupError> {
        self.client
            .send_voice_room_effect(channel_id, &VoiceRoomEffect::confetti())
            .await
    }

    /// 3B Odaya spot ışığı tutar.
    pub async fn stage_spotlight(&self, channel_id: u64, user_id: u64) -> Result<(), MicupError> {
        self.client
            .send_voice_room_effect(channel_id, &VoiceRoomEffect::spotlight(user_id))
            .await
    }

    /// 3B Odadaki sunucu NPC karakterine dans ettirir.
    pub async fn stage_dance(&self, channel_id: u64) -> Result<(), MicupError> {
        self.client
            .send_voice_room_effect(channel_id, &VoiceRoomEffect::dance())
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_construction() {
        let client = Arc::new(Client::new("test_token").unwrap());
        let ctx = Context::new(client);
        assert!(ctx.shard_sender.is_none());

        let shard = crate::gateway::Shard::new("test_token", 0);
        let ctx = ctx.with_shard_sender(shard.sender());
        assert!(ctx.shard_sender.is_some());
    }

    #[tokio::test]
    async fn test_context_voice_without_shard() {
        let client = Arc::new(Client::new("test_token").unwrap());
        let ctx = Context::new(client);
        assert!(ctx.join_voice(1, 2, false, false, false).is_err());
        assert!(ctx.set_camera(1, 2, true).is_err());
        assert!(ctx.leave_voice(1).is_err());
    }

    #[test]
    fn test_context_voice_with_shard() {
        let client = Arc::new(Client::new("test_token").unwrap());
        let shard = crate::gateway::Shard::new("test_token", 0);
        let ctx = Context::new(client).with_shard_sender(shard.sender());
        assert!(ctx.join_voice(1, 2, false, false, true).is_ok());
        assert!(ctx.set_camera(1, 2, false).is_ok());
        assert!(ctx.leave_voice(1).is_ok());
    }
}
