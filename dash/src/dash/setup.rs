use askama::Template;
use axum::{
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    Extension, Form,
};
use axum_extra::extract::PrivateCookieJar;
use saasbase::{
    auth::hash_password,
    axum::{askama::HtmlTemplate, extract, DbExt},
    Router, User,
};

use crate::{config::Config, data, util::create_user, Result};

use super::partial::{self, Head};

pub fn router() -> Router {
    Router::new()
        .route("/setup", get(setup))
        .route("/setup", post(setup_post))
}

#[derive(Clone, Debug, Serialize, Deserialize, Template)]
#[template(path = "pages/setup/setup.html")]
pub struct Setup {
    head: Head,
    // config: Config,
}

async fn setup(user: Option<extract::User>, Extension(db): DbExt) -> Result<Response> {
    if db.get_collection::<User>()?.len() != 0 || user.is_some() {
        return Ok(Redirect::to("/").into_response());
    }

    Ok(HtmlTemplate(Setup {
        head: Head {
            title: "Setup".to_string(),
            ..Default::default()
        },
        // config: (*config).clone(),
    })
    .into_response())
}

#[derive(Debug, Deserialize)]
struct NewAccountParams {
    email: String,
    password: String,
}

async fn setup_post(
    cookies: PrivateCookieJar,
    Extension(db): DbExt,
    Form(NewAccountParams { email, password }): Form<NewAccountParams>,
) -> Result<(PrivateCookieJar, Response)> {
    for n in 0..1000 {
        create_user(&db)?;
    }

    let mut user = create_user(&db)?;
    user.email = email;
    user.password_hash = Some(hash_password(&password)?);
    db.set(&user)?;

    let cookie = saasbase::auth::login::log_in_user_id(&user.id, &db)?;
    let updated_cookies = cookies.add(cookie);

    Ok((
        updated_cookies,
        HtmlTemplate(Setup1 {
            head: Head {
                title: "Setup".to_string(),
                ..Default::default()
            },
            // config: (*config).clone(),
        })
        .into_response(),
    ))
}

#[derive(Clone, Debug, Serialize, Deserialize, Template)]
#[template(path = "pages/setup/setup-1.html")]
pub struct Setup1 {
    head: Head,
    // config: Config,
}
