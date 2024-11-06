use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(sqlx::FromRow, Serialize, Deserialize, ToSchema)]
pub struct Node {
    pub id: i32,
    pub name: String,
    pub fqdn: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, ToSchema)]
pub struct Server {
    pub id: i32,
    pub name: String,
    pub node_id: i32,
    pub ip: String,
    pub port: i32,
}
