use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::context::Context;
use crate::error::MicupError;
use crate::interaction::{CommandInteractionDataOption, Interaction, InteractionType};
use crate::models::{Id, Message};

/// Asenkron komut işleyicisi dönüş türü (`BoxFuture`).
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Önek (prefix) komut işleyici fonksiyon türü.
pub type PrefixCommandHandler =
    Arc<dyn Fn(Context, Message, Args) -> BoxFuture<'static, Result<(), MicupError>> + Send + Sync>;

/// Etkileşim (slash) komut işleyici fonksiyon türü.
pub type InteractionCommandHandler = Arc<
    dyn Fn(Context, Interaction, InteractionArgs) -> BoxFuture<'static, Result<(), MicupError>>
        + Send
        + Sync,
>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandType {
    ChatInput,
    User,
    Message,
    Unknown(u64),
}

impl Serialize for CommandType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ChatInput => serializer.serialize_u8(1),
            Self::User => serializer.serialize_u8(2),
            Self::Message => serializer.serialize_u8(3),
            Self::Unknown(v) => serializer.serialize_u64(*v),
        }
    }
}

impl<'de> Deserialize<'de> for CommandType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = u64::deserialize(deserializer)?;
        Ok(match v {
            1 => Self::ChatInput,
            2 => Self::User,
            3 => Self::Message,
            other => Self::Unknown(other),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandOptionType {
    SubCommand,
    SubCommandGroup,
    String,
    Integer,
    Boolean,
    User,
    Channel,
    Role,
    Mentionable,
    Number,
    Attachment,
    Unknown(u64),
}

impl Serialize for CommandOptionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let val = match self {
            Self::SubCommand => 1,
            Self::SubCommandGroup => 2,
            Self::String => 3,
            Self::Integer => 4,
            Self::Boolean => 5,
            Self::User => 6,
            Self::Channel => 7,
            Self::Role => 8,
            Self::Mentionable => 9,
            Self::Number => 10,
            Self::Attachment => 11,
            Self::Unknown(v) => *v,
        };
        serializer.serialize_u64(val)
    }
}

impl<'de> Deserialize<'de> for CommandOptionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = u64::deserialize(deserializer)?;
        Ok(match v {
            1 => Self::SubCommand,
            2 => Self::SubCommandGroup,
            3 => Self::String,
            4 => Self::Integer,
            5 => Self::Boolean,
            6 => Self::User,
            7 => Self::Channel,
            8 => Self::Role,
            9 => Self::Mentionable,
            10 => Self::Number,
            11 => Self::Attachment,
            other => Self::Unknown(other),
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandOptionChoice {
    pub name: String,
    pub value: serde_json::Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandOption {
    #[serde(rename = "type")]
    pub kind: CommandOptionType,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub required: bool,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub choices: Vec<CommandOptionChoice>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub options: Vec<CommandOption>,
}

impl CommandOption {
    pub fn new(
        kind: CommandOptionType,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            name: name.into(),
            description: description.into(),
            required: false,
            choices: Vec::new(),
            options: Vec::new(),
        }
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn choice(mut self, name: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        self.choices.push(CommandOptionChoice {
            name: name.into(),
            value: value.into(),
        });
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateCommand {
    pub name: String,
    pub description: String,
    #[serde(rename = "type", default = "default_command_type")]
    pub kind: CommandType,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub options: Vec<CommandOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_member_permissions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm_permission: Option<bool>,
}

fn default_command_type() -> CommandType {
    CommandType::ChatInput
}

impl CreateCommand {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            kind: CommandType::ChatInput,
            options: Vec::new(),
            default_member_permissions: None,
            dm_permission: None,
        }
    }

    pub fn option(mut self, option: CommandOption) -> Self {
        self.options.push(option);
        self
    }

    pub fn dm_permission(mut self, allowed: bool) -> Self {
        self.dm_permission = Some(allowed);
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Command {
    pub id: Id,
    pub application_id: Id,
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub kind: CommandType,
    #[serde(default)]
    pub options: Vec<CommandOption>,
    #[serde(default)]
    pub guild_id: Option<Id>,
}

/// Önek (prefix) komutları için ayrıştırılmış parametreler listesi.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Args {
    raw: String,
    tokens: Vec<String>,
}

impl Args {
    /// Ham argüman metnini tırnak ve boşluk kurallarına göre ayrıştırarak yeni bir `Args` nesnesi oluşturur.
    pub fn new(raw: impl Into<String>) -> Self {
        let raw = raw.into();
        let tokens = Self::parse_tokens(&raw);
        Self { raw, tokens }
    }

    /// Tırnak işaretlerini ("..." veya '...') ve kaçış karakterlerini (`\`) dikkate alarak parametre dizisini ayrıştırır.
    pub fn parse_tokens(input: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current = String::new();
        let mut in_quote = None;
        let mut chars = input.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '\\' => {
                    if let Some(next) = chars.next() {
                        current.push(next);
                    }
                }
                '"' | '\'' => {
                    if let Some(q) = in_quote {
                        if q == c {
                            in_quote = None;
                        } else {
                            current.push(c);
                        }
                    } else {
                        in_quote = Some(c);
                    }
                }
                c if c.is_whitespace() && in_quote.is_none() => {
                    if !current.is_empty() {
                        tokens.push(std::mem::take(&mut current));
                    }
                }
                _ => {
                    current.push(c);
                }
            }
        }

        if !current.is_empty() {
            tokens.push(current);
        }

        tokens
    }

    /// Ham argüman dizesini döndürür.
    pub fn raw(&self) -> &str {
        &self.raw
    }

    /// Ayrıştırılmış parametre dizisini referans olarak döndürür.
    pub fn tokens(&self) -> &[String] {
        &self.tokens
    }

    /// Parametre sayısını döndürür.
    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    /// Hiç parametre verilip verilmediğini kontrol eder.
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    /// Belirtilen indisteki parametreyi opsiyonel referans olarak döndürür.
    pub fn get(&self, index: usize) -> Option<&str> {
        self.tokens.get(index).map(|s| s.as_str())
    }

    /// Belirtilen indisteki parametreyi zorunlu olarak alır. Parametre yoksa `ModelValidation` hatası döner.
    pub fn require_str(&self, index: usize, param_name: &str) -> Result<&str, MicupError> {
        self.get(index).ok_or_else(|| {
            MicupError::ModelValidation(format!(
                "Eksik zorunlu parametre: '{}' (indeks: {})",
                param_name, index
            ))
        })
    }

    /// Belirtilen indisteki parametreyi hedef türe (`FromStr`) dönüştürmeyi dener.
    /// Parametre mevcut değilse `Ok(None)` döner; geçersizse `ModelValidation` hatası döner.
    pub fn parse<T: std::str::FromStr>(&self, index: usize) -> Result<Option<T>, MicupError>
    where
        T::Err: std::fmt::Display,
    {
        match self.get(index) {
            Some(val) => val.parse::<T>().map(Some).map_err(|e| {
                MicupError::ModelValidation(format!(
                    "Parametre dönüştürülemedi (indeks: {}): {}",
                    index, e
                ))
            }),
            None => Ok(None),
        }
    }

    /// Belirtilen indisteki parametreyi zorunlu olarak hedef türe (`FromStr`) dönüştürür.
    /// Eksik veya geçersizse uygun `ModelValidation` hatası döner.
    pub fn require<T: std::str::FromStr>(
        &self,
        index: usize,
        param_name: &str,
    ) -> Result<T, MicupError>
    where
        T::Err: std::fmt::Display,
    {
        let raw = self.require_str(index, param_name)?;
        raw.parse::<T>().map_err(|e| {
            MicupError::ModelValidation(format!(
                "Geçersiz '{}' parametre değeri: {}",
                param_name, e
            ))
        })
    }

    /// Belirli bir indisten sonraki tüm parametreleri boşlukla birleştirerek kalan metni döndürür.
    pub fn rest(&self, from_index: usize) -> Option<String> {
        if from_index >= self.tokens.len() {
            None
        } else {
            Some(self.tokens[from_index..].join(" "))
        }
    }

    /// Parametre dilimini döndürür.
    pub fn as_slice(&self) -> &[String] {
        &self.tokens
    }
}

/// Etkileşim (Interaction/Slash komutu) için parametre çözümleyici.
#[derive(Clone, Debug, Default)]
pub struct InteractionArgs {
    options: HashMap<String, CommandInteractionDataOption>,
    list: Vec<CommandInteractionDataOption>,
}

impl InteractionArgs {
    /// Seçenek listesinden yeni bir `InteractionArgs` oluşturur.
    pub fn new(options: Vec<CommandInteractionDataOption>) -> Self {
        let mut map = HashMap::new();
        for opt in &options {
            map.insert(opt.name.clone(), opt.clone());
        }
        Self {
            options: map,
            list: options,
        }
    }

    /// Etkileşim nesnesinden `InteractionArgs` üretir.
    pub fn from_interaction(interaction: &Interaction) -> Self {
        if let Some(ref data) = interaction.data {
            Self::new(data.options.clone())
        } else {
            Self::default()
        }
    }

    /// Adı verilen seçeneği opsiyonel referans olarak döndürür.
    pub fn get(&self, name: &str) -> Option<&CommandInteractionDataOption> {
        self.options.get(name)
    }

    /// Tüm seçeneklerin dilimini döndürür.
    pub fn options(&self) -> &[CommandInteractionDataOption] {
        &self.list
    }

    /// Metin (string) değerini opsiyonel olarak döndürür.
    pub fn get_str(&self, name: &str) -> Option<&str> {
        self.get(name).and_then(|opt| opt.as_str())
    }

    /// Metin değerini zorunlu olarak alır. Parametre yoksa veya metin değilse `ModelValidation` hatası döner.
    pub fn require_str(&self, name: &str) -> Result<&str, MicupError> {
        self.get_str(name).ok_or_else(|| {
            MicupError::ModelValidation(format!("Eksik veya geçersiz metin seçeneği: '{}'", name))
        })
    }

    /// Tamsayı (`i64`) değerini opsiyonel olarak döndürür.
    pub fn get_i64(&self, name: &str) -> Option<i64> {
        self.get(name).and_then(|opt| opt.as_i64())
    }

    /// Tamsayı değerini zorunlu olarak alır. Parametre yoksa veya tamsayı değilse `ModelValidation` hatası döner.
    pub fn require_i64(&self, name: &str) -> Result<i64, MicupError> {
        self.get_i64(name).ok_or_else(|| {
            MicupError::ModelValidation(format!("Eksik veya geçersiz tamsayı seçeneği: '{}'", name))
        })
    }

    /// Pozitif tamsayı (`u64`) değerini opsiyonel olarak döndürür.
    pub fn get_u64(&self, name: &str) -> Option<u64> {
        self.get(name).and_then(|opt| opt.as_u64())
    }

    /// Pozitif tamsayı değerini zorunlu olarak alır.
    pub fn require_u64(&self, name: &str) -> Result<u64, MicupError> {
        self.get_u64(name).ok_or_else(|| {
            MicupError::ModelValidation(format!(
                "Eksik veya geçersiz pozitif tamsayı seçeneği: '{}'",
                name
            ))
        })
    }

    /// Mantıksal (`bool`) değerini opsiyonel olarak döndürür.
    pub fn get_bool(&self, name: &str) -> Option<bool> {
        self.get(name).and_then(|opt| opt.as_bool())
    }

    /// Mantıksal değerini zorunlu olarak alır.
    pub fn require_bool(&self, name: &str) -> Result<bool, MicupError> {
        self.get_bool(name).ok_or_else(|| {
            MicupError::ModelValidation(format!(
                "Eksik veya geçersiz mantıksal (boolean) seçeneği: '{}'",
                name
            ))
        })
    }

    /// Ondalıklı sayı (`f64`) değerini opsiyonel olarak döndürür.
    pub fn get_f64(&self, name: &str) -> Option<f64> {
        self.get(name).and_then(|opt| opt.as_f64())
    }

    /// Ondalıklı sayı değerini zorunlu olarak alır.
    pub fn require_f64(&self, name: &str) -> Result<f64, MicupError> {
        self.get_f64(name).ok_or_else(|| {
            MicupError::ModelValidation(format!(
                "Eksik veya geçersiz ondalıklı sayı seçeneği: '{}'",
                name
            ))
        })
    }
}

/// Önek ve etkileşim tabanlı komutların kaydını tutan ve yöneten Komut Kayıt Yöneticisi (CommandRegistry).
#[derive(Clone)]
pub struct CommandRegistry {
    prefix: String,
    prefix_commands: HashMap<String, PrefixCommandHandler>,
    interaction_commands: HashMap<String, InteractionCommandHandler>,
    command_definitions: HashMap<String, CreateCommand>,
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self {
            prefix: "!".to_string(),
            prefix_commands: HashMap::new(),
            interaction_commands: HashMap::new(),
            command_definitions: HashMap::new(),
        }
    }
}

impl CommandRegistry {
    /// Varsayılan önek (`!`) ile yeni bir `CommandRegistry` oluşturur.
    pub fn new() -> Self {
        Self::default()
    }

    /// Özel bir komut öneki ile yeni bir `CommandRegistry` oluşturur.
    pub fn with_prefix(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
            ..Default::default()
        }
    }

    /// Komut önekini döndürür.
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// Komut önekini günceller.
    pub fn set_prefix(&mut self, prefix: impl Into<String>) -> &mut Self {
        self.prefix = prefix.into();
        self
    }

    /// Yeni bir önek (prefix) komut işleyicisi kaydeder.
    pub fn register_prefix<F, Fut>(&mut self, name: impl Into<String>, handler: F) -> &mut Self
    where
        F: Fn(Context, Message, Args) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), MicupError>> + Send + 'static,
    {
        let handler = Arc::new(move |ctx, msg, args| {
            Box::pin(handler(ctx, msg, args)) as BoxFuture<'static, Result<(), MicupError>>
        });
        self.prefix_commands.insert(name.into(), handler);
        self
    }

    /// Zincirleme çağrı için önek komutu kaydeder (builder-style).
    pub fn register_prefix_cmd<F, Fut>(mut self, name: impl Into<String>, handler: F) -> Self
    where
        F: Fn(Context, Message, Args) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), MicupError>> + Send + 'static,
    {
        self.register_prefix(name, handler);
        self
    }

    /// Yeni bir etkileşim (slash) komut işleyicisi kaydeder.
    pub fn register_interaction<F, Fut>(&mut self, name: impl Into<String>, handler: F) -> &mut Self
    where
        F: Fn(Context, Interaction, InteractionArgs) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), MicupError>> + Send + 'static,
    {
        let handler = Arc::new(move |ctx, interaction, args| {
            Box::pin(handler(ctx, interaction, args)) as BoxFuture<'static, Result<(), MicupError>>
        });
        self.interaction_commands.insert(name.into(), handler);
        self
    }

    /// `register_interaction` metodunun eşanlamlısı.
    pub fn register_slash<F, Fut>(&mut self, name: impl Into<String>, handler: F) -> &mut Self
    where
        F: Fn(Context, Interaction, InteractionArgs) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), MicupError>> + Send + 'static,
    {
        self.register_interaction(name, handler)
    }

    /// Zincirleme çağrı için slash komutu kaydeder (builder-style).
    pub fn register_slash_cmd<F, Fut>(mut self, name: impl Into<String>, handler: F) -> Self
    where
        F: Fn(Context, Interaction, InteractionArgs) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), MicupError>> + Send + 'static,
    {
        self.register_slash(name, handler);
        self
    }

    /// Hem komut tanımını (`CreateCommand`) hem de asenkron işleyicisini birlikte kaydeder.
    pub fn register_command<F, Fut>(&mut self, definition: CreateCommand, handler: F) -> &mut Self
    where
        F: Fn(Context, Interaction, InteractionArgs) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), MicupError>> + Send + 'static,
    {
        let name = definition.name.clone();
        self.command_definitions.insert(name.clone(), definition);
        self.register_interaction(name, handler);
        self
    }

    /// Belirtilen isimde bir önek komutunun kayıtlı olup olmadığını döndürür.
    pub fn has_prefix_command(&self, name: &str) -> bool {
        self.prefix_commands.contains_key(name)
    }

    /// Belirtilen isimde bir etkileşim komutunun kayıtlı olup olmadığını döndürür.
    pub fn has_interaction_command(&self, name: &str) -> bool {
        self.interaction_commands.contains_key(name)
    }

    /// Kayıtlı önek komut isimlerini döndürür.
    pub fn prefix_commands(&self) -> impl Iterator<Item = &String> {
        self.prefix_commands.keys()
    }

    /// Kayıtlı etkileşim komut isimlerini döndürür.
    pub fn interaction_commands(&self) -> impl Iterator<Item = &String> {
        self.interaction_commands.keys()
    }

    /// Kayıtlı komut tanımını döndürür.
    pub fn command_definition(&self, name: &str) -> Option<&CreateCommand> {
        self.command_definitions.get(name)
    }

    /// Gelen mesajı önek komutları ile eşler, parametrelerini ayrıştırır ve eşleşen işleyiciyi Context ile tetikler.
    /// Komut bulundu ve başarıyla çalıştırıldıysa `Ok(true)`, eşleşen komut yoksa `Ok(false)` döner.
    pub async fn dispatch_message(&self, ctx: &Context, msg: &Message) -> Result<bool, MicupError> {
        if !msg.content.starts_with(&self.prefix) {
            return Ok(false);
        }

        let without_prefix = &msg.content[self.prefix.len()..];
        let trimmed = without_prefix.trim_start();
        if trimmed.is_empty() {
            return Ok(false);
        }

        let (cmd_name, rest) = match trimmed.split_once(char::is_whitespace) {
            Some((cmd, r)) => (cmd, r.trim_start()),
            None => (trimmed, ""),
        };

        let handler = match self.prefix_commands.get(cmd_name) {
            Some(h) => Arc::clone(h),
            None => return Ok(false),
        };

        let args = Args::new(rest);
        handler(ctx.clone(), msg.clone(), args).await?;
        Ok(true)
    }

    /// Gelen etkileşimi (Interaction) komut adı ve parametreleri ile eşler, Context ile asenkron işleyiciyi tetikler.
    /// Eksik veya geçersiz parametre durumlarında `ModelValidation` hatası döndürür.
    pub async fn dispatch_interaction(
        &self,
        ctx: &Context,
        interaction: &Interaction,
    ) -> Result<bool, MicupError> {
        if interaction.kind != InteractionType::ApplicationCommand {
            return Ok(false);
        }

        let data = match interaction.data.as_ref() {
            Some(d) => d,
            None => return Ok(false),
        };

        let name = match data.name.as_deref() {
            Some(n) => n,
            None => return Ok(false),
        };

        let handler = match self.interaction_commands.get(name) {
            Some(h) => Arc::clone(h),
            None => return Ok(false),
        };

        let args = InteractionArgs::new(data.options.clone());

        // Eğer komut tanımı varsa zorunlu parametre ve tür doğrulamalarını yap
        if let Some(definition) = self.command_definitions.get(name) {
            for opt in &definition.options {
                if opt.required {
                    let opt_val = args.get(&opt.name).ok_or_else(|| {
                        MicupError::ModelValidation(format!(
                            "Zorunlu komut seçeneği eksik: '{}'",
                            opt.name
                        ))
                    })?;

                    match opt.kind {
                        CommandOptionType::String if opt_val.as_str().is_none() => {
                            return Err(MicupError::ModelValidation(format!(
                                "'{}' seçeneği metin (string) türünde olmalıdır",
                                opt.name
                            )));
                        }
                        CommandOptionType::Integer if opt_val.as_i64().is_none() => {
                            return Err(MicupError::ModelValidation(format!(
                                "'{}' seçeneği tamsayı (integer) türünde olmalıdır",
                                opt.name
                            )));
                        }
                        CommandOptionType::Boolean if opt_val.as_bool().is_none() => {
                            return Err(MicupError::ModelValidation(format!(
                                "'{}' seçeneği mantıksal (boolean) türünde olmalıdır",
                                opt.name
                            )));
                        }
                        CommandOptionType::Number if opt_val.as_f64().is_none() => {
                            return Err(MicupError::ModelValidation(format!(
                                "'{}' seçeneği sayı (number) türünde olmalıdır",
                                opt.name
                            )));
                        }
                        _ => {}
                    }
                }
            }
        }

        handler(ctx.clone(), interaction.clone(), args).await?;
        Ok(true)
    }
}

/// Mesaj ve etkileşim komutlarını yönlendiren merkezi yönlendirici (CommandDispatcher).
#[derive(Clone, Default)]
pub struct CommandDispatcher {
    registry: Arc<CommandRegistry>,
}

impl CommandDispatcher {
    /// Verilen `CommandRegistry` nesnesinden yeni bir `CommandDispatcher` oluşturur.
    pub fn new(registry: CommandRegistry) -> Self {
        Self {
            registry: Arc::new(registry),
        }
    }

    /// `Arc<CommandRegistry>` referansından yönlendirici oluşturur.
    pub fn from_arc(registry: Arc<CommandRegistry>) -> Self {
        Self { registry }
    }

    /// İç kayıt yöneticisine referans döndürür.
    pub fn registry(&self) -> &CommandRegistry {
        &self.registry
    }

    /// Mesajı kayıtlı önek komutlarına yönlendirir.
    pub async fn dispatch_message(&self, ctx: &Context, msg: &Message) -> Result<bool, MicupError> {
        self.registry.dispatch_message(ctx, msg).await
    }

    /// Etkileşimi kayıtlı slash komutlarına yönlendirir.
    pub async fn dispatch_interaction(
        &self,
        ctx: &Context,
        interaction: &Interaction,
    ) -> Result<bool, MicupError> {
        self.registry.dispatch_interaction(ctx, interaction).await
    }
}

impl std::ops::Deref for CommandDispatcher {
    type Target = CommandRegistry;

    fn deref(&self) -> &Self::Target {
        &self.registry
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::Client;
    use crate::interaction::InteractionData;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[test]
    fn test_args_tokenization_and_parsing() {
        let args = Args::new("!ban 12345 7 \"Kural ihlali ve spam\" true");
        assert_eq!(args.len(), 5);
        assert_eq!(args.get(0), Some("!ban"));
        assert_eq!(args.get(1), Some("12345"));
        assert_eq!(args.get(2), Some("7"));
        assert_eq!(args.get(3), Some("Kural ihlali ve spam"));
        assert_eq!(args.get(4), Some("true"));

        let user_id: u64 = args.require(1, "user_id").expect("u64 parse edilmeli");
        assert_eq!(user_id, 12345);

        let days: i32 = args.require(2, "days").expect("i32 parse edilmeli");
        assert_eq!(days, 7);

        let reason = args.require_str(3, "reason").expect("str alinmali");
        assert_eq!(reason, "Kural ihlali ve spam");

        let notify: bool = args.require(4, "notify").expect("bool parse edilmeli");
        assert!(notify);

        // Eksik parametre kontrolü
        let missing = args.require::<u64>(10, "missing");
        assert!(missing.is_err());
        match missing {
            Err(MicupError::ModelValidation(msg)) => {
                assert!(msg.contains("Eksik zorunlu parametre"));
            }
            _ => panic!("ModelValidation hatası bekleniyordu"),
        }

        // Hatalı tip dönüştürme kontrolü
        let invalid = args.require::<i32>(3, "invalid_num");
        assert!(invalid.is_err());
        match invalid {
            Err(MicupError::ModelValidation(msg)) => {
                assert!(msg.contains("Geçersiz"));
            }
            _ => panic!("ModelValidation hatası bekleniyordu"),
        }

        assert_eq!(args.rest(3), Some("Kural ihlali ve spam true".to_string()));
    }

    #[tokio::test]
    async fn test_prefix_command_dispatch_with_context() {
        let mut registry = CommandRegistry::with_prefix("!");
        let executed = Arc::new(AtomicBool::new(false));
        let executed_clone = Arc::clone(&executed);

        registry.register_prefix("selam", move |_ctx, msg, args| {
            let flag = Arc::clone(&executed_clone);
            async move {
                assert_eq!(msg.content, "!selam micup dünya");
                assert_eq!(args.get(0), Some("micup"));
                assert_eq!(args.get(1), Some("dünya"));
                flag.store(true, Ordering::SeqCst);
                Ok(())
            }
        });

        let client = Arc::new(Client::new("test_token").unwrap());
        let ctx = Context::new(client);

        let mut msg = Message {
            id: Id::new(100),
            channel_id: Some(Id::new(200)),
            guild_id: Some(Id::new(300)),
            author: None,
            content: "!selam micup dünya".to_string(),
            timestamp: None,
            edited_timestamp: None,
            pinned: false,
            mention_everyone: false,
            embeds: vec![],
            attachments: vec![],
            components: vec![],
            reactions: vec![],
        };

        let dispatcher = CommandDispatcher::new(registry);
        let handled = dispatcher.dispatch_message(&ctx, &msg).await.unwrap();
        assert!(handled);
        assert!(executed.load(Ordering::SeqCst));

        // Eşleşmeyen komut kontrolü
        msg.content = "!taninmayan_komut".to_string();
        let unhandled = dispatcher.dispatch_message(&ctx, &msg).await.unwrap();
        assert!(!unhandled);
    }

    #[tokio::test]
    async fn test_interaction_command_dispatch_and_validation() {
        let mut registry = CommandRegistry::new();
        let executed = Arc::new(AtomicBool::new(false));
        let executed_clone = Arc::clone(&executed);

        let cmd_def = CreateCommand::new("ban", "Kullanıcıyı uzaklaştırır")
            .option(
                CommandOption::new(CommandOptionType::String, "user", "Kullanıcı").required(true),
            )
            .option(
                CommandOption::new(CommandOptionType::Integer, "days", "Gün sayısı").required(true),
            );

        registry.register_command(cmd_def, move |_ctx, _i, args| {
            let flag = Arc::clone(&executed_clone);
            async move {
                let user = args.require_str("user")?;
                assert_eq!(user, "ali");
                let days = args.require_i64("days")?;
                assert_eq!(days, 5);
                flag.store(true, Ordering::SeqCst);
                Ok(())
            }
        });

        let client = Arc::new(Client::new("test_token").unwrap());
        let ctx = Context::new(client);

        let mut interaction = Interaction {
            id: Id::new(1),
            application_id: Id::new(2),
            kind: InteractionType::ApplicationCommand,
            data: Some(Box::new(InteractionData {
                name: Some("ban".to_string()),
                options: vec![
                    CommandInteractionDataOption {
                        name: "user".to_string(),
                        kind: 3,
                        value: Some(serde_json::json!("ali")),
                        options: vec![],
                        focused: None,
                    },
                    CommandInteractionDataOption {
                        name: "days".to_string(),
                        kind: 4,
                        value: Some(serde_json::json!(5)),
                        options: vec![],
                        focused: None,
                    },
                ],
                ..Default::default()
            })),
            ..Default::default()
        };

        let dispatcher = CommandDispatcher::new(registry);
        let ok = dispatcher
            .dispatch_interaction(&ctx, &interaction)
            .await
            .unwrap();
        assert!(ok);
        assert!(executed.load(Ordering::SeqCst));

        // Eksik zorunlu parametre testi: "days" seçeneğini çıkaralım
        interaction.data.as_mut().unwrap().options.pop();
        let err = dispatcher
            .dispatch_interaction(&ctx, &interaction)
            .await
            .unwrap_err();
        match err {
            MicupError::ModelValidation(msg) => {
                assert!(msg.contains("Zorunlu komut seçeneği eksik: 'days'"));
            }
            _ => panic!("ModelValidation hatası bekleniyordu"),
        }
    }
}
