use chrono::{DateTime, Utc};
use ethers::types::U256;
use serde::{Deserialize, Serialize};

// Define Transaction struct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct Transaction {
    pub id: String,
    pub amount: U256,
    pub recipient: String,
    pub date: DateTime<Utc>,
}
