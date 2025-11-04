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
    fn start(&mut self) {}
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
                println!("Worker {} got a job; executing.", self.id);

                // execute job
                // NOTE: long running jobs can block
                if self.job_registry.lock().unwrap().execute(&job).is_none() {
                    if job.retry_count >= job.max_retries {
                        // push to dead letter queue
                        //
                        println!("dead letter: {:?}", job);
                        continue;
                    }

                    println!("Retrying job: {}", &job.id());
                    job.retry_count += 1;
                    // exponential backoff
                    //
                    let delay = base * (2_u32.pow(job.retry_count as u32));
                    let queue = self.queue.clone();
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
