use yew::prelude::*;

use crate::components::{CreateTaskForm, GuestNavBar};

#[function_component(Index)]
pub fn index() -> Html {
    html! {
        <section class="hero is-info is-fullheight">
            <div class="hero-head">
                <GuestNavBar/>
            </div>

            <div class="hero-body container">
                <div class="columns">
                    <div class="column is-8 is-offset-2">
                        <CreateTaskForm/>
                    </div>
                </div>
            </div>
        </section>
    }
}
