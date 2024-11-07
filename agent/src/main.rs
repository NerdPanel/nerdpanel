use axum::extract::Request;
use bollard::Docker;
use routes::ApiDoc;
use tower_http::trace::TraceLayer;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

mod routes;
mod server;
mod utils;
#[derive(Clone)]
pub struct AppState {
    orchestrator_fqdn: String,
    docker: Docker,
}

#[tokio::main]
async fn main() {
    let file_appender = tracing_appender::rolling::daily("/nerdagent/logs", "orchestrator.log");
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

    let docker = Docker::connect_with_local_defaults().unwrap();
    let state = AppState {
        // TODO: get orchestrator_fqdn from env
        orchestrator_fqdn: "http://localhost:3000".to_string(),
        docker,
    };

    let app = OpenApiRouter::with_openapi(ApiDoc::openapi());
    let app = app.nest("/server", server::server_routes());
    let (app, api) = app.split_for_parts();
    let app = app.with_state(state);
    let app = app.merge(SwaggerUi::new("/api").url("/api/openapi.json", api));
    let app = app.layer(TraceLayer::new_for_http().make_span_with(|req: &Request| {
        let method = req.method();
        let uri = req.uri();
        tracing::info_span!("request", %method, %uri)
    }));
    // run our app with hyper, listening globally on port 5000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

