use markdown::Span;
use yew::prelude::*;

pub fn spans_to_html(spans: &[Span]) -> Html {
    html! {spans.iter().map(|span: &Span| html! {<SpanTag span={span.clone()}/>}).collect::<Html>()}
}

#[derive(Properties, PartialEq)]
pub struct SpanProps {
    pub span: Span,
}

#[function_component]
pub fn SpanTag(SpanProps { span }: &SpanProps) -> Html {
    match span {
        Span::Break => html! {<br/>},
        Span::Text(value) => html! {<span>{value}</span>},
        Span::Code(value) => html! {<code>{value}</code>},
        Span::Link(text, href, title) => {
            html! {
            <a href={href.to_string()} title={title.clone().unwrap_or_default()}>{text}</a>
            }
        }
        Span::Image(alt, src, title) => {
            html! { <img src={src.to_string()} alt={alt.to_string()} title={title.clone().unwrap_or_default()} /> }
        }
        Span::Emphasis(spans) => html! {<i>{spans_to_html(spans)}</i>},
        Span::Strong(spans) => html! {<b>{spans_to_html(spans)}</b>},
    }
}
