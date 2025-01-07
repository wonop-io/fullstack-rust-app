use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use ethers::{
    prelude::*,
    signers::{coins_bip39::English, MnemonicBuilder, Signer},
};
use gloo_console as console;
use rand::{thread_rng, Rng};
use rust_decimal::Decimal;
use uuid::Uuid;
use wallet_api::wallet::{Wallet, WalletError};
use wasm_bindgen_futures::spawn_local;
use yewdux::prelude::*;

use super::super::{actions::WalletAction, state::WalletStore};
use crate::services::crypto::encrypt;

pub fn spawn_generate_wallet(mnemonic: String, password: String, dispatch: Dispatch<WalletStore>) {
    spawn_local(async move {
        let mnemonic = mnemonic.trim().to_string();
        let wallet = MnemonicBuilder::<English>::default()
            .phrase(mnemonic.as_str())
            .build();

        match wallet {
            Ok(wallet) => {
                let address = wallet.address();
                let unencrypted_pk = wallet
                    .signer()
                    .to_bytes()
                    .iter()
                    .map(|&i| format!("{:02X}", i))
                    .collect::<Vec<String>>()
                    .join("");

                let mut salt = [0u8; 16];
                thread_rng().fill(&mut salt);

                let encrypted_pk = encrypt(unencrypted_pk, &password, &salt);
                let new_wallet = Wallet {
                    id: Uuid::new_v4(),
                    user_id: Uuid::new_v4(), // TODO: Replace with actual user_id
                    encrypted_private_key: general_purpose::STANDARD.encode(encrypted_pk.clone()),
                    address: format!("{:?}", address),
                    balance: Decimal::from(0),
                    salt: salt.to_vec(),
                    token_decimals: 18,
                    created_at: Some(Utc::now()),
                    updated_at: Some(Utc::now()),
                };
                dispatch.apply(WalletAction::SetWallet(new_wallet));
                console::log!("Wallet loaded successfully");
            }
            Err(e) => {
                console::error!(format!("Failed setting the wallet up: {}", e));
                dispatch.apply(WalletAction::SetError(WalletError {
                    message: format!("Failed to load wallet: {}", e),
                    timestamp: Utc::now(),
                }));
            }
        }
    });
}
