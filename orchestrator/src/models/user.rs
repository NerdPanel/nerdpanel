use std::fmt::Debug;

use password_auth::generate_hash;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(sqlx::FromRow, Clone, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub pw_hash: String,
    pub email: String,
    pub staff: bool,
}

impl Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("pw_hash", &"********")
            .field("email", &self.email)
            .field("staff", &self.staff)
            .finish()
    }
}

pub async fn get_users(conn: &mut sqlx::PgConnection) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as("SELECT * FROM users")
        .fetch_all(conn)
        .await?;

    Ok(users)
}

pub async fn get_user_by_id(
    conn: &mut sqlx::PgConnection,
    id: i32,
) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(conn)
        .await?;

    Ok(user)
}

pub async fn get_user_by_username(
    conn: &mut sqlx::PgConnection,
    username: &str,
) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as("SELECT * FROM users WHERE username = $1")
        .bind(username)
        .fetch_optional(conn)
        .await?;

    Ok(user)
}

#[derive(Deserialize, ToSchema)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

pub async fn create_user(
    conn: &mut sqlx::PgConnection,
    user: CreateUser,
) -> Result<User, sqlx::Error> {
    let pw_hash = generate_hash(user.password);
    let user = sqlx::query_as("INSERT INTO users (username, pw_hash, email, staff) VALUES ($1, $2, $3, FALSE) RETURNING *")
        .bind(user.username)
        .bind(pw_hash)
        .bind(user.email)
        .fetch_one(conn)
        .await?;

    Ok(user)
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateUser {
    pub id: i32,
    pub email: String,
    pub staff: bool,
}

pub async fn update_user(
    conn: &mut sqlx::PgConnection,
    user: UpdateUser,
) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as("UPDATE users SET email = $1, staff = $2 WHERE id = $3 RETURNING *")
        .bind(user.email)
        .bind(user.staff)
        .bind(user.id)
        .fetch_one(conn)
        .await?;

    Ok(user)
}

pub async fn update_user_password(
    conn: &mut sqlx::PgConnection,
    id: i32,
    password: &str,
) -> Result<(), sqlx::Error> {
    let pw_hash = generate_hash(password);
    sqlx::query("UPDATE users SET pw_hash = $1 WHERE id = $2")
        .bind(pw_hash)
        .bind(id)
        .execute(conn)
        .await?;

    Ok(())
}

pub async fn delete_user(conn: &mut sqlx::PgConnection, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(conn)
        .await?;

    Ok(())
}
