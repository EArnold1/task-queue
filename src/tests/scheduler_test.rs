#![cfg(test)]

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use chrono::Utc;

use crate::{
    job::{Job, Priority},
    queue::{JobQueue, Queue},
    scheduler::Scheduler,
    utils::helper::create_notification_payload,
};

#[test]
fn test_scheduler() {
    let queue = Arc::new(Mutex::new(Queue::new()));
    let mut scheduler = Scheduler::new(queue.clone());

    let scheduled_jobs = vec![
        Job::new(
            "send_notification".into(),
            Priority::Low,
            create_notification_payload(
                "You have a new follower",
                "New Follow",
                Some("some token"),
            ),
        ),
        Job::new(
            "send_notification".into(),
            Priority::High,
            create_notification_payload("You have a new follower", "New Follow", None),
        ),
    ];

    let job_id = scheduled_jobs.get(1).map(|j| j.id());

    let mut duration = Duration::from_millis(100);
    let now = Utc::now();

    for s_job in scheduled_jobs {
        scheduler.add(s_job, now + duration);
        duration = Duration::from_millis(50);
    }

    thread::spawn(move || {
        scheduler.run();
    });

    thread::sleep(Duration::from_millis(500));

    let dequeued_job_id = queue.lock().unwrap().dequeue().map(|j| j.id());

    assert_eq!(job_id, dequeued_job_id)
}
