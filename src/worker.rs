use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::queue::{JobQueue, Queue};

pub trait Consumer {
    fn start(&mut self) {}
}

pub struct Worker {
    id: u8,
    queue: Arc<Mutex<Queue>>,
}

impl Worker {
    pub fn new(id: u8, queue: Arc<Mutex<Queue>>) -> Self {
        Self { id, queue }
    }
}

impl Consumer for Worker {
    /// start worker (polling queues)
    fn start(&mut self) {
        loop {
            if let Some(job) = self.queue.lock().unwrap().dequeue() {
                println!("Worker {} got a job; executing.", self.id);

                // execute job
                // implement retries

                println!("Job: {:?}", job);
            } else {
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
}
