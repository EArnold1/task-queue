// use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq, Eq)]
pub enum Priority {
    High,
    Low,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Job {
    id: uuid::Uuid,
    job_type: String,
    // payload: HashMap<String, T>,
    priority: Priority,
    pub retry_count: u8,
    pub max_retries: u8,
    created_at: DateTime<Utc>,
}

impl Job {
    pub fn new(job_type: String, priority: Priority) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            job_type,
            priority,
            retry_count: 0,
            max_retries: 2,
            created_at: Utc::now(),
        }
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    pub fn job_type(&self) -> &str {
        &self.job_type
    }
}
