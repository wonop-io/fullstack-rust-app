use yew::prelude::*;

#[function_component(WalletSecurity)]
pub fn wallet_security() -> Html {
    html! {
        <div class="bg-indigo-800 rounded-xl p-6">
            <div class="flex justify-between items-center mb-4">
                <h2 class="text-xl font-bold">{ "Wallet Security" }</h2>
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.618 5.984A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016zM12 9v2m0 4h.01"></path>
                </svg>
            </div>
            <div class="flex justify-between items-center mb-2">
                <span>{ "Backup Phrase" }</span>
                <button class="text-indigo-300 hover:text-indigo-200 transition-colors">{ "View" }</button>
            </div>
            <div class="flex justify-between items-center mb-2">
                <span>{ "Private Key" }</span>
                <button class="text-indigo-300 hover:text-indigo-200 transition-colors">{ "Export" }</button>
            </div>
            <div class="flex justify-between items-center">
                <span>{ "Password Protection" }</span>
                <div class="w-12 h-6 bg-green-500 rounded-full relative cursor-pointer">
                    <div class="w-4 h-4 bg-white rounded-full absolute right-1 top-1 transition-transform"></div>
                </div>
            </div>
        </div>
    }
}
