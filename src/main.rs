use crate::views::RootComponent;

pub mod components;
pub mod views;
pub mod routes;

fn main() {
    yew::Renderer::<RootComponent>::new().render();
}
