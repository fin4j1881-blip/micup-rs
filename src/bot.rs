//! Yüksek Seviyeli Bot Çatısı (Bot Framework & Runner).
//!
//! Hem hızlı kodlama (vibe coding) hem de yapısal mimari (structured hand-crafted code)
//! geliştirenler için sıfır boilerplate ile tam donanımlı Micup bot motoru.
//!
//! Tokio görev döngüsü ve `GatewayRunner` ile asenkron ağ geçidi yaşam döngüsünü yönetir.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::client::Client;
use crate::context::Context;
use crate::error::MicupError;
use crate::events::{Event, ReadyEvent, VoiceServerUpdate, VoiceState};
use crate::gateway::Shard;
use crate::intents::Intents;
use crate::interaction::{Interaction, InteractionType};
use crate::models::Message;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Profesyonel / yapısal botlar için olay dinleyici arayüzü (Event Handler Trait).
pub trait EventHandler: Send + Sync + 'static {
    fn on_ready<'a>(&'a self, _ctx: Context, _ready: ReadyEvent) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }
    fn on_message<'a>(&'a self, _ctx: Context, _msg: Message) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }
    fn on_interaction<'a>(&'a self, _ctx: Context, _interaction: Interaction) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }
    fn on_voice_state<'a>(&'a self, _ctx: Context, _voice: VoiceState) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }
    fn on_voice_server<'a>(&'a self, _ctx: Context, _vsu: VoiceServerUpdate) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }
    fn on_event<'a>(&'a self, _ctx: Context, _event: Event) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }
}

type ReadyHandler = Box<dyn Fn(Context, ReadyEvent) -> BoxFuture<'static, ()> + Send + Sync>;
type MessageHandler = Box<dyn Fn(Context, Message) -> BoxFuture<'static, ()> + Send + Sync>;
type InteractionHandler = Box<dyn Fn(Context, Interaction) -> BoxFuture<'static, ()> + Send + Sync>;
type VoiceStateHandler = Box<dyn Fn(Context, VoiceState) -> BoxFuture<'static, ()> + Send + Sync>;
type VoiceServerHandler =
    Box<dyn Fn(Context, VoiceServerUpdate) -> BoxFuture<'static, ()> + Send + Sync>;
type EventHandlerFn = Box<dyn Fn(Context, Event) -> BoxFuture<'static, ()> + Send + Sync>;

/// Tokio tabanlı ağ geçidi (Gateway) görev döngüsünü yöneten ve yürüten çalıştırıcı (Gateway Runner).
pub struct GatewayRunner {
    shard: Shard,
    context: Context,
}

impl GatewayRunner {
    /// Yeni bir `GatewayRunner` örneği oluşturur ve token doğrulamasını yapar.
    pub fn new(
        token: impl Into<String>,
        intents: u64,
        presence_status: impl Into<String>,
        custom_status: Option<impl Into<String>>,
    ) -> Result<Self, MicupError> {
        let token_str = token.into();
        Bot::validate_token(&token_str)?;
        let client = Arc::new(Client::new(&token_str)?);
        let shard = Shard::new(&token_str, intents).with_presence(presence_status, custom_status);
        let context = Context::new(Arc::clone(&client)).with_shard_sender(shard.sender());
        Ok(Self { shard, context })
    }

    /// Bağlantının `Context` referansını döndürür.
    pub fn context(&self) -> &Context {
        &self.context
    }

    /// Alt seviye `Shard` nesnesine erişim sağlar.
    pub fn shard(&self) -> &Shard {
        &self.shard
    }

    /// Bot nesnesi ile Tokio görev döngüsünü hatasız koşturur.
    pub async fn run(self, bot: Arc<Bot>) -> Result<(), MicupError> {
        let ctx = self.context.clone();
        self.shard
            .run_typed(move |event| {
                let ctx = ctx.clone();
                let state = Arc::clone(&bot);
                async move {
                    Bot::dispatch_event(state, ctx, event).await;
                }
            })
            .await
    }

    /// Ham/tür güvenli olay dinleyicisi ile olay döngüsünü koşturur.
    pub async fn run_loop<F, Fut>(&self, on_event: F) -> Result<(), MicupError>
    where
        F: FnMut(Event) -> Fut,
        Fut: Future<Output = ()>,
    {
        self.shard.run_typed(on_event).await
    }
}

/// Hem closure tabanlı hızlı kurulumu hem de Trait tabanlı mimariyi destekleyen Bot motoru.
pub struct Bot {
    token: String,
    intents: u64,
    presence_status: String,
    custom_status: Option<String>,
    ready_handler: Option<ReadyHandler>,
    message_handler: Option<MessageHandler>,
    interaction_handler: Option<InteractionHandler>,
    command_handlers: HashMap<String, InteractionHandler>,
    button_handlers: HashMap<String, InteractionHandler>,
    voice_state_handler: Option<VoiceStateHandler>,
    voice_server_handler: Option<VoiceServerHandler>,
    event_handler: Option<EventHandlerFn>,
    trait_handler: Option<Arc<dyn EventHandler>>,
}

impl Bot {
    /// Token ile yeni bir Bot oluşturur (varsayılan niyetler: GUILDS | GUILD_MESSAGES | MESSAGE_CONTENT).
    pub fn from_token(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            intents: Intents::default_intents().bits(),
            presence_status: "online".to_string(),
            custom_status: None,
            ready_handler: None,
            message_handler: None,
            interaction_handler: None,
            command_handlers: HashMap::new(),
            button_handlers: HashMap::new(),
            voice_state_handler: None,
            voice_server_handler: None,
            event_handler: None,
            trait_handler: None,
        }
    }

    /// Yapıcı (builder) deseni ile token vererek yeni bir Bot başlatır.
    pub fn builder(token: impl Into<String>) -> Self {
        Self::from_token(token)
    }

    /// Token ile yeni bir Bot oluşturur (`from_token` eşanlamlısı).
    pub fn new(token: impl Into<String>) -> Self {
        Self::from_token(token)
    }

    /// Tokeni hemen doğrulayarak yeni bir Bot oluşturur.
    pub fn try_from_token(token: impl Into<String>) -> Result<Self, MicupError> {
        let token_str = token.into();
        Self::validate_token(&token_str)?;
        Ok(Self::from_token(token_str))
    }

    /// Bot token formatını doğrular (boşluk, kontrol karakterleri ve HTTP başlık geçerliliği).
    pub fn validate_token(token: &str) -> Result<(), MicupError> {
        let trimmed = token.trim();
        if trimmed.is_empty() {
            return Err(MicupError::InvalidToken(
                "Bot tokeni boş olamaz".to_string(),
            ));
        }
        if trimmed.chars().any(|c| c.is_ascii_control()) {
            return Err(MicupError::InvalidToken(
                "Bot tokeni kontrol karakterleri içeremez".to_string(),
            ));
        }
        if reqwest::header::HeaderValue::from_str(&format!("Bot {trimmed}")).is_err() {
            return Err(MicupError::InvalidToken(
                "Bot tokeni HTTP başlığı için geçersiz karakterler içeriyor".to_string(),
            ));
        }
        Ok(())
    }

    /// Bot yapılandırmasını ve tokenini doğrular.
    pub fn validate(&self) -> Result<(), MicupError> {
        Self::validate_token(&self.token)
    }

    /// Yapılandırılmış bot tokenini döndürür.
    pub fn token(&self) -> &str {
        &self.token
    }

    /// Yapılandırılmış niyet (intents) bit maskesini döndürür.
    pub fn intents_bits(&self) -> u64 {
        self.intents
    }

    /// Ağ geçidi niyetlerini ayarlar.
    pub fn intents(mut self, intents: Intents) -> Self {
        self.intents = intents.bits();
        self
    }

    /// Ham sayısal niyet bit maskesini ayarlar.
    pub fn intents_raw(mut self, bits: u64) -> Self {
        self.intents = bits;
        self
    }

    /// Bot durumunu ve özel durum metnini ayarlar.
    pub fn with_presence(
        mut self,
        status: impl Into<String>,
        custom_status: impl Into<String>,
    ) -> Self {
        self.presence_status = status.into();
        self.custom_status = Some(custom_status.into());
        self
    }

    /// Botun çevrimiçi durumunu (online, idle, dnd, invisible) ayarlar.
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.presence_status = status.into();
        self
    }

    /// Botun özel durum (custom activity) metnini ayarlar.
    pub fn custom_status(mut self, custom: impl Into<String>) -> Self {
        self.custom_status = Some(custom.into());
        self
    }

    /// Bot hazır olduğunda çalışacak fonksiyon.
    pub fn on_ready<F, Fut>(mut self, handler: F) -> Self
    where
        F: Fn(Context, ReadyEvent) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.ready_handler = Some(Box::new(move |ctx, ready| Box::pin(handler(ctx, ready))));
        self
    }

    /// Yeni bir mesaj geldiğinde çalışacak fonksiyon.
    pub fn on_message<F, Fut>(mut self, handler: F) -> Self
    where
        F: Fn(Context, Message) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.message_handler = Some(Box::new(move |ctx, msg| Box::pin(handler(ctx, msg))));
        self
    }

    /// Belirtilen Slash komutu tetiklendiğinde doğrudan çalışacak yönlendirici fonksiyon.
    pub fn on_command<F, Fut>(mut self, command_name: impl Into<String>, handler: F) -> Self
    where
        F: Fn(Context, Interaction) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.command_handlers.insert(
            command_name.into(),
            Box::new(move |ctx, i| Box::pin(handler(ctx, i))),
        );
        self
    }

    /// Belirtilen Buton (`custom_id`) tıklandığında doğrudan çalışacak yönlendirici fonksiyon.
    pub fn on_button<F, Fut>(mut self, custom_id: impl Into<String>, handler: F) -> Self
    where
        F: Fn(Context, Interaction) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.button_handlers.insert(
            custom_id.into(),
            Box::new(move |ctx, i| Box::pin(handler(ctx, i))),
        );
        self
    }

    /// Gelen tüm etkileşimler (Interaction) için genel dinleyici.
    pub fn on_interaction<F, Fut>(mut self, handler: F) -> Self
    where
        F: Fn(Context, Interaction) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.interaction_handler = Some(Box::new(move |ctx, i| Box::pin(handler(ctx, i))));
        self
    }

    /// Ses durumu değişiklikleri (`VOICE_STATE_UPDATE`) için dinleyici.
    pub fn on_voice_state<F, Fut>(mut self, handler: F) -> Self
    where
        F: Fn(Context, VoiceState) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.voice_state_handler = Some(Box::new(move |ctx, vs| Box::pin(handler(ctx, vs))));
        self
    }

    /// Ses sunucusu güncellemeleri (`VOICE_SERVER_UPDATE` / LiveKit endpoint) için dinleyici.
    pub fn on_voice_server<F, Fut>(mut self, handler: F) -> Self
    where
        F: Fn(Context, VoiceServerUpdate) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.voice_server_handler = Some(Box::new(move |ctx, vsu| Box::pin(handler(ctx, vsu))));
        self
    }

    /// Ham/tür güvenli tüm olaylar için genel dinleyici.
    pub fn on_event<F, Fut>(mut self, handler: F) -> Self
    where
        F: Fn(Context, Event) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.event_handler = Some(Box::new(move |ctx, ev| Box::pin(handler(ctx, ev))));
        self
    }

    /// Yapısal `EventHandler` trait uygulayıcısını bağlar.
    pub fn handler<H: EventHandler>(mut self, handler: H) -> Self {
        self.trait_handler = Some(Arc::new(handler));
        self
    }

    /// Olayları ilgili trait ve closure dinleyicilerine yönlendirir.
    async fn dispatch_event(state: Arc<Self>, ctx: Context, event: Event) {
        // 1. Genel trait handler
        if let Some(ref th) = state.trait_handler {
            th.on_event(ctx.clone(), event.clone()).await;
        }
        // 2. Genel closure handler
        if let Some(ref eh) = state.event_handler {
            eh(ctx.clone(), event.clone()).await;
        }

        match event {
            Event::Ready(ready) => {
                if let Some(ref th) = state.trait_handler {
                    th.on_ready(ctx.clone(), ready.clone()).await;
                }
                if let Some(ref rh) = state.ready_handler {
                    rh(ctx, ready).await;
                }
            }
            Event::MessageCreate(msg) => {
                if let Some(ref th) = state.trait_handler {
                    th.on_message(ctx.clone(), msg.clone()).await;
                }
                if let Some(ref mh) = state.message_handler {
                    mh(ctx, msg).await;
                }
            }
            Event::InteractionCreate(interaction) => {
                if let Some(ref th) = state.trait_handler {
                    th.on_interaction(ctx.clone(), interaction.clone()).await;
                }

                // Slash komut yönlendirmesi
                if interaction.kind == InteractionType::ApplicationCommand {
                    if let Some(ref data) = interaction.data {
                        if let Some(ref name) = data.name {
                            if let Some(cmd_fn) = state.command_handlers.get(name) {
                                cmd_fn(ctx.clone(), interaction.clone()).await;
                                return;
                            }
                        }
                    }
                }

                // Buton tıklama yönlendirmesi
                if interaction.kind == InteractionType::MessageComponent {
                    if let Some(ref data) = interaction.data {
                        if let Some(ref cid) = data.custom_id {
                            if let Some(btn_fn) = state.button_handlers.get(cid) {
                                btn_fn(ctx.clone(), interaction.clone()).await;
                                return;
                            }
                        }
                    }
                }

                if let Some(ref ih) = state.interaction_handler {
                    ih(ctx, interaction).await;
                }
            }
            Event::VoiceStateUpdate(voice) => {
                if let Some(ref th) = state.trait_handler {
                    th.on_voice_state(ctx.clone(), voice.clone()).await;
                }
                if let Some(ref vh) = state.voice_state_handler {
                    vh(ctx, voice).await;
                }
            }
            Event::VoiceServerUpdate(vsu) => {
                if let Some(ref th) = state.trait_handler {
                    th.on_voice_server(ctx.clone(), vsu.clone()).await;
                }
                if let Some(ref vsh) = state.voice_server_handler {
                    vsh(ctx, vsu).await;
                }
            }
            _ => {}
        }
    }

    /// Botu başlatır, REST Client ve Gateway bağlantısını kurup GatewayRunner ile Tokio görev döngüsünü yürütür.
    pub async fn run(self) -> Result<(), MicupError> {
        self.validate()?;
        let runner = GatewayRunner::new(
            &self.token,
            self.intents,
            &self.presence_status,
            self.custom_status.as_deref(),
        )?;
        let bot = Arc::new(self);
        runner.run(bot).await
    }

    /// `run` metodunun asenkron eşanlamlısı (alias). Tokio tabanlı Gateway görev döngüsünü koşturur.
    pub async fn start(self) -> Result<(), MicupError> {
        self.run().await
    }

    /// Botu arka planda bir Tokio görevi (task) olarak başlatır ve görev tanıtıcısını (`JoinHandle`) döndürür.
    pub fn spawn(self) -> tokio::task::JoinHandle<Result<(), MicupError>> {
        tokio::spawn(async move { self.run().await })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bot_from_token_and_builder() {
        let bot = Bot::from_token("valid_bot_token")
            .intents(Intents::new(Intents::GUILDS | Intents::GUILD_MESSAGES))
            .with_presence("idle", "kod yazıyor");

        assert_eq!(bot.token(), "valid_bot_token");
        assert_eq!(bot.presence_status, "idle");
        assert_eq!(bot.custom_status.as_deref(), Some("kod yazıyor"));
        assert_eq!(
            bot.intents_bits(),
            Intents::new(Intents::GUILDS | Intents::GUILD_MESSAGES).bits()
        );

        let bot2 = Bot::builder("another_token")
            .status("dnd")
            .custom_status("rahatsız etmeyin");
        assert_eq!(bot2.token(), "another_token");
        assert_eq!(bot2.presence_status, "dnd");
        assert_eq!(bot2.custom_status.as_deref(), Some("rahatsız etmeyin"));
    }

    #[test]
    fn test_token_validation() {
        assert!(Bot::validate_token("valid_token_123").is_ok());
        assert!(Bot::validate_token("").is_err());
        assert!(Bot::validate_token("   ").is_err());
        assert!(Bot::validate_token("token\nwith\nnewline").is_err());
        assert!(Bot::validate_token("token\x00with_null").is_err());

        assert!(Bot::try_from_token("valid_token").is_ok());
        assert!(Bot::try_from_token("").is_err());
    }

    #[test]
    fn test_gateway_runner_construction() {
        let runner = GatewayRunner::new(
            "valid_token",
            Intents::default_intents().bits(),
            "online",
            Some("micup-rs"),
        );
        assert!(runner.is_ok());

        let invalid_runner = GatewayRunner::new(
            "",
            Intents::default_intents().bits(),
            "online",
            None::<&str>,
        );
        assert!(invalid_runner.is_err());
    }

    #[test]
    fn test_handler_registration_and_builder_methods() {
        let bot = Bot::from_token("token")
            .on_ready(|_, _| async move {})
            .on_message(|_, _| async move {})
            .on_command("ping", |_, _| async move {})
            .on_button("btn_1", |_, _| async move {})
            .on_interaction(|_, _| async move {})
            .on_voice_state(|_, _| async move {})
            .on_voice_server(|_, _| async move {})
            .on_event(|_, _| async move {});

        assert!(bot.ready_handler.is_some());
        assert!(bot.message_handler.is_some());
        assert!(bot.command_handlers.contains_key("ping"));
        assert!(bot.button_handlers.contains_key("btn_1"));
        assert!(bot.interaction_handler.is_some());
        assert!(bot.voice_state_handler.is_some());
        assert!(bot.voice_server_handler.is_some());
        assert!(bot.event_handler.is_some());
    }
}
