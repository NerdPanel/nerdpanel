use auth::AuthBackend;
use axum::{
    extract::Request,
    http::{HeaderName, HeaderValue, Method},
};
use axum_login::{
    tower_sessions::{MemoryStore, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use routes::ApiDoc;
use sqlx::PgPool;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

pub mod auth;
pub mod models;
pub mod routes;
pub mod services;
pub mod utils;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    let file_appender = tracing_appender::rolling::daily("/nerdpanel/logs", "orchestrator.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_writer(std::io::stdout)
        .with_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        );

    let file_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_writer(non_blocking)
        .with_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        );

    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(file_layer)
        .try_init()
        .unwrap();

    let db = services::database::init_db().await;

    // Session layer.
    let session_store = MemoryStore::default();
    // TODO - make proper SameSite and Secure settings
    let session_layer = SessionManagerLayer::new(session_store).with_secure(false);
    // Auth service.
    let backend = AuthBackend { db: db.clone() };
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let state = AppState { db };

    let api_router = routes::api_router(state.clone());

    let app = OpenApiRouter::with_openapi(ApiDoc::openapi());
    let app = app.nest("/api", api_router).with_state(state);
    let (app, api) = app.split_for_parts();
    let app = app.merge(SwaggerUi::new("/api").url("/api/openapi.json", api));
    let app = app.layer(auth_layer);
    // TODO - make proper CORS policy
    let cors_layer = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
        .allow_credentials(true)
        .allow_headers([
            HeaderName::from_static("authorization"),
            HeaderName::from_static("content-type"),
        ]);
    let app = app.layer(cors_layer);
    let app = app.layer(TraceLayer::new_for_http().make_span_with(|req: &Request| {
        let method = req.method();
        let uri = req.uri();
        tracing::info_span!("request", %method, %uri)
    }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
