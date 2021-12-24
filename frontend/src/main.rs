use seed::{prelude::*, *};

pub fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model { counter: 0 }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
pub struct Model {
    counter: i32,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
pub enum Msg {
    Increment,
}

// `update` describes how to handle each `Msg`.
pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter += 1,
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
pub fn view(model: &Model) -> Node<Msg> {
    div![
        "This is a counter: ",
        C!["counter"],
        button![model.counter, ev(Ev::Click, |_| Msg::Increment),],
    ]
}

fn main() {
    App::start("app", init, update, view);
}
