use auth_screens::store::{AuthAction, AuthStore};
use wonopui::*;
use yew::prelude::*;
use yewdux::use_store;

#[function_component(UserMenu)]
pub fn user_menu() -> Html {
    let (_, auth_dispatch) = use_store::<AuthStore>();
    let layout_context = use_context::<LayoutContext>().expect("LayoutContext not found");
    let folded = layout_context.sidebar_folded;

    let onlogout = {
        let auth_dispatch = auth_dispatch.clone();
        Callback::from(move |_| {
            auth_dispatch.apply(AuthAction::Logout);
        })
    };

    let user_menu = vec![DropdownItem::Action {
        label: "Logout".to_string(),
        icon: None,
        onclick: onlogout,
        disabled: false,
    }];
    html! {
        <Dropdown items={user_menu} position={PopoverPosition::EastEnd} class="w-full">
            <div class="text-current flex items-center space-x-4 items-center justify-center px-4 py-2 hover:bg-zinc-200 hover:dark:bg-zinc-700 w-full cursor-pointer h-16">
                <Avatar src="/assets/profile.jpg" size={AvatarSize::Small}/>
                if !folded {
                    <div class="flex flex-grow flex-col space-y-1">
                        <span>{"Profile"}</span>
                    </div>
                }
            </div>
        </Dropdown>
    }
}
