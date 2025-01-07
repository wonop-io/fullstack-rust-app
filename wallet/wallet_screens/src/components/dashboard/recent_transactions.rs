use yew::prelude::*;

use crate::components::common::TransactionTable;

#[function_component(RecentTransactions)]
pub fn recent_transactions() -> Html {
    html! {
        <div class="mt-6 bg-indigo-800 rounded-xl p-6">
            <h2 class="text-xl font-bold mb-4">{ "Recent Transactions" }</h2>
            <TransactionTable limit={Some(5)} />
        </div>
    }
}
