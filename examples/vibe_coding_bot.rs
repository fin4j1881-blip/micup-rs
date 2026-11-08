//! Vibe Coding Micup Bot Örneği.
//!
//! Sıfır karmaşa, tek fonksiyon zinciriyle 30 saniyede çalışan bot.
//!
//! Çalıştırmak için:
//! `MICUP_TOKEN="bot_tokeniniz" cargo run --example vibe_coding_bot`

use micup::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("MICUP_TOKEN").unwrap_or_else(|_| "test_token".to_string());

    println!("Bot baslatiliyor...");

    Bot::from_token(&token)
        .with_presence("online", "micup-rs ile aktif")
        .on_ready(|_, ready| async move {
            println!(
                "Bot baglandi: {} (ID: {})",
                ready.user.username, ready.user.id
            );
        })
        .on_message(|ctx, msg| async move {
            println!("Mesaj: {}", msg.content);

            if msg.content == "!ping" {
                if let Some(channel_id) = msg.channel_id {
                    let _ = ctx.say(channel_id.get(), "Pong!").await;
                }
            } else if msg.content == "!yardim" {
                if let Some(channel_id) = msg.channel_id {
                    let e = embed! {
                        title: "Yardim Menusu",
                        description: "micup-rs kütüphanesiyle kolayca bot yapin.",
                        color: 0x7c5cff,
                        fields: [
                            ("!ping", "Botun calisip calismadigini test eder", true),
                            ("!sahne", "3B sesli odaya TV panosu kurar", true),
                        ],
                    };
                    let _ = ctx.send_embed(channel_id.get(), e).await;
                }
            }
        })
        .on_command("ping", |ctx, interaction| async move {
            let _ = ctx.reply(&interaction, "Pong!").await;
        })
        .on_button("btn_ornek", |ctx, interaction| async move {
            let _ = ctx.reply_ephemeral(&interaction, "Butona tikladiniz.").await;
        })
        .run()
        .await?;

    Ok(())
}
