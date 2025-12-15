use anyhow::Result;
use domain::task::Task;
use uuid::Uuid;

use crate::task::create::{CreateTaskRequest, SaveTaskError};

pub trait TaskService: Clone + Send + Sync + 'static {
    fn create_task(
        &self,
        req: &CreateTaskRequest,
    ) -> impl Future<Output = Result<Uuid, SaveTaskError>> + Send;

    fn get_tasks(&self) -> impl Future<Output = Result<Vec<Task>, anyhow::Error>> + Send;

    fn get_active_tasks(&self) -> impl Future<Output = Result<Vec<Task>, anyhow::Error>> + Send;

    fn get_task(&self, id: &Uuid) -> impl Future<Output = Result<Task, anyhow::Error>> + Send;

    fn complete_task(&self, id: &Uuid) -> impl Future<Output = Result<Uuid, SaveTaskError>> + Send;
}

pub trait TaskRepository: Send + Sync + Clone + 'static {
    fn get_tasks(&self) -> impl Future<Output = Result<Vec<Task>, anyhow::Error>> + Send;

    fn get_active_tasks(&self) -> impl Future<Output = Result<Vec<Task>, anyhow::Error>> + Send;

    fn get_task(&self, id: &Uuid) -> impl Future<Output = Result<Task, anyhow::Error>> + Send;

    fn create_task(&self, task: Task) -> impl Future<Output = Result<Uuid, SaveTaskError>> + Send;

    fn update_task(&self, task: Task) -> impl Future<Output = Result<Uuid, SaveTaskError>> + Send;
}
