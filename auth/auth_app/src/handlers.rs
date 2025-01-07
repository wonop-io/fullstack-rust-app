// Boilerplate code by Wonop ApS.

use std::sync::Arc;

use app_config::ErrorResponse;
use auth_api::{User, UserLogoutResponse};
use axum::{routing::get, Extension, Json, Router};
use tower_sessions::Session;

use crate::{services::AuthService, user_guard::UserAuthenticatedGuard};

async fn my_user_handler(
    Extension(auth_service): Extension<Arc<AuthService>>,
    guard: UserAuthenticatedGuard,
) -> Result<Json<User>, ErrorResponse> {
    let user = auth_service.authenticate(guard.user.id).await?;
    Ok(Json(user))
}

async fn logout_handler(
    Extension(auth_service): Extension<Arc<AuthService>>,
    session: Session,
) -> Result<Json<UserLogoutResponse>, ErrorResponse> {
    auth_service.logout(&session).await?;

    Ok(Json(UserLogoutResponse {
        status: "success".to_string(),
    }))
}

pub fn app() -> Router {
    Router::new()
        .route("/v1/auth/my_user", get(my_user_handler))
        .route("/v1/auth/logout", get(logout_handler))
}
