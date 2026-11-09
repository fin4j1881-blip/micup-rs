use thiserror::Error;
use tokio_tungstenite::tungstenite;

/// micup kütüphanesi için standart sonuç (Result) türü.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// micup kütüphanesi için merkezi hata türü.
#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP Hatası: {0}")]
    Http(#[from] reqwest::Error),

    #[error("WebSocket Gateway Hatası: {0}")]
    Gateway(String),

    #[error("Serde JSON Hatası: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("G/Ç (I/O) Hatası: {0}")]
    Io(#[from] std::io::Error),

    #[error("Micup API Hatası (HTTP {status}): {message}")]
    Api { status: u16, message: String },

    #[error("Hız Sınırı (Rate Limit) Aşıldı: {0}")]
    RateLimited(String),

    #[error("Geçersiz Bot Tokeni: {0}")]
    InvalidToken(String),

    #[error("Model Doğrulama Hatası: {0}")]
    ModelValidation(String),

    #[error("Ses (Voice) Hatası: {0}")]
    Voice(String),

    #[error("3B Sahne (Stage) Hatası: {0}")]
    Stage(String),

    #[error("Yetki Hatası: {0}")]
    Permission(String),

    #[error("CDN Hatası: {0}")]
    Cdn(String),

    #[error("Makro Doğrulama Hatası: {0}")]
    MacroValidation(String),

    #[error("Diğer Hata: {0}")]
    Other(String),
}

/// Geriye dönük uyumluluk ve takma ad desteği.
pub type MicupError = Error;

impl From<tungstenite::Error> for Error {
    fn from(err: tungstenite::Error) -> Self {
        Error::Gateway(err.to_string())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Other(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Other(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_variants() {
        let errs = vec![
            Error::Gateway("bağlantı koptu".into()),
            Error::Api {
                status: 404,
                message: "Bulunamadı".into(),
            },
            Error::RateLimited("429 Too Many Requests".into()),
            Error::InvalidToken("Geçersiz token".into()),
            Error::ModelValidation("Eksik alan".into()),
            Error::Voice("Ses kanalına bağlanılamadı".into()),
            Error::Stage("Sahne başlatılamadı".into()),
            Error::Permission("MANAGE_ROOM_STAGE yetkisi eksik".into()),
            Error::Cdn("Medya linki süresi dolmuş".into()),
            Error::MacroValidation("custom_id 100 karakteri aşıyor".into()),
            Error::Other("Bilinmeyen hata".into()),
        ];

        for e in errs {
            assert!(!e.to_string().is_empty());
        }
    }

    #[test]
    fn test_from_conversions() {
        let json_err: serde_json::Error =
            serde_json::from_str::<String>("geçersiz json").unwrap_err();
        let err: Error = json_err.into();
        assert!(matches!(err, Error::Serde(_)));

        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "dosya yok");
        let err: Error = io_err.into();
        assert!(matches!(err, Error::Io(_)));

        let ws_err = tungstenite::Error::ConnectionClosed;
        let err: Error = ws_err.into();
        assert!(matches!(err, Error::Gateway(_)));

        let str_err: Error = "özel hata".into();
        assert!(matches!(str_err, Error::Other(_)));

        let string_err: Error = String::from("özel hata").into();
        assert!(matches!(string_err, Error::Other(_)));
    }
}
