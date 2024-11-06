use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    models::server::{self, Server},
    utils::DbConn,
    AppState,
};

pub fn server_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(get_servers, create_server, update_server))
        .routes(routes!(get_server, delete_server))
        .routes(routes!(get_servers_by_node_id))
}

#[utoipa::path(
    get,
    path = "",
    responses((status = OK, body = [Server]), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::SERVER_TAG
)]
pub async fn get_servers(DbConn(conn): DbConn) -> impl IntoResponse {
    match server::get_servers(conn).await {
        Ok(servers) => Json(servers).into_response(),
        Err(_e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error".to_string(),
        )
            .into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/{id}",
    params(("id" = i32, Path, description = "server id")),
    responses((status = OK, body = Server), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::SERVER_TAG
)]
pub async fn get_server(
    Path(id): axum::extract::Path<i32>,
    DbConn(conn): DbConn,
) -> impl IntoResponse {
    match server::get_server_by_id(conn, id).await {
        Ok(server) => Json(server).into_response(),
        Err(_e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error".to_string(),
        )
            .into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/node/{node_id}",
    params(("node_id" = i32, Path, description = "node id")),
    responses((status = OK, body = [Server]), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::SERVER_TAG
)]
pub async fn get_servers_by_node_id(
    Path(node_id): Path<i32>,
    DbConn(conn): DbConn,
) -> impl IntoResponse {
    match server::get_servers_by_node_id(conn, node_id).await {
        Ok(servers) => Json(servers).into_response(),
        Err(_e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error".to_string(),
        )
            .into_response(),
    }
}

#[utoipa::path(
    post,
    path = "",
    responses((status = CREATED, body = Server), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::SERVER_TAG
)]
pub async fn create_server(DbConn(conn): DbConn, Json(server): Json<Server>) -> impl IntoResponse {
    match server::create_server(conn, server).await {
        Ok(server) => (StatusCode::CREATED, Json(server)).into_response(),
        Err(_e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error".to_string(),
        )
            .into_response(),
    }
}

#[utoipa::path(
    put,
    path = "",
    responses((status = OK, body = Server), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::SERVER_TAG
)]
pub async fn update_server(DbConn(conn): DbConn, Json(server): Json<Server>) -> impl IntoResponse {
    match server::update_server(conn, server).await {
        Ok(server) => Json(server).into_response(),
        Err(_e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error".to_string(),
        )
            .into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/{id}",
    params(("id" = i32, Path, description = "server id")),
    responses((status = OK, body = ()), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::SERVER_TAG
)]
pub async fn delete_server(Path(id): Path<i32>, DbConn(conn): DbConn) -> StatusCode {
    match server::delete_server(conn, id).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
