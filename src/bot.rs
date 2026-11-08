//! Yüksek Seviyeli Bot Çatısı (Bot Framework & Runner).
//!
//! Hem hızlı kodlama (vibe coding) hem de yapısal mimari (structured hand-crafted code)
//! geliştirenler için sıfır boilerplate ile tam donanımlı Micup bot motoru.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::client::Client;
use crate::context::Context;
use crate::error::MicupError;
use crate::events::{Event, ReadyEvent, VoiceState};
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
    fn on_event<'a>(&'a self, _ctx: Context, _event: Event) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }
}

type ReadyHandler = Box<dyn Fn(Context, ReadyEvent) -> BoxFuture<'static, ()> + Send + Sync>;
type MessageHandler = Box<dyn Fn(Context, Message) -> BoxFuture<'static, ()> + Send + Sync>;
type InteractionHandler = Box<dyn Fn(Context, Interaction) -> BoxFuture<'static, ()> + Send + Sync>;
type EventHandlerFn = Box<dyn Fn(Context, Event) -> BoxFuture<'static, ()> + Send + Sync>;

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
            event_handler: None,
            trait_handler: None,
        }
    }

    /// Ağ geçidi niyetlerini ayarlar.
    pub fn intents(mut self, intents: Intents) -> Self {
        self.intents = intents.bits();
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

    /// Botu başlatır, REST Client ve Gateway bağlantısını kurup olay döngüsünü yürütür.
    pub async fn run(self) -> Result<(), MicupError> {
        let client = Arc::new(Client::new(&self.token)?);
        let shard = Shard::new(&self.token, self.intents)
            .with_presence(self.presence_status.clone(), self.custom_status.clone());
        let ctx = Context::new(Arc::clone(&client)).with_shard_sender(shard.sender());

        let bot_state = Arc::new(self);

        shard
            .run_typed(move |event| {
                let ctx = ctx.clone();
                let state = Arc::clone(&bot_state);
                async move {
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
                                th.on_voice_state(ctx, voice).await;
                            }
                        }
                        _ => {}
                    }
                }
            })
            .await
    }
}
