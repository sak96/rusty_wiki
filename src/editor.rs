use crate::node::split_to_nodes;
use crate::editable_blocks::EditableBlocks;
use std::rc::Rc;

use yew::prelude::*;

pub enum EditorAction {
    /// Replace block at given index with new blocks.
    Replace(usize, String),
    /// Delete block at given index.
    Delete(usize),
    /// Change selected edit block.
    Editable(usize),
    /// Reset Editable to last block.
    ResetEditable,
}

#[derive(Default, Clone)]
pub struct EditorState {
    content: Vec<String>,
    editable: usize,
}

impl Reducible for EditorState {
    type Action = EditorAction;

    fn reduce(mut self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        {
            let &mut EditorState {
                ref mut content,
                ref mut editable,
            } = Rc::make_mut(&mut self);
            match action {
                EditorAction::Replace(idx, new_content) => {
                    if let Some(el) = content.get_mut(idx) {
                        *el = new_content;
                    } else {
                        content.push(new_content);
                    }
                    let c = content.join("\n");
                    *content = split_to_nodes(&c);
                    *editable = content.len();
                }
                EditorAction::Delete(idx) => {
                    content.remove(idx);
                }
                EditorAction::Editable(idx) => {
                    *editable = idx;
                }
                EditorAction::ResetEditable => {
                    *editable = content.len();
                }
            }
            // reset length to proper value.
            if *editable > content.len() {
                *editable = content.len()
            }
        };
        self
    }
}

#[function_component]
pub fn Editor() -> Html {
    let state = use_reducer(EditorState::default);
    let onchange = {
        let state = state.clone();
        Callback::from(move |action| state.dispatch(action))
    };

    html! {
        <>
            <style>{r#"
                * { box-sizing: border-box; }
                .markdown-editor {
                    display: flex;
                    flex-wrap: wrap;
                    min-height: 5 rem;
                }
                .markdown-editor > div {
                    width: 50%;
                    padding: 1rem;
                    border: 2px solid DeepSkyBlue;
                    border-radius: 5px;
                }
                .markdown-editor-help {
                    text-align: center;
                }
                @media (max-width: 500px) {
                    .markdown-editor > div {
                        width: 100%
                    }
                }
            "#}
            </style>
            <div>
            {state.content.iter().enumerate().map(|(i, content)| {
                html!{
                    <EditableBlocks
                        content={content.clone()}
                        editable={state.editable == i}
                        idx={i}
                        onchange={onchange.clone()}
                        />
                }}).collect::<Html>()
            }
            if state.editable >= state.content.len() {
                <EditableBlocks
                    content={"".to_string()}
                    editable={true}
                    idx={state.content.len()}
                    onchange={onchange.clone()}
                />
            }
            </div>
        </>
    }
}
