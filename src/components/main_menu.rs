use crate::routers::main_router::MainRouter;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct MainMenuComponent {}

impl Component for MainMenuComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        MainMenuComponent {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            <div class="list-group">
                <Link<MainRouter> to={MainRouter::Main}>{ "Home" }</Link<MainRouter>>
                <Link<MainRouter> to={MainRouter::Datepicker}>{ "Datepicker" }</Link<MainRouter>>
            </div>
            </>
        }
    }
}
