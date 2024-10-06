use std::env;
use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;

use axum::body::Body;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::{response::IntoResponse, Extension, Json};
// use http::StatusCode;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

use saasbase::api::DeployQuery;
use saasbase::axum::extract;
use saasbase::axum::DbExt;

use crate::{Error, Result};

pub async fn deploy(
    user: extract::User,
    Query(query): Query<DeployQuery>,
    // Extension(db): DbExt,
    body: Body,
) -> Result<impl IntoResponse> {
    // let req: DeployRequest =
    // bincode::deserialize(&axum::body::to_bytes(body, usize::MAX).await.unwrap()).unwrap();
    let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();

    println!("got deploy request");

    let path = format!("test/{}", query.name);
    let mut bin = File::create(&path).await.unwrap();
    bin.write_all(&bytes).await.unwrap();

    let mut perms = bin.metadata().await.unwrap().permissions();
    perms.set_mode(755);
    bin.set_permissions(perms).await.unwrap();
    bin.sync_all().await.unwrap();
    drop(bin);

    let path = format!("{}/{}", env::current_dir().unwrap().to_string_lossy(), path);
    let _ = Command::new(&path).current_dir("./test").spawn().unwrap();

    Ok(StatusCode::OK)
}
