use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use common::models::Node;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    models::node::{self},
    utils::{AppError, DbConn},
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
pub async fn get_nodes(DbConn(mut conn): DbConn) -> Result<impl IntoResponse, AppError> {
    Ok(Json(node::get_nodes(&mut conn).await?))
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
pub async fn get_node_by_id(
    DbConn(mut conn): DbConn,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(node::get_node_by_id(&mut conn, id).await?))
}

#[utoipa::path(
    post,
    path = "",
    responses((status = CREATED, body = Node),(status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::NODE_TAG
)]
pub async fn create_node(
    DbConn(mut conn): DbConn,
    Json(node): Json<Node>,
) -> Result<impl IntoResponse, AppError> {
    let node = node::create_node(&mut conn, node).await?;
    Ok((StatusCode::CREATED, Json(node)))
}

#[utoipa::path(
    put,
    path = "",
    responses((status = OK, body = Node),(status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::NODE_TAG
)]
pub async fn update_node(
    DbConn(mut conn): DbConn,
    Json(node): Json<Node>,
) -> Result<impl IntoResponse, AppError> {
    let node = node::update_node(&mut conn, node).await?;
    Ok(Json(node))
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
pub async fn delete_node(
    DbConn(mut conn): DbConn,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    node::delete_node(&mut conn, id).await?;
    Ok((StatusCode::OK, "Node deleted".to_string()))
}
