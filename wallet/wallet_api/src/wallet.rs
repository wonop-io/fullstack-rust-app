use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct Wallet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub encrypted_private_key: String,
    pub address: String,
    pub balance: Decimal,
    pub salt: Vec<u8>,
    pub token_decimals: i16,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletError {
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

impl Default for WalletError {
    fn default() -> Self {
        WalletError {
            message: String::default(),
            timestamp: Utc::now(),
        }
    }
}
