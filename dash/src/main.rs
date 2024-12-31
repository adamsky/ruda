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
use tokio_util::sync::CancellationToken;
use tower_serve_static::ServeDir;
use uuid::Uuid;

use ruda::runner;
use saasbase::{axum::Router, Database};

use crate::error::{Error, Result};

// embed files from `assets` directory into the binary
static ASSETS: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets");

#[tokio::main]
async fn main() -> Result<()> {
    let cancel = CancellationToken::new();

    let config: Config = saasbase::config::load_from("ruda.toml")?;
    saasbase::tracing::init(&config.base)?;

    let octocrab = if let Some(github) = &config.github {
        let key = jsonwebtoken::EncodingKey::from_rsa_pem(github.app_key.as_bytes())?;
        Some(
            Octocrab::builder()
                .app(github.app_id.parse::<u64>()?.into(), key)
                .build()?,
        )
    } else {
        None
    };

    let db = Database::new()?;

    if config.base.dev.mock {
        mock::generate(&config, &db)?;
    }

    // generate admin user
    let admin = saasbase::User {
        handle: "admin".to_string(),
        password_hash: Some(saasbase::auth::hash_password("admin")?),
        is_admin: true,
        is_verified: true,
        avatar: saasbase::user::new_avatar_image(&db)?,
        ..Default::default()
    };
    db.set(&admin)?;
    let project = data::Project {
        owner: admin.id,
        ..Default::default()
    };
    db.set(&project)?;
    let admin_data = data::UserData {
        id: admin.id,
        current_project: project.id,
        ..Default::default()
    };
    db.set(&admin_data)?;

    println!("before runner");

    // listen to incoming runner connections
    let realtime_handle = realtime::spawn(db.clone(), cancel.clone())?;

    // spawn a local runner
    let runner = config.local_runner.then_some({
        let machine = data::Machine {
            owner: admin.id,
            project: project.id,
            kind: data::machine::Kind::Managed,
            address: "localhost".to_string(),
            ..Default::default()
        };
        db.set(&machine)?;
        runner::spawn(
            runner::Config {
                platform_address: "127.0.0.1:10001".to_string(),
                code: machine.secret,
            },
            cancel,
        )?
    });

    println!("after runner");

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

    let mut router = router
        .nest_service("/assets", ServeDir::new(&ASSETS))
        .layer(Extension(realtime_handle));

    if let Some(octocrab) = octocrab {
        router = router.layer(Extension(octocrab));
    }

    saasbase::axum::start_with(db, router, config.base).await?;
    Ok(())
}
