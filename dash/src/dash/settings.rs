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

pub fn router() -> Router {
    Router::new()
        .route("/settings", get(settings))
        .route("/settings", post(settings_post))
}

#[derive(Clone, Debug, Serialize, Deserialize, Template)]
#[template(path = "pages/settings.html")]
pub struct Settings {
    user: saasbase::User,
    data: UserData,
}

pub async fn settings(
    user: extract::User,
    Extension(db): DbExt,
    Extension(config): ConfigExt,
) -> Result<impl IntoResponse> {
    Ok(HtmlTemplate(Settings {
        user: user.base,
        data: user.data,
    }))
}

#[derive(Deserialize)]
struct SettingsParams {
    dark_mode: bool,
}

pub async fn settings_post(
    mut user: extract::User,
    Extension(config): ConfigExt,
    Extension(db): DbExt,
    Form(settings): Form<SettingsParams>,
) -> Result<impl IntoResponse> {
    user.base.settings.dark_mode = settings.dark_mode;
    db.set(&user.base)?;

    Ok([("HX-Refresh", "true")])
}
