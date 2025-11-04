use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::job::Job;

pub trait JobHandler: Send + Sync {
    fn execute(&self, job: &Job);
}

pub struct JobRegistry {
    handlers: Arc<Mutex<HashMap<String, Arc<dyn JobHandler>>>>,
}

impl JobRegistry {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register_handler(&self, name: &str, handler: Arc<dyn JobHandler>) {
        self.handlers.lock().unwrap().insert(name.into(), handler);
    }

    pub fn execute(&self, job: &Job) -> Option<()> {
        if let Some(handler) = self.handlers.lock().unwrap().get(job.job_type()) {
            handler.execute(job);
            return Some(());
        }

        None
    }
}

pub struct SendEmailHandler;

impl JobHandler for SendEmailHandler {
    fn execute(&self, job: &Job) {
        println!("[Info]: Sending email for job: {}", &job.id());

        thread::sleep(Duration::from_secs(1));

        println!("[Info]: Done Sending email for job: {}", &job.id());
    }
}
