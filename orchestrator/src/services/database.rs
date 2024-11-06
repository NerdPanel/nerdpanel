use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn init_db() -> PgPool {
    // TODO get the database URL from an environment variable
    let db_url = "postgres://postgres:example@localhost:5432/nerdpanel";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations.");
    pool
}
