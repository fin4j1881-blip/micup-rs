//! Micup.gg / Fluxer platformu için resmi olmayan, Micup API ve CDN uyumlu Rust kütüphanesi.
//!
//! # Neden micup-rs?
//! Standart Discord kütüphaneleri (serenity, twilight vb.) Micup üzerinde kullanılırken:
//! - CDN istekleri `cdn.discordapp.com`'a gider ve görseller kırılır.
//! - Micup'ın dönmediği `preferred_locale` ve takviye çubuğu gibi alanlar yüzünden `Guild` modelleri çöker.
//! - Boş buton veya menü dizilerinde API `Invalid form body` hatası verir.
//! - Embed alanlarında 1024 karakter ve 256 başlık sınırı manuel yönetilmezse hata alınır.
//! - 3B Oda Sahnesi (`voice-room/stage`) ve `MANAGE_ROOM_STAGE` (`1 << 62`) gibi Micup'a özgü sistemleri tanımazlar.
//! - Gateway olaylarında tür güvenli `Event` enum ayrıştırması ve otomatik 429 backoff desteği sunulmaz.
//!
//! `micup-rs`, tüm bu platform uyumsuzluklarını ve özel sistemleri dahili olarak çözer.

pub mod bot;
pub mod cdn;
pub mod client;
pub mod commands;
pub mod components;
pub mod context;
pub mod embed;
pub mod error;
pub mod events;
pub mod gateway;
pub mod intents;
pub mod interaction;
#[macro_use]
pub mod macros;
pub mod models;
pub mod model {
    pub use crate::components::*;
    pub use crate::models::*;
}
pub mod permissions;
pub mod stage;

pub use bot::{Bot, BoxFuture, EventHandler};
pub use client::Client;
pub use commands::{
    Command, CommandOption, CommandOptionChoice, CommandOptionType, CommandType, CreateCommand,
};
pub use components::{ButtonStyle, Component, ComponentType, SelectOption, TextInputStyle};
pub use context::Context;
pub use embed::{Embed, EmbedField};
pub use error::MicupError;
pub use events::{
    Event, GuildMemberRemove, MessageDelete, MessageReactionAdd, MessageReactionRemove, ReadyEvent,
    VoiceServerUpdate, VoiceState,
};
pub use gateway::{GatewayPayload, GatewayPresence, Shard, ShardSender};
pub use intents::Intents;
pub use interaction::{
    CommandInteractionDataOption, Interaction, InteractionData, InteractionResponse,
    InteractionResponseData, InteractionResponseMessage, InteractionResponseType, InteractionType,
    ModalResponseData,
};
pub use models::{
    Attachment, AuditLog, AuditLogEntry, Ban, Channel, Emoji, FluxerDiscovery, GatewayBotInfo,
    Guild, Id, Invite, Member, Message, Reaction, Role, User, Webhook,
};
pub use permissions::Permissions;
pub use stage::{
    BoardStyle, ScoreboardEntry, SetVoiceRoomStage, StageBoard, StageHost, StageTv, StageVideo,
    VoiceRoomEffect, VoiceRoomStage, VoiceRoomStageResponse,
};

pub mod prelude {
    pub use crate::bot::{Bot, BoxFuture, EventHandler};
    pub use crate::cdn;
    pub use crate::client::Client;
    pub use crate::commands::{
        Command, CommandOption, CommandOptionChoice, CommandOptionType, CommandType, CreateCommand,
    };
    pub use crate::components::{
        ButtonStyle, Component, ComponentType, SelectOption, TextInputStyle,
    };
    pub use crate::context::Context;
    pub use crate::embed::Embed;
    pub use crate::error::MicupError;
    pub use crate::events::{
        Event, GuildMemberRemove, MessageDelete, MessageReactionAdd, MessageReactionRemove,
        ReadyEvent, VoiceServerUpdate, VoiceState,
    };
    pub use crate::gateway::{GatewayPresence, Shard, ShardSender};
    pub use crate::intents::Intents;
    pub use crate::interaction::{
        CommandInteractionDataOption, Interaction, InteractionData, InteractionResponse,
        InteractionResponseMessage, InteractionResponseType, InteractionType,
    };
    pub use crate::models::{
        Attachment, AuditLog, AuditLogEntry, Ban, Channel, Emoji, FluxerDiscovery, GatewayBotInfo,
        Guild, Id, Invite, Member, Message, Reaction, Role, User, Webhook,
    };
    pub use crate::permissions::Permissions;
    pub use crate::stage::{
        BoardStyle, ScoreboardEntry, SetVoiceRoomStage, StageBoard, StageHost, StageTv, StageVideo,
        VoiceRoomEffect, VoiceRoomStage,
    };
    pub use crate::{action_row, button, embed, select_menu};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdn_urls() {
        let avatar = cdn::avatar_url(1566954479021461510, Some("a313e966"), 0);
        assert_eq!(
            avatar,
            "https://media.micup.gg/avatars/1566954479021461510/a313e966.png?size=4096"
        );

        let default_avatar = cdn::avatar_url(123, None, 2);
        assert_eq!(default_avatar, "https://media.micup.gg/embed/avatars/2.png");

        let icon = cdn::guild_icon_url(1567986170364166147, "cccae7b5");
        assert_eq!(
            icon,
            "https://media.micup.gg/icons/1567986170364166147/cccae7b5.png?size=4096"
        );

        let emoji = cdn::emoji_url(987654321, false);
        assert_eq!(emoji, "https://media.micup.gg/emojis/987654321.png");

        let sticker = cdn::sticker_url(11223344);
        assert_eq!(sticker, "https://media.micup.gg/stickers/11223344.png");

        let future_url =
            "https://media.micup.gg/attachments/1/2/file.png?ex=ffffffff&is=123&hm=abc";
        assert!(!cdn::is_attachment_url_expired(future_url));

        let expired_url =
            "https://media.micup.gg/attachments/1/2/file.png?ex=00000001&is=123&hm=abc";
        assert!(cdn::is_attachment_url_expired(expired_url));
    }

    #[test]
    fn test_macros() {
        let e = embed! {
            title: "Test Başlığı",
            description: "Açıklama",
            color: 0x7c5cff,
            fields: [
                ("Alan 1", "Değer 1", true),
                ("Alan 2", "Değer 2", false),
            ],
        };
        assert_eq!(e.title.as_deref(), Some("Test Başlığı"));
        assert_eq!(e.color, Some(0x7c5cff));
        assert_eq!(e.fields.len(), 2);

        let b = button!("Onayla", "btn_ok", Success);
        let json = serde_json::to_value(&b).unwrap();
        assert_eq!(json["label"], "Onayla");
        assert_eq!(json["custom_id"], "btn_ok");

        let row = action_row![b.clone()];
        let row_json = serde_json::to_value(&row).unwrap();
        assert_eq!(row_json["type"], 1);

        let menu = select_menu! {
            custom_id: "secim",
            placeholder: "Secim yap",
            options: [
                ("A", "val_a"),
                ("B", "val_b", "Detay B"),
            ],
        };
        let menu_json = serde_json::to_value(&menu).unwrap();
        assert_eq!(menu_json["custom_id"], "secim");
        assert_eq!(menu_json["options"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_embed_field_sanitizer() {
        let long_val = "x".repeat(1500);
        let embed = Embed::new().field("Başlık", long_val, false);
        let json = serde_json::to_value(&embed).unwrap();
        let field_val = json["fields"][0]["value"].as_str().unwrap();
        assert_eq!(field_val.chars().count(), 1024);
        assert!(field_val.ends_with("..."));
    }

    #[test]
    fn test_button_serialization_skips_disabled_false() {
        let btn = Component::button(ButtonStyle::Primary, "Test", "custom_1");
        let json = serde_json::to_value(&btn).unwrap();
        assert!(json.get("disabled").is_none());
    }

    #[test]
    fn test_guild_deserialization_with_missing_fields() {
        let raw = r#"{
            "id": "1567986170364166147",
            "name": "Finmos Development",
            "icon": "cccae7b5",
            "owner_id": "1566954479021461510",
            "roles": [],
            "emojis": [],
            "features": []
        }"#;

        let guild: Guild = serde_json::from_str(raw).expect("Micup Guild parse edilmeli");
        assert_eq!(guild.id.get(), 1567986170364166147);
        assert_eq!(guild.name, "Finmos Development");
        assert!(guild.preferred_locale.is_none());
        assert!(guild.premium_progress_bar_enabled.is_none());
    }

    #[test]
    fn test_stage_board_serialization() {
        let board = StageBoard::new()
            .style(BoardStyle::Neon)
            .title("Bilgi Yarışması")
            .text("Soru 1: Rust ne zaman duyuruldu?")
            .accent(0x7c5cff)
            .score(ScoreboardEntry::new("Kerem", 100).with_user(1566954479021461510))
            .message_buttons(1234567890);

        let stage = SetVoiceRoomStage::new()
            .board(board)
            .host(StageHost::new("ithp-kulaklik"))
            .ttl(1800);

        let json = serde_json::to_value(&stage).unwrap();
        assert_eq!(json["ttl_seconds"], 1800);
        assert_eq!(json["tv"]["type"], "board");
        assert_eq!(json["tv"]["style"], "neon");
        assert_eq!(json["tv"]["title"], "Bilgi Yarışması");
        assert_eq!(json["host"]["character"], "ithp-kulaklik");
    }

    #[test]
    fn test_permissions_bitflags() {
        let mut perms = Permissions::new(Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES);
        assert!(!perms.can_manage_stage());

        perms.insert(Permissions::MANAGE_ROOM_STAGE);
        assert!(perms.can_manage_stage());
        assert_eq!(Permissions::MANAGE_ROOM_STAGE, 1u64 << 62);
    }

    #[test]
    fn test_typed_event_parsing() {
        let raw = serde_json::json!({
            "id": "123456789",
            "channel_id": "987654321",
            "content": "Merhaba Micup!",
            "pinned": false,
            "mention_everyone": false
        });

        let event = Event::parse("MESSAGE_CREATE", raw);
        match event {
            Event::MessageCreate(msg) => {
                assert_eq!(msg.id.get(), 123456789);
                assert_eq!(msg.content, "Merhaba Micup!");
            }
            _ => panic!("Beklenen MESSAGE_CREATE olayı gelmedi"),
        }
    }

    #[test]
    fn test_voice_server_update_parsing() {
        let raw = serde_json::json!({
            "token": "livekit_jwt_token_sample",
            "endpoint": "wss://livekit.micup.gg",
            "connection_id": "conn_123",
            "channel_id": "1568369071631110144",
            "guild_id": "1567986170364166147"
        });

        let event = Event::parse("VOICE_SERVER_UPDATE", raw);
        match event {
            Event::VoiceServerUpdate(vsu) => {
                assert_eq!(vsu.token, "livekit_jwt_token_sample");
                assert_eq!(vsu.endpoint, "wss://livekit.micup.gg");
                assert_eq!(vsu.channel_id.get(), 1568369071631110144);
            }
            _ => panic!("VOICE_SERVER_UPDATE beklenen sekilde parse edilemedi"),
        }
    }

    #[test]
    fn test_prelude_exports() {
        use crate::prelude::*;

        let _bot = Bot::from_token("test_token");
        let _ctx = Context::new(std::sync::Arc::new(Client::new("token").unwrap()));
        let _intents = Intents::default_intents();
        let _perms = Permissions::new(Permissions::MANAGE_ROOM_STAGE);
        let _embed = Embed::new().title("Test");
        let _comp = Component::button(ButtonStyle::Primary, "Test", "btn_1");
        let _board = StageBoard::new();
        let _video = StageVideo::new("https://media.micup.gg/video.mp4");
        let _effect = VoiceRoomEffect::confetti();
        let _row = action_row![button!("Click", "btn_click", Primary)];
        let _macro_embed = embed! {
            title: "Test",
            description: "Desc",
        };
        let _select = select_menu! {
            custom_id: "menu_1",
            options: [("Opt1", "val1")],
        };
    }

    #[test]
    fn test_event_handler_trait_box_future() {
        use crate::prelude::*;

        struct CustomHandler;

        impl EventHandler for CustomHandler {
            fn on_ready<'a>(&'a self, _ctx: Context, _ready: ReadyEvent) -> BoxFuture<'a, ()> {
                Box::pin(async move {})
            }

            fn on_message<'a>(&'a self, _ctx: Context, _msg: Message) -> BoxFuture<'a, ()> {
                Box::pin(async move {})
            }

            fn on_interaction<'a>(
                &'a self,
                _ctx: Context,
                _interaction: Interaction,
            ) -> BoxFuture<'a, ()> {
                Box::pin(async move {})
            }

            fn on_event<'a>(&'a self, _ctx: Context, _event: Event) -> BoxFuture<'a, ()> {
                Box::pin(async move {})
            }
        }

        let handler = CustomHandler;
        let ctx = Context::new(std::sync::Arc::new(Client::new("token").unwrap()));
        let raw = serde_json::json!({
            "id": "123456789",
            "content": "Test",
            "pinned": false,
            "mention_everyone": false
        });
        let msg: Message = serde_json::from_value(raw).unwrap();
        let fut = handler.on_message(ctx, msg);
        tokio::runtime::Runtime::new().unwrap().block_on(fut);
    }
}
