use std::str::FromStr;

use chrono::{DateTime, Utc};
use sqlx::{Pool, Sqlite};

use crate::models::QueriesError;

use super::{generate_random_string, QueriesResult};

#[derive(Debug, FromRow, Serialize)]
pub struct Task {
    id: String,
    title: String,
    body: String,
    state: String,
    created_at: String,
}

#[derive(Debug, PartialEq)]
pub enum TaskState {
    Pending,
    Doing,
    Done,
}

impl Task {
    fn task_state(&self) -> TaskState {
        match self.state.as_str() {
            "pending" => TaskState::Pending,
            "doing" => TaskState::Doing,
            "done" => TaskState::Done,
            _ => unreachable!(),
        }
    }

    fn created_at(&self) -> DateTime<Utc> {
        DateTime::from_str(&self.created_at).unwrap()
    }
}

pub struct TaskQueries<'a> {
    pool: &'a Pool<Sqlite>,
}

impl<'a> TaskQueries<'a> {
    pub fn new(pool: &'a Pool<Sqlite>) -> Self {
        TaskQueries { pool }
    }

    pub async fn get_tasks(&self) -> QueriesResult<Vec<Task>> {
        let mut tasks = query_as!(
            Task,
            "SELECT *
            FROM task
            WHERE state != 'done'"
        )
        .fetch_all(self.pool)
        .await?;

        tasks.sort_by_key(|task| task.created_at());

        debug!("Queries tasks");

        Ok(tasks)
    }

    pub async fn get_task(&self, id: &str) -> QueriesResult<Task> {
        query_as!(Task, "SELECT * FROM task WHERE id = ?", id)
            .fetch_optional(self.pool)
            .await
            .map_err(|err| err.into())
            .and_then(|task| {
                task.map(|task| {
                    debug!("Requested task {}", task.id);
                    task
                })
                .ok_or_else(|| {
                    debug!("Requested task {} not found", id);
                    QueriesError::ItemNotFound(format!("Can't find task with id {}", id))
                })
            })
    }

    pub async fn create_task(&self, title: &str, body: &str) -> QueriesResult<Task> {
        let task_id = generate_random_string(12);

        if title.is_empty() {
            return Err(QueriesError::IllegalState(
                "Title can't be empty!".to_string(),
            ));
        }

        if body.is_empty() {
            return Err(QueriesError::IllegalState(
                "Body can't be empty!".to_string(),
            ));
        }

        let now = Utc::now().to_string();
        query!(
            "INSERT INTO task(id, title, body, state, created_at)
            VALUES ($1, $2, $3, 'pending', $4)",
            task_id,
            title,
            body,
            now
        )
        .execute(self.pool)
        .await?;

        let task = self.get_task(&task_id).await?;

        debug!(target: "USER-ACTION", "Created task {}", task.id);

        Ok(task)
    }

    pub async fn start_task(&self, id: &str) -> QueriesResult<()> {
        let task = self.get_task(id).await?;

        if task.task_state() != TaskState::Pending {
            return Err(QueriesError::IllegalState(
                "Task state is not in Pending!".to_string(),
            ));
        }

        query!("UPDATE task SET state = 'doing' WHERE id = ?", id)
            .execute(self.pool)
            .await?;

        debug!(target: "USER-ACTION", "Started task {}", task.id);

        Ok(())
    }

    pub async fn resolve_task(&self, id: &str) -> QueriesResult<()> {
        let task = self.get_task(id).await?;

        let state = task.task_state();
        if state != TaskState::Pending {
            let err = if state == TaskState::Doing {
                QueriesError::IllegalState("Tutor is processing the task already!".to_string())
            } else {
                QueriesError::IllegalState("Request is already completed!".to_string())
            };

            return Err(err);
        }

        query!("UPDATE task SET state = 'done' WHERE id = ?", id)
            .execute(self.pool)
            .await?;

        debug!(target: "USER-ACTION", "Resolved task {}", task.id);

        Ok(())
    }

    pub async fn complete_task(&self, id: &str) -> QueriesResult<()> {
        let task = self.get_task(id).await?;

        if task.task_state() != TaskState::Doing {
            return Err(QueriesError::IllegalState(
                "Task state is not in Doing!".to_string(),
            ));
        }

        query!("UPDATE task SET state = 'done' WHERE id = ?", id)
            .execute(self.pool)
            .await?;

        debug!(target: "USER-ACTION", "Completed task {}", task.id);

        Ok(())
    }

    pub async fn edit_title(&self, id: &str, title: &str) -> QueriesResult<()> {
        query!("UPDATE task SET title = $1 WHERE id = $2", title, id)
            .execute(self.pool)
            .await?;

        debug!("Edited title of {}", id);

        Ok(())
    }

    pub async fn edit_body(&self, id: &str, body: &str) -> QueriesResult<()> {
        query!("UPDATE task SET body = $1 WHERE id = $2", body, id)
            .execute(self.pool)
            .await?;

        debug!("Edited body of {}", id);

        Ok(())
    }
}
