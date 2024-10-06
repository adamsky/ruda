mod auth;
mod deploy;

use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
};

use saasbase::axum::Router;

/// Http API router. Some standard routes are reused from `saasbase` library
/// (e.g. `/auth`, `/me`, `/credits`). Others are redefined or fully
/// application-specific (e.g. `/spawn`).
pub fn router() -> Router {
    Router::new().route("/auth", post(auth::auth)).route(
        "/deploy",
        post(deploy::deploy).layer(DefaultBodyLimit::disable()),
        // .layer(DefaultBodyLimit::max(56000)),
    )
}
