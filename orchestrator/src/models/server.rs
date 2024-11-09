use common::orch_types::EnvVar;
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use utoipa::ToSchema;

use super::node_port::{assign_node_port_to_server, unassign_all_node_port_from_server};

#[derive(sqlx::FromRow, Serialize, Deserialize, ToSchema)]
pub struct ServerModel {
    pub id: i32,
    pub name: String,
    pub node_id: i32,

    pub cpu_limit: Option<i32>,
    pub memory_limit: Option<i32>,
    pub disk_limit: Option<i32>,

    pub pod_id: i32,
    pub image: String,
    pub startup_command: String,
    pub env_vars: Vec<EnvVar>,
}

pub async fn get_servers(conn: &mut PgConnection) -> Result<Vec<ServerModel>, sqlx::Error> {
    let servers = sqlx::query_as::<_, ServerModel>("SELECT * FROM server")
        .fetch_all(&mut *conn)
        .await?;
    Ok(servers)
}

pub async fn get_server_by_id(
    conn: &mut PgConnection,
    id: i32,
) -> Result<ServerModel, sqlx::Error> {
    let server = sqlx::query_as::<_, ServerModel>("SELECT * FROM server WHERE id = $1")
        .bind(id)
        .fetch_one(conn)
        .await?;
    Ok(server)
}

pub async fn get_servers_by_node_id(
    conn: &mut PgConnection,
    node_id: i32,
) -> Result<Vec<ServerModel>, sqlx::Error> {
    let server = sqlx::query_as::<_, ServerModel>("SELECT * FROM server WHERE node_id = $1")
        .bind(node_id)
        .fetch_all(&mut *conn)
        .await?;
    Ok(server)
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateServer {
    pub name: String,
    pub node_id: i32,
    pub cpu_limit: Option<i32>,
    pub memory_limit: Option<i32>,
    pub disk_limit: Option<i32>,

    pub port: i32,
    pub additional_ports: Vec<i32>,

    pub pod_id: i32,
    pub image: String,
    pub startup_command: String,
    pub env_vars: Vec<EnvVar>,
}

pub async fn create_server(
    conn: &mut PgConnection,
    cserver: CreateServer,
) -> Result<ServerModel, sqlx::Error> {
    // TODO verify image and env_vars

    let server = sqlx::query_as::<_, ServerModel>(
        "INSERT INTO server (name, node_id, cpu_limit, memory_limit, disk_limit, pod_id, image, startup_command, env_vars) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *",
    )
    .bind(cserver.name)
    .bind(cserver.node_id)
    .bind(cserver.cpu_limit)
    .bind(cserver.memory_limit)
    .bind(cserver.disk_limit)
    .bind(cserver.pod_id)
    .bind(cserver.image)
    .bind(cserver.startup_command)
    .bind(cserver.env_vars)
    .fetch_one(&mut *conn)
    .await?;

    // TODO verify that the port is not already in use AND that port belongs to the node

    assign_node_port_to_server(conn, cserver.port, server.id, true).await?;

    for port in cserver.additional_ports {
        assign_node_port_to_server(conn, port, server.id, false).await?;
    }

    Ok(server)
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateServer {
    pub id: i32,
    pub name: String,
    pub node_id: i32,

    pub cpu_limit: Option<i32>,
    pub memory_limit: Option<i32>,
    pub disk_limit: Option<i32>,

    pub port: i32,
    pub additional_ports: Vec<i32>,

    pub pod_id: i32,
    pub image: String,
    pub startup_command: String,
    pub env_vars: Vec<EnvVar>,
}

pub async fn update_server(
    conn: &mut PgConnection,
    userver: UpdateServer,
) -> Result<ServerModel, sqlx::Error> {
    // TODO verify image and env_vars

    let server = sqlx::query_as::<_, ServerModel>(
        "UPDATE server SET name = $1, node_id = $2, cpu_limit = $3, memory_limit = $4, disk_limit = $5, pod_id = $6, image = $7, startup_command = $8, env_vars = $9 WHERE id = $10 RETURNING *",
    )
    .bind(userver.name)
    .bind(userver.node_id)
    .bind(userver.cpu_limit)
    .bind(userver.memory_limit)
    .bind(userver.disk_limit)
    .bind(userver.pod_id)
    .bind(userver.image)
    .bind(userver.startup_command)
    .bind(userver.env_vars)
    .bind(userver.id)
    .fetch_one(&mut *conn)
    .await?;

    // TODO verify that the port is not already in use AND that port belongs to the node

    unassign_all_node_port_from_server(conn, server.id).await?;

    assign_node_port_to_server(conn, userver.port, server.id, true).await?;

    for port in userver.additional_ports {
        assign_node_port_to_server(conn, port, server.id, false).await?;
    }
    Ok(server)
}

pub async fn delete_server(conn: &mut PgConnection, id: i32) -> Result<(), sqlx::Error> {
    unassign_all_node_port_from_server(conn, id).await?;
    sqlx::query("DELETE FROM server WHERE id = $1")
        .bind(id)
        .execute(&mut *conn)
        .await?;
    Ok(())
}
