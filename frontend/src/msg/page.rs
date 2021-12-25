use seed::prelude::*;

use crate::model::{
    page::{GuestPages, RequestedGuestIndexData, RequestedGuestPages},
    user::User,
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
}

impl PageMsg {
    pub fn update(self, model: &mut Model, orders: &mut impl Orders<Msg>) {
        match self {
            PageMsg::Submit => {
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
                    orders.send_msg(Msg::Api(ApiMsg::Request(RequestApiMsg::Submit(
                        title,
                        description,
                    ))));
                }
            }
            PageMsg::Edit => model.user.edit_task(),
            PageMsg::Resolve => {
                if let User::RequestedGuest(task, _) = &model.user {
                    orders.send_msg(Msg::Api(ApiMsg::Request(RequestApiMsg::Resolve(
                        task.id.clone(),
                    ))));
                }
            }
            PageMsg::CancelEdit => model.user.cancel_edit_task(),
            PageMsg::ConfirmEdit => {
                if let User::RequestedGuest(
                    task,
                    RequestedGuestPages::Index {
                        page_data:
                            RequestedGuestIndexData::Editing {
                                title_ref,
                                description_ref,
                            },
                        ..
                    },
                ) = &model.user
                {
                    let title = title_ref.get().expect("Title not initialised!").value();
                    let description = description_ref
                        .get()
                        .expect("Description not initialised")
                        .value();

                    orders.send_msg(Msg::Api(ApiMsg::Request(RequestApiMsg::Edit {
                        task_id: task.id.clone(),
                        title,
                        description,
                    })));
                }
            }
        }
    }
}
