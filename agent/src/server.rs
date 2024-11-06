use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use bollard::{
    container::{Config, CreateContainerOptions},
    secret::{HostConfig, Mount, MountTypeEnum, PortBinding},
};
use common::{
    agent_types::{ServerSignal, ServerStatus},
    models::Server,
};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::AppState;
use tokio::fs;

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
    // TODO pull container image

    let folder_path = get_folder(body.id);
    fs::create_dir_all(&folder_path).await.unwrap();
    let folder_path = fs::canonicalize(&folder_path).await.unwrap();

    let mut port_bindings = ::std::collections::HashMap::new();
    port_bindings.insert(
        format!("{}/tcp", body.port),
        Some(vec![PortBinding {
            host_ip: Some(body.ip),
            host_port: Some(body.port.to_string()),
        }]),
    );
    let host_config = HostConfig {
        mounts: Some(vec![Mount {
            target: Some(String::from("/data")),
            source: Some(folder_path.to_string_lossy().to_string()),
            typ: Some(MountTypeEnum::BIND),
            consistency: Some(String::from("default")),
            ..Default::default()
        }]),
        port_bindings: Some(port_bindings),
        ..Default::default()
    };
        

    state
        .docker
        .create_container::<String, String>(
            Some(CreateContainerOptions {
                name: container_name(body.id),
                platform: None,
            }),
            Config {
                image: Some(format!("itzg/minecraft-server")),
                tty: Some(true),
                open_stdin: Some(true),
                env: Some(vec![
                    "EULA=TRUE".to_string(),
                ]),
                host_config: Some(host_config),
                exposed_ports: {
                    let mut map = ::std::collections::HashMap::new();
                    map.insert(format!("{}/tcp", body.port), ::std::collections::HashMap::new());
                    Some(map)
                },
                ..Default::default()
            },

        )
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

    if state.docker.inspect_container(&container_name(id), None).await.unwrap().state.unwrap().running.unwrap() {
        state.docker.stop_container(&container_name(id), None).await.unwrap();
    }

    state.docker.remove_container(&container_name(id), None).await.unwrap();

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
    path = "/{id}/update",
    params(("id" = i32, Path, description = "server id")),
    responses((status = OK, body = String), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = crate::routes::SERVER_TAG
)]
pub async fn update(Path(_id): Path<i32>) -> impl IntoResponse {
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
