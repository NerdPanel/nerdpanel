use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use common::models::Node;
use sqlx::{pool::PoolConnection, Postgres};

use crate::{
    models::{
        node::{self},
        server::{self},
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
) -> Result<Node, sqlx::Error> {
    let server = server::get_server_by_id(conn, server_id).await?;
    let node = node::get_node_by_id(conn, server.node_id).await?;
    Ok(node)
}
