use qrcode_generator::QrCodeEcc;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::WalletStore;

#[function_component(ReceiveForm)]
pub fn receive_form() -> Html {
    let (state, _) = use_store::<WalletStore>();

    match &state.wallet {
        Some(wallet) => {
            let address = wallet.address.clone();

            let qr =
                qrcode_generator::to_svg_to_string(&address, QrCodeEcc::Low, 256, None::<&str>)
                    .unwrap_or_else(|_| String::from("Failed to generate QR code"));

            let qr_dom = Html::from_html_unchecked(AttrValue::from(qr));

            html! {
                <div>
                    <div class="mb-4 flex justify-center">
                        <div class="bg-white p-4 rounded-lg shadow-md mb-4">
                            { qr_dom }
                        </div>
                    </div>
                    <div class="mb-4">
                        <label class="block mb-2">{ "Your wallet address" }</label>
                        <input
                            type="text"
                            value={address.clone()}
                            readonly=true
                            class="w-full bg-indigo-700 rounded-lg py-2 px-4"
                        />
                        <p class="text-xs mt-1 text-indigo-300">
                            { "Share this address to receive funds" }
                        </p>
                    </div>
                </div>
            }
        }
        None => {
            html! {
                <div class="flex flex-col items-center space-y-4">
                    <h2 class="text-2xl font-bold">{ "Receive" }</h2>
                    <p class="text-lg">
                        { "No wallet initialized. Please create or import a wallet." }
                    </p>
                </div>
            }
        }
    }
}
