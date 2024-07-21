use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use password_auth::{verify_password, ParseError, VerifyError};
use sqlx::PgPool;
use tokio::task;

use super::User;

impl AuthUser for User {
    type Id = uuid::Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.pw_hash.as_bytes()
    }
}

pub struct Creds {
    pub username: String,
    pub password: String,
}

#[derive(Clone)]
pub struct AuthBackend {
    pool: PgPool,
}

impl AuthBackend {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),
    #[error(transparent)]
    ParseError(#[from] ParseError),
}

#[async_trait]
impl AuthnBackend for AuthBackend {
    type User = User;
    type Credentials = Creds;
    type Error = Error;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let query = "SELECT * FROM users WHERE name = $1";
        let user = sqlx::query_as::<_, User>(query)
            .bind(creds.username)
            .fetch_optional(&self.pool)
            .await?;
        if let Some(u) = user.clone() {
            match task::spawn_blocking(move || verify_password(creds.password, &u.pw_hash)).await? {
                Ok(_) => Ok(user),
                Err(VerifyError::PasswordInvalid) => Ok(None),
                Err(VerifyError::Parse(e)) => Err(e.into()),
            }
        } else {
            Ok(None)
        }
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let query = "SELECT * FROM users WHERE id = $1";
        sqlx::query_as::<_, User>(query)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(Into::into)
    }
}
