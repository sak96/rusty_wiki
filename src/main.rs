use yew::prelude::*;

mod block;
mod editable_blocks;
mod editor;
mod span;

#[function_component]
fn App() -> Html {
    html! {<editor::Editor/>}
}

fn main() {
    yew::Renderer::<App>::new().render();
}
