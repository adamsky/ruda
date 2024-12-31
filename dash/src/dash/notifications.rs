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
        .route("/notifications", get(notifications))
        .route("/notifications", post(notifications_post))
}

#[derive(Clone, Debug, Serialize, Deserialize, Template)]
#[template(path = "pages/notifications.html")]
pub struct Notifications {
    head: Head,
    sidebar: Sidebar,
    config: saasbase::Config,

    user: saasbase::User,
    data: UserData,
}

pub async fn notifications(
    user: extract::User,
    Extension(db): DbExt,
    Extension(config): ConfigExt,
) -> Result<impl IntoResponse> {
    Ok(HtmlTemplate(Notifications {
        head: Head {
            title: format!("Notification Settings"),
            ..Default::default()
        },
        sidebar: Sidebar::at("Notifications", user.base.id, &db)?,
        config: (*config).clone(),
        user: user.base,
        data: user.data,
    }))
}

#[derive(Deserialize)]
struct NotificationsForm {}

pub async fn notifications_post(
    mut user: extract::User,
    Extension(config): ConfigExt,
    Extension(db): DbExt,
    Form(notifs): Form<NotificationsForm>,
) -> Result<impl IntoResponse> {
    Ok([("HX-Refresh", "true")])
}
