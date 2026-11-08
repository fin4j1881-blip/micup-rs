//! Micup.gg 3B Oda Sahnesi (Voice Room Stage) ve Keşif Demosu.
//!
//! Çalıştırmak için:
//! `MICUP_TOKEN="bot_tokeniniz" cargo run --example voice_room_stage_demo`

use micup::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("MICUP_TOKEN").unwrap_or_else(|_| "test_token".to_string());
    let client = Client::new(&token)?;

    println!("=== 1. Micup Platform Keşfi (/.well-known/fluxer) ===");
    match client.get_discovery().await {
        Ok(discovery) => {
            println!("API Adresi: {}", discovery.endpoints.api);
            println!("Gateway Adresi: {}", discovery.endpoints.gateway);
            println!("Media CDN: {}", discovery.endpoints.media);
        }
        Err(e) => {
            println!("Keşif isteği hatası: {e}");
        }
    }

    println!("\n=== 2. 3B Oda Sahnesi Pano ve Efekt Tanımları ===");
    let board = StageBoard::new()
        .style(BoardStyle::Neon)
        .title("Rust Bilgi Yarışması")
        .text("Soru 1: Micup bot kütüphanesi nedir?\nA) micup-rs\nB) discord.js")
        .accent(0x7c5cff)
        .score(ScoreboardEntry::new("Kerem", 100))
        .score(ScoreboardEntry::new("Oyuncu 2", 80));

    let stage = SetVoiceRoomStage::new()
        .board(board)
        .host(StageHost::new("ithp-kulaklik").shirt_color(0x00FF88))
        .ttl(1800);

    let stage_json = serde_json::to_string_pretty(&stage)?;
    println!("Oluşturulan Sahne Payload'ı:\n{stage_json}");

    println!("\n=== 3. 3B Oda Efektleri ===");
    let confetti = VoiceRoomEffect::confetti();
    println!("Konfeti Efekti JSON: {}", serde_json::to_string(&confetti)?);

    let dance = VoiceRoomEffect::dance();
    println!("Dans Efekti JSON: {}", serde_json::to_string(&dance)?);

    let emoji = VoiceRoomEffect::wave();
    println!("El Sallama Efekti JSON: {}", serde_json::to_string(&emoji)?);

    println!("\n=== 4. İzin Bitleri Kontrolü ===");
    let perms = Permissions::new(
        Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES | Permissions::MANAGE_ROOM_STAGE,
    );
    println!(
        "MANAGE_ROOM_STAGE izni var mı: {}",
        perms.can_manage_stage()
    );
    println!(
        "Davet URL'si: {}",
        Permissions::bot_invite_url(1234567890, perms.0)
    );

    println!("\nDemo basariyla tamamlandi.");
    Ok(())
}
