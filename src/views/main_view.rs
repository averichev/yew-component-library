use chrono::NaiveDate;
use pure_rust_locales::Locale;
use yew::{html, Component, Context, Html};
use yew_datepicker::Datepicker;

pub struct MainView {
    selected_date: Option<NaiveDate>,
    locale: Locale
}

pub enum MainViewMessage {
    DateSelect(NaiveDate),
}

impl Component for MainView {
    type Message = MainViewMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        MainView { selected_date: None, locale: Locale::ru_RU }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MainViewMessage::DateSelect(date) => {
                self.selected_date = Some(date);
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let context = ctx.link().clone();
        let on_select = move |date: NaiveDate| context.send_message(MainViewMessage::DateSelect(date));

        html! {
            <>
            <h1>{"Hello world!"}</h1>
            <Datepicker {on_select} />
            <div>{self.selected_date_string()}</div>
            </>
        }
    }
}


impl MainView{
    fn selected_date_string (&self) -> String{
        match self.selected_date {
            None => {
                "".to_string()
            }
            Some(date) => {
                date.format_localized("%v", self.locale).to_string()
            }
        }
    }
}