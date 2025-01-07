use auth_password_api::AuthLoginSignupForm;
use validator::{Validate, ValidationError};
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, InputEvent, SubmitEvent};
use wonopui::{Alert, AlertType, Button, ButtonVariant, Input, Label};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SignupFormProps {
    pub on_submit: Callback<AuthLoginSignupForm>,
    pub loading: bool,
    pub error: Option<String>,
}

#[function_component(SignupForm)]
pub fn signup_form(props: &SignupFormProps) -> Html {
    let form = use_state(AuthLoginSignupForm::default);
    let name_errors = use_state(Vec::<ValidationError>::new);
    let email_errors = use_state(Vec::<ValidationError>::new);
    let password_errors = use_state(Vec::<ValidationError>::new);

    let handle_input_change = {
        let form = form.clone();
        Callback::from(move |event: InputEvent| {
            let input_element = event.target().unwrap().unchecked_into::<HtmlInputElement>();
            let (name, value) = (input_element.name(), input_element.value());
            let mut data = (*form).clone();

            match name.as_str() {
                "name" => data.name = value,
                "email" => data.email = value,
                "password" => data.password = value,
                _ => (),
            }

            form.set(data);
        })
    };

    let handle_input_blur = {
        let form = form.clone();
        let name_errors = name_errors.clone();
        let email_errors = email_errors.clone();
        let password_errors = password_errors.clone();

        Callback::from(move |event: FocusEvent| {
            let input_element = event.target().unwrap().unchecked_into::<HtmlInputElement>();
            let name = input_element.name();
            let data = (*form).clone();

            if let Err(errs) = data.validate() {
                if let Some(errs) = errs.field_errors().get(name.as_str()) {
                    match name.as_str() {
                        "name" => name_errors.set(errs.to_vec()),
                        "email" => email_errors.set(errs.to_vec()),
                        "password" => password_errors.set(errs.to_vec()),
                        _ => (),
                    }
                } else {
                    match name.as_str() {
                        "name" => name_errors.set(vec![]),
                        "email" => email_errors.set(vec![]),
                        "password" => password_errors.set(vec![]),
                        _ => (),
                    }
                }
            } else {
                name_errors.set(vec![]);
                email_errors.set(vec![]);
                password_errors.set(vec![]);
            }
        })
    };

    let on_submit = {
        let form = form.clone();
        let name_errors = name_errors.clone();
        let email_errors = email_errors.clone();
        let password_errors = password_errors.clone();
        let on_submit = props.on_submit.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let data = (*form).clone();

            if let Err(errs) = data.validate() {
                if let Some(errs) = errs.field_errors().get("name") {
                    name_errors.set(errs.to_vec());
                } else {
                    name_errors.set(vec![]);
                }
                if let Some(errs) = errs.field_errors().get("email") {
                    email_errors.set(errs.to_vec());
                } else {
                    email_errors.set(vec![]);
                }
                if let Some(errs) = errs.field_errors().get("password") {
                    password_errors.set(errs.to_vec());
                } else {
                    password_errors.set(vec![]);
                }
                return;
            }

            on_submit.emit(data);
        })
    };

    html! {
        <div class="bg-white/95 dark:bg-zinc-900/95 p-8 space-y-6 rounded-2xl shadow-lg">

            if let Some(error) = props.error.clone() {
                <Alert alert_type={AlertType::Error} class="mb-4 animate-shake">
                    {error}
                </Alert>
            }

            <form class="space-y-6" onsubmit={on_submit}>
                <div class="space-y-5">
                    <div class="relative group min-h-[5.5rem]">
                        <Label for_id="name" class="text-sm font-medium text-zinc-700 dark:text-zinc-300 mb-1.5 block">
                            { "Wallet Owner Name" }
                        </Label>
                        <Input
                            id="name"
                            name="name"
                            kind="text"
                            placeholder="Enter your full name"
                            value={form.name.clone()}
                            oninput={handle_input_change.clone()}
                            onblur={handle_input_blur.clone()}
                            disabled={props.loading}
                            class="text-zinc-700 dark:text-zinc-300 w-full px-4 py-3 rounded-lg border-2 border-zinc-200 dark:border-zinc-700 bg-white dark:bg-zinc-800 focus:ring-2 focus:ring-blue-400 focus:border-transparent"
                        />
                        <div class="h-6 mt-1">
                            if !name_errors.is_empty() {
                                <p class="text-xs text-red-500 dark:text-red-400 animate-slideDown">
                                    { name_errors[0].message.clone().unwrap_or_default() }
                                </p>
                            }
                        </div>
                    </div>

                    <div class="relative group min-h-[5.5rem]">
                        <Label for_id="email" class="text-sm font-medium text-zinc-700 dark:text-zinc-300 mb-1.5 block">
                            { "Account Email" }
                        </Label>
                        <Input
                            id="email"
                            name="email"
                            kind="email"
                            placeholder="For account recovery and notifications"
                            value={form.email.clone()}
                            oninput={handle_input_change.clone()}
                            onblur={handle_input_blur.clone()}
                            disabled={props.loading}
                            class="text-zinc-700 dark:text-zinc-300 w-full px-4 py-3 rounded-lg border-2 border-zinc-200 dark:border-zinc-700 bg-white dark:bg-zinc-800 focus:ring-2 focus:ring-blue-400 focus:border-transparent"
                        />
                        <div class="h-6 mt-1">
                            if !email_errors.is_empty() {
                                <p class="text-xs text-red-500 dark:text-red-400 animate-slideDown">
                                    { email_errors[0].message.clone().unwrap_or_default() }
                                </p>
                            }
                        </div>
                    </div>

                    <div class="relative group min-h-[5.5rem]">
                        <Label for_id="password" class="text-sm font-medium text-zinc-700 dark:text-zinc-300 mb-1.5 block">
                            { "Account Password" }
                        </Label>
                        <Input
                            id="password"
                            name="password"
                            kind="password"
                            placeholder="Create a strong password to protect your assets"
                            value={form.password.clone()}
                            oninput={handle_input_change.clone()}
                            onblur={handle_input_blur.clone()}
                            disabled={props.loading}
                            class="text-zinc-700 dark:text-zinc-300 w-full px-4 py-3 rounded-lg border-2 border-zinc-200 dark:border-zinc-700 bg-white dark:bg-zinc-800 focus:ring-2 focus:ring-blue-400 focus:border-transparent"
                        />
                        <div class="h-6 mt-1">
                            if !password_errors.is_empty() {
                                <p class="text-xs text-red-500 dark:text-red-400 animate-slideDown">
                                    { password_errors[0].message.clone().unwrap_or_default() }
                                </p>
                            }
                        </div>
                    </div>
                </div>

                <Button
                    kind="submit"
                    variant={ButtonVariant::Primary}
                    disabled={props.loading}
                    class="w-full py-3.5 font-medium text-white bg-gradient-to-r from-blue-500 to-indigo-600 hover:from-blue-600 hover:to-indigo-700 rounded-lg focus:ring-4 focus:ring-blue-400/50 dark:focus:ring-blue-500/50 transform transition-all duration-200 active:scale-[0.98]"
                >
                    if props.loading {
                        <span class="inline-block animate-spin mr-2 text-xl">{"⟳"}</span>
                        { "Creating your wallet..." }
                    } else {
                        { "Create Ethereum Wallet" }
                    }
                </Button>
            </form>
        </div>
    }
}
