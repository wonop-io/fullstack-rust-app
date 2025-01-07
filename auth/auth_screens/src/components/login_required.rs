// Boilerplate code by Wonop ApS.

use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

use crate::store::state::AuthStore;

#[derive(Properties, PartialEq)]
pub struct LoginRequiredProps<R: Routable + Clone + PartialEq + 'static> {
    pub children: Children,
    pub fallback: R,
}

#[function_component(LoginRequired)]
pub fn login_required<R: Routable + Clone + PartialEq + 'static>(
    props: &LoginRequiredProps<R>,
) -> Html {
    let (auth, _) = use_store::<AuthStore>();
    let navigator = use_navigator().unwrap();

    {
        let auth = auth.clone();
        let fallback = props.fallback.clone();
        use_effect_with((auth,), move |(auth,)| {
            let fallback = fallback.clone();
            if !auth.is_authenticated() {
                navigator.push(&fallback);
            }
        });
    }
    if auth.is_authenticated() {
        html! {
            <>{ for props.children.iter() }</>
        }
    } else {
        html! {}
    }
}
