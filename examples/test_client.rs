use std::env;
use micup::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let token = env::var("MICUP_TOKEN").expect("MICUP_TOKEN ortam değişkeni tanımlı olmalı");
    let client = Client::new(&token)?;

    // Bot profilini çek
    let me = client.current_user().await?;
    println!("Giriş yapıldı: {} (ID: {})", me.username, me.id);
    println!("Avatar URL: {}", me.avatar_url());

    // 1567986170364166147 sunucusunu getir
    let guild = client.get_guild(1567986170364166147).await?;
    println!("Sunucu: {} (İkon: {:?})", guild.name, guild.icon_url());

    // Embed oluşturma testi (1024 sınırını otomatik yönetir)
    let embed = Embed::new()
        .title("micup-rs Test Mesajı")
        .description("Bu mesaj bağımsız micup-rs kütüphanesiyle gönderildi!")
        .color(0x00FF00)
        .field("Durum", "Aktif ve stabil", true)
        .field("Otomatik Kırpma", "x".repeat(1500), false);

    println!("Hazırlanan embed field count: {}", embed.fields.len());

    Ok(())
}
