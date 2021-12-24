use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct EditTaskProps {
    pub header: String,
    pub start_title: Option<String>,
    pub start_description: Option<String>,
    pub title_ref: NodeRef,
    pub description_ref: NodeRef,
    pub children: Children,
}

#[function_component(EditTask)]
pub fn edit_task(props: &EditTaskProps) -> Html {
    html! {
        <div class="box">
            <div class="content has-text-centered">
                <p class="title has-text-dark is-2">{&props.header}</p>
            </div>

            <div class="content">
                <p class="title has-text-dark is-5 level-left">{"Title"}</p>
                <input class="input"
                type="text"
                size="50"
                ref={props.title_ref.clone()}
                value={props.start_title.clone()}/>
            </div>

            <div class="content">
                <p class="title has-text-dark is-5 level-left">{"Description"}</p>
                <textarea class="textarea has-fixed-size"
                    type="textarea"
                    size="50"
                    ref={props.description_ref.clone()}
                    value={props.start_description.clone()}/>
            </div>

            {props.children.clone()}
        </div>
    }
}
