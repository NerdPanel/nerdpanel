use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use common::{
    agent_types::{ServerSignal, ServerStatus},
    orch_types::Server,
};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    utils::{container_name, container_options, get_folder, AppError},
    AppState,
};
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
pub async fn status(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let res = state
        .docker
        .inspect_container(&container_name(id), None)
        .await?;
    let status = if res.state.unwrap().running.unwrap() {
        ServerStatus::Running
    } else {
        ServerStatus::Stopped
    };
    Ok((StatusCode::OK, Json(status)))
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
) -> Result<impl IntoResponse, AppError> {
    match body {
        ServerSignal::Start => {
            state
                .docker
                .start_container::<String>(&container_name(id), None)
                .await?;
        }
        ServerSignal::Stop => {
            state
                .docker
                .stop_container(&container_name(id), None)
                .await?;
        }
        ServerSignal::Restart => {
            state
                .docker
                .restart_container(&container_name(id), None)
                .await?;
        }
        ServerSignal::Kill => {
            state
                .docker
                .kill_container::<String>(&container_name(id), None)
                .await?;
        }
    }
    Ok(StatusCode::OK)
}

#[utoipa::path(
    post,
    path = "",
    responses((status = OK, body = String), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = crate::routes::SERVER_TAG
)]
pub async fn create(
    State(state): State<AppState>,
    Json(body): Json<Server>,
) -> Result<impl IntoResponse, AppError> {
    // TODO pull container image

    let folder_path = get_folder(body.id);
    fs::create_dir_all(&folder_path).await.unwrap();

    let (options, config) = container_options(&body);
    state
        .docker
        .create_container::<String, String>(options, config)
        .await?;

    Ok(StatusCode::OK)
}

#[utoipa::path(
    delete,
    path = "/{id}",
    params(("id" = i32, Path, description = "server id")),
    responses((status = OK, body = String), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = crate::routes::SERVER_TAG
)]
pub async fn delete(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    if state
        .docker
        .inspect_container(&container_name(id), None)
        .await?
        .state
        .unwrap()
        .running
        .unwrap()
    {
        state
            .docker
            .stop_container(&container_name(id), None)
            .await?;
    }

    state
        .docker
        .remove_container(&container_name(id), None)
        .await?;

    let folder_path = fs::canonicalize(get_folder(id)).await.unwrap();
    fs::remove_dir_all(&folder_path).await.unwrap();

    Ok(StatusCode::OK)
}

#[utoipa::path(
    post,
    path = "/{id}/install",
    params(("id" = i32, Path, description = "server id")),
    responses((status = OK, body = String), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = crate::routes::SERVER_TAG
)]
pub async fn install(Path(_id): Path<i32>) -> Result<impl IntoResponse, AppError> {
    // TODO
    Ok(((StatusCode::OK), "OK".to_string()))
}

#[utoipa::path(
    put,
    path = "",
    responses((status = OK, body = String), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = crate::routes::SERVER_TAG
)]
pub async fn update(
    State(state): State<AppState>,
    Json(body): Json<Server>,
) -> Result<impl IntoResponse, AppError> {
    if state
        .docker
        .inspect_container(&container_name(body.id), None)
        .await?
        .state
        .unwrap()
        .running
        .unwrap()
    {
        state
            .docker
            .stop_container(&container_name(body.id), None)
            .await?;
    }

    state
        .docker
        .remove_container(&container_name(body.id), None)
        .await?;

    let (options, config) = container_options(&body);
    state
        .docker
        .create_container::<String, String>(options, config)
        .await?;

    Ok(StatusCode::OK)
}
