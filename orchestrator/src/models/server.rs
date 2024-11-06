use common::models::Server;
use sqlx::PgConnection;

pub async fn get_servers(conn: &mut PgConnection) -> Result<Vec<Server>, sqlx::Error> {
    let servers = sqlx::query_as::<_, Server>("SELECT * FROM servers")
        .fetch_all(&mut *conn)
        .await?;
    Ok(servers)
}

pub async fn get_server_by_id(conn: &mut PgConnection, id: i32) -> Result<Server, sqlx::Error> {
    let server = sqlx::query_as::<_, Server>("SELECT * FROM servers WHERE id = $1")
        .bind(id)
        .fetch_one(conn)
        .await?;
    Ok(server)
}

pub async fn get_servers_by_node_id(
    conn: &mut PgConnection,
    node_id: i32,
) -> Result<Server, sqlx::Error> {
    let server = sqlx::query_as::<_, Server>("SELECT * FROM servers WHERE node_id = $1")
        .bind(node_id)
        .fetch_one(&mut *conn)
        .await?;
    Ok(server)
}

pub async fn create_server(conn: &mut PgConnection, server: Server) -> Result<Server, sqlx::Error> {
    let server = sqlx::query_as::<_, Server>(
        "INSERT INTO servers (name, node_id, ip, port) VALUES ($1, $2, $3, $4) RETURNING *",
    )
    .bind(server.name)
    .bind(server.node_id)
    .bind(server.ip)
    .bind(server.port)
    .fetch_one(&mut *conn)
    .await?;
    Ok(server)
}

pub async fn update_server(conn: &mut PgConnection, server: Server) -> Result<Server, sqlx::Error> {
    let server = sqlx::query_as::<_, Server>(
        "UPDATE servers SET name = $1, node_id = $2, ip = $3, port = $4 WHERE id = $5 RETURNING *",
    )
    .bind(server.name)
    .bind(server.node_id)
    .bind(server.ip)
    .bind(server.port)
    .bind(server.id)
    .fetch_one(&mut *conn)
    .await?;
    Ok(server)
}

pub async fn delete_server(conn: &mut PgConnection, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM servers WHERE id = $1")
        .bind(id)
        .execute(&mut *conn)
        .await?;
    Ok(())
}
