use seed::prelude::*;

use crate::model::{page::GuestPages, user::User, Model};

use super::{
    api::{ApiMsg, RequestApiMsg},
    Msg,
};

pub enum PageMsg {
    SubmitTask,
}

impl PageMsg {
    pub fn update(self, model: &mut Model, orders: &mut impl Orders<Msg>) {
        match self {
            PageMsg::SubmitTask => {
                if let User::Guest(GuestPages::Index {
                    title_ref,
                    description_ref,
                    ..
                }) = &model.user
                {
                    let title = title_ref.get().expect("Title not initialised!").value();
                    let description = description_ref
                        .get()
                        .expect("Description not initialised")
                        .value();
                    orders.send_msg(Msg::Api(ApiMsg::Request(RequestApiMsg::SubmitTask(
                        title,
                        description,
                    ))));
                }
            }
        }
    }
}
