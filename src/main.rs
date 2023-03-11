use yew::prelude::*;

mod block;
mod editor;
mod span;

#[function_component]
fn App() -> Html {
    html! {<editor::Editor/>}
}

fn main() {
    yew::Renderer::<App>::new().render();
}
