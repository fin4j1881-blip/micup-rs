# micup-rs 🦀

Micup.gg / Fluxer platformu için resmi olmayan, Micup API ve CDN mimarisine özel geliştirilmiş modern, asenkron ve eksiksiz Rust kütüphanesi.

Standart Discord kütüphaneleri (Serenity, Twilight vb.) Micup üzerinde kullanılırken yaşanan yapısal uyumsuzlukları dahili olarak çözer ve **Micup'a özgü 3B Oda Sahnesi (Voice Room Stage)** gibi sistemleri birinci sınıf destekler.

---

## 🚀 Çözülen Micup / Fluxer Sorunları & Özel Sistemler

1. **3B Oda Sahnesi Desteği (`stage.rs`):**
   - Micup'a özgü sesli kanal 3B odasında botun sunucu NPC karakteri (`host`) olarak durması.
   - TV'de Neon, Tahta ve Sade stil panolar (`StageBoard`), skor tabloları ve video (`StageVideo`) oynatımı.
   - TV panosuna interaktif mesaj butonları bağlama ve tıklama etkileşimlerini karşılama.
   - Odadaki herkese tek seferlik anlar (konfeti, spot ışığı, karakter dansı/dalgası/emojisi) gönderme (`VoiceRoomEffect`).

2. **Micup İzin Bitleri (`permissions.rs`):**
   - Micup'a özgü `MANAGE_ROOM_STAGE` (`1 << 62`), `PIN_MESSAGES` (`1 << 51`), `BYPASS_SLOWMODE` (`1 << 52`), `UPDATE_RTC_REGION` (`1 << 53`) ve `VIEW_CHANNEL_MEMBERS` (`1 << 54`) izin bitleri.
   - Bot davet bağlantısı oluşturma (`Permissions::bot_invite_url`).

3. **Gelişmiş Gateway & Dayanıklılık (`gateway.rs`):**
   - Otomatik `Resume (op 6)` ve `session_id` / `last_seq` takibi ile kesintisiz yeniden bağlanma.
   - Heartbeat ACK takibi ile zombi bağlantı tespiti ve otomatik soket yenileme.
   - `Opcode 7 (Reconnect)` ve `Opcode 9 (Invalid Session)` akıllı kurtarma.
   - Bot durum metni (`Custom Status`) ve presence desteği (`with_presence`).
   - Tür güvenli Gateway Olay Ayrıştırması (`shard.run_typed` & `Event` enum): `MessageCreate`, `InteractionCreate`, `VoiceStateUpdate` vb.

4. **Otomatik Hız Sınırı Koruması (429 Rate-Limit Retry):**
   - API'den `429 Too Many Requests` yanıtı alındığında `Retry-After` süresi kadar asenkron bekleyip isteği otomatik olarak baştan dener (`with_max_retries`).

5. **Doğru Medya CDN & İmzalı Ek Dosya Kontrolü (`cdn.rs`):** 
   - Standart kütüphaneler kırık `cdn.discordapp.com` linkleri üretir.
   - `micup-rs`, avatar, banner, sunucu ikonu, çıkartma ve emojileri doğrudan `https://media.micup.gg` üzerinden sağlar.
   - 6 varsayılan Micup avatarı (`0-5.png`) ve özel avatar formatları (`.png`, `.gif`).
   - İmzalı ek dosya bağlantılarının 24 saatlik geçerliliğini denetleyen `is_attachment_url_expired`.

6. **Toleranslı Guild & Model Çözümleme (`models.rs`):**
   - Micup API'sinde `preferred_locale`, takviye çubuğu veya bazı opsiyonel alanlar `null` döner veya hiç gönderilmez.
   - `micup-rs`, bu alanları otomatik olarak varsayılan değerlere eşler.

7. **Bileşen & Embed Sanitasyonu:**
   - Butonlarda `disabled: false` gönderilmesini engelleyerek API `Invalid form body` hatasını önler.
   - 1024 karakter embed alan sınırını ve boş alanları güvenli biçimde sanitasyon yapar.
   - Modallar ve Text Input (`ComponentType::TextInput`) bileşenleri tam desteklidir.

---

## 📚 Desteklenen API Uç Noktaları

- **3B Oda Sahnesi:** `get_voice_room_stage()`, `set_voice_room_stage()`, `clear_voice_room_stage()`, `send_voice_room_effect()`
- **Keşif & Gateway:** `get_discovery()` (`/.well-known/fluxer`), `get_gateway_bot()` (`/gateway/bot`)
- **Mesajlar:** `send_message()`, `send_message_with_files()`, `get_message()`, `get_channel_messages()`, `edit_message()`, `delete_message()`, `bulk_delete_messages()`
- **Webhook'lar:** `execute_webhook()`, `get_channel_webhooks()`, `create_webhook()`, `delete_webhook()`
- **Kanallar & Roller:** `get_channel()`, `delete_channel()`, `create_channel_invite()`, `create_role()`, `delete_role()`
- **Emojiler:** `get_guild_emojis()`, `get_guild_emoji()`, `delete_guild_emoji()`
- **Tepkiler (Reactions):** `create_reaction()`, `delete_own_reaction()`, `delete_user_reaction()`, `get_reactions()`
- **Sabitleme (Pins):** `pin_message()`, `unpin_message()`, `get_pinned_messages()`
- **Kullanıcılar & Profil:** `current_user()`, `get_user()`, `edit_current_user()`
- **Sunucular & Üyeler:** `get_guild()`, `get_guild_channels()`, `get_guild_roles()`, `create_channel()`, `get_member()`, `search_guild_members()`
- **Moderasyon:** `kick()`, `ban()`, `unban()`, `set_timeout()`, `add_role()`, `remove_role()`
- **Slash Komutları:** `get_global_commands()`, `set_global_commands()`, `get_guild_commands()`, `set_guild_commands()`
- **Etkileşim Yanıtları:** `create_interaction_response()`, `create_followup_message()`
- **Denetim & Güvenlik:** `get_audit_logs()`, `get_bans()`, `get_guild_invites()`

---

## 🛠️ Örnek Kullanım: 3B Oda Sahnesi Kurulumu

```rust
use micup::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("MICUP_BOT_TOKEN")?;
    let channel_id = 1567986170364166155; // Sesli kanal ID

    // 1. TV Panosu ve Skor Tablosu Hazırla
    let board = StageBoard::new()
        .style(BoardStyle::Neon)
        .title("Bilgi Yarışması")
        .text("Soru 1: Micup bot kütüphanesi nedir?")
        .accent(0x7c5cff)
        .score(ScoreboardEntry::new("Kerem", 100))
        .score(ScoreboardEntry::new("Oyuncu 2", 80));

    // 2. Sahneyi TV panosu ve Sunucu NPC karakteri ile kur
    let stage = SetVoiceRoomStage::new()
        .board(board)
        .host(StageHost::new("ithp-kulaklik").shirt_color(0x00FF88))
        .ttl(1800);

    client.set_voice_room_stage(channel_id, &stage).await?;

    // 3. Odaya konfeti efekti gönder
    client.send_voice_room_effect(channel_id, &VoiceRoomEffect::confetti()).await?;

    Ok(())
}
```

---

## 🛠️ Örnek Kullanım: Tür Güvenli Gateway Olay Döngüsü

```rust
use micup::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = "MICUP_BOT_TOKEN";
    let shard = Shard::new(token, 513) // Guilds (1) | GuildMessages (512)
        .with_presence("online", Some("micup-rs v0.3.0"));

    // Olaylar otomatik olarak tür güvenli Event enum'una dönüştürülür
    shard.run_typed(|event| async move {
        match event {
            Event::Ready(ready) => {
                println!("Bot bağlandı! Kullanıcı: {} (ID: {})", ready.user.username, ready.user.id);
            }
            Event::MessageCreate(msg) => {
                println!("[#{:?}] {}: {}", msg.channel_id, msg.author.map(|u| u.username).unwrap_or_default(), msg.content);
            }
            Event::InteractionCreate(interaction) => {
                println!("Yeni etkileşim alındı (ID: {})", interaction.id);
            }
            Event::VoiceStateUpdate(voice) => {
                println!("Ses durumu güncellendi: kullanıcı {}", voice.user_id);
            }
            _ => {}
        }
    }).await?;

    Ok(())
}
```

---

## 📜 Lisans

MIT OR Apache-2.0
