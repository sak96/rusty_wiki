use markdown_it::MarkdownIt;
use yew::prelude::*;

lazy_static! {
    static ref MD_PRASER: MarkdownIt = {
        let mut parser = MarkdownIt::new();
        markdown_it::plugins::cmark::add(&mut parser);
        markdown_it::plugins::extra::add(&mut parser);
        parser
    };
}

pub fn split_to_nodes(text: &str) -> Vec<String> {
    MD_PRASER
        .parse(text)
        .children
        .iter()
        .filter_map(|node| {
            node.srcmap.map(|pos| {
                let (x, y) = pos.get_byte_offsets();
                text[x..y].replace("\n\n", "\n")
            })
        })
        .collect()
}

pub fn node_to_html(content: &str) -> Html {
    let md_html = MD_PRASER.parse(content).render();
    Html::from_html_unchecked(AttrValue::from(md_html))
}
