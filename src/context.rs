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
