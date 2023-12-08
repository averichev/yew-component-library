use yew::{html, Component, Context, Html};
use yew_router::prelude::*;
use crate::routes::{MainRouter, switch};
use crate::components::main_menu::MainMenuComponent;

pub mod datepicker_view;
pub mod main_view;

pub struct RootComponent {}

impl Component for RootComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        RootComponent {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
                    <>
                    <div class="container pt-5">
            <div class="row">

                <div class="col-2">
<MainMenuComponent/>
                </div>
                            <div class="col-2">
<BrowserRouter>
                        <Switch<MainRouter> render={switch} />
                    </BrowserRouter>
                </div>
            </div>
        </div>
        <footer class="footer mt-auto py-3 bg-body-tertiary">
            <div class="container">
                <a href="https://github.com/averichev/yew-starter-template">{"Source of this
                    template"}</a>
            </div>
        </footer>
                    </>
                }
    }
}

