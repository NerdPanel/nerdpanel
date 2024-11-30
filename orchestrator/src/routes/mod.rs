use axum::middleware;
use axum_login::login_required;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use crate::{auth::AuthBackend, utils::auth::require_staff, AppState};

pub mod auth;
pub mod nodes;
pub mod pod;
pub mod server;
pub mod user;

const NODE_TAG: &str = "node";
const SERVER_TAG: &str = "server";
const POD_TAG: &str = "pod";
const USER_TAG: &str = "user";
const AUTH_TAG: &str = "auth";

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = NODE_TAG, description = "Node API endpoints"),
        (name = SERVER_TAG, description = "Server API endpoints"),
        (name = POD_TAG, description = "Pod API endpoints"),
        (name = USER_TAG, description = "User API endpoints"),
        (name = AUTH_TAG, description = "Authentication API endpoints")
    )
)]
pub struct ApiDoc;

pub fn api_router(state: AppState) -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .nest("/node", nodes::nodes_router())
        .nest("/pod", pod::pods_router())
        .route_layer(middleware::from_fn(require_staff))
        .nest("/user", user::user_router())
        .nest("/server", server::server_router(state))
        .layer(login_required!(AuthBackend))
        .nest("/auth", auth::auth_router())
}
