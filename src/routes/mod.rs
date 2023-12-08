use yew::{Html, html};
use yew_router::Routable;
use crate::views::datepicker_view::DatepickerView;
use crate::views::main_view::MainView;

pub fn switch(routes: MainRouter) -> Html {
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

#[derive(Clone, Routable, PartialEq)]
pub enum MainRouter {
    #[at("/")]
    Main,
    #[at("/datepicker")]
    Datepicker,
}
