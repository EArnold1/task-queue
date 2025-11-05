use std::{collections::HashMap, sync::Arc, thread, time::Duration};

use crate::job::Job;

pub trait JobHandler: Send + Sync {
    fn execute(&self, job: &Job) -> Result<(), String>;
}

pub struct JobRegistry {
    // trait object
    handlers: HashMap<String, Arc<dyn JobHandler>>,
}

impl JobRegistry {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register_handler(&mut self, name: &str, handler: Arc<dyn JobHandler>) {
        self.handlers.insert(name.into(), handler);
    }

    pub fn execute(&mut self, job: &Job) -> Result<(), String> {
        if let Some(handler) = self.handlers.get(job.job_type()) {
            handler.execute(job)
        } else {
            Err("No handler found".into())
        }
    }
}

pub struct SendEmailHandler;

impl JobHandler for SendEmailHandler {
    fn execute(&self, job: &Job) -> Result<(), String> {
        println!("[Info]: Sending email for job: {}", &job.id());

        for (key, value) in job.payload() {
            println!("{key}: {value}");
        }

        thread::sleep(Duration::from_secs(1));

        println!("[Info]: Done Sending email for job: {} \n", &job.id());

        Ok(())
    }
}

pub struct NotificationHandler;

impl JobHandler for NotificationHandler {
    fn execute(&self, job: &Job) -> Result<(), String> {
        println!("[Info]: Sending notification for job: {}", &job.id());

        if !job.payload().contains_key("fcm_token") {
            return Err("Fcm token not found".into());
        }

        for (key, value) in job.payload() {
            println!("{key}: {value}");
        }

        thread::sleep(Duration::from_secs(2));

        println!(
            "[Info]: Done Sending notification for job: {} \n",
            &job.id()
        );

        Ok(())
    }
}
