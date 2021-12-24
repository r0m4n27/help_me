use core::fmt;

use seed::prelude::*;

use crate::{
    api::{
        task::{submit_task, Task},
        ApiResult,
    },
    model::{Model, Page, User},
};

pub enum Msg {
    ChangeMenu,
    UrlChanged(subs::UrlChanged),
    RequestApi(RequestApiMsg),
    ResultApi(ResultApiMsg),
}

impl Msg {
    pub fn update(self, model: &mut Model, orders: &mut impl Orders<Msg>) {
        match self {
            Msg::ChangeMenu => model.change_menu(),
            Msg::UrlChanged(subs::UrlChanged(url)) => model.page = Page::init(url),
            Msg::RequestApi(msg) => {
                orders
                    .skip()
                    .perform_cmd(async move { msg.make_request().await });
            }
            Msg::ResultApi(msg) => msg.update(model, orders),
        }

        model.save()
    }
}

pub enum RequestApiMsg {
    SubmitTask(String, String),
}

impl RequestApiMsg {
    async fn make_request(self) -> Option<Msg> {
        let result = match self {
            RequestApiMsg::SubmitTask(title, description) => submit_task(&title, &description)
                .await
                .map(ResultApiMsg::SubmitTask),
        };

        match result {
            Ok(msg) => Some(Msg::ResultApi(msg)),
            Err(err) => {
                log_err(err);
                None
            }
        }
    }
}

pub enum ResultApiMsg {
    SubmitTask(ApiResult<Task>),
}

impl ResultApiMsg {
    pub fn update(self, model: &mut Model, _: &mut impl Orders<Msg>) {
        let res = match self {
            ResultApiMsg::SubmitTask(task) => {
                task.map(|task| model.user = User::RequestedGuest(task))
            }
        };

        if let ApiResult::Err(err) = res {
            model.page.update_error(err.message)
        }
    }
}

fn log_err<T: fmt::Debug>(err: T) -> Option<Msg> {
    let message = format!("{:?}", err);
    log_1(&message.into());
    None
}
