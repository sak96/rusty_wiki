use web_sys::HtmlInputElement;
use yew::*;

#[function_component]
fn App() -> Html {
    let input_node_ref = use_node_ref();

    let input_value_handle = use_state(String::default);

    let oninput = {
        let input_value_handle = input_value_handle.clone();
        Callback::from(move |e: InputEvent| {
            let el = e.target_unchecked_into::<HtmlInputElement>();
            // if let Some(text) = el.inner_text() {
            // log::info!("{:?}", text);
            input_value_handle.set(el.inner_text());
            // }
        })
    };

    html! {
        <>
            <style>{r#"
                * { box-sizing: border-box; }
                .markdown-editor {
                    display: flex;
                    min-height: 100vh;
                    flex-wrap: wrap;
                }
                .markdown-editor > div {
                    align-self: stretch;
                    width: 50%;
                    padding: 1rem;
                }
            "#}
            </style>
            <div class={classes!("markdown-editor")}>
                <div {oninput} ref={input_node_ref} contenteditable="true" />
                <div>{input_value_handle.to_string()}</div>
            </div>
        </>
    }
}
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
