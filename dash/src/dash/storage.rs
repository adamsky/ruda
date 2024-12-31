use askama::Template;
use axum::{
    extract::{Path, Query},
    response::{IntoResponse, Redirect, Response},
    routing::get,
    Extension,
};
use saasbase::{
    axum::{askama::HtmlTemplate, ConfigExt, DbExt},
    Router,
};
use uuid::Uuid;

use crate::{data, extract, Result};

use super::partial::{Head, Sidebar};

pub fn router() -> Router {
    Router::new()
        .route("/storages", get(list))
        .route("/storage/:id", get(single))
        .route("/storage/new", get(new))
}

#[derive(Clone, Debug, Serialize, Deserialize, Template)]
#[template(path = "pages/storage/new.html")]
pub struct New {
    head: Head,
    sidebar: Sidebar,
    user: saasbase::User,
    config: saasbase::Config,
}

pub async fn new(
    user: extract::User,
    Extension(config): ConfigExt,
    Extension(db): DbExt,
) -> Result<impl IntoResponse> {
    Ok("")
}

#[derive(Clone, Debug, Default)]
pub struct Storage {
    pub inner: data::Storage,
}

impl From<data::Storage> for Storage {
    fn from(s: data::Storage) -> Self {
        Self { inner: s }
    }
}

#[derive(Clone, Debug, Template)]
#[template(path = "pages/storage/list.html")]
pub struct List {
    head: Head,
    sidebar: Sidebar,
    user: saasbase::User,
    config: saasbase::Config,

    storages: Vec<Storage>,
}

pub async fn list(
    user: extract::User,
    Extension(config): ConfigExt,
    Extension(db): DbExt,
) -> Result<impl IntoResponse> {
    Ok(HtmlTemplate(List {
        head: Head {
            title: "Storages".to_string(),
            ..Default::default()
        },
        sidebar: Sidebar::at("Storages", user.base.id, &db)?,
        user: user.base,
        config: (*config).clone(),

        storages: db
            .get_collection::<data::Storage>()?
            .into_iter()
            .filter(|s| s.project == user.data.current_project)
            .map(|s| s.into())
            .collect(),
    }))
}

#[derive(Clone, Debug, Template)]
#[template(path = "pages/storage/single.html")]
pub struct Single {
    head: Head,
    sidebar: Sidebar,
    user: saasbase::User,
    config: saasbase::Config,

    storage: Storage,
}

pub async fn single(
    Path(id): Path<Uuid>,
    user: extract::User,
    Extension(db): DbExt,
    Extension(config): ConfigExt,
) -> Result<impl IntoResponse> {
    let mut storage = Storage::default();

    Ok(HtmlTemplate(Single {
        head: Head {
            title: format!("{} | Storage", storage.inner.name),
            ..Default::default()
        },
        sidebar: Sidebar::at("Storages", user.base.id, &db)?,
        user: user.base,
        config: (*config).clone(),
        storage,
    }))
}
