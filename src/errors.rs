use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum DegaussError {
    #[error("File read error")]
    IO(#[from] std::io::Error),

    #[error("Schema parsing error")]
    Schema(#[from] avro_rs::Error),

    #[error("Serializing/Deserializing error")]
    Serde(#[from] serde_json::Error),

    #[error("HTTP Client error")]
    HTTPClient(#[from] isahc::Error),

    #[error("HTTP Protocol error")]
    Http(#[from] isahc::http::Error),

    #[error("Status Code `{error_code}` Message: {message}")]
    SrHttp { error_code: i32, message: String },

    #[error("{0}")]
    Custom(String),
}
