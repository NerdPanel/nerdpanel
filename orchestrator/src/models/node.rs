use common::models::Node;
use sqlx::PgConnection;

pub async fn get_nodes(conn: &mut PgConnection) -> Result<Vec<Node>, sqlx::Error> {
    let nodes = sqlx::query_as::<_, Node>("SELECT * FROM node")
        .fetch_all(&mut *conn)
        .await?;
    Ok(nodes)
}

pub async fn get_node_by_id(conn: &mut PgConnection, id: i32) -> Result<Node, sqlx::Error> {
    let node = sqlx::query_as::<_, Node>("SELECT * FROM node WHERE id = $1")
        .bind(id)
        .fetch_one(&mut *conn)
        .await?;
    Ok(node)
}

pub async fn create_node(conn: &mut PgConnection, node: Node) -> Result<Node, sqlx::Error> {
    let node =
        sqlx::query_as::<_, Node>("INSERT INTO nodes (name, fqdn) VALUES ($1, $2) RETURNING *")
            .bind(node.name)
            .bind(node.fqdn)
            .fetch_one(&mut *conn)
            .await?;
    Ok(node)
}

pub async fn update_node(conn: &mut PgConnection, node: Node) -> Result<Node, sqlx::Error> {
    let node = sqlx::query_as::<_, Node>(
        "UPDATE nodes SET name = $1, fqdn = $2 WHERE id = $3 RETURNING *",
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
