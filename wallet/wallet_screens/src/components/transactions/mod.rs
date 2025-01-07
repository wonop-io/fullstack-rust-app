use yew::prelude::*;
use yewdux::prelude::*;

// TODO: Refactor
use crate::store::{WalletAction, WalletStore};

#[function_component(Transactions)]
pub fn transactions() -> Html {
    let (state, dispatch) = use_store::<WalletStore>();
    let current_page = use_state(|| 1);
    let transactions_per_page = 10;

    {
        let dispatch = dispatch.clone();
        use_effect_with((), move |_| {
            dispatch.apply(WalletAction::RefreshTransactions);
            || ()
        });
    }

    let total_pages = 0; /* // TODO: state.wallet.as_ref().map_or(0, |wallet| {
                             (wallet.total_transactions as f64 / transactions_per_page as f64).ceil() as usize
                         });*/

    let paginated_transactions = state.wallet.as_ref().map(|_wallet| {
        let start = ((*current_page - 1) * transactions_per_page) as usize;
        let end = start + transactions_per_page;
        state
            .transactions
            .iter()
            .skip(start)
            .take(end - start)
            .collect::<Vec<_>>()
    });

    let format = "%Y-%m-%d %H:%M:%S";

    html! {
        <div class="flex-1">
            <div class="bg-indigo-800 rounded-xl p-6">
                <h2 class="text-xl font-bold mb-4">{ "Transaction History" }</h2>
                {
                    match &state.wallet {
                        Some(wallet) => html! {
                            <>
                                <div class="mb-4">
                                    <p class="text-lg">{ format!("Total Transactions: {}", state.transactions.len()) }</p>
                                </div>
                                <table class="w-full text-sm">
                                    <thead>
                                        <tr class="text-left text-indigo-300">
                                            <th class="pb-2">{ "Date" }</th>
                                            <th class="pb-2">{ "Amount (ETH)" }</th>
                                            <th class="pb-2">{ "Recipient" }</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {
                                            paginated_transactions.as_ref().map(|transactions| {
                                                transactions.iter().map(|tx| {
                                                    html! {
                                                        <tr class="border-t border-indigo-700">
                                                            <td class="py-2">{ tx.date.format(format).to_string() }</td>
                                                            <td class="py-2">{ format!("{:.4}", (tx.amount.as_u128() as f64) / (10f64.powi(wallet.token_decimals as i32))) }</td>
                                                            <td class="py-2">{ &tx.recipient[0..10] }{ "..." }</td>
                                                        </tr>
                                                    }
                                                }).collect::<Html>()
                                            }).unwrap_or_default()
                                        }
                                    </tbody>
                                </table>
                                <div class="mt-4 flex justify-between items-center">
                                    <button
                                        class="bg-indigo-600 text-white px-4 py-2 rounded-full disabled:opacity-50"
                                        disabled={*current_page == 1}
                                        onclick={let current_page = current_page.clone(); Callback::from(move |_| current_page.set(*current_page - 1))}
                                    >
                                        { "Previous" }
                                    </button>
                                    <span>{ format!("Page {} of {}", *current_page, total_pages) }</span>
                                    <button
                                        class="bg-indigo-600 text-white px-4 py-2 rounded-full disabled:opacity-50"
                                        disabled={*current_page == total_pages}
                                        onclick={let current_page = current_page.clone(); Callback::from(move |_| current_page.set(*current_page + 1))}
                                    >
                                        { "Next" }
                                    </button>
                                </div>
                            </>
                        },
                        None => html! {
                            <div class="flex flex-col items-center justify-center h-full">
                                <p class="text-lg text-indigo-300">{ "No wallet initialized. Please create or import a wallet to view transactions." }</p>
                            </div>
                        }
                    }
                }
            </div>
        </div>
    }
}
