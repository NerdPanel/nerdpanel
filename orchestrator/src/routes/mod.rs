use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use crate::AppState;

pub mod nodes;
pub mod server;

const NODE_TAG: &str = "node";
const SERVER_TAG: &str = "server";

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = NODE_TAG, description = "Node API endpoints"),
        (name = SERVER_TAG, description = "Server API endpoints")
    )
)]
pub struct ApiDoc;

pub fn api_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .nest("/node", nodes::nodes_router())
        .nest("/server", server::server_router())
}
