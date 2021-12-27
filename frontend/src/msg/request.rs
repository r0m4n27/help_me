use crate::api::{
    admin::{create_invite, delete_invite, Invite},
    auth::{log_out, login, register, RegisterPayload},
    refresh::refresh_admin,
    task::{
        finish_task, get_task, get_tasks, process_task, resolve_task, submit_task, update_task,
        Task,
    },
    user::{change_password, change_username, delete_user, ApiUser},
    ApiError, ApiResult,
};

use super::{log_err, response::ResponseApiMsg, Msg};

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
    ChangeUsername {
        token: String,
        user_name: String,
        user_name_again: String,
    },
    ChangePassword {
        token: String,
        password: String,
        password_again: String,
    },
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
            RequestApiMsg::ChangeUsername {
                token,
                user_name,
                user_name_again,
            } => if user_name == user_name_again {
                change_username(&token, &user_name).await
            } else {
                Ok(ApiResult::Err(ApiError {
                    message: "Usernames don't match!".to_string(),
                }))
            }
            .map(ResponseApiMsg::ChangedSettings),
            RequestApiMsg::ChangePassword {
                token,
                password,
                password_again,
            } => if password == password_again {
                change_password(&token, &password).await
            } else {
                Ok(ApiResult::Err(ApiError {
                    message: "Passwords don't match!".to_string(),
                }))
            }
            .map(ResponseApiMsg::ChangedSettings),
        };

        match result {
            Ok(msg) => Some(Msg::Response(msg)),
            Err(err) => log_err(err),
        }
    }
}
