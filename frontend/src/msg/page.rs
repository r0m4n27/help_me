use seed::prelude::*;

use crate::model::{
    page::{
        guest::GuestPage,
        requested_guest::{RequestedGuestIndexData, RequestedGuestPage},
    },
    Model,
};

use super::{
    api::{ApiMsg, RequestApiMsg},
    Msg,
};

pub enum PageMsg {
    Submit,
    Edit,
    Resolve,
    CancelEdit,
    ConfirmEdit,
    Login,
}

impl PageMsg {
    pub fn update(self, model: &mut Model, orders: &mut impl Orders<Msg>) {
        match self {
            PageMsg::Submit => model.user.as_guest(|data| {
                if let GuestPage::Index(index_data) = &data.0 {
                    let title = index_data
                        .title_ref
                        .get()
                        .expect("Title not initialised!")
                        .value();
                    let description = index_data
                        .description_ref
                        .get()
                        .expect("Description not initialised")
                        .value();
                    orders.send_msg(Msg::Api(ApiMsg::Request(RequestApiMsg::Submit(
                        title,
                        description,
                    ))));
                }
            }),
            PageMsg::Edit => model.user.as_requested_guest(|data| data.start_editing()),
            PageMsg::Resolve => model.user.as_requested_guest(|data| {
                orders.send_msg(Msg::Api(ApiMsg::Request(RequestApiMsg::Resolve(
                    data.task.id.clone(),
                ))));
            }),
            PageMsg::CancelEdit => model.user.as_requested_guest(|data| data.cancel_editing()),
            PageMsg::ConfirmEdit => model.user.as_requested_guest(|data| {
                if let RequestedGuestPage::Index(RequestedGuestIndexData::Editing {
                    title_ref,
                    description_ref,
                    ..
                }) = &data.page
                {
                    let title = title_ref.get().expect("Title not initialised!").value();
                    let description = description_ref
                        .get()
                        .expect("Description not initialised")
                        .value();

                    orders.send_msg(Msg::Api(ApiMsg::Request(RequestApiMsg::Edit {
                        task_id: data.task.id.clone(),
                        title,
                        description,
                    })));
                }
            }),
            PageMsg::Login => todo!(),
        }
    }
}
