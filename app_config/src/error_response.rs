#[cfg(feature = "backend")]
use axum::http::StatusCode;
#[cfg(feature = "backend")]
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
    #[serde(skip)]
    #[cfg(feature = "backend")]
    pub status_code: StatusCode,
    #[serde(skip)]
    #[cfg(not(feature = "backend"))]
    pub status_code: i32,
}

#[cfg(feature = "backend")]
impl axum::response::IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let code = self.status_code;
        let body = axum::Json(self);
        (code, body).into_response()
    }
}

#[cfg(feature = "backend")]
impl From<ErrorResponse> for axum::response::Response {
    fn from(error: ErrorResponse) -> Self {
        error.into_response()
    }
}

impl ErrorResponse {
    #[cfg(feature = "backend")]
    pub fn unauthorized() -> Self {
        Self {
            status: "fail".to_string(),
            message: "You are not logged in.".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        }
    }

    #[cfg(not(feature = "backend"))]
    pub fn unauthorized() -> Self {
        Self {
            status: "fail".to_string(),
            message: "You are not logged in.".to_string(),
            status_code: 401,
        }
    }

    #[cfg(feature = "backend")]
    pub fn internal_error() -> Self {
        Self {
            status: "fail".to_string(),
            message: "Internal error".to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    #[cfg(not(feature = "backend"))]
    pub fn internal_error() -> Self {
        Self {
            status: "fail".to_string(),
            message: "Internal error".to_string(),
            status_code: 500,
        }
    }

    #[cfg(feature = "backend")]
    pub fn insufficient_permissions() -> Self {
        Self {
            status: "fail".to_string(),
            message: "You do not have sufficient permissions to access this resource".to_string(),
            status_code: StatusCode::FORBIDDEN,
        }
    }

    #[cfg(not(feature = "backend"))]
    pub fn insufficient_permissions() -> Self {
        Self {
            status: "fail".to_string(),
            message: "You do not have sufficient permissions to access this resource".to_string(),
            status_code: 403,
        }
    }

    #[cfg(feature = "backend")]
    pub fn bad_request(message: &str) -> Self {
        Self {
            status: "fail".to_string(),
            message: message.to_string(),
            status_code: StatusCode::BAD_REQUEST,
        }
    }

    #[cfg(not(feature = "backend"))]
    pub fn bad_request(message: &str) -> Self {
        Self {
            status: "fail".to_string(),
            message: message.to_string(),
            status_code: 400,
        }
    }

    #[cfg(feature = "backend")]
    pub fn not_found(message: &str) -> Self {
        Self {
            status: "fail".to_string(),
            message: message.to_string(),
            status_code: StatusCode::NOT_FOUND,
        }
    }

    #[cfg(not(feature = "backend"))]
    pub fn not_found(message: &str) -> Self {
        Self {
            status: "fail".to_string(),
            message: message.to_string(),
            status_code: 404,
        }
    }

    #[cfg(feature = "backend")]
    pub fn timeout(message: &str) -> Self {
        Self {
            status: "fail".to_string(),
            message: message.to_string(),
            status_code: StatusCode::REQUEST_TIMEOUT,
        }
    }

    #[cfg(not(feature = "backend"))]
    pub fn timeout(message: &str) -> Self {
        Self {
            status: "fail".to_string(),
            message: message.to_string(),
            status_code: 408,
        }
    }

    #[cfg(feature = "backend")]
    pub fn site_is_overloaded() -> Self {
        Self {
            status: "fail".to_string(),
            message: "The site is temporarily overloaded. Please try again later.".to_string(),
            status_code: StatusCode::from_u16(529).unwrap(),
        }
    }

    #[cfg(not(feature = "backend"))]
    pub fn site_is_overloaded() -> Self {
        Self {
            status: "fail".to_string(),
            message: "The site is temporarily overloaded. Please try again later.".to_string(),
            status_code: 529,
        }
    }

    #[cfg(feature = "backend")]
    pub fn origin_is_unreachable() -> Self {
        Self {
            status: "fail".to_string(),
            message: "Origin is unreachable. Please try again later.".to_string(),
            status_code: StatusCode::from_u16(523).unwrap(),
        }
    }

    #[cfg(not(feature = "backend"))]
    pub fn origin_is_unreachable() -> Self {
        Self {
            status: "fail".to_string(),
            message: "Origin is unreachable. Please try again later.".to_string(),
            status_code: 523,
        }
    }
}
