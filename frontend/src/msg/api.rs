use seed::prelude::*;

use crate::{
    api::{
        task::{submit_task, Task},
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
    SubmitTask(String, String),
}

impl RequestApiMsg {
    pub async fn make_request(self) -> Option<Msg> {
        let result = match self {
            RequestApiMsg::SubmitTask(title, description) => {
                submit_task(&title, &description).await
            }
        };

        match result {
            Ok(msg) => Some(Msg::Api(ApiMsg::Response(msg))),
            Err(err) => log_err(err),
        }
    }
}

pub enum ResponseApiMsg {
    SubmitTask(ApiResult<Task>),
}

impl ResponseApiMsg {
    pub fn update(self, model: &mut Model) {
        let res = match self {
            ResponseApiMsg::SubmitTask(task) => {
                task.map(|task| model.switch_to_requested_user(task))
            }
        };

        if let ApiResult::Err(err) = res {
            model.user.update_error(err.message)
        }
    }
}
