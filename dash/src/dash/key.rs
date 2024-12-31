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
        .route("/keys", get(list))
        .route("/key/:id", get(single))
        .route("/key/new", get(new))
}

#[derive(Clone, Debug, Serialize, Deserialize, Template)]
#[template(path = "pages/key/new.html")]
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
pub struct Key {
    pub inner: data::Key,
}

impl From<data::Key> for Key {
    fn from(s: data::Key) -> Self {
        Self { inner: s }
    }
}

#[derive(Clone, Debug, Template)]
#[template(path = "pages/key/list.html")]
pub struct List {
    head: Head,
    sidebar: Sidebar,
    user: saasbase::User,
    config: saasbase::Config,

    keys: Vec<Key>,
}

pub async fn list(
    user: extract::User,
    Extension(config): ConfigExt,
    Extension(db): DbExt,
) -> Result<impl IntoResponse> {
    Ok(HtmlTemplate(List {
        head: Head {
            title: "Keys".to_string(),
            ..Default::default()
        },
        sidebar: Sidebar::at("Keys", user.base.id, &db)?,
        user: user.base,
        config: (*config).clone(),

        keys: db
            .get_collection::<data::Key>()?
            .into_iter()
            .filter(|s| s.project == user.data.current_project)
            .map(|s| s.into())
            .collect(),
    }))
}

#[derive(Clone, Debug, Template)]
#[template(path = "pages/key/single.html")]
pub struct Single {
    head: Head,
    sidebar: Sidebar,
    user: saasbase::User,
    config: saasbase::Config,

    key: Key,
}

pub async fn single(
    Path(id): Path<Uuid>,
    user: extract::User,
    Extension(db): DbExt,
    Extension(config): ConfigExt,
) -> Result<impl IntoResponse> {
    let mut key = Key::default();

    Ok(HtmlTemplate(Single {
        head: Head {
            title: format!("{} | Key", key.inner.name),
            ..Default::default()
        },
        sidebar: Sidebar::at("Keys", user.base.id, &db)?,
        user: user.base,
        config: (*config).clone(),
        key,
    }))
}
