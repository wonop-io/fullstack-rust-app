use app_config::ErrorResponse;
use auth_api::User;
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use log::info;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

use crate::user_session::UserSession;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAuthenticatedGuard {
    pub user: User,
}

#[async_trait]
impl<S> FromRequestParts<S> for UserAuthenticatedGuard
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<ErrorResponse>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        info!("UserAuthenticatedGuard::from_request_parts");
        let session = Session::from_request_parts(parts, state)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse::internal_error()),
                )
            })?;

        let user_session: Option<UserSession> =
            session.get("user_session").await.map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse::internal_error()),
                )
            })?;

        info!("user_session: {:?}", user_session);
        match user_session {
            Some(user_session) => match user_session.user {
                Some(user) => Ok(UserAuthenticatedGuard { user }),
                None => {
                    info!("UserAuthenticatedGuard::from_request_parts: user_session.user is None");
                    Err((
                        StatusCode::UNAUTHORIZED,
                        Json(ErrorResponse::unauthorized()),
                    ))
                }
            },
            None => {
                info!("UserAuthenticatedGuard::from_request_parts: user_session is None");
                Err((
                    StatusCode::UNAUTHORIZED,
                    Json(ErrorResponse::unauthorized()),
                ))
            }
        }
    }
}

impl UserAuthenticatedGuard {
    pub fn require_role(&self, role: String) -> Result<(), Response> {
        if self.user.role == role {
            Ok(())
        } else {
            Err((
                StatusCode::FORBIDDEN,
                Json(ErrorResponse::insufficient_permissions()),
            )
                .into_response())
        }
    }

    pub fn require_any_role(&self, roles: Vec<String>) -> Result<(), Response> {
        if roles.contains(&self.user.role) {
            Ok(())
        } else {
            Err((
                StatusCode::FORBIDDEN,
                Json(ErrorResponse::insufficient_permissions()),
            )
                .into_response())
        }
    }

    pub fn require_staff(&self) -> Result<(), Response> {
        println!("UserAuthenticatedGuard::require_staff");
        println!(
            "UserAuthenticatedGuard::require_staff: user.role = {}",
            self.user.role
        );
        if self.user.role == "staff" || self.user.role == "admin" {
            Ok(())
        } else {
            Err((
                StatusCode::FORBIDDEN,
                Json(ErrorResponse::insufficient_permissions()),
            )
                .into_response())
        }
    }
}
