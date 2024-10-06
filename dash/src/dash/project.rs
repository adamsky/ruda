use askama::Template;
use axum::{
    extract::{Path, Query},
    http::HeaderMap,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    Extension, Form,
};
use octocrab::Octocrab;
use saasbase::{
    axum::{askama::HtmlTemplate, extract, ConfigExt, DbExt},
    Router,
};
use uuid::Uuid;

use crate::data::{Project, UserData};
use crate::Result;

use super::partial::{Head, Sidebar};

pub fn router() -> Router {
    Router::new()
        .route("/projects", get(list))
        .route("/project/:id", get(single))
        .route("/project/new", get(new))
        .route("/project/current", post(current_post))
}

#[derive(Deserialize)]
struct CurrentParams {
    project_id: Uuid,
}

pub async fn current_post(
    user: extract::User,
    Extension(config): ConfigExt,
    Extension(db): DbExt,
    Form(CurrentParams { project_id }): Form<CurrentParams>,
) -> Result<impl IntoResponse> {
    println!("project id: {}", project_id);

    let mut app_user = db.get_or_create::<UserData>(user.id);
    app_user.current_project = project_id;
    db.set(&app_user)?;

    Ok([("HX-Refresh", "true")])
}

#[derive(Clone, Debug, Serialize, Deserialize, Template)]
#[template(path = "pages/project/new.html")]
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

#[derive(Clone, Debug, Serialize, Deserialize, Template)]
#[template(path = "pages/project/list.html")]
pub struct List {
    head: Head,
    sidebar: Sidebar,
    user: saasbase::User,
    config: saasbase::Config,

    projects: Vec<Project>,
}

pub async fn list(
    user: extract::User,
    Extension(config): ConfigExt,
    Extension(db): DbExt,
) -> Result<impl IntoResponse> {
    let projects = db.get_collection::<Project>()?;

    Ok(HtmlTemplate(List {
        head: Head {
            title: "Projects".to_string(),
            ..Default::default()
        },
        sidebar: Sidebar::at("Projects", user.id, &db)?,
        user: user.0,
        config: (*config).clone(),

        projects,
    }))
}

#[derive(Clone, Debug, Serialize, Deserialize, Template)]
#[template(path = "pages/project/single.html")]
pub struct Single {
    head: Head,
    sidebar: Sidebar,
    user: saasbase::User,
    config: saasbase::Config,

    project: Project,
}

pub async fn single(
    Path(project_id): Path<Uuid>,
    user: extract::User,
    Extension(db): DbExt,
    Extension(config): ConfigExt,
) -> Result<impl IntoResponse> {
    let project = db.get::<Project>(project_id)?;

    Ok(HtmlTemplate(Single {
        head: Head {
            title: format!("Project X"),
            ..Default::default()
        },
        sidebar: Sidebar::at("Projects", user.id, &db)?,
        user: user.0,
        config: (*config).clone(),
        project,
    }))
}
