mod env;

use askama::Template;
use axum::{
    body::HttpBody,
    extract::Path,
    http::StatusCode,
    response::{AppendHeaders, IntoResponse, Redirect, Response},
    routing::{get, post},
    Extension, Form,
};
use futures_util::FutureExt;
use http_body_util::BodyExt;
use saasbase::{
    axum::{askama::HtmlTemplate, ConfigExt, DbExt},
    db::decode,
    Database, Router,
};
use url::Url;
use uuid::Uuid;

use crate::{
    data::{self, UserData},
    extract, realtime, Error, Result,
};

use super::partial::{Head, Sidebar};

pub fn router() -> Router {
    Router::new()
        .route("/apps", get(list))
        .route("/app/new", post(new))
        .route("/app/:id", get(single))
        .route("/app/:id/update", post(single_update))
        .merge(env::router())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Application {
    #[serde(flatten)]
    pub inner: data::App,

    pub owner: String,
    pub status: String,
}

impl Application {
    fn resolve(app: data::App, db: &Database) -> Result<Self> {
        Ok(Self {
            owner: db.get::<saasbase::User>(app.owner)?.name,
            status: match &app.machine {
                Some(machine) => {
                    // TODO: contact machine to get current status
                    "operational".to_string()
                }
                None => "unassigned".to_string(),
            },

            inner: app,
        })
    }
}

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
    let now = std::time::Instant::now();

    let apps = db
        .get_collection::<data::App>()?
        .into_iter()
        .filter(|app| app.project == user.data.current_project)
        .filter_map(|app| Application::resolve(app, &db).ok())
        .collect();

    println!("apps listed in {}ms", now.elapsed().as_millis());

    Ok(HtmlTemplate(List {
        head: Head {
            title: "Applications".to_string(),
            ..Default::default()
        },
        sidebar: Sidebar::at("Applications", user.base.id, &db)?,
        user: user.base,
        config: (*config).clone(),
        apps,
    }))
}

#[derive(Clone, Debug, Serialize, Deserialize, Template)]
#[template(path = "pages/app/single.html")]
pub struct Single {
    head: Head,
    sidebar: Sidebar,
    user: saasbase::User,
    config: saasbase::Config,

    app: Application,

    machines: Vec<(Uuid, String)>,
}

pub async fn single(
    Path(id): Path<Uuid>,
    user: extract::User,
    Extension(config): ConfigExt,
    Extension(db): DbExt,
) -> Result<Response> {
    let app = db.get::<data::App>(id)?;

    // Make sure the application is part of the currently viewed project
    if user.data.current_project != app.project {
        return Ok(Redirect::to("/apps").into_response());
    }

    let machines = db
        .get_collection::<data::Machine>()?
        .into_iter()
        .filter(|m| m.owner == user.base.id)
        .map(|m| (m.id, m.name))
        .collect::<Vec<_>>();

    Ok(HtmlTemplate(Single {
        head: Head {
            title: format!("{}", app.name),
            ..Default::default()
        },
        sidebar: Sidebar::at("Applications", user.base.id, &db)?,
        user: user.base,
        config: (*config).clone(),
        app: Application::resolve(app, &db)?,
        machines,
    })
    .into_response())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateForm {
    machine: Option<String>,
    source_url: Option<String>,
}

pub async fn single_update(
    user: extract::User,
    Path(id): Path<Uuid>,
    Extension(config): ConfigExt,
    Extension(db): DbExt,
    Form(update): Form<UpdateForm>,
) -> Result<impl IntoResponse> {
    let mut app = db.get::<data::App>(id)?;

    if let Some(machine_id) = update.machine {
        if machine_id.as_str() == "None" {
            app.machine = None;
        } else {
            app.machine = Some(machine_id.parse().unwrap());
        }
    }
    if let Some(source_url) = update.source_url {
        app.source_url = source_url;
    }

    db.set(&app)?;

    Ok(StatusCode::OK)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewForm {
    pub name: String,

    // Source information
    pub account: Option<String>,
    pub repo: Option<String>,
    pub url: Option<String>,

    pub domain: String,
}

pub async fn new(
    user: extract::User,
    Extension(config): ConfigExt,
    Extension(db): DbExt,
    Form(new): Form<NewForm>,
) -> Result<impl IntoResponse> {
    let mut app = data::App::new(&db)?;

    app.owner = user.base.id;
    app.project = user.data.current_project;
    app.domain = new.domain;

    if new.name.is_empty() {
        return Err(Error::Other("application name cannot be empty".to_string()).into());
    }
    app.name = new.name;

    if let Some(account) = new.account {
        app.source_account = account;
        app.source_repo = new
            .repo
            .ok_or(Error::Other("source repo required".to_string()))?;
    }

    if let Some(source_url) = new.url {
        app.source_url = source_url;
    } else {
        // TODO: construct the source url
    }

    db.set(&app)?;

    let redir = format!("/app/{}", app.id);

    Ok((StatusCode::OK, AppendHeaders([("HX-Redirect", redir)])))
}
