use seed::prelude::*;

use crate::model::{Model, Page};

pub enum Msg {
    ChangeMenu,
    UrlChanged(subs::UrlChanged),
}

impl Msg {
    pub fn update(self, model: &mut Model, _: &mut impl Orders<Msg>) {
        match self {
            Msg::ChangeMenu => model.change_menu(),
            Msg::UrlChanged(subs::UrlChanged(url)) => model.page = Page::init(url),
        }

        model.save()
    }
}
