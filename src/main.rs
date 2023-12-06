use crate::views::main_view::MainView;

mod views;

fn main() {
    yew::Renderer::<MainView>::new().render();
}
