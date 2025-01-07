mod components {
    pub mod layout;
    pub mod topbar;
    pub mod usermenu;
}

use app_config::AppRoute;
use auth_password_screens::components::{Login, Signup, ThankYouForSigningUp};
use auth_screens::{components::LoginRequired, store::AuthStore};
use components::layout::AppLayout;
use wallet_screens::{
    components::{
        dashboard::Dashboard, setup::Setup, transactions::Transactions, transfer::Transfer,
    },
    store::WalletStore,
};
use wonopui::{DarkModeProvider, LayoutProvider, LayoutState, NotificationProvider, ThemeProvider};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let (auth, _) = use_store::<AuthStore>();
    let (state, dispatch_wallet) = use_store::<WalletStore>();

    // Load wallet when authenticated
    use_effect_with(auth.is_authenticated(), move |is_authenticated| {
        if *is_authenticated {
            dispatch_wallet.apply(wallet_screens::store::WalletAction::LoadWallet);
        } else {
            dispatch_wallet.apply(wallet_screens::store::WalletAction::RemoveWallet);
        }
        || ()
    });

    let render = move |route| {
        if auth.is_authenticated() && state.wallet.is_none() {
            html! { <Setup /> }
        } else {
            match route {
                AppRoute::Dashboard => html! {
                    <LoginRequired<AppRoute> fallback={AppRoute::Login}>
                        <AppLayout>
                            <Dashboard />
                        </AppLayout>
                    </LoginRequired<AppRoute>>
                },
                AppRoute::Transfer => html! {
                    <LoginRequired<AppRoute> fallback={AppRoute::Login}>
                        <AppLayout>
                            <Transfer />
                        </AppLayout>
                    </LoginRequired<AppRoute>>
                },
                AppRoute::Transactions => html! {
                    <LoginRequired<AppRoute> fallback={AppRoute::Login}>
                        <AppLayout>
                            <Transactions />
                        </AppLayout>
                    </LoginRequired<AppRoute>>
                },
                AppRoute::Login => html! { <Login /> },
                AppRoute::Signup => html! { <Signup /> },
                AppRoute::ThankYouForSigningUp => {
                    html! { <ThankYouForSigningUp /> }
                }
            }
        }
    };

    html! {
        <ThemeProvider>
        <LayoutProvider initial_state={LayoutState { sidebar_folded: true, ..LayoutState::new() }}>
        <DarkModeProvider>
        <NotificationProvider>

        <BrowserRouter>
            <Switch<AppRoute> render={render} />
        </BrowserRouter>
        </NotificationProvider>
        </DarkModeProvider>
        </LayoutProvider>
        </ThemeProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
