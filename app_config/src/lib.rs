mod error_response;
use yew_router::prelude::*;
mod logo;

pub use crate::{error_response::ErrorResponse, logo::Logo};

pub fn get_base_url() -> String {
    let window = web_sys::window().unwrap();
    let location = window.location();
    let mut origin = location.origin().unwrap();
    if !origin.ends_with('/') {
        origin.push('/');
    }

    origin.push_str("api");

    origin
}

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum AppRoute {
    #[at("/")]
    Dashboard,
    #[at("/transfer")]
    Transfer,
    #[at("/transactions")]
    Transactions,
    #[at("/signup")]
    Signup,
    #[at("/login")]
    Login,
    #[at("/thank-you")]
    ThankYouForSigningUp,
}
