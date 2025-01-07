// Boilerplate code by Wonop ApS.
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LoginLayoutProps {
    pub children: Children,
}

#[function_component(LoginLayout)]
pub fn login_layout(props: &LoginLayoutProps) -> Html {
    html! {
        <div class="min-h-screen bg-gradient-to-br from-zinc-50 via-zinc-100 to-zinc-50 dark:from-zinc-900 dark:via-zinc-800 dark:to-zinc-900 flex items-center justify-center antialiased py-16 px-4 sm:px-6 lg:px-8">
            <div class="container relative mx-auto flex flex-col items-center justify-center max-w-md">
                { props.children.clone() }
            </div>
        </div>
    }
}
