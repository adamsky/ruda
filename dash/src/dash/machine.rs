use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{AppendHeaders, IntoResponse, Redirect, Response},
    routing::{get, post},
    Extension, Form,
};
use saasbase::{
    axum::{askama::HtmlTemplate, ConfigExt, DbExt},
    Router,
};
use uuid::Uuid;

use crate::Result;
use crate::{data, extract};

use super::partial::{Head, Sidebar};

pub fn router() -> Router {
    Router::new()
        .route("/machines", get(list))
        .route("/machine/:id", get(single))
        .route("/machine/new", post(new))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Machine {
    inner: data::Machine,

    status: String,
    location: String,
}

impl From<data::Machine> for Machine {
    fn from(m: data::Machine) -> Self {
        Self {
            status: m.status.to_string(),
            location: format!("{}", m.address),
            inner: m,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewForm {
    pub name: String,
}

pub async fn new(
    user: extract::User,
    Extension(config): ConfigExt,
    Extension(db): DbExt,
    Form(new): Form<NewForm>,
) -> Result<impl IntoResponse> {
    let mut machine = data::Machine::default();

    machine.owner = user.base.id;
    machine.project = user.data.current_project;
    machine.name = new.name;

    db.set(&machine)?;

    let redir = format!("/machine/{}", machine.id);

    Ok((StatusCode::OK, AppendHeaders([("HX-Redirect", redir)])))
}

#[derive(Clone, Debug, Serialize, Deserialize, Template)]
#[template(path = "pages/machine/list.html")]
pub struct List {
    head: Head,
    sidebar: Sidebar,
    user: saasbase::User,
    config: saasbase::Config,

    machines: Vec<Machine>,
}

pub async fn list(
    user: extract::User,
    Extension(config): ConfigExt,
    Extension(db): DbExt,
) -> Result<impl IntoResponse> {
    // let machines = db.get_collection::<data::Machine>()?;

    Ok(HtmlTemplate(List {
        head: Head {
            title: "Machines".to_string(),
            ..Default::default()
        },
        sidebar: Sidebar::at("Machines", user.base.id, &db)?,
        user: user.base,
        config: (*config).clone(),

        machines: db
            .get_collection::<data::Machine>()?
            .into_iter()
            .filter(|m| m.project == user.data.current_project)
            .map(|m| m.into())
            .collect(),
    }))
}

#[derive(Clone, Debug, Serialize, Deserialize, Template)]
#[template(path = "pages/machine/single.html")]
pub struct Single {
    head: Head,
    sidebar: Sidebar,
    user: saasbase::User,
    config: saasbase::Config,

    machine: Machine,
}

pub async fn single(
    Path(id): Path<Uuid>,
    user: extract::User,
    Extension(config): ConfigExt,
    Extension(db): DbExt,
) -> Result<impl IntoResponse> {
    let machine = db.get::<data::Machine>(id)?;

    Ok(HtmlTemplate(Single {
        head: Head {
            title: format!("Machine X"),
            ..Default::default()
        },
        sidebar: Sidebar::at("Machines", user.base.id, &db)?,
        user: user.base,
        config: (*config).clone(),

        machine: machine.into(),
    }))
}
