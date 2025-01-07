// Boilerplate code by Wonop ApS.

use std::rc::Rc;

#[cfg(target_arch = "wasm32")]
use app_config::get_base_url;
#[cfg(target_arch = "wasm32")]
use app_config::ErrorResponse;
use auth_api::User;
#[cfg(target_arch = "wasm32")]
use reqwest::Client;
#[cfg(target_arch = "wasm32")]
use yew::platform::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::state::{AuthCheckState, AuthStore};

pub enum AuthAction {
    GetCurrentUser,
    SetUser(Option<User>),
    Logout,
    SetError(Option<String>),
}

impl Reducer<AuthStore> for AuthAction {
    #[cfg(not(target_arch = "wasm32"))]
    fn apply(self, orig_state: Rc<AuthStore>) -> Rc<AuthStore> {
        orig_state
    }

    #[cfg(target_arch = "wasm32")]
    fn apply(self, mut orig_state: Rc<AuthStore>) -> Rc<AuthStore> {
        let state = Rc::make_mut(&mut orig_state);
        let dispatch = Dispatch::<AuthStore>::global();
        let base_url = get_base_url();
        match self {
            AuthAction::GetCurrentUser => {
                spawn_local(async move {
                    let client = Client::new();
                    #[cfg(target_arch = "wasm32")]
                    let result = client
                        .get(format!("{}/v1/auth/my_user", base_url))
                        .fetch_credentials_include()
                        .send()
                        .await;

                    #[cfg(not(target_arch = "wasm32"))]
                    let result = client
                        .get(format!("{}/v1/auth/my_user", base_url))
                        .send()
                        .await;
                    let response = match result {
                        Ok(res) => res,
                        Err(_) => {
                            dispatch.apply(AuthAction::SetError(Some(
                                "Failed to fetch current user".to_string(),
                            )));
                            return;
                        }
                    };
                    let status = response.status();
                    if status == 200 {
                        match response.json::<User>().await {
                            Ok(user) => dispatch.apply(AuthAction::SetUser(Some(user))),
                            Err(_) => dispatch.apply(AuthAction::SetError(Some(
                                "Failed to parse user response".to_string(),
                            ))),
                        }
                    } else {
                        match response.json::<ErrorResponse>().await {
                            Ok(error_response) => {
                                dispatch.apply(AuthAction::SetError(Some(error_response.message)))
                            }
                            Err(_) => {
                                let error_message = format!("API error: {}", status);
                                dispatch.apply(AuthAction::SetError(Some(error_message)));
                            }
                        }
                    }
                });
            }
            AuthAction::SetUser(user) => {
                log::info!("{}", format!("Setting user {:#?}", user));

                state.user = user;
                if state.user.is_some() {
                    state.check_state = AuthCheckState::Authenticated;
                } else {
                    state.check_state = AuthCheckState::Unauthenticated;
                }
            }
            AuthAction::Logout => {
                state.user = None;
                state.email = None;
                state.check_state = AuthCheckState::Unauthenticated;
                spawn_local(async move {
                    let client = Client::new();
                    #[cfg(target_arch = "wasm32")]
                    let result = client
                        .get(format!("{}/v1/auth/logout", base_url))
                        .fetch_credentials_include()
                        .send()
                        .await;

                    #[cfg(not(target_arch = "wasm32"))]
                    let result = client
                        .get(format!("{}/v1/auth/logout", base_url))
                        .send()
                        .await;
                    match result {
                        Ok(_) => (),
                        Err(_) => {
                            dispatch
                                .apply(AuthAction::SetError(Some("Failed to log out".to_string())));
                            return;
                        }
                    };
                });
            }
            AuthAction::SetError(error) => {
                if error.is_some() {
                    state.email = None;
                    state.user = None;
                    state.check_state = AuthCheckState::Unauthenticated;
                }
                state.error = error;
            }
        }

        orig_state
    }
}

#[hook]
pub fn use_auth_check() {
    let (store, dispatch) = use_store::<AuthStore>();

    use_effect_with(store.check_state.clone(), move |check_state| {
        if *check_state == AuthCheckState::Loading {
            dispatch.apply(AuthAction::GetCurrentUser);
        }
        || ()
    });
}
