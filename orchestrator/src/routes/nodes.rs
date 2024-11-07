use axum::{extract::Path, http::StatusCode, Json};
use common::orch_types::{Node, NodePort};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    models::{node::{self, CreateNode, NodeModel}, node_port::{self, CreateNodePort}},
    utils::{node_model_to_node, AppError, DbConn},
    AppState,
};


pub fn nodes_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(get_nodes, create_node, update_node))
        .routes(routes!(get_node_by_id, delete_node))
        .routes(routes!(get_node_port, create_node_port, delete_node_port))
}

#[utoipa::path(
    get,
    path = "",
    responses((status = OK, body = [Node]),(status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::NODE_TAG
)]
pub async fn get_nodes(DbConn(mut conn): DbConn) -> Result<Json<Vec<Node>>, AppError> {
    let nodes = node::get_nodes(&mut conn).await?;
    let nodes = {
        let mut new = vec![];
        for node in nodes {
            new.push(node_model_to_node(node, &mut conn).await?);
        }
        new
    };
    Ok(Json(nodes))
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
) -> Result<Json<Node>, AppError> {
    let node = node::get_node_by_id(&mut conn, id).await?;
    let node = node_model_to_node(node, &mut conn).await?;
    Ok(Json(node))
}

#[utoipa::path(
    post,
    path = "",
    responses((status = CREATED, body = Node),(status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::NODE_TAG
)]
pub async fn create_node(
    DbConn(mut conn): DbConn,
    Json(node): Json<CreateNode>,
) -> Result<(StatusCode, Json<Node>), AppError> {
    let node = node::create_node(&mut conn, node).await?;
    let node = node_model_to_node(node, &mut conn).await?;
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
    Json(node): Json<NodeModel>,
) -> Result<Json<Node>, AppError> {
    let node = node::update_node(&mut conn, node).await?;
    let node = node_model_to_node(node, &mut conn).await?;
    Ok(Json(node))
}

#[utoipa::path(
    delete,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "node id")
    ),
    responses((status = OK),(status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::NODE_TAG
)]
pub async fn delete_node(
    DbConn(mut conn): DbConn,
    Path(id): Path<i32>,
) -> Result<(), AppError> {
    node::delete_node(&mut conn, id).await?;
    Ok(())
}

#[utoipa::path(
    get,
    path = "/{id}/port",
    params(("id" = i32, Path, description = "node port id")),
    responses((status = OK, body = [NodePort]), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::NODE_TAG
)]
pub async fn get_node_port(
    Path(id): axum::extract::Path<i32>,
    DbConn(mut conn): DbConn,
) -> Result<Json<Vec<NodePort>>, AppError> {
    let node_ports = node_port::get_node_ports_by_node_id(&mut conn, id).await?;
    let node_ports: Vec<NodePort> = node_ports.into_iter().map(Into::into).collect();
    Ok(Json(node_ports))
}

#[utoipa::path(
    post,
    path = "/{id}/port",
    params(("id" = i32, Path, description = "node id")),
    responses((status = CREATED, body = NodePort), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::NODE_TAG
)]
pub async fn create_node_port(
    Path(node_id): Path<i32>,
    DbConn(mut conn): DbConn,
    Json(node_port): Json<CreateNodePort>,
) -> Result<(StatusCode, Json<NodePort>), AppError> {
    let node_port = node_port::create_node_port(&mut conn,node_id,node_port).await?;
    Ok((StatusCode::CREATED, Json(node_port.into())))
}

#[utoipa::path(
    delete,
    path = "/port/{id}",
    params(("id" = i32, Path, description = "node port id")),
    responses((status = OK), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::NODE_TAG
)]
pub async fn delete_node_port(
    Path(id): Path<i32>,
    DbConn(mut conn): DbConn,
) -> Result<(), AppError> {
    node_port::delete_node_port(&mut conn, id).await?;
    Ok(())
}