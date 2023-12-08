use crate::views::datepicker_view::DatepickerView;
use crate::views::main_view::MainView;
use yew::{html, Component, Context, Html};
use yew_router::{Routable, Switch};

#[derive(Clone, Routable, PartialEq)]
pub enum MainRouter {
    #[at("/")]
    Main,
    #[at("/datepicker")]
    Datepicker,
}

pub struct MainRouterSwitcher;

impl Component for MainRouterSwitcher {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        MainRouterSwitcher {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <Switch<MainRouter> render={MainRouterSwitcher::switch} />
        }
    }
}

impl MainRouterSwitcher {
    fn switch(routes: MainRouter) -> Html {
        match routes {
            MainRouter::Main => {
                html! {
                    <MainView />
                }
            }
            MainRouter::Datepicker => {
                html! {
                    <DatepickerView />
                }
            }
        }
    }
}
