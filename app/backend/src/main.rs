mod connect_db;

use std::sync::Arc;

use auth_app::{app as auth_app, AuthService, PostgresStore};
use auth_password_app::{app as auth_password_app, AuthPasswordService};
use axum::{routing::get, Extension, Json, Router};
use axum_extra::extract::cookie::{Key, SameSite};
use base64::Engine;
use connect_db::connect_to_datebase;
use dotenvy::dotenv;
use log::info;
use serde_json::json;
use tower_http::cors::CorsLayer;
use tower_sessions::{Expiry, SessionManagerLayer};
use wallet_app::{app as wallet_app, WalletService};

async fn healthcheck() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok"
    }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "trace");
    }

    dotenv().ok();
    env_logger::init();

    info!("🚀 Server for Wallet backend lifting off!");
    let db = match connect_to_datebase().await {
        Ok(db) => db,
        Err(e) => {
            panic!("Failed to connect to database: {}", e)
        }
    };

    let server_secret = std::env::var("SERVER_SECRET").unwrap_or_else(|_| "".into());
    let decoded_secret = base64::engine::general_purpose::STANDARD
        .decode(&server_secret)
        .expect("Failed to decode SERVER_SECRET from base64");
    let key = Key::from(&decoded_secret);

    let session_store = PostgresStore::new(db.clone());
    session_store.migrate().await?;

    let _deletion_task = tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
    );

    let domain = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".into());
    let domain_parts: Vec<&str> = domain.split('.').collect();
    let domain_group = if domain_parts.len() > 1 {
        format!(".{}", domain_parts[1..].join("."))
    } else {
        domain
    };

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(true)
        .with_domain(domain_group)
        .with_same_site(SameSite::None)
        .with_expiry(Expiry::OnInactivity(time::Duration::seconds(
            24 * 60 * 60 * 365,
        )))
        .with_signed(key);

    let auth_service = Arc::new(AuthService::new(db.clone()));
    let auth_password_service = Arc::new(AuthPasswordService::new(db.clone()));
    let wallet_service = Arc::new(WalletService::new(db.clone()));
    let app = Router::new()
        .route("/api/v1/health", get(healthcheck))
        .nest("/api", auth_app())
        .nest("/api", auth_password_app())
        .merge(wallet_app())
        .layer(CorsLayer::very_permissive())
        .layer(session_layer)
        .layer(Extension(auth_service))
        .layer(Extension(auth_password_service))
        .layer(Extension(wallet_service))
        .layer(Extension(db));

    let addr = "0.0.0.0:8000";

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!(
        "🚀🚀 Wallet backend is off the ground and served at {}",
        addr
    );

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
