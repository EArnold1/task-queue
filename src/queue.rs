use std::collections::VecDeque;

use crate::job::Job;

pub trait JobQueue {
    fn enqueue(&mut self, _job: Job) {}

    fn dequeue(&mut self) -> Option<Job>;
}

pub struct Queue {
    queue: VecDeque<Job>,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
}

impl JobQueue for Queue {
    fn enqueue(&mut self, job: Job) {
        // add to queue
        // add by priority
        self.queue.push_front(job);
    }

    fn dequeue(&mut self) -> Option<Job> {
        // remove from queue
        self.queue.pop_front()
    }
}
