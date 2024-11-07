use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use common::{
    agent_types::{ServerSignal, ServerStatus},
    models::Server,
};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{utils::{container_name, container_options, get_folder}, AppState};
use tokio::fs;

pub fn server_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(create, status, update, delete))
        .routes(routes!(signal))
        .routes(routes!(install))
}

#[utoipa::path(
    get,
    path = "/{id}",
    params(("id" = i32, Path, description = "server id")),
    responses((status = OK, body = ServerStatus), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = crate::routes::SERVER_TAG
)]
pub async fn status(Path(id): Path<i32>, State(state): State<AppState>) -> impl IntoResponse {
    let res = state
        .docker
        .inspect_container(&container_name(id), None)
        .await
        .unwrap();
    let status = if res.state.unwrap().running.unwrap() {
        ServerStatus::Running
    } else {
        ServerStatus::Stopped
    };
    (StatusCode::OK, Json(status)).into_response()
}

#[utoipa::path(
    post,
    path = "/{id}/signal",
    params(("id" = i32, Path, description = "server id")),
    responses((status = OK, body = String), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = crate::routes::SERVER_TAG
)]
pub async fn signal(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(body): Json<ServerSignal>,
) -> impl IntoResponse {
    match body {
        ServerSignal::Start => {
            state
                .docker
                .start_container::<String>(&container_name(id), None)
                .await
                .unwrap();
        }
        ServerSignal::Stop => {
            state
                .docker
                .stop_container(&container_name(id), None)
                .await
                .unwrap();
        }
        ServerSignal::Restart => {
            state
                .docker
                .restart_container(&container_name(id), None)
                .await
                .unwrap();
        }
        ServerSignal::Kill => {
            state
                .docker
                .kill_container::<String>(&container_name(id), None)
                .await
                .unwrap();
        }
    }
    StatusCode::OK.into_response()
}

#[utoipa::path(
    post,
    path = "",
    responses((status = OK, body = String), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = crate::routes::SERVER_TAG
)]
pub async fn create(State(state): State<AppState>, Json(body): Json<Server>) -> impl IntoResponse {
    // TODO pull container image

    let folder_path = get_folder(body.id);
    fs::create_dir_all(&folder_path).await.unwrap();

    let (options, config) = container_options(&body);
    state
        .docker
        .create_container::<String, String>(options, config)
        .await
        .unwrap();

    StatusCode::OK
}

#[utoipa::path(
    delete,
    path = "/{id}",
    params(("id" = i32, Path, description = "server id")),
    responses((status = OK, body = String), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = crate::routes::SERVER_TAG
)]
pub async fn delete(Path(id): Path<i32>, State(state): State<AppState>) -> impl IntoResponse {
    if state
        .docker
        .inspect_container(&container_name(id), None)
        .await
        .unwrap()
        .state
        .unwrap()
        .running
        .unwrap()
    {
        state
            .docker
            .stop_container(&container_name(id), None)
            .await
            .unwrap();
    }

    state
        .docker
        .remove_container(&container_name(id), None)
        .await
        .unwrap();

    let folder_path = fs::canonicalize(get_folder(id)).await.unwrap();
    fs::remove_dir_all(&folder_path).await.unwrap();

    StatusCode::OK
}

#[utoipa::path(
    post,
    path = "/{id}/install",
    params(("id" = i32, Path, description = "server id")),
    responses((status = OK, body = String), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = crate::routes::SERVER_TAG
)]
pub async fn install(Path(_id): Path<i32>) -> impl IntoResponse {
    // TODO
    (StatusCode::OK, "OK".to_string()).into_response()
}

#[utoipa::path(
    put,
    path = "",
    responses((status = OK, body = String), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = crate::routes::SERVER_TAG
)]
pub async fn update(State(state): State<AppState>, Json(body): Json<Server>) -> impl IntoResponse {
    if state
        .docker
        .inspect_container(&container_name(body.id), None)
        .await
        .unwrap()
        .state
        .unwrap()
        .running
        .unwrap()
    {
        state
            .docker
            .stop_container(&container_name(body.id), None)
            .await
            .unwrap();
    }

    state
        .docker
        .remove_container(&container_name(body.id), None)
        .await
        .unwrap();

    let (options, config) = container_options(&body);
    state
        .docker
        .create_container::<String, String>(options, config)
        .await
        .unwrap();
}
