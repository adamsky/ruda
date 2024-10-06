#![allow(warnings)]

#[macro_use]
extern crate serde_derive;

mod config;
mod data;
mod error;
mod extract;
mod mock;
mod util;

mod api;
mod dash;
mod realtime;

use std::{net::SocketAddr, sync::Arc};

use axum::{
    http::StatusCode,
    middleware,
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
    Extension,
};
use config::Config;
use include_dir::{include_dir, Dir};
use octocrab::Octocrab;
use serde::{Deserialize, Serialize};
use tower_serve_static::ServeDir;

use saasbase::{axum::Router, Database};

use crate::error::{Error, Result};

// embed files from `assets` directory into the binary
static ASSETS: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets");

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("sled=warn")
        .finish();

    let config: Config = saasbase::config::load()?;

    let key = jsonwebtoken::EncodingKey::from_rsa_pem(config.github.app_key.as_bytes())?;
    let octocrab = Octocrab::builder()
        .app(config.github.app_id.parse::<u64>()?.into(), key)
        .build()?;

    let db = Database::new()?;
    mock::generate(&config, &db)?;

    // listen to incoming runner connections
    realtime::spawn();

    // Create the custom router for the platform app.
    let mut router = Router::new()
        .merge(dash::router())
        .nest("/api", api::router())
        .route(
            "/favicon.ico",
            get(|| async move { Redirect::to("/assets/images/favicon.ico") }),
        )
        .fallback(dash::login_or_not_found);

    // Merge the router with the default saasbase router.
    let router = saasbase::axum::router(router, &config.base);

    let router = router
        .nest_service("/assets", ServeDir::new(&ASSETS))
        .layer(Extension(octocrab));

    saasbase::axum::start_with(db, router, config.base).await?;
    Ok(())
}
