use std::fmt::Debug;

use chrono::{DateTime, Local};
use password_auth::generate_hash;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use tokio::task;
use uuid::Uuid;

pub mod api;
pub mod auth;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),
}

#[derive(Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub pw_hash: String,
    pub created_at: DateTime<Local>,
}

impl Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("email", &self.email)
            .field("pw_hash", &"********")
            .field("created_at", &self.created_at)
            .finish()
    }
}

#[derive(Deserialize, Serialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

// TODO better name :pray:
#[derive(Deserialize, Serialize)]
pub struct FUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

impl From<User> for FUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
        }
    }
}

pub async fn list(pool: &PgPool) -> Result<Vec<FUser>, Error> {
    let query = "SELECT id, name, email, pw_hash, created_at FROM users";
    sqlx::query_as::<_, User>(query)
        .fetch_all(pool)
        .await
        .map_err(Into::into)
        .map(|users| users.into_iter().map(FUser::from).collect())
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<FUser>, Error> {
    let query = "SELECT id, name, email, pw_hash, created_at FROM users WHERE id = $1";
    sqlx::query_as::<_, User>(query)
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(Into::into)
        .map(|user| user.map(FUser::from))
}

// TODO - Gracefully handle duplicate name or email
pub async fn create(pool: &PgPool, new_user: NewUser) -> Result<FUser, Error> {
    let query = "INSERT INTO users (name, email, pw_hash) VALUES ($1, $2, $3) RETURNING id, name, email, pw_hash, created_at";
    let pw_hash = task::spawn_blocking(move || generate_hash(new_user.password)).await?;
    sqlx::query_as::<_, User>(query)
        .bind(new_user.name)
        .bind(new_user.email)
        .bind(pw_hash)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
        .map(FUser::from)
}

pub async fn update(pool: &PgPool, user: FUser) -> Result<FUser, Error> {
    let query = "UPDATE users SET name = $1, email = $2, WHERE id = $3 RETURNING id, name, email, pw_hash, created_at";
    sqlx::query_as::<_, User>(query)
        .bind(user.name)
        .bind(user.email)
        .bind(user.id)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
        .map(FUser::from)
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), Error> {
    let query = "DELETE FROM users WHERE id = $1";
    sqlx::query(query)
        .bind(id)
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(Into::into)
}
