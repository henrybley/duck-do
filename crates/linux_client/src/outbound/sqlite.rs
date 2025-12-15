use anyhow::Context;
use chrono::DateTime;
use client_core::task::{create::SaveTaskError, ports::TaskRepository};
use domain::task::Task;
use sqlx::{Executor, SqlitePool, sqlite::SqliteConnectOptions};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Sqlite {
    pool: SqlitePool,
}

impl Sqlite {
    pub async fn new(path: &str) -> Result<Sqlite, anyhow::Error> {
        let pool = SqlitePool::connect_with(
            SqliteConnectOptions::from_str(path)
                .with_context(|| format!("invalid database path {}", path))?
                .pragma("foreign_keys", "ON"),
        )
        .await
        .with_context(|| format!("failed to open database at {}", path))?;

        Ok(Sqlite { pool })
    }
}

impl TaskRepository for Sqlite {
    async fn get_tasks(&self) -> Result<Vec<Task>, anyhow::Error> {
        let query = sqlx::query!("SELECT * FROM tasks");
        let rows = query.fetch_all(&self.pool).await?;

        let tasks = rows
            .into_iter()
            .map(|row| {
                let id = Uuid::parse_str(&row.id).expect("invalid UUID in DB");
                let completed = row.completed.map(|ts| DateTime::from_timestamp(ts, 0).unwrap());
                Task::new(
                    id,
                    row.title,
                    DateTime::from_timestamp(row.created, 0).unwrap(),
                    completed,
                )
            })
            .collect();

        Ok(tasks)
    }

    async fn get_active_tasks(&self) -> Result<Vec<Task>, anyhow::Error> {
        let query = sqlx::query!("SELECT * FROM tasks WHERE completed IS NULL");
        let rows = query.fetch_all(&self.pool).await?;

        let tasks = rows
            .into_iter()
            .map(|row| {
                let id = Uuid::parse_str(&row.id).expect("invalid UUID in DB");
                let completed = row.completed.map(|ts| DateTime::from_timestamp(ts, 0).unwrap());
                Task::new(
                    id,
                    row.title,
                    DateTime::from_timestamp(row.created, 0).unwrap(),
                    completed,
                )
            })
            .collect();

        Ok(tasks)
    }

    async fn get_task(&self, id: &Uuid) -> Result<Task, anyhow::Error> {
        let id_string = id.to_string();
        let query = sqlx::query!("SELECT * FROM tasks WHERE id = $1", id_string);
        let row = query.fetch_one(&self.pool).await?;
        let id = Uuid::parse_str(&row.id).expect("invalid UUID in DB");
        let task = Task::new(
            id,
            row.title,
            DateTime::from_timestamp(row.created, 0).unwrap(),
            None,
        );

        Ok(task)
    }

    async fn create_task(&self, task: Task) -> anyhow::Result<Uuid, SaveTaskError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .context("failed to start SQLite transaction")?;

        let id = task.id().to_string();
        let title = task.title();
        let created = task.created().timestamp();
        let completed = task.completed().map(|dt| dt.timestamp());

        let query = sqlx::query!(
            "INSERT INTO tasks (id, title, created, completed) VALUES ($1, $2, $3, $4)",
            id,
            title,
            created,
            completed
        );

        tx.execute(query)
            .await
            .map_err(|e| map_sqlx_error(e, task.title()))?;

        tx.commit()
            .await
            .context("failed to commit SQLite transaction")?;

        Ok(task.id().clone())
    }
    
    async fn update_task(&self, task: Task) -> anyhow::Result<Uuid, SaveTaskError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .context("failed to start SQLite transaction")?;

        let id = task.id().to_string();
        let title = task.title();
        let created = task.created().timestamp();
        let completed = task.completed().map(|dt| dt.timestamp());

        let query = sqlx::query!(
            "UPDATE tasks SET title = $1, created = $2, completed = $3 WHERE id = $4",
            title,
            created,
            completed,
            id,
        );

        tx.execute(query)
            .await
            .map_err(|e| map_sqlx_error(e, task.title()))?;

        tx.commit()
            .await
            .context("failed to commit SQLite transaction")?;

        Ok(task.id().clone())
    }
}

fn map_sqlx_error(err: sqlx::Error, title: &str) -> SaveTaskError {
    match err {
        sqlx::Error::Database(db_err) if db_err.is_unique_violation() => SaveTaskError::Duplicate {
            title: title.to_owned(),
        },
        other => SaveTaskError::Unknown(anyhow::Error::new(other)),
    }
}
