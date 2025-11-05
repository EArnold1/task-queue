#![cfg(test)]

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{
    handler::{JobRegistry, NotificationHandler, SendEmailHandler},
    job::{Job, Priority},
    queue::{JobQueue, Queue},
    utils::helper::create_notification_payload,
    worker::{Consumer, Worker},
};

#[test]
fn test_worker() {
    let job = Job::new(
        "send_notification".into(),
        Priority::Low,
        create_notification_payload("You have a new follower", "New Follow", Some("token")),
    );

    let mut queue = Queue::new();

    queue.enqueue(job);

    // Create a job registry and register handlers.
    let registry = Arc::new(Mutex::new(JobRegistry::new()));
    let mut registry_lock = registry.lock().unwrap();

    registry_lock.register_handler("send_notification", Arc::new(NotificationHandler));

    drop(registry_lock);

    let queue = Arc::new(Mutex::new(queue));

    let q_clone = queue.clone();

    thread::spawn(move || {
        let mut w = Worker::new(1, q_clone.clone(), registry.clone());
        w.start();
    });

    thread::sleep(Duration::from_millis(500));

    let q_size = queue.lock().unwrap().queue_len();

    assert_eq!(q_size, 0);
}

#[test]
fn test_worker_retries() {
    let mut analytics_payload = HashMap::new();
    analytics_payload.insert("metadata".into(), "some data".into());
    let jobs = vec![
        Job::new(
            "compute_analytics".into(),
            Priority::High,
            analytics_payload,
        ),
        Job::new(
            "send_notification".into(),
            Priority::Low,
            create_notification_payload("You have a new follower", "New Follow", None),
        ),
    ];

    let mut queue = Queue::new();

    for job in jobs {
        queue.enqueue(job);
    }

    let registry = Arc::new(Mutex::new(JobRegistry::new()));
    let mut registry_lock = registry.lock().unwrap();

    registry_lock.register_handler("send_email", Arc::new(SendEmailHandler));
    registry_lock.register_handler("send_notification", Arc::new(NotificationHandler));

    drop(registry_lock);

    let queue = Arc::new(Mutex::new(queue));

    let q_clone = queue.clone();

    thread::spawn(move || {
        let mut w = Worker::new(1, q_clone.clone(), registry.clone());
        w.start();
    });

    thread::sleep(Duration::from_secs(3));

    let dlq_size = queue.lock().unwrap().dlq_len();

    assert!(dlq_size > 0)
}
