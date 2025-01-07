use app_config::ErrorResponse;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, SaltString},
    Argon2, PasswordVerifier,
};
use auth_password_api::AuthPasswordOnly;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug)]
pub struct AuthPasswordService {
    pool: PgPool,
}

impl AuthPasswordService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn verify_password(
        &self,
        user_id: Uuid,
        password: &str,
    ) -> Result<bool, ErrorResponse> {
        let stored_password = sqlx::query_as::<_, AuthPasswordOnly>(
            "SELECT password FROM auth_passwords_passwords WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|_| ErrorResponse::internal_error())?
        .password;

        let is_valid = PasswordHash::new(&stored_password)
            .and_then(|parsed_hash| {
                Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
            })
            .is_ok();

        Ok(is_valid)
    }

    pub async fn update_password(
        &self,
        user_id: Uuid,
        current_password: &str,
        new_password: &str,
    ) -> Result<(), ErrorResponse> {
        // Verify current password first
        if !self.verify_password(user_id, current_password).await? {
            return Err(ErrorResponse::bad_request("Current password is incorrect"));
        }

        // Generate new password hash
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = Argon2::default()
            .hash_password(new_password.as_bytes(), &salt)
            .map_err(|_| ErrorResponse::internal_error())?
            .to_string();

        // Update password in database
        sqlx::query("UPDATE auth_passwords_passwords SET password = $1 WHERE user_id = $2")
            .bind(hashed_password)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|_| ErrorResponse::internal_error())?;

        Ok(())
    }

    pub async fn create_password(
        &self,
        user_id: Uuid,
        password: &str,
    ) -> Result<(), ErrorResponse> {
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| ErrorResponse::internal_error())?
            .to_string();

        sqlx::query("INSERT INTO auth_passwords_passwords (user_id, password) VALUES ($1, $2)")
            .bind(user_id)
            .bind(hashed_password)
            .execute(&self.pool)
            .await
            .map_err(|_| ErrorResponse::internal_error())?;

        Ok(())
    }

    pub async fn create_reset_token(&self, user_id: Uuid) -> Result<Uuid, ErrorResponse> {
        let token = Uuid::new_v4();
        let expiry = chrono::Utc::now() + chrono::Duration::hours(24);

        sqlx::query(
            "INSERT INTO auth_password_reset_tokens (user_id, token, expires_at, used)
             VALUES ($1, $2, $3, false)",
        )
        .bind(user_id)
        .bind(token)
        .bind(expiry)
        .execute(&self.pool)
        .await
        .map_err(|_| ErrorResponse::internal_error())?;

        Ok(token)
    }

    pub async fn validate_reset_token(&self, token: Uuid) -> Result<Uuid, ErrorResponse> {
        let user_id = sqlx::query_scalar::<_, Uuid>(
            "SELECT user_id FROM auth_password_reset_tokens
             WHERE token = $1
             AND expires_at > CURRENT_TIMESTAMP
             AND used = false",
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| ErrorResponse::internal_error())?
        .ok_or_else(|| ErrorResponse::bad_request("Invalid or expired reset token"))?;

        Ok(user_id)
    }

    pub async fn reset_password(
        &self,
        user_id: Uuid,
        reset_token: Uuid,
        new_password: &str,
    ) -> Result<(), ErrorResponse> {
        // Verify reset token
        let is_valid = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(
                SELECT 1 FROM auth_password_reset_tokens
                WHERE token = $1
                AND expires_at > CURRENT_TIMESTAMP
                AND used = false
            )",
        )
        .bind(reset_token)
        .fetch_one(&self.pool)
        .await
        .map_err(|_| ErrorResponse::internal_error())?;

        if !is_valid {
            return Err(ErrorResponse::bad_request("Invalid or expired reset token"));
        }

        // Generate new password hash
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = Argon2::default()
            .hash_password(new_password.as_bytes(), &salt)
            .map_err(|_| ErrorResponse::internal_error())?
            .to_string();

        // Update password
        sqlx::query("UPDATE auth_passwords_passwords SET password = $1 WHERE user_id = $2")
            .bind(hashed_password)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|_| ErrorResponse::internal_error())?;

        // Mark reset token as used
        sqlx::query("UPDATE auth_password_reset_tokens SET used = true WHERE token = $1")
            .bind(reset_token)
            .execute(&self.pool)
            .await
            .map_err(|_| ErrorResponse::internal_error())?;

        Ok(())
    }
}
