//! Micup Ses Kanalina Katilma ve Kamera Acma Ornegi.
//!
//! Botun bir ses kanalina baglanmasini, kamerasini acmasini (`self_video: true`)
//! ve Micup'tan LiveKit baglanti anahtarini (`VOICE_SERVER_UPDATE`) almasini gosterir.
//!
//! Calistirmak icin:
//! `MICUP_TOKEN="bot_tokeniniz" cargo run --example voice_camera_bot`

use micup::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("MICUP_TOKEN").unwrap_or_else(|_| "test_token".to_string());

    println!("Bot baslatiliyor...");

    Bot::from_token(&token)
        .intents(Intents::default_intents() | Intents::GUILD_VOICE_STATES)
        .on_ready(|_, ready| async move {
            println!("Bot baglandi: {}", ready.user.username);
            println!("Komut: !kamera-ac <kanal_id> veya !kamera-kapat");
        })
        .on_message(|ctx, msg| async move {
            let Some(guild_id) = msg.guild_id.map(|g| g.get()) else {
                return;
            };

            let parts: Vec<&str> = msg.content.split_whitespace().collect();
            if parts.is_empty() {
                return;
            }

            match parts[0] {
                "!kamera-ac" => {
                    let channel_id = parts.get(1).and_then(|id| id.parse::<u64>().ok());
                    if let Some(ch) = channel_id {
                        // self_mute: false, self_deaf: false, self_video: true (Kamera acik)
                        if let Err(e) = ctx.join_voice(guild_id, ch, false, false, true) {
                            println!("Ses kanalina baglanilamadi: {e}");
                        } else if let Some(text_ch) = msg.channel_id {
                            let _ = ctx.say(text_ch.get(), "Kamera acilarak ses kanalina baglanildi.").await;
                        }
                    }
                }
                "!kamera-kapat" => {
                    let _ = ctx.leave_voice(guild_id);
                    if let Some(text_ch) = msg.channel_id {
                        let _ = ctx.say(text_ch.get(), "Ses kanalindan ayrilindi.").await;
                    }
                }
                _ => {}
            }
        })
        .on_event(|_ctx, event| async move {
            match event {
                Event::VoiceServerUpdate(grant) => {
                    println!("\n=== LiveKit Baglanti Yetkisi Alindi ===");
                    println!("LiveKit Endpoint : {}", grant.endpoint);
                    println!("LiveKit Token    : {}", grant.token);
                    println!("Kanal ID         : {}", grant.channel_id);
                    println!("Bu endpoint ve token ile LiveKit SDK uzerinden odaya video yayini yapabilirsiniz.\n");
                }
                Event::VoiceStateUpdate(voice) => {
                    println!("Kullanici {} ses durumu: self_video={}", voice.user_id, voice.self_video);
                }
                _ => {}
            }
        })
        .run()
        .await?;

    Ok(())
}
