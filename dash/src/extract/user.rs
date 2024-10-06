use std::sync::Arc;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use axum_extra::extract::cookie::Key as CookieKey;

use crate::{data::UserData, error::Error};

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub base: saasbase::User,
    pub data: UserData,
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for User
where
    // Database: FromRef<S>,
    // Config: FromRef<S>,
    CookieKey: FromRef<S>,
{
    type Rejection = Error;

    async fn from_request_parts(mut parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let db = parts
            .extensions
            .get::<Arc<saasbase::Database>>()
            .expect("database extension unavailable")
            .clone();
        let user = saasbase::axum::extract::User::from_request_parts(parts, state).await?;
        let data = db.get::<UserData>(user.id)?;

        Ok(User { base: user.0, data })
    }
}
