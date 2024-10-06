use axum::{response::IntoResponse, Extension, Json};

use ruda::api::{AuthDuration, AuthResponse, AuthScope};
use saasbase::{
    auth::{validate_password, TokenMeta},
    axum::DbExt,
};

use crate::Result;

/// Auth request to be sent to `api/auth` endpoint.
///
/// If credentials match a new access token will be generated and sent back
/// to the caller. The token will be generated using information provided in
/// the request.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
    /// Scope of information that shall be available when using the resulting
    /// token.
    pub scope: AuthScope,
    /// General duration for which the resulting token shall be valid.
    pub term: AuthDuration,
    /// Context in which the resulting token is being requested, e.g.
    /// application name or other additional information.
    pub context: String,
}

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
            .ok_or(anyhow::anyhow!("user does not have password"))?,
    )?;

    // generate a new token
    let token = TokenMeta::new(user.id);
    db.set(&token).expect("failed inserting access token");

    let response = Json(AuthResponse {
        token: token.id.to_string(),
    });

    Ok(response.into_response())
}
