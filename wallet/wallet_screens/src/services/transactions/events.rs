use ethers::types::{Transaction, TransactionRequest, H256};
use serde::{Deserialize, Serialize};

use super::status::TransactionStatus;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TransactionEvent {
    Started,
    Building,
    Built(TransactionRequest),
    Signing,
    Signed(Transaction),
    Submitted(H256),
    Confirmed { tx_hash: H256, block_number: u64 },
    Failed(String),
}

impl TransactionEvent {
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            TransactionEvent::Confirmed { .. } | TransactionEvent::Failed(_)
        )
    }

    pub fn is_error(&self) -> bool {
        matches!(self, TransactionEvent::Failed(_))
    }

    pub fn get_error(&self) -> Option<&str> {
        match self {
            TransactionEvent::Failed(err) => Some(err),
            _ => None,
        }
    }
}

impl From<TransactionEvent> for TransactionStatus {
    fn from(event: TransactionEvent) -> Self {
        match event {
            TransactionEvent::Started => Self::Preparing,
            TransactionEvent::Building => Self::Building,
            TransactionEvent::Built(_) => Self::Building,
            TransactionEvent::Signing => Self::Signing { tx: 0.into() },
            TransactionEvent::Signed(_) => Self::Signing { tx: 0.into() },
            TransactionEvent::Submitted(tx_hash) => Self::Submitting {
                tx_hash: Some(tx_hash),
            },
            TransactionEvent::Confirmed {
                tx_hash,
                block_number,
            } => Self::Confirmed {
                tx_hash,
                block_number,
            },
            TransactionEvent::Failed(error) => Self::Failed { error },
        }
    }
}
