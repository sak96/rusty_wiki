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
        Span::Link(_, _, _) => todo!(),
        Span::Image(_, _, _) => todo!(),
        Span::Emphasis(_) => todo!(),
        Span::Strong(_) => todo!(),
    }
}
