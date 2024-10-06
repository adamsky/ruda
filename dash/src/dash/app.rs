use askama::Template;
use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Extension,
};
use saasbase::axum::{askama::HtmlTemplate, ConfigExt, DbExt};
use uuid::Uuid;

use crate::Result;
use crate::{
    data::{self, Application},
    extract,
};

use super::partial::{Head, Sidebar};

#[derive(Clone, Debug, Serialize, Deserialize, Template)]
#[template(path = "pages/app/list.html")]
pub struct List {
    head: Head,
    sidebar: Sidebar,
    user: saasbase::User,
    config: saasbase::Config,

    apps: Vec<Application>,
}

pub async fn list(
    user: extract::User,
    Extension(config): ConfigExt,
    Extension(db): DbExt,
) -> Result<impl IntoResponse> {
    Ok(HtmlTemplate(List {
        head: Head {
            title: "Applications".to_string(),
            ..Default::default()
        },
        sidebar: Sidebar::at("Applications", user.base.id, &db)?,
        user: user.base,
        config: (*config).clone(),

        apps: db
            .get_collection::<Application>()?
            .into_iter()
            .filter(|app| app.project == user.data.current_project)
            .collect(),
    }))
}

#[derive(Clone, Debug, Serialize, Deserialize, Template)]
#[template(path = "pages/app/single.html")]
pub struct Single {
    head: Head,
    sidebar: Sidebar,
    user: saasbase::User,
    config: saasbase::Config,

    app: data::Application,
}

pub async fn single(
    Path(id): Path<Uuid>,
    user: extract::User,
    Extension(config): ConfigExt,
    Extension(db): DbExt,
) -> Result<impl IntoResponse> {
    let app = db.get::<Application>(id)?;

    Ok(HtmlTemplate(Single {
        head: Head {
            title: format!("App X"),
            ..Default::default()
        },
        sidebar: Sidebar::at("Applications", user.base.id, &db)?,
        user: user.base,
        config: (*config).clone(),
        app,
    }))
}
