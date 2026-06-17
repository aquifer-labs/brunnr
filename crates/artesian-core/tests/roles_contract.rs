// SPDX-License-Identifier: Apache-2.0

use artesian_core::{Job, JobStatus, Queue, Role};

#[test]
fn role_aliases_resolve_canonical_names() {
    assert_eq!("master".parse::<Role>(), Ok(Role::Master));
    assert_eq!("worker".parse::<Role>(), Ok(Role::Worker));
    assert_eq!("judge".parse::<Role>(), Ok(Role::Judge));
    assert!("overseer".parse::<Role>().is_err());
}

#[test]
fn queue_preserves_fifo_order() {
    let first = Job {
        id: "0001".to_string(),
        title: "Scaffold".to_string(),
        role: Role::Worker,
        status: JobStatus::Todo,
    };
    let second = Job {
        id: "0002".to_string(),
        title: "Review".to_string(),
        role: Role::Judge,
        status: JobStatus::Todo,
    };
    let mut queue = Queue::default();

    queue.push(first.clone());
    queue.push(second.clone());

    assert_eq!(queue.pop_next(), Some(first));
    assert_eq!(queue.pop_next(), Some(second));
    assert_eq!(queue.pop_next(), None);
}
