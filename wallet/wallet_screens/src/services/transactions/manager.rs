use std::{error::Error, str::FromStr};

use ethers::{
    prelude::*,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    types::{
        transaction::eip2718::TypedTransaction, Address, Transaction, TransactionRequest, H256,
        U256,
    },
};

pub struct TransactionManager {
    provider: Provider<Http>,
    chain_id: u64,
    wallet: Option<LocalWallet>,
}

impl TransactionManager {
    pub async fn new(rpc_url: &str) -> Result<Self, Box<dyn Error>> {
        let provider = Provider::<Http>::try_from(rpc_url)?;
        let chain_id = provider.get_chainid().await?.as_u64();

        Ok(Self {
            provider,
            chain_id,
            wallet: None,
        })
    }

    pub fn unlock_wallet(&mut self, private_key: &str) -> Result<(), Box<dyn Error>> {
        let wallet = LocalWallet::from_str(private_key)
            .map_err(|e| format!("Invalid private key: {}", e))?;
        let wallet = wallet.with_chain_id(self.chain_id);
        self.wallet = Some(wallet);
        Ok(())
    }

    pub async fn build_transaction(
        &self,
        to: Address,
        amount: U256,
    ) -> Result<TransactionRequest, Box<dyn Error>> {
        if let Some(wallet) = &self.wallet {
            let from = wallet.address();
            let nonce = self.provider.get_transaction_count(from, None).await?;
            let gas_price = self.provider.get_gas_price().await?;

            // Add gas estimate
            let tx = TransactionRequest::new()
                .to(to)
                .from(from)
                .value(amount)
                .nonce(nonce)
                .gas_price(gas_price)
                .chain_id(self.chain_id);

            let typed_tx = TypedTransaction::Legacy(tx.clone());
            let gas = self.provider.estimate_gas(&typed_tx, None).await?;
            Ok(tx.gas(gas))
        } else {
            Err("Wallet not unlocked".into())
        }
    }

    pub async fn sign_transaction(
        &self,
        tx: TransactionRequest,
    ) -> Result<Transaction, Box<dyn Error>> {
        if let Some(wallet) = &self.wallet {
            let typed_tx = TypedTransaction::Legacy(tx);
            let signature = wallet.sign_transaction(&typed_tx).await?;
            let signed_tx = Transaction {
                hash: H256::zero(), // Will be set when submitted
                nonce: *typed_tx.nonce().unwrap_or(&U256::zero()),
                block_hash: None,
                block_number: None,
                transaction_index: None,
                from: wallet.address(),
                to: typed_tx.to().cloned().and_then(|addr| match addr {
                    NameOrAddress::Address(a) => Some(a),
                    _ => None,
                }),
                value: *typed_tx.value().unwrap_or(&U256::zero()),
                gas_price: typed_tx.gas_price(),
                gas: *typed_tx.gas().unwrap_or(&U256::zero()),
                input: typed_tx.data().cloned().unwrap_or_default(),
                v: signature.v.into(),
                r: signature.r,
                s: signature.s,
                transaction_type: None,
                access_list: None,
                max_priority_fee_per_gas: None,
                max_fee_per_gas: None,
                chain_id: Some(self.chain_id.into()),
                other: Default::default(),
            };
            Ok(signed_tx)
        } else {
            Err("Wallet not unlocked".into())
        }
    }

    pub async fn submit_transaction(&self, tx: Transaction) -> Result<H256, Box<dyn Error>> {
        let rlp_bytes = tx.rlp();
        let pending_tx = self.provider.send_raw_transaction(rlp_bytes).await?;
        Ok(pending_tx.tx_hash())
    }

    pub async fn get_transaction_status(
        &self,
        tx_hash: H256,
    ) -> Result<Option<u64>, Box<dyn Error>> {
        if let Some(receipt) = self.provider.get_transaction_receipt(tx_hash).await? {
            Ok(receipt.block_number.map(|n| n.as_u64()))
        } else {
            Ok(None)
        }
    }
}
