mod setup_form;
pub use setup_form::SetupForm;
use yew::prelude::*;

#[function_component(Setup)]
pub fn setup() -> Html {
    html! {
        <div class="min-h-screen bg-gradient-to-br from-zinc-50 via-zinc-100 to-zinc-50 dark:from-zinc-900 dark:via-zinc-800 dark:to-zinc-900 flex items-center justify-center antialiased py-16 px-4 sm:px-6 lg:px-8">
            <div class="container relative mx-auto flex flex-col items-center justify-center max-w-md">
                <div class="bg-white/95 dark:bg-zinc-900/95 backdrop-blur-xl border border-zinc-200/50 dark:border-zinc-700/50 rounded-2xl shadow-2xl overflow-hidden ring-1 ring-blue-500/20">
                    <div class="px-8 pt-8 pb-6 text-center">
                        <h2 class="text-3xl font-bold bg-gradient-to-r from-blue-600 to-indigo-600 dark:from-blue-400 dark:to-indigo-400 bg-clip-text text-transparent">
                            {"Setup Your Wallet"}
                        </h2>
                        <p class="mt-2 text-zinc-600 dark:text-zinc-400">
                            {"Configure your Ethereum wallet settings"}
                        </p>
                    </div>
                    <SetupForm />
                </div>
            </div>
        </div>
    }
}
