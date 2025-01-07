use wonopui::*;
use yew::prelude::*;

#[function_component(AppTopbar)]
pub fn app_topbar() -> Html {
    html! {
        <Topbar>
            <Container variant={ContainerVariant::Responsive} class="w-full flex items-center justify-end lg:hidden">
                <MobileMenuButton>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-6 w-6 dark:stroke-zinc-300">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
                    </svg>
                </MobileMenuButton>
            </Container>
        </Topbar>
    }
}
