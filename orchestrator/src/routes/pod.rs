use axum::{extract::Path, Json};
use common::orch_types::Pod;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    models::pod,
    utils::{AppError, DbConn},
    AppState,
};

pub fn pods_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(get_pods, create_pod, update_pod))
        .routes(routes!(get_pod_by_id, delete_pod))
}

#[utoipa::path(
    get,
    path = "",
    responses((status = OK, body = [Pod]),(status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::POD_TAG
)]
pub async fn get_pods(DbConn(mut conn): DbConn) -> Result<Json<Vec<Pod>>, AppError> {
    let pods = pod::get_pods(&mut conn).await?;
    let pods = pods.into_iter().map(|p| p.into()).collect();
    Ok(Json(pods))
}

#[utoipa::path(
    get,
    path = "/{id}",
    params(("id" = u32, Path, description = "pod id")),
    responses((status = OK, body = Pod),(status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::POD_TAG
)]
pub async fn get_pod_by_id(
    DbConn(mut conn): DbConn,
    Path(id): Path<u32>,
) -> Result<Json<Pod>, AppError> {
    let pod = pod::get_pod_by_id(&mut conn, id as i32).await?;
    Ok(Json(pod.into()))
}

#[utoipa::path(
    post,
    path = "",
    responses((status = OK, body = Pod),(status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::POD_TAG
)]
pub async fn create_pod(
    DbConn(mut conn): DbConn,
    Json(pod): Json<pod::CreatePod>,
) -> Result<Json<Pod>, AppError> {
    let pod = pod::create_pod(&mut conn, pod).await?;
    Ok(Json(pod.into()))
}

#[utoipa::path(
    put,
    path = "",
    responses((status = OK, body = Pod),(status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::POD_TAG
)]
pub async fn update_pod(
    DbConn(mut conn): DbConn,
    Json(pod): Json<pod::PodModel>,
) -> Result<Json<Pod>, AppError> {
    let pod = pod::update_pod(&mut conn, pod).await?;
    Ok(Json(pod.into()))
}

#[utoipa::path(
    delete,
    path = "/{id}",
    params(("id" = u32, Path, description = "pod id")),
    responses((status = OK),(status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::POD_TAG
)]
pub async fn delete_pod(DbConn(mut conn): DbConn, Path(id): Path<u32>) -> Result<(), AppError> {
    pod::delete_pod(&mut conn, id as i32).await?;
    Ok(())
}
