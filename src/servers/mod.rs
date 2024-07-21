use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

pub mod api;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Server {
    pub server_id: Uuid,
    pub server_name: String,
    pub server_ip: String,
    pub server_port: i32,
    pub server_status: ServerStatus,
    pub cpu: i32,
    pub memory: i32,
    pub disk: i32,
    pub node_id: Uuid,
    pub owner_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "server_status", rename_all = "lowercase")]
pub enum ServerStatus {
    Running,
    Stopped,
    Starting,
    Stopping,
    Restarting,
    Installing,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewServer {
    pub server_name: String,
    pub server_ip: String,
    pub server_port: i32,
    pub server_status: ServerStatus,
    pub cpu: i32,
    pub memory: i32,
    pub disk: i32,
    pub node_id: Uuid,
    pub owner_id: Uuid,
}

async fn list(pool: &sqlx::PgPool) -> Result<Vec<Server>, sqlx::Error> {
    let servers = sqlx::query_as::<_, Server>(
        r#"
        SELECT server_id, server_name, server_ip, server_port, server_status, cpu, memory, disk, node_id, owner_id
        FROM servers
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(servers)
}

async fn find_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<Option<Server>, sqlx::Error> {
    let server = sqlx::query_as::<_, Server>(
        r#"
        SELECT server_id, server_name, server_ip, server_port, server_status, cpu, memory, disk, node_id, owner_id
        FROM servers
        WHERE server_id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(server)
}

async fn find_by_node_id(pool: &sqlx::PgPool, id: Uuid) -> Result<Vec<Server>, sqlx::Error> {
    let servers = sqlx::query_as::<_, Server>(
        r#"
        SELECT server_id, server_name, server_ip, server_port, server_status, cpu, memory, disk, node_id, owner_id
        FROM servers
        WHERE node_id = $1
        "#,
    )
    .bind(id)
    .fetch_all(pool)
    .await?;
    Ok(servers)
}

async fn find_by_owner_id(pool: &sqlx::PgPool, id: Uuid) -> Result<Vec<Server>, sqlx::Error> {
    let servers = sqlx::query_as::<_, Server>(
        r#"
        SELECT server_id, server_name, server_ip, server_port, server_status, cpu, memory, disk, node_id, owner_id
        FROM servers
        WHERE owner_id = $1
        "#,
    )
    .bind(id)
    .fetch_all(pool)
    .await?;
    Ok(servers)
}

async fn create(pool: &sqlx::PgPool, new_server: NewServer) -> Result<Server, sqlx::Error> {
    let server = sqlx::query_as::<_, Server>(
        r#"
        INSERT INTO servers (server_name, server_ip, server_port, server_status, cpu, memory, disk, node_id, owner_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING server_id, server_name, server_ip, server_port, server_status, cpu, memory, disk, node_id, owner_id
        "#,
    )
    .bind(new_server.server_name)
    .bind(new_server.server_ip)
    .bind(new_server.server_port)
    .bind(new_server.server_status)
    .bind(new_server.cpu)
    .bind(new_server.memory)
    .bind(new_server.disk)
    .bind(new_server.node_id)
    .bind(new_server.owner_id)
    .fetch_one(pool)
    .await?;
    Ok(server)
}

async fn update(pool: &sqlx::PgPool, server: Server) -> Result<Server, sqlx::Error> {
    let server = sqlx::query_as::<_, Server>(
        r#"
        UPDATE servers
        SET server_name = $1, server_ip = $2, server_port = $3, server_status = $4, cpu = $5, memory = $6, disk = $7, node_id = $8, owner_id = $9
        WHERE server_id = $10
        RETURNING server_id, server_name, server_ip, server_port, server_status, cpu, memory, disk, node_id, owner_id
        "#,
    )
    .bind(server.server_name)
    .bind(server.server_ip)
    .bind(server.server_port)
    .bind(server.server_status)
    .bind(server.cpu)
    .bind(server.memory)
    .bind(server.disk)
    .bind(server.node_id)
    .bind(server.owner_id)
    .bind(server.server_id)
    .fetch_one(pool)
    .await?;
    Ok(server)
}

async fn delete(pool: &sqlx::PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM servers
        WHERE server_id = $1
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}
