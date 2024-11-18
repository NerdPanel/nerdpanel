use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use crate::AppState;

#[derive(OpenApi)]
#[openapi(tags())]
pub struct ApiDoc;

pub fn api_router(state: AppState) -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
}
