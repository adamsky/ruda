use askama::Template;
use axum::{
    response::{IntoResponse, Response},
    Extension,
};
use saasbase::axum::{askama::HtmlTemplate, ConfigExt, DbExt};

use crate::Result;
use crate::{data, extract};

use super::partial::{Head, Sidebar};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Machine {
    inner: data::Machine,

    specs: String,
    location: String,
}

impl From<data::Machine> for Machine {
    fn from(m: data::Machine) -> Self {
        Self {
            specs: m.status.clone(),
            location: m.owner.to_string(),
            inner: m,
        }
    }
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
}

pub async fn single(
    user: extract::User,
    Extension(config): ConfigExt,
    Extension(db): DbExt,
) -> Result<impl IntoResponse> {
    Ok(HtmlTemplate(Single {
        head: Head {
            title: format!("Machine X"),
            ..Default::default()
        },
        sidebar: Sidebar::at("Machines", user.base.id, &db)?,
        user: user.base,
        config: (*config).clone(),
    }))
}
