#[macro_use]
extern crate seed;

use msg::Msg;
use pages::page_view;
use seed::prelude::*;

use model::Model;

mod api;
mod model;
mod msg;
mod pages;
mod views;

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    Model::init(url, orders)
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    msg.update(model, orders)
}

fn view(model: &Model) -> Node<Msg> {
    page_view(model)
}

fn main() {
    App::start("app", init, update, view);
}
