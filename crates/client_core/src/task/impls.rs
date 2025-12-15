use domain::task::Task;
use uuid::Uuid;

use crate::task::{create::SaveTaskError, ports::{TaskRepository, TaskService}};

#[derive(Debug, Clone)]
pub struct Service<R>
where
    R: TaskRepository,
{
    repo: R,
}

impl<R> Service<R>
where
    R: TaskRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl<R: TaskRepository> TaskService for Service<R> {
    async fn create_task(
        &self,
        req: &super::create::CreateTaskRequest,
    ) -> Result<Uuid, super::create::SaveTaskError> {
        let task = Task::new(Uuid::new_v4(), req.title().to_string(), chrono::Utc::now(), None);

        self.repo.create_task(task).await

    }

    async fn get_active_tasks(&self) -> Result<Vec<Task>, anyhow::Error> {
        let tasks = self.repo.get_active_tasks().await;

        tasks
    }

    async fn get_tasks(&self) -> Result<Vec<Task>, anyhow::Error> {
        let tasks = self.repo.get_tasks().await;

        tasks
    }

    async fn get_task(&self, id: &Uuid) -> Result<Task, anyhow::Error> {
        let task = self.repo.get_task(id).await;

        task
    }

    async fn complete_task(&self, id: &Uuid) -> Result<Uuid, SaveTaskError> {
        let mut task = self.repo.get_task(id).await?;

        task.complete();

        self.repo.update_task(task).await
    }
}
