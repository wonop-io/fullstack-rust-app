mod receive_form;
mod send_form;
pub use receive_form::ReceiveForm;
pub use send_form::SendForm;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::WalletStore;

#[derive(Debug, Clone, PartialEq, Eq)]
enum TransferView {
    Send,
    Receive,
}

#[function_component(Transfer)]
pub fn transfer() -> Html {
    let (_state, _dispatch) = use_store::<WalletStore>();

    let active_tab = use_state(|| TransferView::Send);

    let on_send_click = {
        let active_tab = active_tab.clone();
        Callback::from(move |_| {
            active_tab.set(TransferView::Send);
        })
    };

    let on_receive_click = {
        let active_tab = active_tab.clone();
        Callback::from(move |_| {
            active_tab.set(TransferView::Receive);
        })
    };

    html! {
        <div class="mt-6 bg-indigo-800 rounded-xl p-6">
            <h2 class="text-xl font-bold mb-4">{ "Transfer" }</h2>
            <div class="flex mb-4">
                <button
                    class={format!("flex-1 text-center py-2 {}", if *active_tab == TransferView::Send { "border-b-2 border-white" } else { "text-indigo-300" })}
                    onclick={on_send_click}
                >
                    { "Send" }
                </button>
                <button
                    class={format!("flex-1 text-center py-2 {}", if *active_tab == TransferView::Receive { "border-b-2 border-white" } else { "text-indigo-300" })}
                    onclick={on_receive_click}
                >
                    { "Receive" }
                </button>
            </div>
            {
                match *active_tab {
                    TransferView::Send => html! { <SendForm /> },
                    TransferView::Receive => html! { <ReceiveForm /> },
                }
            }
        </div>
    }
}
