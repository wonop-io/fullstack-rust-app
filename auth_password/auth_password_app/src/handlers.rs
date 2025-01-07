use std::sync::Arc;

use app_config::ErrorResponse;
use auth_api::User;
use auth_app::{user_session::UserSession, AuthService};
use auth_password_api::{
    AuthLoginForm, AuthLoginRegisterResponse, AuthLoginSignupForm, AuthLoginUserResponse,
};
use axum::{
    response::{IntoResponse, Response},
    routing::post,
    Extension, Json, Router,
};
use tower_sessions::Session;
use uuid::Uuid;
use validator::Validate;

use crate::AuthPasswordService;

async fn register_user_handler(
    Extension(auth_service): Extension<Arc<AuthService>>,
    Extension(password_service): Extension<Arc<AuthPasswordService>>,
    Json(body): Json<AuthLoginSignupForm>,
) -> Result<Json<AuthLoginRegisterResponse>, Response> {
    if let Err(error) = body.validate() {
        log::error!("Validation error in register_user_handler: {:#?}", error);
        return Err(ErrorResponse::bad_request("Invalid data submitted.").into_response());
    }

    let user = User {
        id: Uuid::new_v4(),
        name: Some(body.name),
        username: body.email.to_lowercase(),
        role: "user".to_string(),
        created_at: Some(chrono::Utc::now()),
        updated_at: Some(chrono::Utc::now()),
    };

    let user = auth_service.create_user(user).await.map_err(|e| {
        log::error!("Failed to create user: {:#?}", e);
        e.into_response()
    })?;

    password_service
        .create_password(user.id, &body.password)
        .await
        .map_err(|e| {
            log::error!("Failed to create password: {:#?}", e);
            e.into_response()
        })?;

    Ok(Json(AuthLoginRegisterResponse {
        status: "success".to_string(),
        data: user,
    }))
}

async fn login_user_handler(
    Extension(auth_service): Extension<Arc<AuthService>>,
    Extension(password_service): Extension<Arc<AuthPasswordService>>,
    session: Session,
    Json(body): Json<AuthLoginForm>,
) -> Result<Json<AuthLoginUserResponse>, Response> {
    if let Err(error) = body.validate() {
        log::error!("Validation error in login_user_handler: {:#?}", error);
        return Err(ErrorResponse::bad_request("Invalid data submitted.").into_response());
    }

    let user = auth_service
        .verify_credentials(body.email)
        .await
        .map_err(|e| {
            log::error!("Invalid credentials");
            e.into_response()
        })?;

    let is_valid = password_service
        .verify_password(user.id, &body.password)
        .await
        .map_err(|e| {
            log::error!("Failed to verify password: {:#?}", e);
            e.into_response()
        })?;

    if !is_valid {
        log::error!("Invalid password attempt for user: {}", user.username);
        return Err(ErrorResponse::bad_request("Invalid username or password").into_response());
    }

    let user_session = UserSession::new(Some(user.clone()));
    session
        .insert("user_session", user_session)
        .await
        .map_err(|e| {
            log::error!("Failed to create session: {:#?}", e);
            ErrorResponse::internal_error().into_response()
        })?;

    Ok(Json(AuthLoginUserResponse {
        status: "success".to_string(),
        data: user,
    }))
}

pub fn app() -> Router {
    Router::new()
        .route("/v1/auth/register", post(register_user_handler))
        .route("/v1/auth/login", post(login_user_handler))
}
