use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use bollard::container::{CreateContainerOptions, InspectContainerOptions, NetworkingConfig};
use common::{agent_types::{ServerSignal, ServerStatus}, models::Server};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::AppState;

pub fn server_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(status))
        .routes(routes!(signal))
        .routes(routes!(update))
        .routes(routes!(create))
        .routes(routes!(install))
        .routes(routes!(delete))
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
    // TODO create volume, pull container, expose port, and more
    
    state
        .docker
        .create_container::<String, String>(
            Some(CreateContainerOptions {
                name: container_name(body.id),
                platform: None
            }),
            bollard::container::Config {
                image: Some(format!("itzg/minecraft-server")),
                tty: Some(true),
                open_stdin: Some(true),
                ..Default::default()
            },
        ).await.unwrap();

    StatusCode::OK
}

#[utoipa::path(
    delete,
    path = "/{id}/delete",
    params(("id" = i32, Path, description = "server id")),
    responses((status = OK, body = String), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = crate::routes::SERVER_TAG
)]
pub async fn delete(Path(id): Path<i32>) -> impl IntoResponse {
    // TODO delete volume, remove container, and more
    (StatusCode::OK, "OK".to_string()).into_response()
}

#[utoipa::path(
    post,
    path = "/{id}/install",
    params(("id" = i32, Path, description = "server id")),
    responses((status = OK, body = String), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = crate::routes::SERVER_TAG
)]
pub async fn install(Path(id): Path<i32>) -> impl IntoResponse {
    // TODO
    (StatusCode::OK, "OK".to_string()).into_response()
}

#[utoipa::path(
    put,
    path = "/{id}/update",
    params(("id" = i32, Path, description = "server id")),
    responses((status = OK, body = String), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = crate::routes::SERVER_TAG
)]
pub async fn update(Path(id): Path<i32>) -> impl IntoResponse {
    // TODO
    (StatusCode::OK, "OK".to_string()).into_response()
}

fn container_name(id: i32) -> String {
    format!("nerdpanel-server-{}", id)
}

fn get_folder(id: i32) -> String {
    // TODO get from env
    format!("run/nerdpanel/volumes/{}", container_name(id))
}