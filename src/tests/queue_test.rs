#![cfg(test)]

use crate::{
    job::{Job, Priority},
    queue::{JobQueue, Queue},
    utils::helper::{create_email_payload, create_notification_payload},
};

#[test]
fn test_enqueue_dequeue() {
    let job = Job::new(
        "send_email".into(),
        Priority::High,
        create_email_payload("test@gamil.com", "Hi there", "Welcome message"),
    );

    let job_id = job.id();

    let mut queue = Queue::new();

    queue.enqueue(job);

    let dequeued_job_id = queue.dequeue().map(|j| j.id());

    assert_eq!(Some(job_id), dequeued_job_id)
}

#[test]
fn test_empty_queue() {
    let mut queue = Queue::new();

    let dlq_job_id = queue.dequeue();

    assert!(dlq_job_id.is_none())
}

#[test]
fn test_dlq() {
    let jobs = vec![
        Job::new(
            "send_email".into(),
            Priority::High,
            create_email_payload("test@gamil.com", "Hi there", "Welcome message"),
        ),
        Job::new(
            "send_notification".into(),
            Priority::Low,
            create_notification_payload(
                "You have a new follower",
                "New Follow",
                Some("some token"),
            ),
        ),
    ];

    let job_id = jobs.first().map(|j| j.id());

    let mut queue = Queue::new();

    for job in jobs {
        queue.push_to_dlq("missing API key".into(), job);
    }

    let dlq_job_id = queue.pop_dlq().map(|d| d.job.id());

    assert_eq!(job_id, dlq_job_id)
}
