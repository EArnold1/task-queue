use std::collections::HashMap;

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
    payload: HashMap<String, String>,
    priority: Priority,
    retry_count: u8,
    max_retries: u8,
    created_at: DateTime<Utc>,
}

impl Job {
    pub fn new(job_type: String, priority: Priority, payload: HashMap<String, String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            job_type,
            priority,
            payload,
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

    pub fn retry_count(&self) -> u8 {
        self.retry_count
    }

    pub fn update_retry_count(&mut self) {
        self.retry_count += 1;
    }

    pub fn max_retries(&mut self) -> u8 {
        self.max_retries
    }

    pub fn priority(&self) -> &Priority {
        &self.priority
    }

    pub fn payload(&self) -> &HashMap<String, String> {
        &self.payload
    }
}
