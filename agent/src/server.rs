use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::AppState;

pub fn server_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(status))
        .routes(routes!(signal))
        .routes(routes!(update))
        .routes(routes!(create))
        .routes(routes!(install))
        .routes(routes!(delete))
}

#[utoipa::path(
    get,
    path = "/{id}",
    params(("id" = i32, Path, description = "server id")),
    responses((status = OK, body = String), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = crate::routes::SERVER_TAG
)]
pub async fn status(Path(id): Path<i32>) -> impl IntoResponse {
    (StatusCode::OK, "OK".to_string()).into_response()
}

#[utoipa::path(
    post,
    path = "/{id}/signal",
    params(("id" = i32, Path, description = "server id")),
    responses((status = OK, body = String), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = crate::routes::SERVER_TAG
)]
pub async fn signal(Path(id): Path<i32>) -> impl IntoResponse {
    (StatusCode::OK, "OK".to_string()).into_response()
}

#[utoipa::path(
    post,
    path = "/{id}/create",
    params(("id" = i32, Path, description = "server id")),
    responses((status = OK, body = String), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = crate::routes::SERVER_TAG
)]
pub async fn create(Path(id): Path<i32>) -> impl IntoResponse {
    (StatusCode::OK, "OK".to_string()).into_response()
}

#[utoipa::path(
    delete,
    path = "/{id}/delete",
    params(("id" = i32, Path, description = "server id")),
    responses((status = OK, body = String), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = crate::routes::SERVER_TAG
)]
pub async fn delete(Path(id): Path<i32>) -> impl IntoResponse {
    (StatusCode::OK, "OK".to_string()).into_response()
}

#[utoipa::path(
    post,
    path = "/{id}/install",
    params(("id" = i32, Path, description = "server id")),
    responses((status = OK, body = String), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = crate::routes::SERVER_TAG
)]
pub async fn install(Path(id): Path<i32>) -> impl IntoResponse {
    (StatusCode::OK, "OK".to_string()).into_response()
}

#[utoipa::path(
    put,
    path = "/{id}/update",
    params(("id" = i32, Path, description = "server id")),
    responses((status = OK, body = String), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = crate::routes::SERVER_TAG
)]
pub async fn update(Path(id): Path<i32>) -> impl IntoResponse {
    (StatusCode::OK, "OK".to_string()).into_response()
}
