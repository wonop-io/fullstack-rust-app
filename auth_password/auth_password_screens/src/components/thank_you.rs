use app_config::{AppRoute, Logo};
use wonopui::{Col, Container, ContainerVariant};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(ThankYouForSigningUp)]
pub fn thank_you_for_signing_up() -> Html {
    html! {
        <Col class="justify-center h-screen max-h-screen bg-white dark:bg-zinc-900 transition-colors duration-200">
        <Container variant={ContainerVariant::Responsive}>
            <div class="sm:mx-auto sm:w-full sm:max-w-md dark:text-white">
                <Logo class="mx-auto h-16 w-auto" large={true} />
                <h2 class="mt-6 text-center text-3xl font-bold leading-9 tracking-tight bg-gradient-to-r from-blue-600 to-indigo-600 dark:from-blue-400 dark:to-indigo-400 bg-clip-text text-transparent">
                    { "Your Wallet is Ready!" }
                </h2>
            </div>
            <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-[480px]">
                <p class="text-center text-lg text-zinc-600 dark:text-zinc-400">
                    { "Your secure Ethereum wallet has been created. Please check your email to verify your account and start managing your assets." }
                </p>
                <div class="mt-8 flex items-center justify-center">
                    <Link<AppRoute> to={AppRoute::Login}
                        classes="px-4 py-2 rounded-lg font-semibold text-white bg-gradient-to-r from-blue-500 to-indigo-600 hover:from-blue-600 hover:to-indigo-700 dark:from-blue-400 dark:to-indigo-500 dark:hover:from-blue-500 dark:hover:to-indigo-600 transition-all duration-200 shadow-lg hover:shadow-xl">
                        { "Access Your Wallet" }
                    </Link<AppRoute>>
                </div>
            </div>
        </Container>
        </Col>
    }
}
