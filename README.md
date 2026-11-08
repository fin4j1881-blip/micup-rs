# micup-rs 🦀

Micup.gg / Fluxer platformu için resmi olmayan, Micup API ve CDN mimarisine özel geliştirilmiş modern, asenkron ve eksiksiz Rust kütüphanesi.

Hem **hızlı geliştirme (vibe coding)** yapanlar için tek satırda çalışan fonksiyon zincirleri ve makrolar (`embed!`, `button!`), hem de **yapısal mimari (hand-crafted code)** kuranlar için `EventHandler` trait'i, tür güvenli olaylar (`Event`), zengin `Context` ve granular REST/Gateway API'si sunar.

---

## ⚡ 1. Vibe Coding: 30 Saniyede Sıfır Boilerplate Bot

Kodlama yaparken veya yapay zekaya kod yazdırırken karmaşık şablonlarla uğraşmanıza gerek yok. Tek bir zincirle botunuz hazır:

```rust
use micup::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Bot::from_token("MICUP_BOT_TOKEN")
        .with_presence("online", "micup-rs ile aktif 🚀")
        .on_ready(|_, ready| async move {
            println!("✅ Bot bağlandı: {}", ready.user.username);
        })
        .on_message(|ctx, msg| async move {
            if msg.content == "!ping" {
                if let Some(ch) = msg.channel_id {
                    let _ = ctx.say(ch.get(), "Pong! 🏓").await;
                }
            } else if msg.content == "!yardim" {
                if let Some(ch) = msg.channel_id {
                    let e = embed! {
                        title: "Yardım Menüsü",
                        description: "micup-rs ile bot yazmak artık çok kolay!",
                        color: 0x7c5cff,
                        fields: [
                            ("!ping", "Gecikme testi", true),
                            ("!sahne", "3B ses odasına TV panosu kurar", true),
                        ],
                    };
                    let _ = ctx.send_embed(ch.get(), e).await;
                }
            }
        })
        .on_command("ping", |ctx, i| async move {
            let _ = ctx.reply(&i, "Pong! 🏓").await;
        })
        .on_button("btn_ornek", |ctx, i| async move {
            let _ = ctx.reply_ephemeral(&i, "Butona tıkladınız!").await;
        })
        .run()
        .await?;

    Ok(())
}
```

---

## 🏛️ 2. Yapısal & Modüler Mimari (`EventHandler` Trait)

Büyük ve sürdürülebilir kurumsal projeler için `EventHandler` trait'i ile modüler mimari:

```rust
use micup::prelude::*;

struct MyBot;

impl EventHandler for MyBot {
    fn on_ready<'a>(&'a self, _ctx: Context, ready: ReadyEvent) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            println!("Bot çevrimiçi: {} (Oturum: {})", ready.user.username, ready.session_id);
        })
    }

    fn on_message<'a>(&'a self, ctx: Context, msg: Message) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            if msg.content == "!selam" {
                if let Some(ch) = msg.channel_id {
                    let _ = ctx.say(ch.get(), "Aleyküm selam!").await;
                }
            }
        })
    }

    fn on_interaction<'a>(&'a self, ctx: Context, interaction: Interaction) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            let _ = ctx.reply_ephemeral(&interaction, "Etkileşim alındı.").await;
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Bot::from_token("MICUP_BOT_TOKEN")
        .intents(Intents::default_intents() | Intents::GUILD_VOICE_STATES)
        .handler(MyBot)
        .run()
        .await?;

    Ok(())
}
```

---

## 🚀 Çözülen Micup / Fluxer Sorunları & Özel Sistemler

1. **3B Oda Sahnesi Desteği (`stage.rs`):**
   - Sesli kanal 3B odasında botun sunucu NPC karakteri (`host`) olarak durması.
   - TV'de Neon, Tahta ve Sade stil panolar (`StageBoard`), skor tabloları ve video (`StageVideo`) oynatımı.
   - TV panosuna interaktif mesaj butonları bağlama ve tıklama etkileşimlerini karşılama.
   - Odadaki herkese tek seferlik anlar (konfeti, spot ışığı, karakter dansı/dalgası/emojisi) gönderme (`VoiceRoomEffect`).
   - `ctx.set_stage_board(...)`, `ctx.stage_confetti(...)`, `ctx.stage_spotlight(...)` kısayolları.

2. **Micup İzin Bitleri (`permissions.rs`):**
   - Micup'a özgü `MANAGE_ROOM_STAGE` (`1 << 62`), `PIN_MESSAGES` (`1 << 51`), `BYPASS_SLOWMODE` (`1 << 52`), `UPDATE_RTC_REGION` (`1 << 53`) ve `VIEW_CHANNEL_MEMBERS` (`1 << 54`) izin bitleri.
   - Bot davet bağlantısı oluşturma (`Permissions::bot_invite_url`).

3. **Gelişmiş Gateway & Dayanıklılık (`gateway.rs`):**
   - Otomatik `Resume (op 6)` ve `session_id` / `last_seq` takibi ile kesintisiz yeniden bağlanma.
   - Heartbeat ACK takibi ile zombi bağlantı tespiti ve otomatik soket yenileme.
   - `Opcode 7 (Reconnect)` ve `Opcode 9 (Invalid Session)` akıllı kurtarma.
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

## 📜 Lisans

MIT OR Apache-2.0
