use chrono::Utc;
use ethers::{
    prelude::*,
    providers::{Http, Provider},
};
use gloo_console as console;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use wallet_api::wallet::WalletError;
use wasm_bindgen_futures::spawn_local;
use yewdux::prelude::*;

use super::super::{actions::WalletAction, state::WalletStore};

pub fn spawn_refresh_balance(address: String, dispatch: Dispatch<WalletStore>) {
    spawn_local(async move {
        let provider = Provider::<Http>::try_from("http://localhost:8545")
            .expect("could not instantiate HTTP Provider");
        match provider
            .get_balance(address.parse::<Address>().unwrap(), None)
            .await
        {
            Ok(balance) => {
                console::log!(format!("Balance: {}", balance));
                let balance_eth =
                    Decimal::from_u128(balance.as_u128()).unwrap() / Decimal::from(10u64.pow(18));
                console::log!(format!("Balance: {}", balance_eth));
                dispatch.apply(WalletAction::UpdateBalance(balance_eth));
                console::log!(format!("Balance updated: {} ETH", balance_eth));
            }
            Err(e) => {
                dispatch.apply(WalletAction::SetError(WalletError {
                    message: format!("Failed to refresh balance: {}", e),
                    timestamp: Utc::now(),
                }));
            }
        }
    });
}
