use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::WalletStore;

#[function_component(WalletDetails)]
pub fn wallet_details() -> Html {
    let (state, _) = use_store::<WalletStore>();

    html! {
        <div class="bg-indigo-800 rounded-xl p-6">
            <div class="flex justify-between items-center mb-4">
                <h2 class="text-xl font-bold">{ "Wallet Details" }</h2>
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path>
                </svg>
            </div>
            {
                if let Some(wallet) = &state.wallet {
                    let address = wallet.address.clone();
                    html! {
                        <>
                            <div class="mb-2">
                                <span class="text-indigo-300">{ "Wallet Name :" }</span>{ " My Ethereum Wallet" }
                            </div>
                            <div class="mb-2">
                                <span class="text-indigo-300">{ "Network :" }</span>{ " Ethereum Mainnet" }
                            </div>
                            <div class="flex items-center">
                                <span class="text-indigo-300 mr-2">{ "Address :" }</span>
                                <span class="truncate flex-1">{ &address }</span>
                                <button class="text-indigo-300 ml-2" onclick={Callback::from(move |_| {
                                    let _ = web_sys::window()
                                        .unwrap()
                                        .navigator()
                                        .clipboard()
                                        .write_text(&address);
                                })}>
                                    { "Copy" }
                                </button>
                            </div>
                        </>
                    }
                } else {
                    html! {
                        <div class="text-indigo-300">
                            { "No wallet connected" }
                        </div>
                    }
                }
            }
        </div>
    }
}
