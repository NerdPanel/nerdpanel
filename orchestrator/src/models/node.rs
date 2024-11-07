use sqlx::PgConnection;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(sqlx::FromRow, Serialize, Deserialize, ToSchema)]
pub struct NodeModel {
    pub id: i32,
    pub name: String,
    pub fqdn: String,
}

pub async fn get_nodes(conn: &mut PgConnection) -> Result<Vec<NodeModel>, sqlx::Error> {
    let nodes = sqlx::query_as::<_, NodeModel>("SELECT * FROM node")
        .fetch_all(&mut *conn)
        .await?;
    Ok(nodes)
}

pub async fn get_node_by_id(conn: &mut PgConnection, id: i32) -> Result<NodeModel, sqlx::Error> {
    let node = sqlx::query_as::<_, NodeModel>("SELECT * FROM node WHERE id = $1")
        .bind(id)
        .fetch_one(&mut *conn)
        .await?;
    Ok(node)
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateNode {
    pub name: String,
    pub fqdn: String,
}

pub async fn create_node(
    conn: &mut PgConnection,
    node: CreateNode,
) -> Result<NodeModel, sqlx::Error> {
    let node =
        sqlx::query_as::<_, NodeModel>("INSERT INTO node (name, fqdn) VALUES ($1, $2) RETURNING *")
            .bind(node.name)
            .bind(node.fqdn)
            .fetch_one(&mut *conn)
            .await?;
    Ok(node)
}

pub async fn update_node(
    conn: &mut PgConnection,
    node: NodeModel,
) -> Result<NodeModel, sqlx::Error> {
    let node = sqlx::query_as::<_, NodeModel>(
        "UPDATE node SET name = $1, fqdn = $2 WHERE id = $3 RETURNING *",
    )
    .bind(node.name)
    .bind(node.fqdn)
    .bind(node.id)
    .fetch_one(&mut *conn)
    .await?;
    Ok(node)
}

pub async fn delete_node(conn: &mut PgConnection, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM node WHERE id = $1")
        .bind(id)
        .execute(&mut *conn)
        .await?;
    Ok(())
}
