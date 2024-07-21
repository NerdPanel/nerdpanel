use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use tracing::error;
use uuid::Uuid;

use crate::AppState;

use super::{create, delete as delete_userdb, find_by_id, list, update, FUser, NewUser};

pub fn router() -> Router<AppState> {
    // TODO - Add authentication middleware
    Router::new()
        .route("/users", get(get_users))
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user))
        // TODO - Add password update route
        .route("/users/update", post(edit_user))
        .route("/users/:id", delete(delete_user))
}

async fn get_users(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<FUser>>), StatusCode> {
    match list(&state.pool).await {
        Ok(users) => Ok((StatusCode::OK, Json(users))),
        Err(e) => {
            error!("Failed to list users: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<FUser>), StatusCode> {
    match find_by_id(&state.pool, id).await {
        Ok(user) => match user {
            Some(user) => Ok((StatusCode::OK, Json(user))),
            None => Err(StatusCode::NOT_FOUND),
        },
        Err(e) => {
            error!("Failed to find user: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<NewUser>,
) -> Result<(StatusCode, Json<FUser>), StatusCode> {
    match create(&state.pool, payload).await {
        Ok(user) => Ok((StatusCode::CREATED, Json(user))),
        Err(e) => {
            error!("Failed to create user: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn edit_user(
    State(state): State<AppState>,
    Json(payload): Json<FUser>,
) -> Result<(StatusCode, Json<FUser>), StatusCode> {
    match update(&state.pool, payload).await {
        Ok(user) => Ok((StatusCode::OK, Json(user))),
        Err(e) => {
            error!("Failed to update user: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn delete_user(State(state): State<AppState>, Path(id): Path<Uuid>) -> StatusCode {
    match delete_userdb(&state.pool, id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(e) => {
            error!("Failed to delete user: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
