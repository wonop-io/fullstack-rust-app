mod balance_card;
mod recent_transactions;
mod wallet_details;
mod wallet_security;

pub use balance_card::BalanceCard;
pub use recent_transactions::RecentTransactions;
pub use wallet_details::WalletDetails;
pub use wallet_security::WalletSecurity;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    components::common::QrCodeCard,
    store::{WalletAction, WalletStore},
};

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let (_, dispatch) = use_store::<WalletStore>();

    {
        let dispatch = dispatch.clone();
        use_effect_with((), move |_| {
            dispatch.apply(WalletAction::RefreshBalance);
            dispatch.apply(WalletAction::RefreshTransactions);
            || ()
        });
    }

    html! {
        <div class="flex-1">
            <div class="grid grid-cols-3 gap-4">
                <BalanceCard />
                <QrCodeCard />
            </div>

            <RecentTransactions />

            <div class="grid grid-cols-2 gap-4 mt-6">
                <WalletDetails />
                <WalletSecurity />
            </div>
        </div>
    }
}
