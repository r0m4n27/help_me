use seed::{
    prelude::{web_sys::HtmlInputElement, *},
    Attrs,
};

pub fn box_header_view<Msg>(header: &str) -> Node<Msg> {
    div![
        C!["content", "has-text-centered"],
        p![C!["title", "has-text-dark", "is-2"], header],
    ]
}

pub fn input_view<Msg>(
    title: &str,
    input_ref: &ElRef<HtmlInputElement>,
    attrs: Attrs,
) -> Node<Msg> {
    div![
        C!["content"],
        p![C!["title", "has-text-dark", "is-5"], title],
        input![C!["input"], el_ref(input_ref), attrs]
    ]
}
