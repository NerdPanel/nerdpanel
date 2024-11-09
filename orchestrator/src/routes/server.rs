use axum::{extract::Path, http::StatusCode, middleware, response::IntoResponse, Json};
use common::{
    agent_types::{ServerSignal, ServerStatus},
    orch_types::Server,
};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    models::server::{self, CreateServer, UpdateServer, UpdateServerStaff},
    utils::{
        auth::{require_server_owner_staff, require_server_owner_staff_path, require_staff},
        get_node_from_server_id, server_model_to_server, AppError, DbConn,
    },
    AppState,
};

pub fn server_router(state: AppState) -> OpenApiRouter<AppState> {
    let staff_router = OpenApiRouter::new()
        .routes(routes!(update_server_staff))
        .routes(routes!(create_server))
        .routes(routes!(delete_server))
        .routes(routes!(get_servers_by_node_id))
        .route_layer(middleware::from_fn(require_staff));

    OpenApiRouter::new()
        .routes(routes!(status))
        .routes(routes!(signal))
        .routes(routes!(install))
        .routes(routes!(get_server))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_server_owner_staff_path,
        ))
        .routes(routes!(update_server).layer(middleware::from_fn_with_state(
            state,
            require_server_owner_staff,
        )))
        .merge(staff_router)
        .routes(routes!(get_servers))
}

#[utoipa::path(
    get,
    path = "",
    responses((status = OK, body = [Server]), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::SERVER_TAG
)]
pub async fn get_servers(DbConn(mut conn): DbConn) -> Result<Json<Vec<Server>>, AppError> {
    let servers = server::get_servers(&mut conn).await?;

    let servers: Vec<Server> = {
        let mut new = vec![];
        for server in servers {
            new.push(server_model_to_server(server, &mut conn).await?);
        }
        new
    };
    tracing::info!("Fetched {} servers", servers.len());
    Ok(Json(servers))
}

#[utoipa::path(
    get,
    path = "/{id}",
    params(("id" = i32, Path, description = "server id")),
    responses((status = OK, body = Server), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::SERVER_TAG
)]
pub async fn get_server(
    Path(id): axum::extract::Path<i32>,
    DbConn(mut conn): DbConn,
) -> Result<Json<Server>, AppError> {
    let server = server::get_server_by_id(&mut conn, id).await?;
    let server = server_model_to_server(server, &mut conn).await?;
    Ok(Json(server))
}

#[utoipa::path(
    get,
    path = "/node/{node_id}",
    params(("node_id" = i32, Path, description = "node id")),
    responses((status = OK, body = [Server]), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::SERVER_TAG
)]
pub async fn get_servers_by_node_id(
    Path(node_id): Path<i32>,
    DbConn(mut conn): DbConn,
) -> Result<Json<Vec<Server>>, AppError> {
    let servers = server::get_servers_by_node_id(&mut conn, node_id).await?;
    let servers: Vec<Server> = {
        let mut new = vec![];
        for server in servers {
            new.push(server_model_to_server(server, &mut conn).await?);
        }
        new
    };
    Ok(Json(servers))
}

#[utoipa::path(
    post,
    path = "",
    responses((status = CREATED, body = Server), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::SERVER_TAG
)]
pub async fn create_server(
    DbConn(mut conn): DbConn,
    Json(server): Json<CreateServer>,
) -> Result<Json<Server>, AppError> {
    let server = server::create_server(&mut conn, server).await?;
    let node = get_node_from_server_id(server.id, &mut conn).await?;
    let server = server_model_to_server(server, &mut conn).await?;
    let res = reqwest::Client::new()
        .post(format!("http://{}/server", node.fqdn))
        .json(&server)
        .send()
        .await?;
    if res.status() != StatusCode::OK {
        return Err(AppError::NodeError(res.text().await?));
    }
    Ok(Json(server))
}

#[utoipa::path(
    put,
    path = "",
    responses((status = OK, body = Server), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::SERVER_TAG
)]
pub async fn update_server(
    DbConn(mut conn): DbConn,
    Json(server): Json<UpdateServer>,
) -> Result<Json<Server>, AppError> {
    let server = server::update_server(&mut conn, server).await?;
    let node = get_node_from_server_id(server.id, &mut conn).await?;
    let server = server_model_to_server(server, &mut conn).await?;
    let res = reqwest::Client::new()
        .put(format!("http://{}/server", node.fqdn))
        .json(&server)
        .send()
        .await?;
    if res.status() != StatusCode::OK {
        return Err(AppError::NodeError(res.text().await?));
    }
    Ok(Json(server))
}

#[utoipa::path(
    put,
    path = "/staff",
    responses((status = OK, body = Server), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::SERVER_TAG
)]
pub async fn update_server_staff(
    DbConn(mut conn): DbConn,
    Json(server): Json<UpdateServerStaff>,
) -> Result<Json<Server>, AppError> {
    let server = server::update_server_staff(&mut conn, server).await?;
    let node = get_node_from_server_id(server.id, &mut conn).await?;
    let server = server_model_to_server(server, &mut conn).await?;
    let res = reqwest::Client::new()
        .put(format!("http://{}/server", node.fqdn))
        .json(&server)
        .send()
        .await?;
    if res.status() != StatusCode::OK {
        return Err(AppError::NodeError(res.text().await?));
    }
    Ok(Json(server))
}

#[utoipa::path(
    delete,
    path = "/{id}",
    params(("id" = i32, Path, description = "server id")),
    responses((status = OK), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::SERVER_TAG
)]
pub async fn delete_server(Path(id): Path<i32>, DbConn(mut conn): DbConn) -> Result<(), AppError> {
    let node = get_node_from_server_id(id, &mut conn).await?;
    let res = reqwest::Client::new()
        .delete(format!("http://{}/server/{}", node.fqdn, id))
        .send()
        .await?;
    if res.status() != StatusCode::OK {
        return Err(AppError::NodeError(res.text().await?));
    }
    server::delete_server(&mut conn, id).await?;
    Ok(())
}

#[utoipa::path(
    get,
    path = "/{id}/status",
    params(("id" = i32, Path, description = "server id")),
    responses((status = OK, body = ServerStatus), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::SERVER_TAG

)]
pub async fn status(
    Path(id): Path<i32>,
    DbConn(mut conn): DbConn,
) -> Result<Json<ServerStatus>, AppError> {
    let node = get_node_from_server_id(id, &mut conn).await?;
    let res = reqwest::get(format!("http://{}/server/{}", node.fqdn, id)).await?;
    if res.status() != StatusCode::OK {
        return Err(AppError::NodeError(res.text().await?));
    }
    let status: ServerStatus = res.json().await?;
    Ok(Json(status))
}

#[utoipa::path(
    post,
    path = "/{id}/signal",
    params(("id" = i32, Path, description = "server id")),
    responses((status = OK, body = ()), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::SERVER_TAG
)]
pub async fn signal(
    Path(id): Path<i32>,
    DbConn(mut conn): DbConn,
    Json(body): Json<ServerSignal>,
) -> Result<(), AppError> {
    let node = get_node_from_server_id(id, &mut conn).await?;
    let res = reqwest::Client::new()
        .post(format!("http://{}/server/{}/signal", node.fqdn, id))
        .json(&body)
        .send()
        .await?;
    if res.status() != StatusCode::OK {
        return Err(AppError::NodeError(res.text().await?));
    }
    Ok(())
}

// TODO DO THIS PROPERLY

#[utoipa::path(
    post,
    path = "/{id}/install",
    params(("id" = i32, Path, description = "server id")),
    responses((status = OK, body = String), (status = INTERNAL_SERVER_ERROR, body = String)),
    tag = super::SERVER_TAG
)]
pub async fn install(
    Path(id): Path<i32>,
    DbConn(mut conn): DbConn,
    Json(body): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    let node = get_node_from_server_id(id, &mut conn).await?;
    let status = reqwest::Client::new()
        .post(format!("http://{}/server/{}/install", node.fqdn, id))
        .json(&body)
        .send()
        .await?
        .text()
        .await?;
    Ok((StatusCode::OK, status))
}
