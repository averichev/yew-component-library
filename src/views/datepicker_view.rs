use chrono::NaiveDate;
use pure_rust_locales::Locale;
use yew::{html, Component, Context, Html};
use yew_datepicker::Datepicker;

pub struct DatepickerView {
    selected_date: Option<NaiveDate>,
    locale: Locale,
}

pub enum DatepickerViewMessage {
    DateSelect(NaiveDate),
}

impl Component for DatepickerView {
    type Message = DatepickerViewMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        DatepickerView {
            selected_date: None,
            locale: Locale::ru_RU,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DatepickerViewMessage::DateSelect(date) => {
                self.selected_date = Some(date);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let context = ctx.link().clone();
        let on_select =
            move |date: NaiveDate| context.send_message(DatepickerViewMessage::DateSelect(date));
        html! {
            <>
            <Datepicker {on_select} />
            <div>{self.selected_date_string()}</div>
            </>
        }
    }
}

impl DatepickerView {
    fn selected_date_string(&self) -> String {
        match self.selected_date {
            None => "".to_string(),
            Some(date) => date.format_localized("%v", self.locale).to_string(),
        }
    }
}
