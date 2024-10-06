use askama::Template;
use axum::{
    extract::Query,
    response::{IntoResponse, Redirect, Response},
    routing::get,
    Extension,
};
use partial::{Head, Sidebar};
use saasbase::{
    axum::{askama::HtmlTemplate, extract, ConfigExt, DbExt},
    Router,
};

use crate::Result;

mod partial;

mod app;
mod machine;
mod project;
mod source;

mod auth;
mod settings;
mod setup;
mod signup;

pub fn router() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/machines", get(machine::list))
        .route("/machine/:id", get(machine::single))
        .route("/apps", get(app::list))
        .route("/app/:id", get(app::single))
        .merge(auth::router())
        .merge(settings::router())
        .merge(source::router())
        .merge(project::router())
        .merge(setup::router())
}

#[derive(Clone, Debug, Serialize, Deserialize, Template)]
#[template(path = "pages/summary.html")]
pub struct Summary {
    head: partial::Head,
    sidebar: partial::Sidebar,
    user: saasbase::User,
    config: saasbase::Config,
}

async fn home(
    user: extract::User,
    Extension(config): ConfigExt,
    Extension(db): DbExt,
) -> Result<impl IntoResponse> {
    Ok(HtmlTemplate(Summary {
        head: Head {
            title: "Summary".to_string(),
            ..Default::default()
        },
        sidebar: Sidebar::at("Summary", user.id, &db)?,
        user: user.0,
        config: (*config).clone(),
    }))
}

#[derive(Clone, Debug, Serialize, Deserialize, Template)]
#[template(path = "pages/404.html")]
pub struct NotFound {
    head: partial::Head,
    user: Option<saasbase::User>,
}

/// Custom fallback ensuring we only show 404 to logged-in users.
/// Everyone else gets the login page.
pub async fn login_or_not_found(user: Option<extract::User>) -> Response {
    if let Some(user) = user {
        HtmlTemplate(NotFound {
            head: partial::Head {
                title: "404".to_string(),
                ..Default::default()
            },
            user: Some(user.0),
        })
        .into_response()
    } else {
        Redirect::to("/login").into_response()
    }
}
