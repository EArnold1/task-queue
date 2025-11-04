mod handler;
mod job;
mod queue;
mod scheduler;
mod worker;

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use chrono::Utc;
use job::{Job, Priority};

use handler::{JobRegistry, NotificationHandler, SendEmailHandler};
use queue::{JobQueue, Queue};
use scheduler::Scheduler;
use worker::{Consumer, Worker};

fn main() {
    // creates jobs
    let job = Job::new("send_email".into(), Priority::High);
    let jobs = vec![
        Job::new("compute_analytics".into(), Priority::High),
        // Job::new("send_reminders".into(), Priority::Low),
        // Job::new("fix_bug".into(), Priority::High),
        // Job::new("another job".into(), Priority::High),
        // Job::new("some_job".into(), Priority::Low),
        // Job::new("ok_job".into(), Priority::Low),
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
        let scheduled_job_two = Job::new("send_notification".into(), Priority::High);

        scheduler.add(scheduled_job, Utc::now() + Duration::from_secs(10));
        scheduler.add(scheduled_job_two, Utc::now() + Duration::from_secs(3));

        thread::spawn(move || {
            scheduler.run();
        });
    }

    // Create a job registry and register handlers.
    let registry = Arc::new(Mutex::new(JobRegistry::new()));
    let mut registry_lock = registry.lock().unwrap();

    registry_lock.register_handler("send_email", Arc::new(SendEmailHandler));
    registry_lock.register_handler("send_notification", Arc::new(NotificationHandler));

    drop(registry_lock);

    let mut workers = Vec::new();

    {
        // create and start workers
        for i in 1..=4 {
            let q = queue.clone();
            let r = registry.clone();
            workers.push(thread::spawn(move || {
                let mut w = Worker::new(i, q, r);
                w.start();
            }));
        }
    }

    for worker in workers {
        worker.join().unwrap();
    }

    // for d in queue.clone().lock().unwrap().list_dlq() {
    //     println!("[DLQ]: reason {}, job_id: {:?}", d.reason, d.job.id());
    // }
}
