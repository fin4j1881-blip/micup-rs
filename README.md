# micup-rs 🦀

Micup.gg / Fluxer platformu için resmi olmayan, Micup API ve CDN mimarisine özel geliştirilmiş modern, asenkron Rust kütüphanesi.

Standart Discord kütüphaneleri (Serenity, Twilight vb.) Micup üzerinde kullanılırken yaşanan yapısal ve şema uyumsuzluklarını dahili olarak çözer.

---

## 🚀 Çözülen Micup / Fluxer Sorunları

1. **Doğru Medya CDN URL'leri:** 
   - Standart kütüphaneler kırık `cdn.discordapp.com` linkleri üretir.
   - `micup-rs`, avatar, banner, sunucu ikonu ve animasyonlu GIF'leri doğrudan `https://media.micup.gg` üzerinden 4096px çözünürlükle sunar.

2. **Toleranslı Guild / Model Çözümleme:**
   - Micup API'sinde `preferred_locale`, takviye çubuğu veya bazı opsiyonel Discord alanları `null` döner veya hiç gönderilmez.
   - `micup-rs`, bu alanları otomatik olarak varsayılan değerlere eşler ve serileştirme hatası fırlatmaz.

3. **Otomatik Embed Alanı Sanitasyonu:**
   - Discord/Micup API'si 1024 karakteri aşan embed alanlarında veya boş başlıklarda `400 Bad Request (Invalid form body)` hatası verir.
   - `micup-rs::Embed`, 1024 sınırını aşan metinleri güvenli bir şekilde `...` ile kırpar ve boş alanları düzeltir.

4. **Akıllı ID Ayrıştırma:**
   - Hem JSON string (`"1567986170364166147"`) hem de sayısal tamsayı (`1567986170364166147`) formatındaki Snowflake ID'lerini şeffaf şekilde çözer.

5. **Dahili Gateway & Kalp Atışı:**
   - `wss://micup.gg/gateway` soketine otomatik bağlanır, `Hello (op 10)` paketinden aralığı okur ve periyodik `Heartbeat (op 1)` atışlarını arka planda yürütür.

---

## 📦 Kurulum

`Cargo.toml` dosyanıza ekleyin:

```toml
[dependencies]
micup = { path = "/mnt/depo/micup-rs" }
tokio = { version = "1", features = ["full"] }
```

---

## 🛠️ Örnek Kullanım

### REST İstemcisi ve Embed Gönderimi

```rust
use micup::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("MICUP_BOT_TOKEN")?;

    // Bot profilini çek
    let me = client.current_user().await?;
    println!("Bot: {} (Avatar: {})", me.username, me.avatar_url());

    // Sunucu bilgilerini al
    let guild = client.get_guild(1567986170364166147).await?;
    println!("Sunucu Adı: {}", guild.name);

    // Otomatik kırpma korumalı Embed oluştur
    let embed = Embed::new()
        .title("Hoş Geldiniz!")
        .description("Micup.gg üzerinde çalışan modern bot.")
        .color(0x00FF88)
        .field("Durum", "Aktif", true)
        .field("Açıklama", "x".repeat(1500), false); // 1024 karakter sınırına otomatik kırpılır

    // Mesaj gönder
    client.send_message(1567986170364166150, None, vec![embed]).await?;

    Ok(())
}
```

### Canlı Gateway (WebSocket) Olay Dinleyicisi

```rust
use micup::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shard = Shard::new("MICUP_BOT_TOKEN", 3276799); // Intents

    shard.run(|event_name, data| async move {
        match event_name.as_str() {
            "READY" => println!("Bot başarıyla bağlandı!"),
            "MESSAGE_CREATE" => {
                let content = data.get("content").and_then(|c| c.as_str()).unwrap_or("");
                println!("Yeni Mesaj: {}", content);
            }
            _ => {}
        }
    }).await?;

    Ok(())
}
```

---

## 📄 Lisans
MIT veya Apache-2.0
