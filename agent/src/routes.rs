use utoipa::OpenApi;

pub const SERVER_TAG: &str = "server";

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = SERVER_TAG, description = "Server API endpoints")
    )
)]
pub struct ApiDoc;
