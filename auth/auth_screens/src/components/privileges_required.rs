// Boilerplate code by Wonop ApS.
use wonopui::use_notify;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::store::{actions::AuthAction, state::AuthStore};

#[derive(Properties, PartialEq)]
pub struct PrivilegesRequiredProps<R: Routable + Clone + PartialEq + 'static> {
    pub children: Children,
    pub accepted_roles: Vec<String>,
    pub fallback: R,
}

#[function_component(PrivilegesRequired)]
pub fn privileges_required<R: Routable + Clone + PartialEq + 'static>(
    props: &PrivilegesRequiredProps<R>,
) -> Html {
    let (auth, dispatch) = use_store::<AuthStore>();
    let navigator = use_navigator().unwrap();
    let accepted_roles = props.accepted_roles.clone();
    let notify = use_notify();

    {
        let auth = auth.clone();
        let fallback = props.fallback.clone();
        let accepted_roles = accepted_roles.clone();
        let notify = notify.clone();
        let dispatch = dispatch.clone();
        use_effect_with((auth,), move |(auth,)| {
            if !accepted_roles.iter().any(|role| auth.has_role(role)) {
                log::trace!("User does not have the required roles, redirecting to fallback route");
                notify.emit((
                    "Access denied".to_string(),
                    "You do not have the required privileges".to_string(),
                    None,
                ));
                dispatch.apply(AuthAction::Logout);
                navigator.push(&fallback);
            }
        });
    }
    if accepted_roles.iter().any(|role| auth.has_role(role)) {
        html! {
            <>{ for props.children.iter() }</>
        }
    } else {
        html! {
            <span>{"You do not have the required roles to view this page"}</span>
        }
    }
}
