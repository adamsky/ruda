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
        .route("/integrations", get(integrations))
        // .route("/integrations/github", get(github_get).post(github_post))
        .route("/integrations/hrobot", post(hrobot_post))
        .route("/integrations/cloudflare", post(cloudflare_post))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HrobotForm {
    pub key: String,
}

pub async fn hrobot_post(
    user: extract::User,
    Extension(config): ConfigExt,
    Extension(db): DbExt,
    Form(hetz): Form<HrobotForm>,
) -> Result<impl IntoResponse> {
    // TODO

    Ok((StatusCode::OK, AppendHeaders([("HX-Refresh", "true")])))
}

#[derive(Clone, Debug, Serialize, Deserialize, Template)]
#[template(path = "pages/integrations.html")]
pub struct Integrations {
    head: Head,
    sidebar: Sidebar,
    user: saasbase::User,
    config: saasbase::Config,
    github: bool,
}

pub async fn integrations(
    user: extract::User,
    Extension(config): ConfigExt,
    Extension(db): DbExt,
) -> Result<impl IntoResponse> {
    Ok(HtmlTemplate(Integrations {
        head: Head {
            title: "Integrations".to_string(),
            ..Default::default()
        },
        sidebar: Sidebar::at("Integrations", user.base.id, &db)?,

        github: user.base.linked_accounts.iter().any(|a| match a {
            saasbase::oauth::Link::Github { .. } => true,
            _ => false,
        }),

        user: user.base,
        config: (*config).clone(),
    }))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CloudflareForm {
    pub key: String,
}

pub async fn cloudflare_post(
    user: extract::User,
    Extension(config): ConfigExt,
    Extension(db): DbExt,
    Form(cf): Form<CloudflareForm>,
) -> Result<impl IntoResponse> {
    // TODO

    Ok((StatusCode::OK, AppendHeaders([("HX-Refresh", "true")])))
}
