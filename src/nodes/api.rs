use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use tracing::error;
use uuid::Uuid;

use crate::AppState;

use super::{create, delete as delete_nodedb, find_by_id, list, update, NewNode, Node};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/nodes", get(get_nodes))
        .route("/nodes", post(create_node))
        .route("/nodes/:id", get(get_node))
        .route("/nodes/update", post(update_node))
        .route("/nodes/:id", delete(delete_node))
}

async fn get_nodes(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<Node>>), StatusCode> {
    match list(&state.pool).await {
        Ok(nodes) => Ok((StatusCode::OK, Json(nodes))),
        Err(e) => {
            error!("Failed to list nodes: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn create_node(
    State(state): State<AppState>,
    Json(node): Json<NewNode>,
) -> Result<StatusCode, StatusCode> {
    match create(&state.pool, node).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            error!("Failed to create node: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_node(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Node>), StatusCode> {
    match find_by_id(&state.pool, id).await {
        Ok(node) => match node {
            Some(node) => Ok((StatusCode::OK, Json(node))),
            None => Err(StatusCode::NOT_FOUND),
        },
        Err(e) => {
            error!("Failed to find node: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn update_node(
    State(state): State<AppState>,
    Json(node): Json<Node>,
) -> Result<(StatusCode, Json<Node>), StatusCode> {
    match update(&state.pool, node).await {
        Ok(node) => Ok((StatusCode::OK, Json(node))),
        Err(e) => {
            error!("Failed to update node: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn delete_node(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    match delete_nodedb(&state.pool, id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => {
            error!("Failed to delete node: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
