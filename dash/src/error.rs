use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("core ruda error: {0}")]
    CoreRudaError(#[from] ruda::Error),
    #[error("saasbase: {0}")]
    SaasbaseError(#[from] saasbase::Error),
    #[error("io error: {0}")]
    StdIoError(#[from] std::io::Error),

    #[error("octocrab error: {0}")]
    OctocrabError(#[from] octocrab::Error),
    #[error("jsonwebtoken error: {0}")]
    JsonWebTokenError(#[from] jsonwebtoken::errors::Error),

    #[error("parse int error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("network error: {0}")]
    NetworkError(String),

    #[error("generic: {0}")]
    Other(String),
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::SaasbaseError(e) => e.into_response(),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {}", self),
            )
                .into_response(),
        }
    }
}

// // This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// // `Result<_, AppError>`. That way you don't need to do that manually.
// impl<E> From<E> for Error
// where
//     E: Into<anyhow::Error>,
// {
//     fn from(err: E) -> Self {
//         Self(err.into())
//     }
// }
