use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct Task {
    id: Uuid,
    title: String,
    created: DateTime<Utc>,
    completed: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(
        id: Uuid,
        title: String,
        created: DateTime<Utc>,
        completed: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            title,
            created,
            completed,
        }
    }

    pub fn complete(&mut self) {
        self.completed = Some(chrono::Utc::now());
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn created(&self) -> &DateTime<Utc> {
        &self.created
    }

    pub fn completed(&self) -> &Option<DateTime<Utc>> {
        &self.completed
    }
}
