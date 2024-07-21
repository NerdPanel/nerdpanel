use std::error::Error;

use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;

pub mod nodes;
pub mod servers;
pub mod users;

#[derive(Clone)]
pub struct AppState {
    pool: sqlx::PgPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        // TODO - Get database URL from environment variable
        .connect("postgres://hackos:hackos123@localhost/nerdpanel")
        .await
        .unwrap();

    sqlx::migrate!("./migrations").run(&pool).await?;

    let state = AppState { pool };

    // TODO nest everything in /api
    let app = Router::new()
        .route("/", get(root))
        .merge(users::api::router())
        .merge(nodes::api::router())
        .merge(servers::api::router())
        .with_state(state);

    // TODO - Get port from environment variable
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    ""
}
