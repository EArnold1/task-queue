use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::queue::{Queue, QueueTrait};

pub trait WorkerTrait {
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

impl WorkerTrait for Worker {
    fn start(&mut self) {
        loop {
            if let Some(job) = self.queue.lock().unwrap().dequeue() {
                println!("Worker {} got a job; executing.", self.id);

                println!("Job: {:?}", job);
            }

            thread::sleep(Duration::from_secs(1));
        }
        // start worker (polling queues)
    }
}
