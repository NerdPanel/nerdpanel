use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use tracing::error;
use uuid::Uuid;

use crate::AppState;

use super::{
    create, find_by_id, find_by_node_id, find_by_owner_id, list, update, NewServer, Server,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/servers", get(get_servers))
        .route("/servers/:id", get(get_server))
        .route("/servers/node/:id", get(get_servers_for_node))
        .route("/servers/owner/:id", get(get_servers_for_owner))
        .route("/servers", post(create_server))
        .route("/servers/update", post(update_server))
        .route("/servers/:id", delete(delete_server))
}

async fn get_servers(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<Server>>), StatusCode> {
    match list(&state.pool).await {
        Ok(servers) => Ok((StatusCode::OK, Json(servers))),
        Err(e) => {
            error!("Failed to list servers: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_server(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Server>), StatusCode> {
    match find_by_id(&state.pool, id).await {
        Ok(server) => match server {
            Some(server) => Ok((StatusCode::OK, Json(server))),
            None => Err(StatusCode::NOT_FOUND),
        },
        Err(e) => {
            error!("Failed to find server: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_servers_for_node(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Vec<Server>>), StatusCode> {
    match find_by_node_id(&state.pool, id).await {
        Ok(servers) => Ok((StatusCode::OK, Json(servers))),
        Err(e)  => {
            error!("Failed to find servers for node: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_servers_for_owner(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Vec<Server>>), StatusCode> {
    match find_by_owner_id(&state.pool, id).await {
        Ok(servers) => Ok((StatusCode::OK, Json(servers))),
        Err(e) => {
            error!("Failed to find servers for owner: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn create_server(
    State(state): State<AppState>,
    Json(new_server): Json<NewServer>,
) -> Result<(StatusCode, Json<Server>), StatusCode> {
    match create(&state.pool, new_server).await {
        Ok(server) => Ok((StatusCode::CREATED, Json(server))),
        Err(e) => {
            error!("Failed to create server: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn update_server(
    State(state): State<AppState>,
    Json(server): Json<Server>,
) -> Result<(StatusCode, Json<Server>), StatusCode> {
    match update(&state.pool, server).await {
        Ok(server) => Ok((StatusCode::OK, Json(server))),
        Err(e) => {
            error!("Failed to update server: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn delete_server(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    match super::delete(&state.pool, id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => {
            error!("Failed to delete server: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
