use app_config::ErrorResponse;
use auth_api::User;
use chrono::Utc;
use sqlx::PgPool;
use tower_sessions::Session;
use uuid::Uuid;

use crate::user_session::UserSession;

#[derive(Debug)]
pub struct AuthService {
    pool: PgPool,
}

impl AuthService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn authenticate(&self, user_id: Uuid) -> Result<User, ErrorResponse> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM auth_users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| ErrorResponse::internal_error())?;

        match user {
            Some(user) => Ok(user),
            None => Err(ErrorResponse::bad_request("User not found")),
        }
    }

    pub async fn login(&self, user_id: Uuid, session: &Session) -> Result<User, ErrorResponse> {
        let user = self.authenticate(user_id).await?;
        let user_session = UserSession::new(Some(user.clone()));
        session
            .insert("user_session", user_session)
            .await
            .map_err(|e| {
                log::error!("Failed to create session: {:#?}", e);
                ErrorResponse::internal_error()
            })?;

        Ok(user)
    }

    pub async fn logout(&self, session: &Session) -> Result<(), ErrorResponse> {
        session
            .remove::<UserSession>("user_session")
            .await
            .map_err(|_| {
                log::error!("Failed to remove user_session");

                ErrorResponse::internal_error()
            })?;
        Ok(())
    }

    pub async fn get_user(&self, user_id: Uuid) -> Result<User, ErrorResponse> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM auth_users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| ErrorResponse::internal_error())?;

        match user {
            Some(user) => Ok(user),
            None => Err(ErrorResponse::not_found("User not found")),
        }
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<User, ErrorResponse> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM auth_users WHERE username = $1")
            .bind(username)
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| ErrorResponse::internal_error())?;

        match user {
            Some(user) => Ok(user),
            None => Err(ErrorResponse::not_found("User not found")),
        }
    }

    pub async fn get_or_create_user_by_username(
        &self,
        username: &str,
    ) -> Result<User, ErrorResponse> {
        if let Ok(user) = self.get_user_by_username(username).await {
            return Ok(user);
        }

        let new_user = User {
            id: Uuid::new_v4(),
            username: username.to_string(),
            name: Some(username.to_string()), // Default to username as name
            role: "user".to_string(),         // Default role
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };

        self.create_user(new_user).await
    }

    pub async fn create_user(&self, new_user: User) -> Result<User, ErrorResponse> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO auth_users (id, name, username, role, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        )
        .bind(new_user.id)
        .bind(&new_user.name)
        .bind(&new_user.username)
        .bind(&new_user.role)
        .bind(new_user.created_at)
        .bind(new_user.updated_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            log::error!("Failed to create user: {:#?}", e);
            ErrorResponse::bad_request("User already exists")
        })?;

        Ok(user)
    }

    pub async fn delete_user(&self, user_id: Uuid) -> Result<(), ErrorResponse> {
        let result = sqlx::query("DELETE FROM auth_users WHERE id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|_| ErrorResponse::bad_request("Failed to delete user"))?;

        if result.rows_affected() == 0 {
            return Err(ErrorResponse::not_found("User not found"));
        }

        Ok(())
    }

    pub async fn update_user(&self, user_id: Uuid, payload: User) -> Result<User, ErrorResponse> {
        let user = sqlx::query_as::<_, User>(
            "UPDATE auth_users SET name = $1, username = $2, role = $3, updated_at = $4 WHERE id = $5 RETURNING *",
        )
        .bind(&payload.name)
        .bind(&payload.username)
        .bind(&payload.role)
        .bind(payload.updated_at)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| ErrorResponse::bad_request("Failed to update user"))?;

        match user {
            Some(user) => Ok(user),
            None => Err(ErrorResponse::not_found("User not found")),
        }
    }

    pub async fn verify_credentials(&self, username: String) -> Result<User, ErrorResponse> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM auth_users WHERE username = $1")
            .bind(username)
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| ErrorResponse::internal_error())?;

        match user {
            Some(user) => Ok(user),
            None => Err(ErrorResponse::bad_request("Invalid credentials")),
        }
    }
}
