// Boilerplate code by Wonop ApS.
use std::collections::HashMap;

use app_config::{get_base_url, AppRoute, ErrorResponse};
use auth_password_api::{AuthLoginForm, AuthLoginUserResponse};
use auth_screens::store::{actions::AuthAction, state::AuthStore};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use yew::{platform::spawn_local, prelude::*};
use yew_router::{hooks::use_navigator, prelude::*, AnyRoute};
use yewdux::prelude::*;

use crate::components::{layout::LoginLayout, login_form::LoginForm};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginParameters {
    pub next: Option<String>,
}

fn redirect(params: LoginParameters, navigator: &Navigator) {
    match params.next {
        Some(path) => {
            let params = HashMap::new();
            let route = AnyRoute::from_path(&path, &params).unwrap();
            navigator.replace(&route);
        }
        None => {
            navigator.replace(&AppRoute::Dashboard);
        }
    }
}

fn perform_login<F>(set_error: F, dispatch: Dispatch<AuthStore>, payload: AuthLoginForm)
where
    F: FnOnce(Option<String>) + 'static,
{
    let credentials = serde_json::json!(payload).to_string();
    let base_url = get_base_url();

    spawn_local(async move {
        let client = Client::new();
        let _auth_store = dispatch.get();
        #[cfg(target_arch = "wasm32")]
        let request = client
            .post(format!("{}/v1/auth/login", base_url))
            .header("Content-Type", "application/json")
            .fetch_credentials_include()
            .body(credentials);

        #[cfg(not(target_arch = "wasm32"))]
        let request = client
            .post(format!("{}/v1/auth/login", base_url))
            .header("Content-Type", "application/json")
            .body(credentials);

        match request.send().await {
            Ok(response) => {
                if response.status() == 200 {
                    match response.json::<AuthLoginUserResponse>().await {
                        Ok(res) => {
                            dispatch.apply(AuthAction::SetUser(Some(res.data)));
                        }
                        Err(_) => set_error(Some("Failed to parse login response".to_string())),
                    }
                } else {
                    let status = response.status();
                    match response.json::<ErrorResponse>().await {
                        Ok(error_response) => set_error(Some(error_response.message)),
                        Err(_) => set_error(Some(format!("API error: {}", status))),
                    }
                }
            }
            Err(_) => set_error(Some("Failed to make login request".to_string())),
        }
    });
}

#[function_component(Login)]
pub fn login() -> Html {
    let (auth, dispatch) = use_store::<AuthStore>();
    let navigator = use_navigator().unwrap();
    let generic_error = use_state(|| None::<String>);
    let loading = use_state(|| false);

    let location = use_location().unwrap();
    let login_parameters = match location.query::<LoginParameters>() {
        Ok(v) => v,
        Err(_) => LoginParameters { next: None },
    };

    {
        let auth = auth.clone();
        use_effect_with((auth,), move |(auth,)| {
            if auth.is_authenticated() {
                redirect(login_parameters, &navigator);
            }
        });
    }

    let handle_submit = {
        let dispatch = dispatch.clone();
        let generic_error = generic_error.clone();
        let loading = loading.clone();

        Callback::from(move |form: AuthLoginForm| {
            loading.set(true);
            let dispatch = dispatch.clone();
            let generic_error = generic_error.clone();
            let loading = loading.clone();

            let set_generic_error = move |error: Option<String>| {
                loading.set(false);
                generic_error.set(error);
            };

            perform_login(set_generic_error, dispatch, form);
        })
    };

    html! {
        <LoginLayout>
            <div class="bg-white/95 dark:bg-zinc-900/95 backdrop-blur-xl border border-zinc-200/50 dark:border-zinc-700/50 rounded-2xl shadow-2xl overflow-hidden ring-1 ring-blue-500/20">
                <div class="px-8 pt-8 pb-6 text-center">
                    <h2 class="text-3xl font-bold bg-gradient-to-r from-blue-600 to-indigo-600 dark:from-blue-400 dark:to-indigo-400 bg-clip-text text-transparent">{"Access Your Wallet"}</h2>
                    <p class="mt-2 text-zinc-600 dark:text-zinc-400">{"Sign in to manage your Ethereum assets"}</p>
                </div>

                <LoginForm
                    on_submit={handle_submit}
                    loading={*loading}
                    error={(*generic_error).clone()}
                />

                <div class="px-8 pb-8 pt-2 text-center">
                    <p class="text-zinc-600 dark:text-zinc-400">
                        { "Don't have a wallet? " }
                        <Link<AppRoute>
                            to={AppRoute::Signup}
                            classes="font-medium text-blue-600 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300 transition-colors underline-offset-4 hover:underline"
                        >
                            { "Create one now" }
                        </Link<AppRoute>>
                    </p>
                </div>
            </div>
        </LoginLayout>
    }
}
