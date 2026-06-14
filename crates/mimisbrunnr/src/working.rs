// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{MemoryTier, StoreMemory};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkingTurn {
    pub role: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

impl WorkingTurn {
    pub fn new(role: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: role.into(),
            content: content.into(),
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkingMemoryMode {
    #[default]
    Buffer,
    SlidingWindow {
        k: usize,
    },
    SummaryBuffer {
        window: usize,
        summarize_to: Option<MemoryTier>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkingMemoryView {
    pub turns: Vec<WorkingTurn>,
    pub pending_summary: Option<StoreMemory>,
}

pub trait WorkingMemory: Send + Sync {
    fn push(&mut self, turn: WorkingTurn);

    fn view(&self) -> WorkingMemoryView;

    fn clear(&mut self);
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryWorkingMemory {
    mode: WorkingMemoryMode,
    turns: Vec<WorkingTurn>,
}

impl InMemoryWorkingMemory {
    pub fn new(mode: WorkingMemoryMode) -> Self {
        Self {
            mode,
            turns: Vec::new(),
        }
    }

    pub fn mode(&self) -> &WorkingMemoryMode {
        &self.mode
    }

    fn visible_turns(&self) -> Vec<WorkingTurn> {
        match self.mode {
            WorkingMemoryMode::Buffer => self.turns.clone(),
            WorkingMemoryMode::SlidingWindow { k } => last_turns(&self.turns, k),
            WorkingMemoryMode::SummaryBuffer { window, .. } => last_turns(&self.turns, window),
        }
    }

    fn pending_summary(&self) -> Option<StoreMemory> {
        let WorkingMemoryMode::SummaryBuffer {
            window,
            summarize_to: Some(tier),
        } = self.mode
        else {
            return None;
        };
        if self.turns.len() <= window {
            return None;
        }
        let older = &self.turns[..self.turns.len() - window];
        let content = older
            .iter()
            .map(|turn| format!("{}: {}", turn.role, turn.content))
            .collect::<Vec<_>>()
            .join("\n");
        let mut memory = StoreMemory::atom(content);
        memory.tier = tier;
        memory.tags = vec!["working-memory-summary".to_string()];
        Some(memory)
    }
}

impl WorkingMemory for InMemoryWorkingMemory {
    fn push(&mut self, turn: WorkingTurn) {
        self.turns.push(turn);
    }

    fn view(&self) -> WorkingMemoryView {
        WorkingMemoryView {
            turns: self.visible_turns(),
            pending_summary: self.pending_summary(),
        }
    }

    fn clear(&mut self) {
        self.turns.clear();
    }
}

fn last_turns(turns: &[WorkingTurn], k: usize) -> Vec<WorkingTurn> {
    let start = turns.len().saturating_sub(k);
    turns[start..].to_vec()
}
