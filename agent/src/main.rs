use bollard::Docker;
use routes::ApiDoc;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

mod routes;
mod server;

#[derive(Clone)]
pub struct AppState {
    orchestrator_fqdn: String,
    docker: Docker,
}

#[tokio::main]
async fn main() {
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

    // run our app with hyper, listening globally on port 5000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// TODO add logs
