use common::orch_types::{NodePort, ServerNodePort};
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use utoipa::ToSchema;

#[derive(sqlx::FromRow, Serialize, Deserialize, ToSchema)]
pub struct NodePortModel {
    pub id: i32,
    pub node_id: i32,
    pub server_id: Option<i32>,
    pub is_primary: bool,
    pub ip: String,
    pub port: i32,
}

pub async fn get_node_ports_by_node_id(
    conn: &mut PgConnection,
    node_id: i32,
) -> Result<Vec<NodePortModel>, sqlx::Error> {
    let node_ports = sqlx::query_as::<_, NodePortModel>("SELECT * FROM node_port WHERE node_id = $1")
        .bind(node_id)
        .fetch_all(&mut *conn)
        .await?;
    Ok(node_ports)
}

pub async fn get_node_ports_by_server_id(
    conn: &mut PgConnection,
    server_id: i32,
) -> Result<Vec<NodePortModel>, sqlx::Error> {
    let node_ports = sqlx::query_as::<_, NodePortModel>("SELECT * FROM node_port WHERE server_id = $1")
        .bind(server_id)
        .fetch_all(&mut *conn)
        .await?;
    Ok(node_ports)
}

pub async fn get_primary_node_port_by_server_id(
    conn: &mut PgConnection,
    server_id: i32,
) -> Result<NodePortModel, sqlx::Error> {
    let node_port = sqlx::query_as::<_, NodePortModel>(
        "SELECT * FROM node_port WHERE server_id = $1 AND is_primary = TRUE",
    )
    .bind(server_id)
    .fetch_one(&mut *conn)
    .await?;
    Ok(node_port)
}

pub async fn get_node_port_by_id(
    conn: &mut PgConnection,
    id: i32,
) -> Result<NodePortModel, sqlx::Error> {
    let node_port = sqlx::query_as::<_, NodePortModel>("SELECT * FROM node_port WHERE id = $1")
        .bind(id)
        .fetch_one(&mut *conn)
        .await?;
    Ok(node_port)
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateNodePort {
    pub ip: String,
    pub port: i32,
}

pub async fn create_node_port(
    conn: &mut PgConnection,
    node_id: i32,
    node_port: CreateNodePort,
) -> Result<NodePortModel, sqlx::Error> {
    let node_port = sqlx::query_as::<_, NodePortModel>(
        "INSERT INTO node_port (node_id, ip, port) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(node_id)
    .bind(node_port.ip)
    .bind(node_port.port)
    .fetch_one(&mut *conn)
    .await?;
    Ok(node_port)
}

pub async fn assign_node_port_to_server(
    conn: &mut PgConnection,
    node_port_id: i32,
    server_id: i32,
    is_primary: bool,
) -> Result<NodePortModel, sqlx::Error> {
    let node_port = sqlx::query_as::<_, NodePortModel>(
        "UPDATE node_port SET server_id = $1, is_primary = $2 WHERE id = $3 RETURNING *",
    )
    .bind(server_id)
    .bind(is_primary)
    .bind(node_port_id)
    .fetch_one(&mut *conn)
    .await?;
    Ok(node_port)
}

pub async fn unassign_node_port_from_server(
    conn: &mut PgConnection,
    node_port_id: i32,
) -> Result<NodePortModel, sqlx::Error> {
    let node_port = sqlx::query_as::<_, NodePortModel>(
        "UPDATE node_port SET server_id = NULL, is_primary = FALSE WHERE id = $1 RETURNING *",
    )
    .bind(node_port_id)
    .fetch_one(&mut *conn)
    .await?;
    Ok(node_port)
}

pub async fn unassign_all_node_port_from_server(
    conn: &mut PgConnection,
    server_id: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE node_port SET server_id = NULL, is_primary = FALSE WHERE server_id = $1")
        .bind(server_id)
        .execute(&mut *conn)
        .await?;
    Ok(())
}

pub async fn delete_node_port(conn: &mut PgConnection, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM node_port WHERE id = $1")
        .bind(id)
        .execute(&mut *conn)
        .await?;
    Ok(())
}

impl From<NodePortModel> for NodePort {
    fn from(node_port: NodePortModel) -> Self {
        NodePort {
            id: node_port.id,
            server_id: node_port.server_id,
            is_primary: node_port.is_primary,
            ip: node_port.ip,
            port: node_port.port,
        }
    }
    
}

impl From<NodePortModel> for ServerNodePort {
    fn from(node_port: NodePortModel) -> Self {
        ServerNodePort {
            id: node_port.id,
            ip: node_port.ip,
            port: node_port.port,
        }
    }
    
}