use qrcode_generator::QrCodeEcc;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::WalletStore;

#[derive(Properties, PartialEq)]
pub struct QrCodeProps {
    pub data: String,
    pub size: Option<usize>,
}

#[function_component(QrCode)]
pub fn qr_code(props: &QrCodeProps) -> Html {
    let size = props.size.unwrap_or(128);

    let qr_code =
        qrcode_generator::to_svg_to_string(&props.data, QrCodeEcc::Low, size, None::<&str>)
            .unwrap_or_else(|_| String::from("Failed to generate QR code"));

    let qr_dom = Html::from_html_unchecked(AttrValue::from(qr_code));

    html! {
        <div class="bg-white p-4 rounded-lg">
            { qr_dom }
        </div>
    }
}

#[function_component(QrCodeCard)]
pub fn qr_code_card() -> Html {
    let (state, _) = use_store::<WalletStore>();

    let address = match &state.wallet {
        Some(wallet) => wallet.address.clone(),
        None => String::from("No wallet address available"),
    };

    html! {
        <div class="bg-gradient-to-br from-pink-500 to-orange-400 rounded-xl p-6 flex items-center justify-center">
            <QrCode data={address} size={Some(128)} />
        </div>
    }
}
