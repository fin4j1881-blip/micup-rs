use micup::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("MICUP_TOKEN").expect("MICUP_TOKEN ortam değişkeni tanımlı olmalı");
    let client = Client::new(&token)?;

    println!("=== 1. Bot ve Uygulama Bilgisi ===");
    let me = client.current_user().await?;
    println!("Bot: {} (ID: {})", me.username, me.id);

    let guild_id = 1567986170364166147;
    println!("\n=== 2. Sunucu Detayları ({}) ===", guild_id);
    let guild = client.get_guild(guild_id).await?;
    println!("Sunucu Adı: {}", guild.name);
    println!("İkon URL: {:?}", guild.icon_url());
    println!("Afiş (Banner) URL: {:?}", guild.banner_url());

    println!("\n=== 3. Kanallar ve Rollerin Listesi ===");
    let channels = client.get_guild_channels(guild_id).await?;
    println!("Toplam Kanal: {}", channels.len());
    for c in channels.iter().take(3) {
        println!(
            "  #{} (id={}, type={})",
            c.name.as_deref().unwrap_or("-"),
            c.id,
            c.kind
        );
    }

    let roles = client.get_guild_roles(guild_id).await?;
    println!("Toplam Rol: {}", roles.len());
    for r in roles.iter().take(3) {
        println!("  @{} (id={}, pos={})", r.name, r.id, r.position);
    }

    println!("\n=== 4. Sunucu Denetim Kaydı (Audit Log) ===");
    let audit = client.get_audit_logs(guild_id, Some(3)).await?;
    println!(
        "Son denetim girişi sayısı: {}",
        audit.audit_log_entries.len()
    );
    for e in &audit.audit_log_entries {
        println!(
            "  İşlem ID: {}, Tip: {}, Sebep: {:?}",
            e.id, e.action_type, e.reason
        );
    }

    println!("\n=== 5. Slash Komutları ===");
    let commands = client.get_guild_commands(me.id.get(), guild_id).await?;
    println!("Sunucuda kayıtlı komut sayısı: {}", commands.len());
    for cmd in commands.iter().take(3) {
        println!("  /{} -> {}", cmd.name, cmd.description);
    }

    println!("\n=== 6. Buton ve Embed Sanitasyon Testi ===");
    let btn = Component::button(ButtonStyle::Success, "Kabul Et", "btn_accept");
    let row = Component::action_row(vec![btn]);
    let embed = Embed::new()
        .title("micup-rs Tam Kapsam Testi")
        .description("Tüm API katmanları eksiksiz çalışıyor!")
        .color(0x00FF88)
        .field("Durum", "Mükemmel", true)
        .field("Uzun Alan", "x".repeat(1500), false);

    let embed_json = serde_json::to_string(&embed)?;
    let row_json = serde_json::to_string(&row)?;
    println!("Embed JSON uzunluğu: {} bayt", embed_json.len());
    println!("ActionRow JSON: {}", row_json);

    println!("\n✅ Tüm testler başarıyla geçti!");
    Ok(())
}
