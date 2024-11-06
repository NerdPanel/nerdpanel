use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use common::{
    agent_types::{ServerSignal, ServerStatus},
    models::Server,
};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    models::server::{self},
    utils::{get_node_from_server_id, DbConn},
    AppState,
};

pub fn server_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(get_servers, create_server, update_server))
        .routes(routes!(get_server, delete_server))
        .routes(routes!(get_servers_by_node_id))
        .routes(routes!(status))
        .routes(routes!(signal))
        .routes(routes!(install))
}

#[utoipa::path(
    get,
    path = "",
    responses((status = OK, body = [Server]), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::SERVER_TAG
)]
pub async fn get_servers(DbConn(mut conn): DbConn) -> impl IntoResponse {
    match server::get_servers(&mut conn).await {
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
    DbConn(mut conn): DbConn,
) -> impl IntoResponse {
    match server::get_server_by_id(&mut conn, id).await {
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
    DbConn(mut conn): DbConn,
) -> impl IntoResponse {
    match server::get_servers_by_node_id(&mut conn, node_id).await {
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
pub async fn create_server(
    DbConn(mut conn): DbConn,
    Json(server): Json<Server>,
) -> impl IntoResponse {
    match server::create_server(&mut conn, server).await {
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
pub async fn update_server(
    DbConn(mut conn): DbConn,
    Json(server): Json<Server>,
) -> impl IntoResponse {
    match server::update_server(&mut conn, server).await {
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
pub async fn delete_server(Path(id): Path<i32>, DbConn(mut conn): DbConn) -> StatusCode {
    match server::delete_server(&mut conn, id).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[utoipa::path(
    get,
    path = "/{id}/status",
    params(("id" = i32, Path, description = "server id")),
    responses((status = OK, body = ServerStatus), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::SERVER_TAG

)]
pub async fn status(Path(id): Path<i32>, DbConn(mut conn): DbConn) -> impl IntoResponse {
    match get_node_from_server_id(id, &mut conn).await {
        Ok(node) => {
            let status: ServerStatus = reqwest::get(&format!("http://{}/server/{}", node.fqdn, id))
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            (StatusCode::OK, Json(status)).into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error".to_string(),
        )
            .into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/{id}/signal",
    params(("id" = i32, Path, description = "server id")),
    responses((status = OK, body = ()), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::SERVER_TAG
)]
pub async fn signal(
    Path(id): Path<i32>,
    DbConn(mut conn): DbConn,
    Json(body): Json<ServerSignal>,
) -> impl IntoResponse {
    match get_node_from_server_id(id, &mut conn).await {
        Ok(node) => {
            reqwest::Client::new()
                .post(&format!("http://{}/server/{}/signal", node.fqdn, id))
                .json(&body)
                .send()
                .await
                .unwrap();
            StatusCode::OK.into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error".to_string(),
        )
            .into_response(),
    }
}

// TODO DO THIS PROPERLY

#[utoipa::path(
    post,
    path = "/{id}/install",
    params(("id" = i32, Path, description = "server id")),
    responses((status = OK, body = String), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::SERVER_TAG
)]
pub async fn install(
    Path(id): Path<i32>,
    DbConn(mut conn): DbConn,
    Json(body): Json<serde_json::Value>,
) -> impl IntoResponse {
    match get_node_from_server_id(id, &mut conn).await {
        Ok(node) => {
            let status = reqwest::Client::new()
                .post(&format!("http://{}/server/{}/install", node.fqdn, id))
                .json(&body)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            (StatusCode::OK, status).into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error".to_string(),
        )
            .into_response(),
    }
}
