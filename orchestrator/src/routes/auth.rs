use axum::Json;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    auth::{AuthSession, Creds},
    utils::AppError,
    AppState,
};

pub fn auth_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(login, logout))
}

#[utoipa::path(
    post,
    path = "/login",
    responses((status = OK), (status = INTERNAL_SERVER_ERROR, body = String), (status = UNAUTHORIZED, body = String)),
    tag = super::AUTH_TAG
)]
pub async fn login(
    mut auth_session: AuthSession,
    Json(creds): Json<Creds>,
) -> Result<(), AppError> {
    let user = match auth_session.authenticate(creds).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(AppError::Unauthorized),
        Err(e) => match e {
            axum_login::Error::Session(_) => return Err(AppError::Unauthorized),
            axum_login::Error::Backend(e) => return Err(e),
        },
    };

    if auth_session.login(&user).await.is_err() {
        tracing::error!("Failed to login user: {:?}", user);
        return Err(AppError::InternalServerError);
    }

    Ok(())
}

#[utoipa::path(
    get,
    path = "/logout",
    responses((status = OK), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::AUTH_TAG
)]
pub async fn logout(mut auth_session: AuthSession) -> Result<(), AppError> {
    if auth_session.logout().await.is_err() {
        return Err(AppError::InternalServerError);
    }

    Ok(())
}
