// SPDX-License-Identifier: Apache-2.0

use aquifer::{InMemoryWorkingMemory, MemoryTier, WorkingMemory, WorkingMemoryMode, WorkingTurn};

#[test]
fn buffer_mode_keeps_all_turns_without_summary() {
    let mut memory = InMemoryWorkingMemory::new(WorkingMemoryMode::Buffer);
    memory.push(WorkingTurn::new("worker", "first"));
    memory.push(WorkingTurn::new("judge", "second"));

    let view = memory.view();

    assert_eq!(view.turns.len(), 2);
    assert!(view.pending_summary.is_none());
}

#[test]
fn sliding_window_mode_keeps_last_k_turns() {
    let mut memory = InMemoryWorkingMemory::new(WorkingMemoryMode::SlidingWindow { k: 2 });
    memory.push(WorkingTurn::new("worker", "first"));
    memory.push(WorkingTurn::new("worker", "second"));
    memory.push(WorkingTurn::new("worker", "third"));

    let view = memory.view();

    assert_eq!(
        view.turns
            .iter()
            .map(|turn| turn.content.as_str())
            .collect::<Vec<_>>(),
        vec!["second", "third"]
    );
    assert!(view.pending_summary.is_none());
}

#[test]
fn summary_buffer_only_emits_summary_memory_when_opted_in() {
    let mut disabled = InMemoryWorkingMemory::new(WorkingMemoryMode::SummaryBuffer {
        window: 1,
        summarize_to: None,
    });
    disabled.push(WorkingTurn::new("worker", "older"));
    disabled.push(WorkingTurn::new("worker", "visible"));

    assert!(disabled.view().pending_summary.is_none());

    let mut enabled = InMemoryWorkingMemory::new(WorkingMemoryMode::SummaryBuffer {
        window: 1,
        summarize_to: Some(MemoryTier::L2Scenario),
    });
    enabled.push(WorkingTurn::new("worker", "older"));
    enabled.push(WorkingTurn::new("worker", "visible"));

    let view = enabled.view();
    let summary = view.pending_summary.expect("summary should be pending");
    assert_eq!(view.turns[0].content, "visible");
    assert_eq!(summary.tier, MemoryTier::L2Scenario);
    assert!(summary.content.contains("older"));
}
