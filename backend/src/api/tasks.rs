use rocket::{serde::json::Json, Route, State};

use crate::models::{Queries, Task};

use super::{
    api_result::{ok, ApiError, ApiResult},
    guards::UserGuard,
};

#[derive(Deserialize)]
struct CreateTaskForm {
    title: String,
    body: String,
}

#[derive(Deserialize)]
struct EditTaskForm {
    title: Option<String>,
    body: Option<String>,
}

#[derive(Serialize)]
struct TaskResponse {
    id: String,
    title: String,
    body: String,
    state: String,
    created_at: String,
    how_many_ahead: usize,
}

impl TaskResponse {
    fn new(task: Task, how_many_ahead: usize) -> Self {
        Self {
            id: task.id,
            title: task.title,
            body: task.body,
            state: task.state,
            created_at: task.created_at,
            how_many_ahead,
        }
    }
}

#[get("/")]
async fn get_tasks(
    user_guard: Result<UserGuard<'_>, ApiError>,
    queries: &State<Queries>,
) -> ApiResult<Vec<TaskResponse>> {
    user_guard?;

    let tasks = queries.task.get_tasks().await?;

    let mut output = Vec::new();

    for task in tasks.into_iter() {
        let ahead = queries.task.how_many_ahead(&task).await?;
        output.push(TaskResponse::new(task, ahead))
    }

    ok(output)
}

#[post("/", data = "<data>")]
async fn create_task(
    queries: &State<Queries>,
    data: Json<CreateTaskForm>,
) -> ApiResult<TaskResponse> {
    let task = queries.task.create_task(&data.title, &data.body).await?;
    let ahead = queries.task.how_many_ahead(&task).await?;

    ok(TaskResponse::new(task, ahead))
}

#[get("/<task_id>")]
async fn get_task(queries: &State<Queries>, task_id: String) -> ApiResult<TaskResponse> {
    let task = queries.task.get_task(&task_id).await?;
    let ahead = queries.task.how_many_ahead(&task).await?;

    ok(TaskResponse::new(task, ahead))
}

#[post("/<task_id>/resolve")]
async fn resolve_task(queries: &State<Queries>, task_id: String) -> ApiResult<TaskResponse> {
    queries.task.resolve_task(&task_id).await?;
    let task = queries.task.get_task(&task_id).await?;
    let ahead = queries.task.how_many_ahead(&task).await?;

    ok(TaskResponse::new(task, ahead))
}

#[post("/<task_id>/start")]
async fn start_task(
    user_guard: Result<UserGuard<'_>, ApiError>,
    queries: &State<Queries>,
    task_id: String,
) -> ApiResult<TaskResponse> {
    user_guard?;
    queries.task.start_task(&task_id).await?;
    let task = queries.task.get_task(&task_id).await?;
    let ahead = queries.task.how_many_ahead(&task).await?;

    ok(TaskResponse::new(task, ahead))
}

#[post("/<task_id>/complete")]
async fn complete_task(
    user_guard: Result<UserGuard<'_>, ApiError>,
    queries: &State<Queries>,
    task_id: String,
) -> ApiResult<TaskResponse> {
    user_guard?;
    queries.task.complete_task(&task_id).await?;
    let task = queries.task.get_task(&task_id).await?;
    let ahead = queries.task.how_many_ahead(&task).await?;

    ok(TaskResponse::new(task, ahead))
}

#[patch("/<task_id>", data = "<data>")]
async fn edit_task(
    queries: &State<Queries>,
    data: Json<EditTaskForm>,
    task_id: String,
) -> ApiResult<TaskResponse> {
    if let Some(ref title) = data.title {
        queries.task.edit_title(&task_id, title).await?;
    }

    if let Some(ref body) = data.body {
        queries.task.edit_body(&task_id, body).await?;
    }
    let task = queries.task.get_task(&task_id).await?;
    let ahead = queries.task.how_many_ahead(&task).await?;

    ok(TaskResponse::new(task, ahead))
}

pub fn tasks_routes() -> Vec<Route> {
    routes![
        get_tasks,
        create_task,
        get_task,
        resolve_task,
        start_task,
        complete_task,
        edit_task
    ]
}
