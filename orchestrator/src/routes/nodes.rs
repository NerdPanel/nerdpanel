use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use common::models::Node;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    models::node::{self},
    utils::DbConn,
    AppState,
};

pub fn nodes_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(get_nodes, create_node, update_node))
        .routes(routes!(get_node_by_id, delete_node))
}

#[utoipa::path(
    get,
    path = "",
    responses((status = OK, body = [Node]),(status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::NODE_TAG
)]
pub async fn get_nodes(DbConn(mut conn): DbConn) -> impl IntoResponse {
    match node::get_nodes(&mut conn).await {
        Ok(nodes) => Json(nodes).into_response(),
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
    params(
        ("id" = i32, Path, description = "node id")
    ),
    responses((status = OK, body = Node),(status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::NODE_TAG
)]
pub async fn get_node_by_id(DbConn(mut conn): DbConn, Path(id): Path<i32>) -> impl IntoResponse {
    match node::get_node_by_id(&mut conn, id).await {
        Ok(node) => Json(node).into_response(),
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
    responses((status = CREATED, body = Node),(status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::NODE_TAG
)]
pub async fn create_node(DbConn(mut conn): DbConn, Json(node): Json<Node>) -> impl IntoResponse {
    match node::create_node(&mut conn, node).await {
        Ok(node) => (StatusCode::CREATED, Json(node)).into_response(),
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
    responses((status = OK, body = Node),(status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::NODE_TAG
)]
pub async fn update_node(DbConn(mut conn): DbConn, Json(node): Json<Node>) -> impl IntoResponse {
    match node::update_node(&mut conn, node).await {
        Ok(node) => Json(node).into_response(),
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
    params(
        ("id" = i32, Path, description = "node id")
    ),
    responses((status = OK, body = Node),(status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::NODE_TAG
)]
pub async fn delete_node(DbConn(mut conn): DbConn, Path(id): Path<i32>) -> impl IntoResponse {
    match node::delete_node(&mut conn, id).await {
        Ok(_) => (StatusCode::OK, "Node deleted".to_string()).into_response(),
        Err(_e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error".to_string(),
        )
            .into_response(),
    }
}
