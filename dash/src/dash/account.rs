use askama::Template;
use axum::{
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Form,
};
use saasbase::{
    axum::{askama::HtmlTemplate, ConfigExt, DbExt},
    Router,
};
use uuid::Uuid;

use crate::Result;
use crate::{data::UserData, extract};

use super::partial::{Head, Sidebar};

pub fn router() -> Router {
    Router::new()
        .route("/account", get(account))
        .route("/account", post(account_post))
}

#[derive(Clone, Debug, Serialize, Deserialize, Template)]
#[template(path = "pages/account.html")]
pub struct Account {
    head: Head,
    sidebar: Sidebar,
    config: saasbase::Config,

    user: saasbase::User,
    data: UserData,
}

pub async fn account(
    user: extract::User,
    Extension(db): DbExt,
    Extension(config): ConfigExt,
) -> Result<impl IntoResponse> {
    Ok(HtmlTemplate(Account {
        head: Head {
            title: format!("Account"),
            ..Default::default()
        },
        sidebar: Sidebar::at("Account", user.base.id, &db)?,
        config: (*config).clone(),
        user: user.base,
        data: user.data,
    }))
}

#[derive(Deserialize)]
struct AccountParams {
    dark_mode: Option<bool>,
    email: Option<String>,
}

pub async fn account_post(
    mut user: extract::User,
    Extension(config): ConfigExt,
    Extension(db): DbExt,
    Form(account): Form<AccountParams>,
) -> Result<Response> {
    if let Some(email) = account.email {
        user.base.email = email;
    }
    if let Some(dark_mode) = account.dark_mode {
        user.base.settings.dark_mode = dark_mode;
    }

    db.set(&user.base)?;

    if account.dark_mode.is_some() {
        return Ok([("HX-Refresh", "true")].into_response());
    }

    Ok(().into_response())
}
