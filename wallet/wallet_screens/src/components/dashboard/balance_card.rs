use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::WalletStore;

#[function_component(BalanceCard)]
pub fn balance_card() -> Html {
    let (state, _) = use_store::<WalletStore>();

    html! {
        <div class="col-span-2 bg-gradient-to-br from-pink-500 to-orange-400 rounded-xl p-6 relative overflow-hidden">
            <div class="flex justify-between mb-8">
                <svg class="w-8 h-8" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M20 4H4c-1.11 0-1.99.89-1.99 2L2 18c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V6c0-1.11-.89-2-2-2zm0 14H4v-6h16v6zm0-10H4V6h16v2z"/>
                </svg>
                <svg class="w-8 h-8" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z"/>
                </svg>
            </div>
            <div class="text-2xl mb-4">{ "My Ethereum Wallet" }</div>
            {
                if let Some(wallet) = &state.wallet {
                    html! {
                        <div class="text-3xl font-bold">{ format!("{:.2} ETH", wallet.balance) }</div>
                    }
                } else {
                    html! {
                        <div class="text-3xl font-bold">{ "0.00 ETH" }</div>
                    }
                }
            }
            <svg class="absolute bottom-0 right-0 w-32 h-32 text-white opacity-10" viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z"/>
            </svg>
            <a href="/transfer" class="absolute bottom-4 right-4 bg-white text-pink-500 px-4 py-2 rounded-full font-bold hover:bg-pink-100 transition-colors">
                { "Send" }
            </a>
        </div>
    }
}
