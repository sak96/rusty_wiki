use crate::span::spans_to_html;
use markdown::Block;
use yew::prelude::*;

pub fn block_to_html(blocks: &[Block]) -> Html {
    html! {blocks.iter().map(|block: &Block| html! {<BlockTag block={block.clone()}/>}).collect::<Html>()}
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
        Block::Blockquote(_) => todo!(),
        Block::CodeBlock(_, _) => todo!(),
        Block::OrderedList(_, _) => todo!(),
        Block::UnorderedList(_) => todo!(),
        Block::Raw(_) => todo!(),
    }
}
