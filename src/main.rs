mod handler;
mod job;
mod queue;
mod scheduler;
mod utils;
mod worker;

use std::{
    collections::HashMap,
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

use crate::utils::helper::{create_email_payload, create_notification_payload};

fn main() {
    // creates jobs

    let mut analytics_payload = HashMap::new();
    analytics_payload.insert("metadata".into(), "some data".into());

    let jobs = vec![
        Job::new(
            "send_email".into(),
            Priority::High,
            create_email_payload("test@gamil.com", "Hi there", "Welcome message"),
        ),
        Job::new(
            "compute_analytics".into(),
            Priority::High,
            analytics_payload,
        ),
    ];

    let queue = Arc::new(Mutex::new(Queue::new()));

    for j in jobs {
        queue.lock().unwrap().enqueue(j);
    }

    {
        // schedule jobs
        let mut scheduler = Scheduler::new(queue.clone());

        let scheduled_job = Job::new(
            "send_notification".into(),
            Priority::Low,
            create_notification_payload(
                "You have a new follower",
                "New Follow",
                Some("some token"),
            ),
        );
        let scheduled_job_two = Job::new(
            "send_notification".into(),
            Priority::High,
            create_notification_payload("You have a new follower", "New Follow", None),
        );

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
