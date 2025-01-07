use std::fmt;

use ethers::types::{H256, U256};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TransactionStatus {
    None,
    Preparing,
    Building,
    Signing { tx: U256 },
    Submitting { tx_hash: Option<H256> },
    Confirmed { tx_hash: H256, block_number: u64 },
    Failed { error: String },
}

impl TransactionStatus {
    pub fn is_complete(&self) -> bool {
        matches!(
            self,
            TransactionStatus::Confirmed { .. } | TransactionStatus::Failed { .. }
        )
    }

    pub fn is_error(&self) -> bool {
        matches!(self, TransactionStatus::Failed { .. })
    }

    pub fn get_error(&self) -> Option<&str> {
        match self {
            TransactionStatus::Failed { error } => Some(error),
            _ => None,
        }
    }

    pub fn get_tx_hash(&self) -> Option<H256> {
        match self {
            TransactionStatus::Submitting { tx_hash } => *tx_hash,
            TransactionStatus::Confirmed { tx_hash, .. } => Some(*tx_hash),
            _ => None,
        }
    }
}

impl Default for TransactionStatus {
    fn default() -> Self {
        Self::None
    }
}

impl fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionStatus::None => write!(f, "No transaction"),
            TransactionStatus::Preparing => write!(f, "Preparing transaction"),
            TransactionStatus::Building => write!(f, "Building transaction"),
            TransactionStatus::Signing { tx } => write!(f, "Signing transaction {}", tx),
            TransactionStatus::Submitting { tx_hash } => match tx_hash {
                Some(hash) => write!(f, "Transaction submitted: {}", hash),
                None => write!(f, "Submitting transaction"),
            },
            TransactionStatus::Confirmed {
                tx_hash,
                block_number,
            } => {
                write!(
                    f,
                    "Transaction {} confirmed in block {}",
                    tx_hash, block_number
                )
            }
            TransactionStatus::Failed { error } => write!(f, "Transaction failed: {}", error),
        }
    }
}
