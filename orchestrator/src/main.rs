use routes::ApiDoc;
use sqlx::PgPool;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

pub mod models;
pub mod openapi;
pub mod routes;
pub mod services;
pub mod utils;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    let db = services::database::init_db().await;

    let state = AppState { db };

    let api_router = routes::api_router();

    let app = OpenApiRouter::with_openapi(ApiDoc::openapi());
    let app = app.nest("/api", api_router).with_state(state);
    let (app, api) = app.split_for_parts();
    let app = app.merge(SwaggerUi::new("/api").url("/api/openapi.json", api));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
