use axum::{response::IntoResponse, Extension, Json};

use ruda::api::{AuthDuration, AuthRequest, AuthResponse, AuthScope};
use saasbase::{
    auth::{validate_password, TokenMeta},
    axum::DbExt,
};

use crate::{Error, Result};

/// Authenticates user using email and password, sending back newly generated
/// token that can be used for authenticating subsequent requests.
pub async fn auth(
    Extension(db): DbExt,
    Json(auth_request): Json<AuthRequest>,
) -> Result<impl IntoResponse> {
    // find user by the provided email address
    let user = saasbase::util::find_user_by_email(&db, &auth_request.email)?;
    // confirm that the provided password is valid
    validate_password(
        auth_request.password.as_bytes(),
        // &user.password_hash.ok_or(Error::UserDoesNotHavePassword)?,
        &user
            .password_hash
            .ok_or(Error::Other("user does not have password".to_string()))?,
    )?;

    // generate a new token
    let token = TokenMeta::new(user.id);
    db.set(&token).expect("failed inserting access token");

    let response = Json(AuthResponse {
        token: token.id.to_string(),
    });

    Ok(response.into_response())
}
