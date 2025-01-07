use std::sync::Arc;

use auth_app::user_guard::UserAuthenticatedGuard;
use axum::{
    response::{IntoResponse, Response},
    Extension, Json, Router,
};
use wallet_api::Wallet;

use crate::WalletService;

async fn get_wallet_details(
    Extension(wallet_service): Extension<Arc<WalletService>>,
    user_guard: UserAuthenticatedGuard,
) -> Result<Json<Wallet>, Response> {
    let wallet = wallet_service.get_wallet(user_guard.user.id).await;

    match wallet {
        Ok(wallet) => Ok(Json(wallet)),
        Err(e) => Err(e.into_response()),
    }
}

async fn update_wallet_details(
    Extension(wallet_service): Extension<Arc<WalletService>>,
    user_guard: UserAuthenticatedGuard,
    Json(wallet): Json<Wallet>,
) -> Result<Json<Wallet>, Response> {
    match wallet_service
        .update_wallet(user_guard.user.id, wallet)
        .await
    {
        Ok(updated_wallet) => Ok(Json(updated_wallet)),
        Err(e) => Err(e.into_response()),
    }
}

pub fn app() -> Router {
    Router::new()
        .route("/api/v1/wallet", axum::routing::get(get_wallet_details))
        .route("/api/v1/wallet", axum::routing::put(update_wallet_details))
}
