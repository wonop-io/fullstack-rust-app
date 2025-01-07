use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::WalletStore;

#[derive(Properties, PartialEq)]
pub struct TransactionTableProps {
    pub limit: Option<usize>,
}

#[function_component(TransactionTable)]
pub fn transaction_table(props: &TransactionTableProps) -> Html {
    let (state, _) = use_store::<WalletStore>();

    html! {
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
                    if let Some(wallet) = &state.wallet {
                        let transactions = match props.limit {
                            Some(limit) => state.transactions.iter().take(limit),
                            None => state.transactions.iter().take(state.transactions.len())
                        };

                        transactions.map(|tx| {
                            html! {
                                <tr class="border-t border-indigo-700">
                                    <td class="py-2">{ tx.date.format("%Y-%m-%d %H:%M").to_string() }</td>
                                    <td class="py-2">{ format!("{:.4}", (tx.amount.as_u128() as f64) / (10f64.powi(wallet.token_decimals as i32))) }</td>
                                    <td class="py-2">{ &tx.recipient[0..10] }{ "..." }</td>
                                </tr>
                            }
                        }).collect::<Html>()
                    } else {
                        html! {
                            <tr>
                                <td colspan="3" class="py-2 text-center text-indigo-300">
                                    { "No transactions to display. Please create or import a wallet." }
                                </td>
                            </tr>
                        }
                    }
                }
            </tbody>
        </table>
    }
}
