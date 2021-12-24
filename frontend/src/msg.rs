use seed::prelude::*;

use crate::model::Model;

pub enum Msg {
    ChangeMenu,
}

impl Msg {
    pub fn update(&self, model: &mut Model, _: &mut impl Orders<Msg>) {
        match self {
            Msg::ChangeMenu => model.change_menu(),
        }
    }
}
