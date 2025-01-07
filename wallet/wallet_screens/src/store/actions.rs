use std::rc::Rc;

use base64::{engine::general_purpose, Engine as _};
use ethers::types::{Address, U256};
use gloo_console as console;
use rust_decimal::Decimal;
use wallet_api::{
    transaction::Transaction,
    wallet::{Wallet, WalletError},
};
use yewdux::prelude::*;

use super::state::WalletStore;
use crate::{
    services::{
        crypto::decrypt,
        transactions::{TransactionEvent, TransactionStatus, TransactionWorker},
    },
    store::{spawn_generate_wallet, spawn_refresh_balance, wallet_load_save::SaveableWallet},
};

pub enum WalletAction {
    GenerateWallet {
        mnemonic: String,
        password: String,
    },
    LoadWallet,
    SendTransaction {
        to: Address,
        amount: U256,
        password: String,
    },
    UpdateTransactionStatus(TransactionStatus),
    RefreshBalance,
    RefreshTransactions,
    SetWallet(Wallet),
    RemoveWallet,
    SetError(WalletError),
    UpdateBalance(Decimal),
    UpdateTransactions(Vec<Transaction>),
    AddTransactions(Vec<Transaction>),
    ClearErrorMessage,
}

impl Reducer<WalletStore> for WalletAction {
    fn apply(self, state: Rc<WalletStore>) -> Rc<WalletStore> {
        let mut new_state = (*state).clone();

        match self {
            WalletAction::ClearErrorMessage => {
                new_state.last_error = None;
            }
            WalletAction::GenerateWallet { mnemonic, password } => {
                console::log!(format!("Loading wallet with mnemonic: {}", mnemonic));
                if let Some(dispatch) = state.dispatcher.as_ref() {
                    spawn_generate_wallet(mnemonic, password, dispatch.clone());
                }
            }
            WalletAction::LoadWallet => {
                console::log!("Loading wallet");
                if let Some(dispatch) = state.dispatcher.as_ref() {
                    let dispatch = dispatch.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        if let Ok(wallet) = Wallet::load().await {
                            dispatch.apply(super::actions::WalletAction::SetWallet(wallet));
                        } else {
                            dispatch.apply(super::actions::WalletAction::RemoveWallet);
                        }
                    });
                }
            }
            WalletAction::SendTransaction {
                to,
                amount,
                password,
            } => {
                console::log!(format!(
                    "Sending transaction to {} with amount {}",
                    to, amount
                ));
                if let Some(wallet) = &new_state.wallet {
                    if let Some(dispatch) = state.dispatcher.as_ref() {
                        use std::sync::mpsc::channel;

                        use wasm_bindgen_futures::spawn_local;

                        let (tx, rx) = channel::<TransactionEvent>();
                        let dispatch_clone = dispatch.clone();

                        // Decrypt private key
                        let encrypted_pk =
                            match general_purpose::STANDARD.decode(&wallet.encrypted_private_key) {
                                Ok(pk) => pk,
                                Err(e) => {
                                    console::error!(
                                        "{}",
                                        format!("Failed to decode private key: {:#?}", e)
                                    );
                                    return Rc::new(new_state);
                                }
                            };

                        let private_key = match decrypt(&encrypted_pk, &password, &wallet.salt) {
                            Ok(pk) => pk,
                            Err(e) => {
                                console::error!("Failed to decrypt private key: {}", e);
                                return Rc::new(new_state);
                            }
                        };

                        // Spawn a task to handle transaction events
                        spawn_local(async move {
                            use gloo_timers::future::TimeoutFuture;
                            loop {
                                match rx.try_recv() {
                                    Ok(event) => {
                                        let status: TransactionStatus = event.clone().into();
                                        dispatch_clone.reduce_mut(|store| {
                                            store.transaction_status = status.clone();
                                        });

                                        if status.is_complete() {
                                            // Add transaction to list when complete
                                            match event {
                                                TransactionEvent::Confirmed {
                                                    tx_hash,
                                                    block_number: _,
                                                } => {
                                                    let tx = Transaction {
                                                        id: tx_hash.to_string(),
                                                        recipient: to.to_string(),
                                                        amount,
                                                        date: chrono::Utc::now(),
                                                    };
                                                    dispatch_clone.apply(
                                                        WalletAction::AddTransactions(vec![tx]),
                                                    );
                                                }
                                                TransactionEvent::Failed(error) => {
                                                    console::error!(
                                                        "Transaction failed: {}",
                                                        error
                                                    );
                                                    let tx = Transaction {
                                                        id: "failed".to_string(),
                                                        recipient: to.to_string(),
                                                        amount,
                                                        date: chrono::Utc::now(),
                                                    };
                                                    dispatch_clone.apply(
                                                        WalletAction::AddTransactions(vec![tx]),
                                                    );
                                                }
                                                _ => {}
                                            }
                                            break;
                                        }
                                    }
                                    Err(_) => {
                                        TimeoutFuture::new(100).await;
                                    }
                                }
                            }
                        });

                        let dispatch_clone = dispatch.clone();
                        spawn_local(async move {
                            if let Ok(mut worker) =
                                TransactionWorker::new("http://localhost:8545", tx).await
                            {
                                if let Err(e) =
                                    worker.process_transaction(to, amount, &private_key).await
                                {
                                    let failed_tx = TransactionStatus::Failed {
                                        error: e.to_string(),
                                    };
                                    dispatch_clone
                                        .apply(WalletAction::UpdateTransactionStatus(failed_tx));
                                    console::error!(format!(
                                        "Transaction failed: {}",
                                        e.to_string()
                                    ));
                                }
                            }
                        });

                        new_state.transaction_status = TransactionStatus::Preparing;
                    } else {
                        console::error!("No dispatcher found");
                    }
                }
            }
            WalletAction::UpdateTransactionStatus(status) => {
                if status == TransactionStatus::None {
                    new_state.transaction_status = TransactionStatus::None;
                } else {
                    match new_state.transaction_status {
                        TransactionStatus::Confirmed { .. } | TransactionStatus::Failed { .. } => {
                            console::error!(format!(
                                "Attempted to set status on completed transaction: {:#?}",
                                status
                            ));
                        }
                        _ => {
                            new_state.transaction_status = status;
                        }
                    }
                }
            }
            WalletAction::RefreshBalance => {
                console::log!("Refreshing balance");
                if let Some(wallet) = &new_state.wallet {
                    if let Some(dispatch) = state.dispatcher.as_ref() {
                        let address = wallet.address.clone();
                        spawn_refresh_balance(address, dispatch.clone());
                    }
                }
            }
            WalletAction::RefreshTransactions => {
                console::log!("Refreshing transactions");
                // TODO: Placeholder for future backend that indexes transactions
            }
            WalletAction::SetWallet(wallet) => {
                new_state.wallet = Some(wallet);
            }
            WalletAction::RemoveWallet => {
                new_state.wallet = None;
                new_state.transactions = vec![];
                new_state.transaction_status = TransactionStatus::None;
            }
            WalletAction::SetError(error) => {
                console::error!(format!("Setting error: {:?}", error.message));
                new_state.last_error = Some(error);
            }
            WalletAction::UpdateBalance(balance) => {
                if let Some(wallet) = new_state.wallet.as_mut() {
                    wallet.balance = balance;
                }
            }
            WalletAction::UpdateTransactions(transactions) => {
                new_state.transactions = transactions;
            }
            WalletAction::AddTransactions(transactions) => {
                new_state.transactions.extend(transactions);
            }
        }
        Rc::new(new_state)
    }
}
