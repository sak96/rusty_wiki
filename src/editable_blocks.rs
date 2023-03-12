use crate::editor::EditorAction;
use crate::node::node_to_html;
use web_sys::{HtmlElement, KeyboardEvent};

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EditableBlockProps {
    pub content: String,
    pub editable: bool,
    pub idx: usize,
    pub onchange: Callback<EditorAction>,
}

#[function_component]
pub fn EditableBlocks(
    EditableBlockProps {
        content,
        editable,
        idx,
        onchange,
    }: &EditableBlockProps,
) -> Html {
    let content_editable = use_node_ref();
    let input_value_handle = use_state(String::default);
    let idx = *idx;
    {
        let input_value_handle = input_value_handle.clone();
        let content_editable = content_editable.clone();
        let content = content.clone();
        use_effect_with_deps(
            move |(content, _idx)| {
                input_value_handle.set(content.to_string());
                if let Some(el) = content_editable.cast::<HtmlElement>() {
                    el.set_inner_text(content.as_str());
                }
            },
            (content, idx),
        )
    }
    let oninput = {
        let input_value_handle = input_value_handle.clone();
        Callback::from(move |e: InputEvent| {
            let el = e
                .target_dyn_into::<HtmlElement>()
                .expect("element conversion failed for input element");
            input_value_handle.set(el.inner_text());
        })
    };
    let make_editable = {
        let onchange = onchange.clone();
        Callback::from(move |_| {
            onchange.emit(EditorAction::Editable(idx));
        })
    };
    let save = {
        let onchange = onchange.clone();
        let input_value_handle = input_value_handle.clone();
        Callback::from(move |_| {
            onchange.emit(EditorAction::Replace(idx, input_value_handle.to_string()));
        })
    };
    let del = {
        let onchange = onchange.clone();
        Callback::from(move |_| onchange.emit(EditorAction::Delete(idx)))
    };
    let exit = {
        let onchange = onchange.clone();
        let input_value_handle = input_value_handle.clone();
        let content = content.clone();
        Callback::from(move |_| {
            input_value_handle.set(content.to_string());
            onchange.emit(EditorAction::ResetEditable);
        })
    };
    let save_exit_del = {
        let exit = exit.clone();
        let save = save.clone();
        let del = del.clone();
        Callback::from(move |k: KeyboardEvent| match k.key().as_str() {
            "Enter" if k.shift_key() => {
                save.emit(());
                k.prevent_default();
            }
            "Delete" if k.shift_key() => {
                del.emit(());
                k.prevent_default();
            }
            "Escape" => {
                exit.emit(());
                k.prevent_default();
            }
            _ => {}
        })
    };

    let mut class = None;
    if *editable {
        class = Some("markdown-editor");
    }

    html! {
        <>
            <div class={class}>
                <div
                    hidden={!*editable}
                    onkeydown={save_exit_del}
                    {oninput}
                    contenteditable="true"
                    ref={content_editable}
                    />
                    <div ondblclick={make_editable} >{node_to_html(&input_value_handle)}</div>
            </div>
            <div class="markdown-editor-help" hidden={!*editable}>
                <div>
                    <button onclick={move |_| save.emit(())}>{"\u{1F4BE}"}</button>
                    if !content.is_empty(){
                        <button onclick={move |_| del.emit(())}>{"\u{1F5D1}"}</button>
                        <button onclick={move |_| exit.emit(())}>{"\u{2716}"}</button>
                    }
                </div>
                <strong>{"dbl-click -> select | Shift+enter -> save |  Shift+Del -> delete | ESC -> exit"}</strong>
            </div>
        </>
    }
}
