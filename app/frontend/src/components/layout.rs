use app_config::Logo;
use wonopui::*;
use yew::prelude::*;

use crate::{
    components::{topbar::AppTopbar, usermenu::UserMenu},
    AppRoute,
};

#[function_component(SidebarHeaderWithLogo)]
pub fn sidebar_header_with_logo() -> Html {
    let layout_context = use_context::<LayoutContext>().expect("LayoutContext not found");
    let folded = layout_context.sidebar_folded;
    let toggle_folded = {
        let layout_context = layout_context.clone();
        Callback::from(move |_| {
            layout_context.dispatch(LayoutAction::SetSidebarFolded(!folded));
        })
    };

    html! {
        <SidebarHeader onclick={toggle_folded} class="cursor-pointer">
            {
                if folded {
                    html! {
                        <div class="flex items-center justify-start p-4 w-full h-16 space-x-2">
                            <Logo class="h-10 w-10"/>
                        </div>
                    }
                } else {
                    html! {
                        <div class="flex items-center justify-start p-4 w-full h-16 space-x-2">
                            <Logo class="h-10 w-10"/>
                            <div class="text-2xl font-bold">{"Wallet"}</div>
                        </div>
                    }
                }
            }
        </SidebarHeader>
    }
}

#[function_component(PrimaryMainMenu)]
pub fn primary_main_menu() -> Html {
    html! {
        <SidebarColumn
            header={html!{
                <SidebarHeaderWithLogo />
            }}
            footer={html!{
                <SidebarFooter>
                    <UserMenu />
                </SidebarFooter>
            }}>
            <SidebarHeading>
                {"Main Menu"}
            </SidebarHeading>


            <SidebarMenu>
                <SidebarLink<AppRoute> to={AppRoute::Dashboard} label={"Wallet"} icon={html!{
                    <svg class={classes!("w-5", "h-5")} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z"/>
                    </svg>
                }} />
                <SidebarLink<AppRoute> to={AppRoute::Transfer} label={"Transfer"} icon={html!{
                    <svg class={classes!("w-5", "h-5")} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                    </svg>
                }} />
                <SidebarLink<AppRoute> to={AppRoute::Transactions} label={"History"} icon={html!{
                    <svg class={classes!("w-5", "h-5")} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                          <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 12h16.5m-16.5 3.75h16.5M3.75 19.5h16.5M5.625 4.5h12.75a1.875 1.875 0 0 1 0 3.75H5.625a1.875 1.875 0 0 1 0-3.75Z" />
                    </svg>
                }} />
            </SidebarMenu>
        </SidebarColumn>
    }
}

#[derive(Properties, PartialEq)]
pub struct AppSidebarProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AppSidebar)]
pub fn app_sidebar(props: &AppSidebarProps) -> Html {
    let layout_context = use_context::<LayoutContext>().expect("LayoutContext not found");
    let mobile_menu_open = layout_context.mobile_menu_open; // Use sidebar_folded from LayoutContext
    let open_mobile_menu = {
        let layout_context = layout_context.clone();
        Callback::from(move |_| {
            layout_context.dispatch(LayoutAction::SetMobileMenuOpen(!mobile_menu_open));
        })
    };
    html! {
        <MultiColumnSidebar
            curtain_content={html!{
                <div class="absolute top-2 left-2 cursor-pointer dark:text-white" onclick={open_mobile_menu}>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-6 w-6">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
                    </svg>
                </div>
            }}>
            {props.children.clone()}
        </MultiColumnSidebar>
    }
}

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Children,
}

#[function_component(Footer)]
fn footer() -> Html {
    html! {
        <Container
            variant={ContainerVariant::Responsive}
            class={classes!("flex", "flex-row", "justify-end", "items-center", "space-x-4")}
        >
            <span>{"Copyright © 2024 Wonop ApS"}</span>
            <a href="https://twitter.com/troelsfr" target="_blank" rel="noopener noreferrer" class={classes!("text-white", "hover:text-indigo-300")}>
                <svg class={classes!("w-4", "h-4")} fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                    <path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z" />
                </svg>
            </a>
        </Container>
    }
}

#[function_component(AppLayout)]
pub fn app_layout(props: &LayoutProps) -> Html {
    html! {
        <Layout sidebar={html! { <AppSidebar>
                <PrimaryMainMenu />
            </AppSidebar>
            }}
            topbar={html! { <AppTopbar /> }}
            footer={html! { <Footer /> }}

            direction={LayoutDirection::Vertical}
        >
            <Container variant={ContainerVariant::Responsive}>
                {for props.children.iter()}
            </Container>
        </Layout>
    }
}
