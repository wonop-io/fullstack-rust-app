use ethers::{types::H160, utils::parse_ether};
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    services::transactions::status::TransactionStatus,
    store::{WalletAction, WalletStore},
};

#[function_component(SendForm)]
pub fn send_form() -> Html {
    let (state, dispatch) = use_store::<WalletStore>();

    let recipient = use_state(String::new);
    let amount = use_state(String::new);
    let password = use_state(String::new);

    let on_new_transaction = {
        let dispatch = dispatch.clone();
        Callback::from(move |_| {
            dispatch.apply(WalletAction::UpdateTransactionStatus(
                TransactionStatus::None,
            ));
        })
    };

    let on_submit = {
        let recipient = recipient.clone();
        let amount = amount.clone();
        let password = password.clone();
        let dispatch = dispatch.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if let Ok(amount_value) = amount.parse::<f64>() {
                let wei_amount = parse_ether(amount_value.to_string()).unwrap();

                if let Ok(recipient_address) = recipient.parse::<H160>() {
                    dispatch.apply(WalletAction::SendTransaction {
                        to: recipient_address,
                        amount: wei_amount,
                        password: (*password).clone(),
                    });
                    recipient.set(String::new());
                    amount.set(String::new());
                    password.set(String::new());
                }
            }
        })
    };

    match &state.transaction_status {
        TransactionStatus::None => html! {
            <div>
                <div class="mb-4">
                    <label class="block mb-2">{ "Pay to" }</label>
                    <input
                        type="text"
                        placeholder="0x..."
                        value={(*recipient).clone()}
                        oninput={Callback::from(move |e: InputEvent| recipient.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))}
                        class="w-full bg-indigo-700 rounded-lg py-2 px-4"
                    />
                    <p class="text-xs mt-1 text-indigo-300">{ "Please enter the Wallet ID" }</p>
                </div>
                <div class="flex mb-4">
                    <div class="flex-1 mr-2">
                        <label class="block mb-2">{ "Amount" }</label>
                        <div class="w-full bg-indigo-700 rounded-lg py-2 px-4 flex items-center space-x-2">
                            <span class="text-indigo-300">{ "$" }</span>
                            <input
                                type="text"
                                value={(*amount).clone()}
                                placeholder="Amount to send"
                                oninput={Callback::from(move |e: InputEvent| amount.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))}
                                class="bg-transparent flex-1 outline-none focus:outline-none"
                            />
                        </div>
                    </div>
                </div>
                <div class="mb-4">
                    <label class="block mb-2">{ "Password" }</label>
                    <input
                        type="password"
                        value={(*password).clone()}
                        oninput={Callback::from(move |e: InputEvent| password.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))}
                        class="w-full bg-indigo-700 rounded-lg py-2 px-4"
                        placeholder="Enter your password"
                    />
                </div>
                <button
                    onclick={on_submit}
                    class="w-full bg-gradient-to-r from-pink-500 to-orange-400 rounded-lg py-3 font-bold"
                >
                    { "Send" }
                </button>
            </div>
        },
        TransactionStatus::Building
        | TransactionStatus::Preparing
        | TransactionStatus::Signing { .. }
        | TransactionStatus::Submitting { .. } => html! {
            <div class="flex items-center justify-center h-full">
                <div class="animate-spin rounded-full h-32 w-32 border-t-2 border-b-2 border-indigo-300"></div>
            </div>
        },
        TransactionStatus::Confirmed {
            tx_hash,
            block_number,
        } => html! {
            <div class="bg-indigo-800 rounded-xl p-6">
                <h3 class="text-xl font-bold mb-4">{ "Transaction Receipt" }</h3>
                <div class="mb-2">
                    <span class="text-indigo-300">{ "Transaction Hash: " }</span>{ tx_hash.to_string() }
                </div>
                <div class="mb-2">
                    <span class="text-indigo-300">{ "Block Number: " }</span>{ block_number.to_string() }
                </div>
                <button
                    onclick={on_new_transaction}
                    class="w-full bg-gradient-to-r from-pink-500 to-orange-400 rounded-lg py-3 font-bold"
                >
                    { "Send New Transaction" }
                </button>
            </div>
        },
        TransactionStatus::Failed { error } => html! {
            <div class="bg-indigo-800 rounded-xl p-6">
                <h3 class="text-xl font-bold mb-4">{ "Transaction Failed" }</h3>
                <div class="mb-4">
                    <span class="text-indigo-300">{ "Error: " }</span>{ error }
                </div>
                <button
                    onclick={on_new_transaction}
                    class="w-full bg-gradient-to-r from-pink-500 to-orange-400 rounded-lg py-3 font-bold"
                >
                    { "Try Again" }
                </button>
            </div>
        },
    }
}
