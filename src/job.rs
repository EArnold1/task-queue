// use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Priority {
    High,
    Low,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Job {
    id: uuid::Uuid,
    job_type: String,
    // payload: HashMap<String, T>,
    priority: Priority,
    retry_count: u8,
    max_retries: u8,
    pub created_at: DateTime<Utc>,
}

impl Job {
    pub fn new(job_type: String, priority: Priority) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            job_type,
            priority,
            retry_count: 4,
            max_retries: 4,
            created_at: Utc::now(),
        }
    }
}
