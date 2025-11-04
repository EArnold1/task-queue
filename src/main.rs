use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use chrono::Utc;
use job::{Job, Priority};

use queue::{JobQueue, Queue};
use scheduler::Scheduler;
use worker::{Consumer, Worker};

mod job;
mod queue;
mod scheduler;
mod worker;

fn main() {
    // creates jobs
    let job = Job::new("send_email".into(), Priority::High);
    let jobs = vec![
        Job::new("compute_analytics".into(), Priority::High),
        Job::new("send_reminders".into(), Priority::Low),
        Job::new("fix_bug".into(), Priority::High),
        Job::new("another job".into(), Priority::High),
        Job::new("some_job".into(), Priority::Low),
        Job::new("ok_job".into(), Priority::Low),
    ];

    let queue = Arc::new(Mutex::new(Queue::new()));

    // queue jobs
    queue.lock().unwrap().enqueue(job);

    for j in jobs {
        queue.lock().unwrap().enqueue(j);
    }

    {
        // schedule jobs
        let mut scheduler = Scheduler::new(queue.clone());

        let scheduled_job = Job::new("send_notification".into(), Priority::Low);
        let scheduled_job_two = Job::new("send_notification 2".into(), Priority::Low);

        scheduler.add(scheduled_job, Utc::now() + Duration::from_secs(10));
        scheduler.add(scheduled_job_two, Utc::now() + Duration::from_secs(3));

        thread::spawn(move || {
            scheduler.run();
        });
    }

    let mut workers = Vec::new();

    {
        // create and start workers
        for i in 1..=4 {
            let q = queue.clone();
            workers.push(thread::spawn(move || {
                let mut w = Worker::new(i, q);
                w.start();
            }));
        }
    }

    for worker in workers {
        worker.join().unwrap();
    }
}
