use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

pub mod api;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Node {
    pub node_id: Uuid,
    pub node_name: String,
    pub cpu: i32,
    pub memory: i32,
    pub disk: i32,
    pub api_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewNode {
    pub node_name: String,
    pub cpu: i32,
    pub memory: i32,
    pub disk: i32,
    pub api_url: String,
}

pub async fn list(pool: &sqlx::PgPool) -> Result<Vec<Node>, sqlx::Error> {
    let nodes = sqlx::query_as::<_, Node>(
        r#"
        SELECT node_id, node_name, cpu, memory, disk, api_url
        FROM nodes
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(nodes)
}

pub async fn find_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<Option<Node>, sqlx::Error> {
    let node = sqlx::query_as::<_, Node>(
        r#"
        SELECT node_id, node_name, cpu, memory, disk, api_url
        FROM nodes
        WHERE node_id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(node)
}

pub async fn create(pool: &sqlx::PgPool, new_node: NewNode) -> Result<Node, sqlx::Error> {
    let node = sqlx::query_as::<_, Node>(
        r#"
        INSERT INTO nodes (node_name, cpu, memory, disk, api_url)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING node_id, node_name, cpu, memory, disk, api_url
        "#,
    )
    .bind(new_node.node_name)
    .bind(new_node.cpu)
    .bind(new_node.memory)
    .bind(new_node.disk)
    .bind(new_node.api_url)
    .fetch_one(pool)
    .await?;
    Ok(node)
}

pub async fn update(pool: &sqlx::PgPool, node: Node) -> Result<Node, sqlx::Error> {
    let node = sqlx::query_as::<_, Node>(
        r#"
        UPDATE nodes
        SET node_name = $1, cpu = $2, memory = $3, disk = $4, api_url = $5
        WHERE node_id = $6
        RETURNING node_id, node_name, cpu, memory, disk, api_url
        "#,
    )
    .bind(node.node_name)
    .bind(node.cpu)
    .bind(node.memory)
    .bind(node.disk)
    .bind(node.api_url)
    .bind(node.node_id)
    .fetch_one(pool)
    .await?;
    Ok(node)
}

pub async fn delete(pool: &sqlx::PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM nodes
        WHERE node_id = $1
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}
