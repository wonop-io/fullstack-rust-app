use serde::{Deserialize, Serialize};
use wallet_api::{
    transaction::Transaction as ApiTransaction,
    wallet::{Wallet, WalletError},
};
use yewdux::prelude::*;
#[cfg(target_arch = "wasm32")]
use yewdux::{init_listener, storage, Listener};

use crate::{
    services::transactions::status::TransactionStatus, store::wallet_load_save::SaveableWallet,
};

#[derive(Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WalletStore {
    pub last_error: Option<WalletError>,
    pub wallet: Option<Wallet>,
    pub transactions: Vec<ApiTransaction>,
    pub transaction_status: TransactionStatus,
    #[serde(skip)]
    pub dispatcher: Option<Dispatch<WalletStore>>,
}

#[cfg(target_arch = "wasm32")]
struct StorageListener;

#[cfg(target_arch = "wasm32")]
impl Listener for StorageListener {
    type Store = WalletStore;

    fn on_change(&mut self, _cx: &yewdux::Context, state: std::rc::Rc<Self::Store>) {
        if let Err(err) = storage::save(state.as_ref(), storage::Area::Local) {
            log::error!("Error saving state to storage: {:?}", err);
        }

        if let Some(wallet) = &state.wallet {
            let wallet = wallet.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Err(e) = wallet.save().await {
                    log::error!("Error saving wallet to backend: {:?}", e);
                }
            });
        }
    }
}

impl Store for WalletStore {
    #[cfg(not(target_arch = "wasm32"))]
    fn new(ctx: &yewdux::Context) -> Self {
        let dispatcher = Dispatch::<WalletStore>::new(ctx);
        let store = Self {
            last_error: None,
            wallet: None,
            transactions: Vec::new(),
            transaction_status: TransactionStatus::default(),
            dispatcher: Some(dispatcher.clone()),
        };

        let dispatch = dispatcher.clone();
        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(wallet) = Wallet::load().await {
                dispatch.apply(super::actions::WalletAction::SetWallet(wallet));
            } else {
                dispatch.apply(super::actions::WalletAction::RemoveWallet);
            }
        });

        store
    }

    #[cfg(target_arch = "wasm32")]
    fn new(ctx: &yewdux::Context) -> Self {
        init_listener(StorageListener, ctx);

        let mut ret = storage::load(storage::Area::Local)
            .ok()
            .flatten()
            .unwrap_or(Self {
                last_error: None,
                wallet: None,
                transactions: Vec::new(),
                transaction_status: TransactionStatus::default(),
                dispatcher: None,
            });
        let dispatcher = Dispatch::<WalletStore>::new(ctx);
        ret.dispatcher = Some(dispatcher.clone());

        let dispatch = dispatcher.clone();
        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(wallet) = Wallet::load().await {
                dispatch.apply(super::actions::WalletAction::SetWallet(wallet));
            }
        });

        ret
    }

    fn should_notify(&self, _: &Self) -> bool {
        true
    }
}
