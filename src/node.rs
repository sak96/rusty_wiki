use markdown::{Options, ParseOptions};
use yew::prelude::*;

pub fn split_to_nodes(text: &str) -> Vec<String> {
    markdown::to_mdast(text, &ParseOptions::gfm())
        .unwrap()
        .children()
        .map(|nodes| {
            nodes
                .iter()
                .map(|node| node.to_string())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

pub fn node_to_html(content: &str) -> Html {
    let md_html =
        markdown::to_html_with_options(content, &Options::gfm()).expect("parsing failed!");
    Html::from_html_unchecked(AttrValue::from(md_html))
}
