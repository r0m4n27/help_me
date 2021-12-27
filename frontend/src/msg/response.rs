use std::collections::{HashMap, HashSet};

use seed::prelude::*;
use serde_json::Value;

use crate::{
    api::{admin::Invite, auth::Token, task::Task, user::ApiUser, ApiResult},
    model::Model,
};

use super::Msg;

pub enum ResponseApiMsg {
    Submit(ApiResult<Task>),
    Resolve(ApiResult<Task>),
    Edit(ApiResult<Task>),
    CreateInvite(ApiResult<Invite>),
    DeleteInvite(ApiResult<Invite>),
    DeleteUser(ApiResult<ApiUser>),
    ProcessTask(ApiResult<Task>),
    FinishTask(ApiResult<Task>),
    Login(ApiResult<(Token, ApiUser)>),
    Logout(ApiResult<Value>),
    RefreshRequestedGuest(ApiResult<Task>),
    RefreshAdmin(ApiResult<(String, Vec<Invite>, Vec<ApiUser>)>),
    RefreshTutor(ApiResult<(String, Vec<Task>)>),
    ChangedSettings(ApiResult<Value>),
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
                    model.switch_to_tutor(token.token, HashMap::new())
                };
                model.goto_index();
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
            ResponseApiMsg::RefreshTutor(res) => res.map(|(token, tasks)| {
                let map: HashMap<_, _> = tasks
                    .into_iter()
                    .map(|task| (task.id.clone(), task))
                    .collect();

                model.switch_to_tutor(token, map)
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
            ResponseApiMsg::ProcessTask(res) => res.map(|task| {
                model.user.as_tutor(|data| {
                    data.tasks.insert(task.id.clone(), task);
                })
            }),
            ResponseApiMsg::FinishTask(res) => res.map(|task| {
                model.user.as_tutor(|data| {
                    data.tasks.remove(&task.id);
                });

                model.goto_index()
            }),
            ResponseApiMsg::ChangedSettings(res) => res.map(|_| model.goto_index()),
        };

        if let ApiResult::Err(err) = res {
            if err.message == "Provided token is invalid!" {
                model.switch_to_guest();
                model.goto_index()
            } else {
                model.user.page_mut().set_error_message(err.message)
            }
        }
    }
}
