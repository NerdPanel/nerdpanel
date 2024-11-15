use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use password_auth::{verify_password, VerifyError};
use serde::Deserialize;
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::{
    models::user::{self, User},
    utils::AppError,
};

impl AuthUser for User {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.pw_hash.as_bytes()
    }
}

#[derive(Clone)]
pub struct AuthBackend {
    pub db: PgPool,
}

#[derive(Deserialize, ToSchema)]
pub struct Creds {
    pub username: String,
    pub password: String,
}

#[async_trait]
impl AuthnBackend for AuthBackend {
    type User = User;
    type Credentials = Creds;
    type Error = AppError;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let conn = &mut self.db.acquire().await?;
        let user = user::get_user_by_username(conn, &creds.username).await?;
        let user = match user {
            Some(user) => user,
            None => return Ok(None),
        };
        match verify_password(&creds.password, &user.pw_hash) {
            Ok(_) => Ok(Some(user)),
            Err(e) => {
                if let VerifyError::PasswordInvalid = e {
                    Ok(None)
                } else {
                    tracing::error!("Hash error: {:?}", e);
                    Err(AppError::HashError)
                }
            }
        }
    }

    async fn get_user(&self, id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let conn = &mut self.db.acquire().await?;
        let user = user::get_user_by_id(conn, *id).await?;
        Ok(user)
    }
}

pub type AuthSession = axum_login::AuthSession<AuthBackend>;
