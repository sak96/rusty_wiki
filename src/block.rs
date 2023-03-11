use crate::span::spans_to_html;
use markdown::{Block, ListItem};
use yew::prelude::*;

pub fn block_to_html(blocks: &[Block]) -> Html {
    html! {blocks.iter().map(|block: &Block| html! {<BlockTag block={block.clone()}/>}).collect::<Html>()}
}

pub fn list_to_html(items: &[ListItem]) -> Html {
    html! {items.iter().map(|block: &ListItem|{
        let item = match block {
            ListItem::Simple(spans) => {spans_to_html(spans)},
            ListItem::Paragraph(blocks) => {block_to_html(blocks)},
        };
        html! {<li>{item}</li>}
    }
    ).collect::<Html>()}
}

#[derive(Properties, PartialEq)]
pub struct BlockProps {
    pub block: Block,
}

#[function_component]
pub fn BlockTag(BlockProps { block }: &BlockProps) -> Html {
    match block {
        Block::Hr => html! {<hr/>},
        Block::Header(spans, level) => {
            html! { <@{format!("h{}",level)}>{spans_to_html(spans)}</@> }
        }
        Block::Paragraph(spans) => html! { <p>{spans_to_html(spans)}</p> },
        Block::Blockquote(blocks) => html! { <code>{block_to_html(blocks)}</code> },
        Block::CodeBlock(_, code) => html! { <pre><code>{code}</code></pre> },
        Block::OrderedList(items, ty) => {
            html! { <ol type={ty.0.to_string()}>{list_to_html(items)}</ol>}
        }
        Block::UnorderedList(items) => html! { <ul>{list_to_html(items)}</ul>},
        Block::Raw(value) => html! {{value}},
    }
}
