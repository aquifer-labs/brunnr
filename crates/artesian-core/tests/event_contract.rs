// SPDX-License-Identifier: Apache-2.0

use artesian_core::{Barrier, EventEnvelope, EventSender, EventType, Role, TokenAccounting};
use serde_json::json;

#[test]
fn event_envelope_round_trips_with_correlation_id() {
    let event = EventEnvelope::new(
        "event-1",
        "corr-1",
        EventSender {
            role: Role::Worker,
            agent_id: "worker-a".to_string(),
        },
        EventType::TaskClaimed,
        json!({ "task_id": "task-a" }),
    );

    let encoded = serde_json::to_string(&event).expect("event should encode");
    let decoded: EventEnvelope = serde_json::from_str(&encoded).expect("event should decode");

    assert_eq!(decoded.correlation_id, "corr-1");
    assert_eq!(decoded.event_type, EventType::TaskClaimed);
    assert_eq!(decoded.payload["task_id"], "task-a");
    assert!(encoded.contains(r#""type":"TASK_CLAIMED""#));
}

#[test]
fn barrier_waits_for_all_parallel_dependencies() {
    let barrier = Barrier::new("synthesis", ["task-a".to_string(), "task-b".to_string()]);
    let partial = ["task-a".to_string()];
    let complete = ["task-a".to_string(), "task-b".to_string()];

    assert!(!barrier.is_satisfied_by(partial.iter()));
    assert!(barrier.is_satisfied_by(complete.iter()));
}

#[test]
fn token_accounting_tracks_agent_session_usage() {
    let accounting = TokenAccounting {
        agent_id: "worker-a".to_string(),
        session_id: Some("session-a".to_string()),
        prompt_tokens: 100,
        completion_tokens: 25,
    };

    assert_eq!(accounting.total_tokens(), 125);
}
