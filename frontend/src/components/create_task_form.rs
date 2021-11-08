use yew::prelude::*;

#[function_component(CreateTaskForm)]
pub fn create_task_form() -> Html {
    let title = use_state(String::new);
    let description = use_state(String::new);
    let password = use_state(String::new);

    let on_title = Callback::from(move |event: InputEvent| {
        if let Some(data) = event.data() {
            title.set(data)
        }
    });

    let on_description = Callback::from(move |event: InputEvent| {
        if let Some(data) = event.data() {
            description.set(data)
        }
    });

    let on_password = Callback::from(move |event: InputEvent| {
        if let Some(data) = event.data() {
            password.set(data)
        }
    });

    let on_submit = Callback::once(move |_| {
        // TODO: Make network calls
    });

    html! {
        <form class="box">
            <FormHeader/>

            <FormTitle on_input={on_title}/>
            <FormDescription on_input={on_description}/>

            <FormSubmit on_password={on_password} on_submit={on_submit}/>
        </form>
    }
}

#[function_component(FormHeader)]
fn form_header() -> Html {
    let password_text = "You have to provide a
        password for your request.
        With it you can later change the title and description
        or resolve it by yourself.";

    html! {
        <>
            <div class="content has-text-centered">
                <p class="title has-text-dark is-2">{"Submit Request"}</p>
            </div>

            <div class="content columns">
                <p class="column is-8 is-offset-2">{password_text}
                </p>
            </div>
        </>
    }
}

#[derive(PartialEq, Properties)]
struct FormInputProps {
    on_input: Callback<InputEvent>,
}

#[function_component(FormTitle)]
fn form_title(props: &FormInputProps) -> Html {
    html! {
        <div class="content">
            <p class="title has-text-dark is-5 level-left">{"Title"}</p>
            <input class="input" type="text" oninput={&props.on_input}/>
        </div>
    }
}

#[function_component(FormDescription)]
fn form_description(props: &FormInputProps) -> Html {
    html! {
        <div class="content">
            <p class="title has-text-dark is-5 level-left">{"Description"}</p>
            <textarea class="textarea has-fixed-size"
                type="textarea"
                oninput={&props.on_input}/>
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct FormSubmitProps {
    on_password: Callback<InputEvent>,
    on_submit: Callback<MouseEvent>,
}

#[function_component(FormSubmit)]
fn form_submit(props: &FormSubmitProps) -> Html {
    html! {
        <>
            <div class="content">
                <p class="title has-text-dark is-5 level-left">
                    {"Password"}
                </p>
            </div>


            <div class="columns">
                <div class="content column is-4">
                    <input class="input" type="password" oninput={&props.on_password}/>
                 </div>

                <div class="content column">
                    <button type="submit" class="button is-primary" onclick={&props.on_submit}>
                        <strong>{"Submit"}</strong>
                    </button>
                </div>
            </div>
        </>
    }
}
