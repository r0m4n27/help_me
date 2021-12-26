use std::collections::HashSet;

use seed::prelude::*;
use serde_json::Value;

use crate::{
    api::{
        admin::{create_invite, delete_invite, Invite},
        auth::{log_out, login, register, RegisterPayload, Token},
        refresh::refresh_admin,
        task::{get_task, resolve_task, submit_task, update_task, Task},
        user::{delete_user, ApiUser},
        ApiResult,
    },
    model::Model,
};

use super::{log_err, Msg};

pub enum ApiMsg {
    Request(RequestApiMsg),
    Response(ResponseApiMsg),
}

impl ApiMsg {
    pub fn update(self, model: &mut Model, orders: &mut impl Orders<Msg>) {
        match self {
            ApiMsg::Request(request) => {
                orders
                    .skip()
                    .perform_cmd(async move { request.make_request().await });
            }
            ApiMsg::Response(response) => response.update(model, orders),
        }
    }
}

pub enum RequestApiMsg {
    Submit(String, String),
    Resolve(String),
    Edit {
        task_id: String,
        title: String,
        description: String,
    },
    CreateInvite(String),
    DeleteInvite(String, Invite),
    DeleteUser(String, ApiUser),
    Login(String, String),
    Register(RegisterPayload),
    Logout(String),
    RefreshRequestedGuest(String),
    RefreshAdmin(String),
}

impl RequestApiMsg {
    pub async fn make_request(self) -> Option<Msg> {
        let result = match self {
            RequestApiMsg::Submit(title, description) => submit_task(&title, &description)
                .await
                .map(ResponseApiMsg::Submit),
            RequestApiMsg::Resolve(task_id) => {
                resolve_task(&task_id).await.map(ResponseApiMsg::Resolve)
            }
            RequestApiMsg::Edit {
                task_id,
                title,
                description,
            } => update_task(&task_id, &title, &description)
                .await
                .map(ResponseApiMsg::Edit),
            RequestApiMsg::Login(user_name, password) => login(&user_name, &password)
                .await
                .map(ResponseApiMsg::Login),
            RequestApiMsg::Register(payload) => register(&payload).await.map(ResponseApiMsg::Login),
            RequestApiMsg::Logout(token) => log_out(&token).await.map(ResponseApiMsg::Logout),
            RequestApiMsg::RefreshRequestedGuest(task_id) => get_task(&task_id)
                .await
                .map(ResponseApiMsg::RefreshRequestedGuest),
            RequestApiMsg::RefreshAdmin(token) => {
                refresh_admin(token).await.map(ResponseApiMsg::RefreshAdmin)
            }
            RequestApiMsg::CreateInvite(token) => create_invite(&token)
                .await
                .map(ResponseApiMsg::CreateInvite),
            RequestApiMsg::DeleteInvite(token, invite) => delete_invite(&token, invite)
                .await
                .map(ResponseApiMsg::DeleteInvite),
            RequestApiMsg::DeleteUser(token, user) => delete_user(&token, user)
                .await
                .map(ResponseApiMsg::DeleteUser),
        };

        match result {
            Ok(msg) => Some(Msg::Api(ApiMsg::Response(msg))),
            Err(err) => log_err(err),
        }
    }
}

pub enum ResponseApiMsg {
    Submit(ApiResult<Task>),
    Resolve(ApiResult<Task>),
    Edit(ApiResult<Task>),
    CreateInvite(ApiResult<Invite>),
    DeleteInvite(ApiResult<Invite>),
    DeleteUser(ApiResult<ApiUser>),
    Login(ApiResult<(Token, ApiUser)>),
    Logout(ApiResult<Value>),
    RefreshRequestedGuest(ApiResult<Task>),
    RefreshAdmin(ApiResult<(String, Vec<Invite>, Vec<ApiUser>)>),
}

impl ResponseApiMsg {
    pub fn update(self, model: &mut Model, orders: &mut impl Orders<Msg>) {
        let res = match self {
            ResponseApiMsg::Submit(task) => task.map(|task| model.switch_to_requested_user(task)),
            ResponseApiMsg::Resolve(res) => res.map(|_| model.switch_to_guest()),
            ResponseApiMsg::Edit(res) => res.map(|task| {
                if task.state == "done" {
                    model.switch_to_guest()
                } else {
                    model.user.as_requested_guest(|data| data.update_task(task))
                }
            }),
            ResponseApiMsg::Login(res) => res.map(|(token, user)| {
                if &user.user_type == "admin" {
                    model.switch_to_admin(token.token, HashSet::new(), HashSet::new())
                } else {
                    model.switch_to_tutor(token.token)
                }
                orders.send_msg(Msg::Refresh);
            }),
            ResponseApiMsg::Logout(res) => res.map(|_| model.switch_to_guest()),
            ResponseApiMsg::RefreshRequestedGuest(res) => res.map(|task| {
                if task.state == "done" {
                    model.switch_to_guest()
                } else {
                    model.switch_to_requested_user(task)
                }
            }),
            ResponseApiMsg::RefreshAdmin(res) => res.map(|(token, invites, users)| {
                model.switch_to_admin(
                    token,
                    invites.into_iter().collect(),
                    users.into_iter().collect(),
                )
            }),
            ResponseApiMsg::CreateInvite(res) => {
                res.map(|invite| model.user.as_admin(|admin| admin.add_invite(invite)))
            }
            ResponseApiMsg::DeleteInvite(res) => {
                res.map(|invite| model.user.as_admin(|admin| admin.remove_invite(&invite)))
            }
            ResponseApiMsg::DeleteUser(res) => {
                res.map(|user| model.user.as_admin(|admin| admin.remove_user(&user)))
            }
        };

        if let ApiResult::Err(err) = res {
            model.user.page_mut().set_error_message(err.message)
        }
    }
}
