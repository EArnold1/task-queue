use std::{
    collections::BinaryHeap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use chrono::{DateTime, Utc};

use crate::{
    job::Job,
    queue::{Queue, QueueTrait},
};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct ScheduledJob {
    job: Job,
    scheduled_at: DateTime<Utc>,
}

// implement Ord(compare time)

pub struct Scheduler {
    // alternatively we can implement a heap data structure
    jobs: BinaryHeap<ScheduledJob>,
    queue: Arc<Mutex<Queue>>,
}

impl Scheduler {
    pub fn new(queue: Arc<Mutex<Queue>>) -> Self {
        Self {
            jobs: BinaryHeap::new(),
            queue,
        }
    }

    pub fn add(&mut self, job: Job, scheduled_at: DateTime<Utc>) {
        assert!(scheduled_at > Utc::now());

        self.jobs.push(ScheduledJob { job, scheduled_at });
    }

    pub fn run(&mut self) {
        loop {
            if let Some(scheduled_job) = self.jobs.peek()
                && Utc::now() > scheduled_job.scheduled_at
                && let Some(j) = self.jobs.pop()
            {
                self.queue.lock().unwrap().enqueue(j.job);
            }

            thread::sleep(Duration::from_millis(500));
        }
    }
}
