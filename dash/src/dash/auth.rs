use askama::Template;
use axum::{
    extract::Query,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    Extension,
};
use saasbase::{
    axum::{askama::HtmlTemplate, extract, ConfigExt, DbExt},
    Router,
};

use crate::Result;

use super::partial::Head;

pub fn router() -> Router {
    Router::new()
        .route("/login", get(login))
        .route("/register", get(register).post(Redirect::to("/signup")))
}

#[derive(Clone, Debug, Serialize, Deserialize, Template)]
#[template(path = "pages/login.html")]
pub struct Login {
    head: Head,
    config: saasbase::Config,
    msg: Option<String>,
}

#[derive(Deserialize)]
pub struct Params {
    msg: Option<String>,
}

pub async fn login(
    Query(params): Query<Params>,
    Extension(config): ConfigExt,
    Extension(db): DbExt,
) -> Result<Response> {
    if db.get_collection::<saasbase::User>()?.len() == 0 {
        Ok(Redirect::to("/setup").into_response())
    } else {
        Ok(HtmlTemplate(Login {
            head: Head {
                title: "Sign In | ruda".to_string(),
                ..Default::default()
            },
            config: (*config).clone(),
            msg: params.msg,
        })
        .into_response())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Template)]
#[template(path = "pages/register.html")]
pub struct Register {
    head: Head,
    config: saasbase::Config,
}

pub async fn register(Extension(config): ConfigExt, Extension(db): DbExt) -> Response {
    if !config.registration.enabled {
        return saasbase::Error::new(saasbase::ErrorKind::RegistrationClosed("".to_string()))
            .into_response();
    }

    HtmlTemplate(Register {
        head: Head {
            title: "Sign In | ruda".to_string(),
            ..Default::default()
        },
        config: (*config).clone(),
    })
    .into_response()
}
