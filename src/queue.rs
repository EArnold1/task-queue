use std::collections::VecDeque;

use crate::job::Job;

pub trait JobQueue {
    fn enqueue(&mut self, job: Job);

    fn dequeue(&mut self) -> Option<Job>;

    fn push_to_dlq(&mut self, reason: String, job: Job);

    fn pop_dlq(&mut self) -> Option<&DeadLetterQ>;

    // Inside a DLQ, the following can be implemented
    /*
     * 1. Retries
     * 2. Archiving or discarding
     * 3. Monitoring/review
     */
}

/*
Note:
Configurable Rules for DLQ:
    Administrators can set specific rules that define when a message should be moved to a DLQ.
    These rules might include maximum retry attempts or specific error types.
*/

pub struct DeadLetterQ {
    pub reason: String, // can be a custom error type
    pub job: Job,
}

pub struct Queue {
    queue: VecDeque<Job>,
    dead_letter_queue: Vec<DeadLetterQ>,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            dead_letter_queue: Vec::new(),
        }
    }

    pub fn list_dlq(&self) -> &Vec<DeadLetterQ> {
        &self.dead_letter_queue
    }
}

impl JobQueue for Queue {
    fn enqueue(&mut self, job: Job) {
        // add to queue
        // TODO: sort by priority
        self.queue.push_front(job);
    }

    fn dequeue(&mut self) -> Option<Job> {
        // remove from queue
        self.queue.pop_front()
    }

    fn push_to_dlq(&mut self, reason: String, job: Job) {
        self.dead_letter_queue.push(DeadLetterQ { reason, job });
    }

    /// returns the last job inserted into the dlq
    fn pop_dlq(&mut self) -> Option<&DeadLetterQ> {
        self.dead_letter_queue.first()
    }
}
