use thiserror::Error;

#[derive(Error, Debug)]
pub enum MicupError {
    #[error("HTTP Hatası: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Serde JSON Hatası: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Micup API Hatası (HTTP {status}): {message}")]
    Api { status: u16, message: String },

    #[error("WebSocket Gateway Hatası: {0}")]
    Gateway(String),

    #[error("Diğer Hata: {0}")]
    Other(String),
}
