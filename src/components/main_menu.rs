use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::MainRouter;

pub struct MainMenuComponent {}

impl Component for MainMenuComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        MainMenuComponent {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let navigator = ctx.link().navigator().expect("NO NAVIGATOR");
        let onclick = Callback::from(move |_| navigator.push(&MainRouter::Datepicker));
        html! {
            <>
            <div class="list-group">

                    <button {onclick} type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>


            </div>
            </>
        }
    }
}
