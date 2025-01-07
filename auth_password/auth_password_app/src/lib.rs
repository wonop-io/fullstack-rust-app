// Boilerplate code by Wonop ApS.

mod handlers;
mod services;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use auth_api::User;
pub use handlers::app;
pub use services::AuthPasswordService;
use sqlx::PgPool;
pub async fn setup_admin(db: &PgPool, username: &str, password: &str) -> Result<(), sqlx::Error> {
    // Check if the user already exists
    let user_exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM auth_users WHERE username = $1)")
            .bind(username)
            .fetch_one(db)
            .await?;
    // Hash the password
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string();

    if !user_exists {
        // Create the user
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO auth_users (name, username, role) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(username)
        .bind(username.to_lowercase())
        .bind("admin")
        .fetch_one(db)
        .await?;

        // Store the password
        sqlx::query("INSERT INTO auth_passwords_passwords (user_id, password) VALUES ($1, $2)")
            .bind(user.id)
            .bind(hashed_password)
            .execute(db)
            .await?;
    }

    Ok(())
}
