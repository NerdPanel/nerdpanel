use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use sqlx::{pool::PoolConnection, Postgres};

use crate::AppState;

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
