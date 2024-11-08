use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use axum_thiserror::ErrorStatus;
use common::orch_types::{Node, Server};
use sqlx::{pool::PoolConnection, Postgres};
use thiserror::Error;

use crate::{
    models::{
        node::{self, NodeModel}, node_port::{get_node_ports_by_node_id, get_node_ports_by_server_id, get_primary_node_port_by_server_id}, server::{self, ServerModel}
    },
    AppState,
};

pub struct DbConn(pub PoolConnection<Postgres>);

#[async_trait]
impl FromRequestParts<AppState> for DbConn {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let pool = state.db.clone();

        let conn = pool
            .acquire()
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(Self(conn))
    }
}

pub async fn get_node_from_server_id(
    server_id: i32,
    conn: &mut PoolConnection<Postgres>,
) -> Result<NodeModel, sqlx::Error> {
    let server = server::get_server_by_id(conn, server_id).await?;
    let node = node::get_node_by_id(conn, server.node_id).await?;
    Ok(node)
}

pub async fn server_model_to_server(
    server: ServerModel,
    conn: &mut PoolConnection<Postgres>,
) -> Result<Server, sqlx::Error> {
    let is_primary = get_primary_node_port_by_server_id(conn, server.id).await?;
    let mut additional_ports = get_node_ports_by_server_id(conn, server.id).await?;
    additional_ports.retain(|port| !port.is_primary);
    Ok(Server {
        id: server.id,
        node_id: server.node_id,
        name: server.name,
        cpu_limit: server.cpu_limit,
        memory_limit: server.memory_limit,
        disk_limit: server.disk_limit,
        primary_port: is_primary.into(),
        additional_ports: additional_ports.into_iter().map(|port| port.into()).collect(),
        pod_id: server.pod_id,
        image: server.image,
        startup_command: server.startup_command,
        env_vars: server.env_vars,
    })
}

pub async fn node_model_to_node(
    node: NodeModel,
    conn: &mut PoolConnection<Postgres>,
) -> Result<Node, sqlx::Error> {
    Ok(Node {
        id: node.id,
        name: node.name,
        fqdn: node.fqdn,
        ports: get_node_ports_by_node_id(conn, node.id).await?.into_iter().map(|port| port.into()).collect(),
    })
}

#[derive(Error, Debug, ErrorStatus)]
pub enum AppError {
    #[error("Database error")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    DatabaseError(sqlx::Error),
    #[error("not found")]
    #[status(StatusCode::NOT_FOUND)]
    NotFound,
    #[error("error connecting to agent")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    AgentRequestError(reqwest::Error),
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => Self::NotFound,
            _ => {
                tracing::error!("Database error: {:?}", e);
                Self::DatabaseError(e)
            }
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        tracing::error!("Error connecting to agent: {:?}", e);
        Self::AgentRequestError(e)
    }
}
