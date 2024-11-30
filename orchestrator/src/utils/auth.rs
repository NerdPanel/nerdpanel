use axum::{
    body::Body,
    extract::{Path, Request},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use http_body_util::BodyExt;

use crate::{
    auth::AuthSession,
    models::server::{self, UpdateServer},
};

use super::DbConn;

pub async fn require_staff(
    auth_session: AuthSession,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let user = auth_session.user.unwrap();
    if !user.staff {
        return Err(StatusCode::FORBIDDEN);
    }

    let response = next.run(request).await;
    Ok(response)
}

pub async fn require_server_owner_staff_path(
    auth_session: AuthSession,
    Path(server_id): Path<i32>,
    DbConn(mut conn): DbConn,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let user = auth_session.user.unwrap();
    if user.staff {
        return Ok(next.run(request).await);
    }
    let server = server::get_server_by_id(&mut conn, server_id).await;
    if let Ok(server) = server {
        if server.owner_id != user.id {
            return Err(StatusCode::FORBIDDEN);
        }
    } else {
        return Err(StatusCode::NOT_FOUND);
    }

    let response = next.run(request).await;
    Ok(response)
}

pub async fn require_server_owner_staff(
    auth_session: AuthSession,
    DbConn(mut conn): DbConn,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let user = auth_session.user.unwrap();
    if user.staff {
        return Ok(next.run(request).await);
    }

    let (parts, body) = request.into_parts();
    let bytes = body.collect().await.unwrap().to_bytes();
    let server: UpdateServer = serde_json::from_slice(&bytes).unwrap();
    let request = Request::from_parts(parts, Body::from(bytes));

    let server = server::get_server_by_id(&mut conn, server.id).await;
    if let Ok(server) = server {
        if server.owner_id != user.id {
            return Err(StatusCode::FORBIDDEN);
        }
    } else {
        return Err(StatusCode::NOT_FOUND);
    }

    let response = next.run(request).await;
    Ok(response)
}
