use thiserror::Error;

pub struct CreateTaskRequest {
    title: String,
}

impl CreateTaskRequest {
    pub fn new(title: String) -> Self {
        Self { title }
    }
    pub fn title(&self) -> &String {
        &self.title
    }
}

#[derive(Debug, Error)]
pub enum SaveTaskError {
    #[error("active task with title {title} already exists")]
    Duplicate { title: String },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
