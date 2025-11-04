use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use chrono::{DateTime, Utc};

use crate::{
    job::Job,
    queue::{JobQueue, Queue},
};

#[derive(Debug, PartialEq, Eq)]
struct ScheduledJob {
    job: Job,
    scheduled_at: DateTime<Utc>,
}

impl Ord for ScheduledJob {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // the lower the scheduled_at time, the greater the importance
        match (self, other) {
            (s, o) if s.scheduled_at < o.scheduled_at => Ordering::Greater,
            (s, o) if s.scheduled_at > o.scheduled_at => Ordering::Less,
            _ => Ordering::Equal,
        }
    }
}

impl PartialOrd for ScheduledJob {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Scheduler {
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
