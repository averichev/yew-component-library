pub mod main_menu;
pub mod some_component;

use crate::routers::main_router::MainRouterSwitcher;
use crate::components::main_menu::MainMenuComponent;
use yew::{html, Component, Context, Html};
use yew_router::HashRouter;
use yew_template::template_html;

pub struct RootComponent {}

impl Component for RootComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        RootComponent {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let root = template_html!("src/templates/root.html");
        html! {
            <HashRouter>
            {root}
            </HashRouter>
        }
    }
}
