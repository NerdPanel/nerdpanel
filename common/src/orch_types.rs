use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Node {
    pub id: i32,
    pub name: String,
    pub fqdn: String,
    pub ports: Vec<NodePort>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct NodePort {
    pub id: i32,
    pub server_id: Option<i32>,
    pub is_primary: bool,
    pub ip: String,
    pub port: i32,
}

#[derive(sqlx::Type, Serialize, Deserialize, ToSchema)]
#[sqlx(type_name = "env_var_type")]
pub struct EnvVar {
    pub key: String,
    pub value: String,
}

#[derive(sqlx::Type, Serialize, Deserialize, ToSchema)]
#[sqlx(type_name = "image_type")]
pub struct Image {
    pub name: String,
    pub tag: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, ToSchema)]
pub struct Pod {
    pub id: i32,
    pub name: String,
    pub images: Vec<Image>,
    pub startup_command: String,
    pub installer_image: String,
    pub env_vars: Vec<EnvVar>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Server {
    pub id: i32,
    pub name: String,
    pub owner_id: i32,
    pub node_id: i32,

    pub cpu_limit: Option<i32>,
    pub memory_limit: Option<i32>,
    pub disk_limit: Option<i32>,

    pub primary_port: ServerNodePort,
    pub additional_ports: Vec<ServerNodePort>,

    pub pod_id: i32,
    pub image: String,
    pub startup_command: String,
    pub env_vars: Vec<EnvVar>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ServerNodePort {
    pub id: i32,
    pub ip: String,
    pub port: i32,
}
