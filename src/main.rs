use yew::prelude::*;

mod editable_blocks;
mod editor;
mod node;

#[function_component]
fn App() -> Html {
    html! {<editor::Editor/>}
}

fn main() {
    yew::Renderer::<App>::new().render();
}
