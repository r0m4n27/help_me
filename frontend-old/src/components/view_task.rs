use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ViewTaskProps {
    pub header: String,
    pub title: String,
    pub description: String,
    pub children: Children,
}

#[function_component(ViewTask)]
pub fn view_task(props: &ViewTaskProps) -> Html {
    html! {
        <div class="box">
            <div class="content has-text-centered">
                <p class="title has-text-dark is-2">{&props.header}</p>
            </div>

            <div class="content">
                <p class="is-size-4 has-text-weight-bold" style="word-wrap: break-word;">{&props.title}</p>
            </div>

            <div class="content">
                <p class="is-size-5" style="word-wrap: break-word;">{&props.description}</p>
            </div>

            {props.children.clone()}
        </div>
    }
}
