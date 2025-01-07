// Boilerplate code by Wonop ApS.

use auth_api::User;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Debug, Clone, Serialize, Deserialize)]
pub struct AuthLoginRegisterResponse {
    pub status: String,
    pub data: User,
}

#[derive(Validate, Debug, Clone, Serialize, Deserialize)]
pub struct AuthLoginUserResponse {
    pub status: String,
    pub data: User,
}

#[derive(Validate, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct AuthLoginPassword {
    pub user_id: uuid::Uuid,
    pub password: String,
}

#[derive(Validate, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct AuthPasswordOnly {
    pub password: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct AuthLoginForm {
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub password: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct AuthLoginSignupForm {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub password: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct AuthForgotPasswordForm {
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct AuthResetPasswordForm {
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub one_time_password: String,
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub password: String,
    #[validate(
        length(min = 1, message = "Confirm password is required"),
        must_match(other = "password", message = "Passwords do not match")
    )]
    pub confirm_password: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct UpdatePasswordForm {
    #[validate(
        length(min = 1, message = "Current password is required"),
        length(min = 6, message = "Current password must be at least 6 characters")
    )]
    pub current_password: String,
    #[validate(
        length(min = 1, message = "New password is required"),
        length(min = 6, message = "New password must be at least 6 characters")
    )]
    pub new_password: String,
    #[validate(
        length(min = 1, message = "Confirm new password is required"),
        must_match(other = "new_password", message = "New passwords do not match")
    )]
    pub confirm_new_password: String,
}
