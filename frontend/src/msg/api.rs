use seed::prelude::*;

use crate::{
    api::{
        task::{resolve_task, submit_task, update_task, Task},
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
            ApiMsg::Response(response) => response.update(model),
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
}

impl ResponseApiMsg {
    pub fn update(self, model: &mut Model) {
        let res = match self {
            ResponseApiMsg::Submit(task) => task.map(|task| model.switch_to_requested_user(task)),
            ResponseApiMsg::Resolve(res) => res.map(|_| model.switch_to_guest()),
            ResponseApiMsg::Edit(res) => {
                res.map(|task| model.user.as_requested_guest(|data| data.update_task(task)))
            }
        };

        if let ApiResult::Err(err) = res {
            model.user.page_mut().set_error_message(err.message)
        }
    }
}
