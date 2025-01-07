use std::{error::Error, sync::mpsc::Sender};

use ethers::types::{Address, H256, U256};
use gloo_timers::future::TimeoutFuture;

use super::{events::TransactionEvent, manager::TransactionManager};

pub struct TransactionWorker {
    manager: TransactionManager,
    event_sender: Sender<TransactionEvent>,
}

impl TransactionWorker {
    pub async fn new(
        rpc_url: &str,
        event_sender: Sender<TransactionEvent>,
    ) -> Result<Self, Box<dyn Error>> {
        let manager = TransactionManager::new(rpc_url).await?;

        Ok(Self {
            manager,
            event_sender,
        })
    }

    pub async fn process_transaction(
        &mut self,
        to: Address,
        amount: U256,
        private_key: &str,
    ) -> Result<(), Box<dyn Error>> {
        self.event_sender.send(TransactionEvent::Started)?;

        // Unlock wallet
        self.manager.unlock_wallet(private_key)?;

        // Build transaction
        self.event_sender.send(TransactionEvent::Building)?;
        let tx_request = self.manager.build_transaction(to, amount).await?;
        self.event_sender
            .send(TransactionEvent::Built(tx_request.clone()))?;

        // Sign transaction
        self.event_sender.send(TransactionEvent::Signing)?;
        let signed_tx = self.manager.sign_transaction(tx_request).await?;
        self.event_sender
            .send(TransactionEvent::Signed(signed_tx.clone()))?;

        // Submit transaction
        let tx_hash = self.manager.submit_transaction(signed_tx).await?;
        self.event_sender
            .send(TransactionEvent::Submitted(tx_hash))?;

        // Monitor transaction
        self.monitor_transaction(tx_hash).await?;

        Ok(())
    }

    async fn monitor_transaction(&self, tx_hash: H256) -> Result<(), Box<dyn Error>> {
        loop {
            match self.manager.get_transaction_status(tx_hash).await? {
                Some(block_number) => {
                    self.event_sender.send(TransactionEvent::Confirmed {
                        tx_hash,
                        block_number,
                    })?;
                    break;
                }
                None => {
                    TimeoutFuture::new(1_000).await; // Wait 1 second before polling again
                }
            }
        }
        Ok(())
    }
}
