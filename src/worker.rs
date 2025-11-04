use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{
    handler::JobRegistry,
    queue::{JobQueue, Queue},
};

type SharedMut<T> = Arc<Mutex<T>>;

pub trait Consumer {
    fn start(&mut self);
}

pub struct Worker {
    id: u8,
    queue: SharedMut<Queue>,
    job_registry: SharedMut<JobRegistry>,
}

impl Worker {
    pub fn new(id: u8, queue: SharedMut<Queue>, job_registry: SharedMut<JobRegistry>) -> Self {
        Self {
            id,
            queue,
            job_registry,
        }
    }
}

impl Consumer for Worker {
    /// start worker (polling queues)
    fn start(&mut self) {
        let base = Duration::from_secs(1);
        loop {
            if let Some(mut job) = self.queue.lock().unwrap().dequeue() {
                println!(
                    "Worker {} got a job; executing job: {}. \n",
                    self.id,
                    job.id()
                );

                // execute job
                // NOTE: long running jobs can block

                let retry_count = job.retry_count();
                let max_retries = job.max_retries();
                if let Err(reason) = self.job_registry.lock().unwrap().execute(&job) {
                    let queue = self.queue.clone();

                    if reason == "No handler found" {
                        println!("pushed job: {} to dead letter queue: \n", job.id());

                        thread::spawn(move || {
                            queue.lock().unwrap().push_to_dlq(reason, job);
                        });

                        continue;
                    }

                    if retry_count >= max_retries {
                        // push to dead letter queue
                        println!("pushed job: {} to dead letter queue: \n", job.id());

                        thread::spawn(move || {
                            queue.lock().unwrap().push_to_dlq(reason, job);
                        });

                        continue;
                    }

                    println!("Retrying job: {} \n", &job.id());

                    job.update_retry_count();

                    // exponential backoff
                    let delay = base * (2_u32.pow(retry_count as u32));

                    thread::spawn(move || {
                        thread::sleep(delay);
                        queue.lock().unwrap().enqueue(job);
                    });
                }
            } else {
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
}
