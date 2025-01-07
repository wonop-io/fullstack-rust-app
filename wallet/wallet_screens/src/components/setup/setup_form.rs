use ethers::signers::coins_bip39::{English, Mnemonic};
use gloo_console as console;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::{WalletAction, WalletStore};

#[function_component(SetupForm)]
pub fn setup_form() -> Html {
    let (_, dispatch) = use_store::<WalletStore>();
    let mnemonic = use_state(String::new);
    let password = use_state(String::new);

    let generate_mnemonic = {
        let mnemonic = mnemonic.clone();
        Callback::from(move |_| {
            let mut rng = rand::thread_rng();
            let new_mnemonic = Mnemonic::<English>::new(&mut rng);
            mnemonic.set(new_mnemonic.to_phrase());
        })
    };

    let set_anvil_mnemonic = {
        let mnemonic = mnemonic.clone();
        Callback::from(move |_| {
            mnemonic.set("test test test test test test test test test test test junk".to_string());
        })
    };

    let on_password_change = {
        let password = password.clone();
        Callback::from(move |e: Event| {
            let value = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            password.set(value);
        })
    };

    let on_submit = {
        let mnemonic = mnemonic.clone();
        let password = password.clone();
        let dispatch = dispatch.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            console::log!("Setting up wallet");
            dispatch.apply(WalletAction::GenerateWallet {
                mnemonic: (*mnemonic).clone(),
                password: (*password).clone(),
            });
        })
    };

    html! {
        <div class="bg-white/95 dark:bg-zinc-900/95 p-8 space-y-6 rounded-2xl shadow-lg">
            <form onsubmit={on_submit} class="space-y-6">
                <div class="space-y-5">
                    <div class="relative group min-h-[5.5rem]">
                        <label for="mnemonic" class="text-sm font-medium text-zinc-700 dark:text-zinc-300 mb-1.5 block">
                            {"Recovery Phrase (Mnemonic)"}
                        </label>
                        <textarea
                            id="mnemonic"
                            class="w-full px-4 py-3 rounded-lg border-2 border-zinc-200 dark:border-zinc-700 bg-white dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300 focus:ring-2 focus:ring-blue-400 focus:border-transparent transition-all duration-200"
                            rows="3"
                            value={(*mnemonic).clone()}
                            readonly=true
                        />
                        <div class="flex gap-2 mt-2">
                            <button
                                type="button"
                                onclick={generate_mnemonic}
                                class="w-full py-3.5 font-medium text-white bg-gradient-to-r from-blue-500 to-indigo-600 hover:from-blue-600 hover:to-indigo-700 rounded-lg focus:ring-4 focus:ring-blue-400/50 dark:focus:ring-blue-500/50 transform transition-all duration-200 active:scale-[0.98]"
                            >
                                {"Generate New Recovery Phrase"}
                            </button>
                            <button
                                type="button"
                                onclick={set_anvil_mnemonic}
                                class="w-full py-3.5 font-medium text-white bg-gradient-to-r from-blue-500 to-indigo-600 hover:from-blue-600 hover:to-indigo-700 rounded-lg focus:ring-4 focus:ring-blue-400/50 dark:focus:ring-blue-500/50 transform transition-all duration-200 active:scale-[0.98]"
                            >
                                {"Use Anvil Recovery Phrase"}
                            </button>
                        </div>
                    </div>

                    <div class="relative group min-h-[5.5rem]">
                        <label for="password" class="text-sm font-medium text-zinc-700 dark:text-zinc-300 mb-1.5 block">
                            {"Encryption Password"}
                        </label>
                        <input
                            type="password"
                            id="password"
                            placeholder="Enter a strong password to encrypt your wallet"
                            class="w-full px-4 py-3 rounded-lg border-2 border-zinc-200 dark:border-zinc-700 bg-white dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300 focus:ring-2 focus:ring-blue-400 focus:border-transparent transition-all duration-200"
                            onchange={on_password_change}
                        />
                    </div>
                </div>

                <button
                    type="submit"
                    class="w-full py-3.5 font-medium text-white bg-gradient-to-r from-blue-500 to-indigo-600 hover:from-blue-600 hover:to-indigo-700 rounded-lg focus:ring-4 focus:ring-blue-400/50 dark:focus:ring-blue-500/50 transform transition-all duration-200 active:scale-[0.98]"
                >
                    {"Setup Wallet"}
                </button>
            </form>
        </div>
    }
}
