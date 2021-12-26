use seed::prelude::*;

use crate::{
    api::auth::RegisterPayload,
    model::{
        page::{
            guest::GuestPage,
            requested_guest::{RequestedGuestIndexData, RequestedGuestPage},
        },
        Model,
    },
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
    Register,
    Logout,
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

                    orders
                        .skip()
                        .send_msg(Msg::Api(ApiMsg::Request(RequestApiMsg::Submit(
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

                    orders
                        .skip()
                        .send_msg(Msg::Api(ApiMsg::Request(RequestApiMsg::Edit {
                            task_id: data.task.id.clone(),
                            title,
                            description,
                        })));
                }
            }),
            PageMsg::Login => {
                if let Some(data) = model.user.page().login_data() {
                    let user_name = data
                        .user_name_ref
                        .get()
                        .expect("User name not initialised!")
                        .value();
                    let password = data
                        .password_ref
                        .get()
                        .expect("User name not initialised!")
                        .value();

                    orders
                        .skip()
                        .send_msg(Msg::Api(ApiMsg::Request(RequestApiMsg::Login(
                            user_name, password,
                        ))));
                }
            }
            PageMsg::Register => {
                if let Some(data) = model.user.page().register_data() {
                    let user_name = data
                        .user_name_ref
                        .get()
                        .expect("User name not initialised!")
                        .value();
                    let password = data
                        .password_ref
                        .get()
                        .expect("User name not initialised!")
                        .value();

                    let invite_code = data
                        .invite_code_ref
                        .get()
                        .expect("Invite code not initialised!")
                        .value();

                    orders
                        .skip()
                        .send_msg(Msg::Api(ApiMsg::Request(RequestApiMsg::Register(
                            RegisterPayload::new(user_name, password, invite_code),
                        ))));
                }
            }
            PageMsg::Logout => {
                if let Some(token) = model.user.get_token() {
                    orders
                        .skip()
                        .send_msg(Msg::Api(ApiMsg::Request(RequestApiMsg::Logout(
                            token.clone(),
                        ))));
                }
            }
        }
    }
}
