use std::collections::{HashMap, HashSet};

use seed::prelude::*;
use serde_json::Value;

use crate::{
    api::{
        admin::{create_invite, delete_invite, Invite},
        auth::{log_out, login, register, RegisterPayload, Token},
        refresh::refresh_admin,
        task::{
            finish_task, get_task, get_tasks, process_task, resolve_task, submit_task, update_task,
            Task,
        },
        user::{change_password, change_username, delete_user, ApiUser},
        ApiError, ApiResult,
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
    ProcessTask(String, Task),
    FinishTask(String, Task),
    Login(String, String),
    Register(RegisterPayload),
    Logout(String),
    RefreshRequestedGuest(String),
    RefreshAdmin(String),
    RefreshTutor(String),
    ChangeUsername(String, String, String),
    ChangePassword(String, String, String),
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
            RequestApiMsg::RefreshTutor(token) => {
                get_tasks(token).await.map(ResponseApiMsg::RefreshTutor)
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
            RequestApiMsg::ProcessTask(token, task) => process_task(&token, &task.id)
                .await
                .map(ResponseApiMsg::ProcessTask),
            RequestApiMsg::FinishTask(token, task) => finish_task(&token, &task.id)
                .await
                .map(ResponseApiMsg::FinishTask),
            RequestApiMsg::ChangeUsername(token, user_name, user_name_again) => {
                if user_name == user_name_again {
                    change_username(&token, &user_name).await
                } else {
                    Ok(ApiResult::Err(ApiError {
                        message: "Usernames don't match!".to_string(),
                    }))
                }
                .map(ResponseApiMsg::ChangedSettings)
            }
            RequestApiMsg::ChangePassword(token, password, password_again) => {
                if password == password_again {
                    change_password(&token, &password).await
                } else {
                    Ok(ApiResult::Err(ApiError {
                        message: "Passwords don't match!".to_string(),
                    }))
                }
                .map(ResponseApiMsg::ChangedSettings)
            }
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
