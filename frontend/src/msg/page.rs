use seed::prelude::{
    web_sys::{HtmlInputElement, HtmlTextAreaElement},
    *,
};

use crate::{
    api::{admin::Invite, auth::RegisterPayload, task::Task, user::ApiUser},
    model::{
        page::{
            guest::GuestPage,
            requested_guest::{RequestedGuestIndexData, RequestedGuestPage},
        },
        Model,
    },
};

use super::{request::RequestApiMsg, Msg};

trait InputExt {
    fn value(&self) -> String;
}

impl InputExt for ElRef<HtmlInputElement> {
    fn value(&self) -> String {
        self.get().expect("Input not initialised!").value()
    }
}

impl InputExt for ElRef<HtmlTextAreaElement> {
    fn value(&self) -> String {
        self.get().expect("Textarea not initialised!").value()
    }
}

pub enum PageMsg {
    SubmitTask,
    EditTask,
    ResolveTask,
    CancelEdit,
    ConfirmEdit,
    Login,
    Register,
    Logout,
    CreateInvite,
    DeleteInvite(Invite),
    DeleteUser(ApiUser),
    ProcessTask(Task),
    FinishTask(Task),
    ChangeUsername,
    ChangePassword,
}

impl PageMsg {
    pub fn update(self, model: &mut Model, orders: &mut impl Orders<Msg>) {
        match self {
            PageMsg::SubmitTask => model.user.as_guest(|data| {
                if let GuestPage::Index(index_data) = &data.0 {
                    let title = index_data.title_ref.value();
                    let description = index_data.description_ref.value();

                    orders
                        .skip()
                        .send_msg(RequestApiMsg::Submit(title, description).into());
                }
            }),
            PageMsg::EditTask => model.user.as_requested_guest(|data| data.start_editing()),
            PageMsg::ResolveTask => model.user.as_requested_guest(|data| {
                orders.send_msg(RequestApiMsg::Resolve(data.task.id.clone()).into());
            }),
            PageMsg::CancelEdit => model.user.as_requested_guest(|data| data.cancel_editing()),
            PageMsg::ConfirmEdit => model.user.as_requested_guest(|data| {
                if let RequestedGuestPage::Index(RequestedGuestIndexData::Editing {
                    title_ref,
                    description_ref,
                    ..
                }) = &data.page
                {
                    let title = title_ref.value();
                    let description = description_ref.value();

                    orders.skip().send_msg(
                        RequestApiMsg::Edit {
                            task_id: data.task.id.clone(),
                            title,
                            description,
                        }
                        .into(),
                    );
                }
            }),
            PageMsg::Login => {
                if let Some(data) = model.user.page().login_data() {
                    let user_name = data.user_name_ref.value();
                    let password = data.password_ref.value();

                    orders
                        .skip()
                        .send_msg(RequestApiMsg::Login(user_name, password).into());
                }
            }
            PageMsg::Register => {
                if let Some(data) = model.user.page().register_data() {
                    let user_name = data.user_name_ref.value();
                    let password = data.password_ref.value();

                    let invite_code = data.invite_code_ref.value();

                    orders.skip().send_msg(
                        RequestApiMsg::Register(RegisterPayload::new(
                            user_name,
                            password,
                            invite_code,
                        ))
                        .into(),
                    );
                }
            }
            PageMsg::Logout => {
                if let Some(token) = model.user.get_token() {
                    orders
                        .skip()
                        .send_msg(RequestApiMsg::Logout(token.clone()).into());
                }
            }
            PageMsg::CreateInvite => model.user.as_admin(|data| {
                orders
                    .skip()
                    .send_msg(RequestApiMsg::CreateInvite(data.token.clone()).into());
            }),
            PageMsg::DeleteInvite(invite) => model.user.as_admin(|data| {
                orders
                    .skip()
                    .send_msg(RequestApiMsg::DeleteInvite(data.token.clone(), invite).into());
            }),
            PageMsg::DeleteUser(user) => model.user.as_admin(|data| {
                orders
                    .skip()
                    .send_msg(RequestApiMsg::DeleteUser(data.token.clone(), user).into());
            }),
            PageMsg::ProcessTask(task) => model.user.as_tutor(|data| {
                orders
                    .skip()
                    .send_msg(RequestApiMsg::ProcessTask(data.token.clone(), task).into());
            }),
            PageMsg::FinishTask(task) => model.user.as_tutor(|data| {
                orders
                    .skip()
                    .send_msg(RequestApiMsg::FinishTask(data.token.clone(), task).into());
            }),
            PageMsg::ChangeUsername => {
                if let Some(data) = model.user.page().settings_data() {
                    if let Some(token) = model.user.get_token() {
                        let user_name = data.user_name_ref.value();
                        let user_name_again = data.user_name_again_ref.value();

                        orders.skip().send_msg(
                            RequestApiMsg::ChangeUsername {
                                token: token.clone(),
                                user_name,
                                user_name_again,
                            }
                            .into(),
                        );
                    }
                }
            }
            PageMsg::ChangePassword => {
                if let Some(data) = model.user.page().settings_data() {
                    if let Some(token) = model.user.get_token() {
                        let password = data.password_ref.value();
                        let password_again = data.password_again_ref.value();

                        orders.skip().send_msg(
                            RequestApiMsg::ChangePassword {
                                token: token.clone(),
                                password,
                                password_again,
                            }
                            .into(),
                        );
                    }
                }
            }
        }
    }
}
