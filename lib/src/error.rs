pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error: {0}")]
    StdIoError(#[from] std::io::Error),

    #[error("tungstenite error: {0}")]
    TungsteniteError(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("pot error: {0}")]
    PotError(#[from] pot::Error),
    #[error("config error: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("other error: {0}")]
    Other(String),
}
