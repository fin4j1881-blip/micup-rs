//! Yapısal / Kurumsal Mimari ile Micup Bot Örneği.
//!
//! `EventHandler` trait'i ile modüler ve büyük projeler için temiz mimari.
//!
//! Çalıştırmak için:
//! `MICUP_TOKEN="bot_tokeniniz" cargo run --example structured_bot`

use micup::prelude::*;
use std::env;

struct MyBotHandler;

impl EventHandler for MyBotHandler {
    fn on_ready<'a>(&'a self, _ctx: Context, ready: ReadyEvent) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            println!(
                "Bot çevrimiçi: {} (Oturum: {})",
                ready.user.username, ready.session_id
            );
        })
    }

    fn on_message<'a>(&'a self, ctx: Context, msg: Message) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            if msg.content == "!selam" {
                if let Some(channel_id) = msg.channel_id {
                    let _ = ctx
                        .say(channel_id.get(), "Aleyküm selam! Hoş geldiniz.")
                        .await;
                }
            }
        })
    }

    fn on_interaction<'a>(&'a self, ctx: Context, interaction: Interaction) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            let _ = ctx
                .reply_ephemeral(&interaction, "Etkileşim işlendi.")
                .await;
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("MICUP_TOKEN").unwrap_or_else(|_| "test_token".to_string());

    Bot::from_token(&token)
        .intents(Intents::default_intents() | Intents::GUILD_VOICE_STATES)
        .handler(MyBotHandler)
        .run()
        .await?;

    Ok(())
}
