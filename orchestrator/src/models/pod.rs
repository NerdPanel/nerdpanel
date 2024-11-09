use common::orch_types::{EnvVar, Image, Pod};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(sqlx::FromRow, Serialize, Deserialize, ToSchema)]
pub struct PodModel {
    pub id: i32,
    pub name: String,
    pub images: Vec<Image>,
    pub startup_command: String,
    pub installer_image: String,
    pub env_vars: Vec<EnvVar>,
}

pub async fn get_pods(conn: &mut sqlx::PgConnection) -> Result<Vec<PodModel>, sqlx::Error> {
    let pods = sqlx::query_as::<_, PodModel>("SELECT * FROM pod")
        .fetch_all(&mut *conn)
        .await?;
    Ok(pods)
}

pub async fn get_pod_by_id(
    conn: &mut sqlx::PgConnection,
    id: i32,
) -> Result<PodModel, sqlx::Error> {
    let pod = sqlx::query_as::<_, PodModel>("SELECT * FROM pod WHERE id = $1")
        .bind(id)
        .fetch_one(&mut *conn)
        .await?;
    Ok(pod)
}

pub async fn get_pod_for_server(
    conn: &mut sqlx::PgConnection,
    server_id: i32,
) -> Result<PodModel, sqlx::Error> {
    let pod = sqlx::query_as::<_, PodModel>(
        "SELECT * FROM pod WHERE id = (SELECT pod_id FROM server WHERE id = $1)",
    )
    .bind(server_id)
    .fetch_one(&mut *conn)
    .await?;
    Ok(pod)
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreatePod {
    pub name: String,
    pub images: Vec<Image>,
    pub startup_command: String,
    pub installer_image: String,
    pub env_vars: Vec<EnvVar>,
}

pub async fn create_pod(
    conn: &mut sqlx::PgConnection,
    pod: CreatePod,
) -> Result<PodModel, sqlx::Error> {
    let pod = sqlx::query_as::<_, PodModel>(
        "INSERT INTO pod (name, images, startup_command, installer_image, env_vars) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(pod.name)
    .bind(pod.images)
    .bind(pod.startup_command)
    .bind(pod.installer_image)
    .bind(pod.env_vars)
    .fetch_one(&mut *conn)
    .await?;
    Ok(pod)
}

pub async fn update_pod(
    conn: &mut sqlx::PgConnection,
    pod: PodModel,
) -> Result<PodModel, sqlx::Error> {
    let pod = sqlx::query_as::<_, PodModel>(
        "UPDATE pod SET name = $1, images = $2, startup_command = $3, installer_image = $4, env_vars = $5 WHERE id = $6 RETURNING *"
    )
    .bind(pod.name)
    .bind(pod.images)
    .bind(pod.startup_command)
    .bind(pod.installer_image)
    .bind(pod.env_vars)
    .bind(pod.id)
    .fetch_one(&mut *conn)
    .await?;
    Ok(pod)
}

pub async fn delete_pod(conn: &mut sqlx::PgConnection, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM pod WHERE id = $1")
        .bind(id)
        .execute(&mut *conn)
        .await?;
    Ok(())
}

impl From<PodModel> for Pod {
    fn from(pod: PodModel) -> Self {
        Pod {
            id: pod.id,
            name: pod.name,
            images: pod.images,
            startup_command: pod.startup_command,
            installer_image: pod.installer_image,
            env_vars: pod.env_vars,
        }
    }
}
