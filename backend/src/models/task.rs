use sqlx::{Pool, Sqlite};

use crate::models::QueriesError;

use super::{generate_random_string, QueriesResult};

#[derive(Debug, FromRow, Serialize)]
pub struct Task {
    id: String,
    title: String,
    body: String,
    state: String,
    pin: i64,
}

#[derive(Debug, PartialEq)]
pub enum TaskState {
    Pending,
    Doing,
    Done,
}

impl Task {
    pub fn task_state(&self) -> TaskState {
        match self.state.as_str() {
            "pending" => TaskState::Pending,
            "doing" => TaskState::Doing,
            "done" => TaskState::Done,
            _ => unreachable!(),
        }
    }
}

pub struct TaskQueries<'a> {
    pool: &'a Pool<Sqlite>,
}

const WRONG_PIN_MESSAGE: &str = "Wrong pin provided!";

impl<'a> TaskQueries<'a> {
    pub fn new(pool: &'a Pool<Sqlite>) -> Self {
        TaskQueries { pool }
    }

    pub async fn get_tasks(&self) -> QueriesResult<Vec<Task>> {
        let tasks = query_as!(Task, "SELECT * from task")
            .fetch_all(self.pool)
            .await?;

        Ok(tasks)
    }

    pub async fn get_task(&self, id: &str) -> QueriesResult<Task> {
        query_as!(Task, "SELECT * FROM task WHERE id = ?", id)
            .fetch_optional(self.pool)
            .await
            .map_err(|err| err.into())
            .and_then(|task| {
                task.ok_or_else(|| {
                    QueriesError::ItemNotFound(format!("Can't find task with id {}", id))
                })
            })
    }

    pub async fn create_task(&self, title: &str, body: &str, pin: i64) -> QueriesResult<Task> {
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

        query!(
            "INSERT INTO task(id, title, body, state, pin)
            VALUES ($1, $2, $3, 'pending', $4)",
            task_id,
            title,
            body,
            pin
        )
        .execute(self.pool)
        .await?;

        let task = self.get_task(&task_id).await?;

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

        Ok(())
    }

    pub async fn resolve_task(&self, id: &str, pin: i64) -> QueriesResult<()> {
        let task = self.get_task(id).await?;

        if task.pin != pin {
            return Err(QueriesError::IllegalState(WRONG_PIN_MESSAGE.to_string()));
        }

        if task.task_state() != TaskState::Pending {
            return Err(QueriesError::IllegalState(
                "Task state is not in Pending!".to_string(),
            ));
        }

        query!("UPDATE task SET state = 'done' WHERE id = ?", id)
            .execute(self.pool)
            .await?;

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

        Ok(())
    }

    pub async fn edit_title(&self, id: &str, pin: i64, title: &str) -> QueriesResult<()> {
        let task = self.get_task(id).await?;

        if task.pin != pin {
            return Err(QueriesError::IllegalState(WRONG_PIN_MESSAGE.to_string()));
        }

        query!("UPDATE task SET title = $1 WHERE id = $2", title, id)
            .execute(self.pool)
            .await?;

        Ok(())
    }

    pub async fn edit_body(&self, id: &str, pin: i64, body: &str) -> QueriesResult<()> {
        let task = self.get_task(id).await?;

        if task.pin != pin {
            return Err(QueriesError::IllegalState(WRONG_PIN_MESSAGE.to_string()));
        }

        query!("UPDATE task SET body = $1 WHERE id = $2", body, id)
            .execute(self.pool)
            .await?;

        Ok(())
    }
}
