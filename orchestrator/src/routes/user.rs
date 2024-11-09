use axum::{extract::Path, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    models::user::{self, CreateUser, UpdateUser, User},
    utils::{AppError, DbConn},
    AppState,
};

pub fn user_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(get_users, create_user, update_user))
        .routes(routes!(get_user, delete_user))
}

#[utoipa::path(
    get,
    path = "",
    responses((status = OK, body = [User]), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::USER_TAG
)]
pub async fn get_users(DbConn(mut conn): DbConn) -> Result<Json<Vec<User>>, AppError> {
    let users = user::get_users(&mut conn).await?;

    tracing::info!("Fetched {} users", users.len());
    Ok(Json(users))
}

#[utoipa::path(
    get,
    path = "/{id}",
    params(("id" = i32, Path, description = "user id")),
    responses((status = OK, body = User), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::USER_TAG
)]
pub async fn get_user(
    Path(id): axum::extract::Path<i32>,
    DbConn(mut conn): DbConn,
) -> Result<Json<User>, AppError> {
    let user = user::get_user_by_id(&mut conn, id).await?;
    match user {
        Some(user) => Ok(Json(user)),
        None => Err(AppError::NotFound),
    }
}

#[utoipa::path(
    post,
    path = "",
    responses((status = CREATED, body = User), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::USER_TAG
)]
pub async fn create_user(
    DbConn(mut conn): DbConn,
    Json(user): Json<CreateUser>,
) -> Result<Json<User>, AppError> {
    let user = user::create_user(&mut conn, user).await?;
    Ok(Json(user))
}

#[utoipa::path(
    put,
    path = "",
    responses((status = OK, body = User), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::USER_TAG
)]
pub async fn update_user(
    DbConn(mut conn): DbConn,
    Json(user): Json<UpdateUser>,
) -> Result<Json<User>, AppError> {
    let user = user::update_user(&mut conn, user).await?;
    Ok(Json(user))
}

#[utoipa::path(
    delete,
    path = "/{id}",
    params(("id" = i32, Path, description = "user id")),
    responses((status = OK), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::USER_TAG
)]
pub async fn delete_user(Path(id): Path<i32>, DbConn(mut conn): DbConn) -> Result<(), AppError> {
    user::delete_user(&mut conn, id).await?;
    Ok(())
}
