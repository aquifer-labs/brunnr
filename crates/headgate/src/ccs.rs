// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeSet;
use std::fmt::Write as _;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::metrics::count_tokens;

/// The schema that governs the committed context state: a fixed set of named slots into
/// which committed entries are filed. "Schema-governed" is the ACC property that the
/// committed state has typed structure, not an undifferentiated blob.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CcsSchema {
    pub slots: Vec<String>,
}

impl CcsSchema {
    pub fn new(slots: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            slots: slots.into_iter().map(Into::into).collect(),
        }
    }

    pub fn contains(&self, slot: &str) -> bool {
        self.slots.iter().any(|candidate| candidate == slot)
    }

    pub fn default_slot(&self) -> &str {
        self.slots.first().map(String::as_str).unwrap_or("fact")
    }
}

impl Default for CcsSchema {
    /// A general-purpose default schema for agent working knowledge.
    fn default() -> Self {
        Self::new(["decision", "constraint", "fact", "task-state"])
    }
}

/// One unit of committed knowledge, filed into a schema slot and costed in tokens.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommittedEntry {
    pub id: String,
    pub slot: String,
    pub content: String,
    pub tokens: usize,
    pub score: f32,
    pub committed_at: DateTime<Utc>,
}

impl CommittedEntry {
    pub fn new(
        id: impl Into<String>,
        slot: impl Into<String>,
        content: impl Into<String>,
        score: f32,
    ) -> Self {
        let content = content.into();
        let tokens = count_tokens(&content);
        Self {
            id: id.into(),
            slot: slot.into(),
            content,
            tokens,
            score,
            committed_at: Utc::now(),
        }
    }
}

/// Bounded, schema-governed Committed Context State (CCS) — the authoritative working
/// context the agent reads. Bounded by a token budget (the saturation level); governed by
/// a schema (typed slots). The qualify-gate decides what enters; this type enforces the
/// structure and exposes the signals (token count, redundancy) the controller acts on.
#[derive(Debug, Clone)]
pub struct CommittedContextState {
    schema: CcsSchema,
    budget_tokens: usize,
    entries: Vec<CommittedEntry>,
}

impl CommittedContextState {
    pub fn new(schema: CcsSchema, budget_tokens: usize) -> Self {
        Self {
            schema,
            budget_tokens,
            entries: Vec::new(),
        }
    }

    pub fn schema(&self) -> &CcsSchema {
        &self.schema
    }

    pub fn budget_tokens(&self) -> usize {
        self.budget_tokens
    }

    pub fn entries(&self) -> &[CommittedEntry] {
        &self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Total committed footprint in tokens.
    pub fn token_count(&self) -> usize {
        self.entries.iter().map(|entry| entry.tokens).sum()
    }

    /// Tokens still available before the budget is reached.
    pub fn headroom(&self) -> usize {
        self.budget_tokens.saturating_sub(self.token_count())
    }

    /// Whether the committed state has reached or exceeded its budget.
    pub fn is_saturated(&self) -> bool {
        self.token_count() >= self.budget_tokens
    }

    /// Whether an entry with this id is already committed.
    pub fn contains(&self, id: &str) -> bool {
        self.entries.iter().any(|entry| entry.id == id)
    }

    /// Maximum token-overlap (Jaccard over word sets) of `content` against any committed
    /// entry — the redundancy signal the gate uses to reject near-duplicates.
    pub fn max_overlap(&self, content: &str) -> f32 {
        let candidate = word_set(content);
        if candidate.is_empty() {
            return 0.0;
        }
        self.entries
            .iter()
            .map(|entry| jaccard(&candidate, &word_set(&entry.content)))
            .fold(0.0_f32, f32::max)
    }

    /// Append an entry. Eligibility and budget are decided by the gate and controller; the
    /// CCS does not re-check them here.
    pub fn admit(&mut self, entry: CommittedEntry) {
        self.entries.push(entry);
    }

    /// Remove and return the lowest-scoring entry — the controller's eviction primitive
    /// under saturation.
    pub fn evict_lowest(&mut self) -> Option<CommittedEntry> {
        let index = self
            .entries
            .iter()
            .enumerate()
            .min_by(|(_, left), (_, right)| {
                left.score
                    .partial_cmp(&right.score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(index, _)| index)?;
        Some(self.entries.remove(index))
    }

    /// The lowest committed score, or `None` when empty.
    pub fn lowest_score(&self) -> Option<f32> {
        self.entries
            .iter()
            .map(|entry| entry.score)
            .fold(None, |acc, score| match acc {
                Some(current) if current <= score => Some(current),
                _ => Some(score),
            })
    }

    /// Render the committed context grouped by schema slot — the markdown the agent reads.
    pub fn render(&self) -> String {
        let mut out = String::new();
        for slot in &self.schema.slots {
            let mut wrote_header = false;
            for entry in self.entries.iter().filter(|entry| &entry.slot == slot) {
                if !wrote_header {
                    let _ = writeln!(out, "## {slot}");
                    wrote_header = true;
                }
                let _ = writeln!(out, "- {}", entry.content);
            }
            if wrote_header {
                out.push('\n');
            }
        }
        let mut wrote_other = false;
        for entry in self
            .entries
            .iter()
            .filter(|entry| !self.schema.contains(&entry.slot))
        {
            if !wrote_other {
                out.push_str("## other\n");
                wrote_other = true;
            }
            let _ = writeln!(out, "- {}", entry.content);
        }
        out.trim_end().to_string()
    }
}

fn word_set(text: &str) -> BTreeSet<String> {
    text.to_lowercase()
        .split(|character: char| !character.is_alphanumeric())
        .filter(|word| word.len() > 2)
        .map(str::to_string)
        .collect()
}

fn jaccard(left: &BTreeSet<String>, right: &BTreeSet<String>) -> f32 {
    let union = left.union(right).count();
    if union == 0 {
        return 0.0;
    }
    let intersection = left.intersection(right).count();
    intersection as f32 / union as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn budget_and_headroom_track_committed_tokens() {
        let mut ccs = CommittedContextState::new(CcsSchema::default(), 1000);
        assert_eq!(ccs.headroom(), 1000);
        assert!(!ccs.is_saturated());
        let entry = CommittedEntry::new("a", "fact", "the build uses cargo workspaces", 1.0);
        let tokens = entry.tokens;
        ccs.admit(entry);
        assert_eq!(ccs.token_count(), tokens);
        assert_eq!(ccs.headroom(), 1000 - tokens);
    }

    #[test]
    fn evict_lowest_removes_min_score() {
        let mut ccs = CommittedContextState::new(CcsSchema::default(), 1000);
        ccs.admit(CommittedEntry::new("a", "fact", "alpha", 0.9));
        ccs.admit(CommittedEntry::new("b", "fact", "beta", 0.2));
        ccs.admit(CommittedEntry::new("c", "fact", "gamma", 0.5));
        let evicted = ccs.evict_lowest().expect("entry");
        assert_eq!(evicted.id, "b");
        assert_eq!(ccs.len(), 2);
    }

    #[test]
    fn redundancy_overlap_detects_near_duplicates() {
        let mut ccs = CommittedContextState::new(CcsSchema::default(), 1000);
        ccs.admit(CommittedEntry::new(
            "a",
            "decision",
            "the team decided to use Rust for the backend service",
            1.0,
        ));
        let high = ccs.max_overlap("the team decided to use Rust for the backend service");
        assert!(
            high > 0.9,
            "identical content should overlap heavily: {high}"
        );
        let low = ccs.max_overlap("deployment runs on a kubernetes cluster nightly");
        assert!(low < 0.2, "unrelated content should barely overlap: {low}");
    }

    #[test]
    fn render_groups_by_slot_in_schema_order() {
        let mut ccs = CommittedContextState::new(CcsSchema::default(), 1000);
        ccs.admit(CommittedEntry::new("a", "fact", "uses cargo", 1.0));
        ccs.admit(CommittedEntry::new("b", "decision", "chose Rust", 1.0));
        let rendered = ccs.render();
        let decision_at = rendered.find("## decision").expect("decision header");
        let fact_at = rendered.find("## fact").expect("fact header");
        assert!(decision_at < fact_at, "schema order: decision before fact");
        assert!(rendered.contains("- chose Rust"));
    }
}
