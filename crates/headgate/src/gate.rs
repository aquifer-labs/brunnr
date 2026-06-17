// SPDX-License-Identifier: Apache-2.0

use futures_util::{future::BoxFuture, FutureExt};
use serde::{Deserialize, Serialize};

use crate::{CommittedContextState, RecallItem};

/// The qualify-gate's verdict on a single recall candidate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualifyDecision {
    pub admitted: bool,
    pub reason: String,
    pub slot: Option<String>,
    pub score: f32,
}

impl QualifyDecision {
    pub fn admit(slot: impl Into<String>, score: f32) -> Self {
        Self {
            admitted: true,
            reason: "qualified".to_string(),
            slot: Some(slot.into()),
            score,
        }
    }

    pub fn reject(reason: impl Into<String>, score: f32) -> Self {
        Self {
            admitted: false,
            reason: reason.into(),
            slot: None,
            score,
        }
    }
}

/// The qualify-gate — the ACC trust boundary. Only candidates that qualify (relevant,
/// non-duplicate, non-redundant) are eligible to enter the committed state. The default
/// implementation is deterministic; the feature-gated LLM judge-eval gate
/// ([`crate::JudgeQualifyGate`], scoring drift / hallucination) is a drop-in replacement.
///
/// `qualify` is async so an implementation may consult an external judge (an LLM); the
/// deterministic gate resolves immediately. An implementation that cannot reach its judge
/// should return a conservative reject rather than surface an error.
pub trait QualifyGate: Send + Sync {
    fn qualify<'a>(
        &'a self,
        item: &'a RecallItem,
        ccs: &'a CommittedContextState,
    ) -> BoxFuture<'a, QualifyDecision>;
}

/// Deterministic default gate: relevance threshold + redundancy rejection + slot routing.
#[derive(Debug, Clone)]
pub struct DefaultQualifyGate {
    /// Minimum candidate score to qualify (interpreted on the recall store's score scale).
    pub min_score: f32,
    /// Token-overlap at or above which a candidate is treated as redundant.
    pub redundancy_threshold: f32,
    slot_keywords: Vec<(String, Vec<String>)>,
}

impl DefaultQualifyGate {
    pub fn new(min_score: f32, redundancy_threshold: f32) -> Self {
        Self {
            min_score,
            redundancy_threshold,
            slot_keywords: default_slot_keywords(),
        }
    }

    /// Override the keyword → slot routing table.
    pub fn with_slot_keywords(mut self, slot_keywords: Vec<(String, Vec<String>)>) -> Self {
        self.slot_keywords = slot_keywords;
        self
    }

    fn route_slot(&self, item: &RecallItem, ccs: &CommittedContextState) -> String {
        let lower = item.content.to_lowercase();
        for (slot, keywords) in &self.slot_keywords {
            if ccs.schema().contains(slot) && keywords.iter().any(|keyword| lower.contains(keyword))
            {
                return slot.clone();
            }
        }
        ccs.schema().default_slot().to_string()
    }
}

impl Default for DefaultQualifyGate {
    fn default() -> Self {
        Self::new(0.2, 0.8)
    }
}

impl DefaultQualifyGate {
    fn decide(&self, item: &RecallItem, ccs: &CommittedContextState) -> QualifyDecision {
        if item.score < self.min_score {
            return QualifyDecision::reject(
                format!(
                    "below relevance threshold ({:.3} < {:.3})",
                    item.score, self.min_score
                ),
                item.score,
            );
        }
        if ccs.contains(&item.id) {
            return QualifyDecision::reject("already committed", item.score);
        }
        let overlap = ccs.max_overlap(&item.content);
        if overlap >= self.redundancy_threshold {
            return QualifyDecision::reject(
                format!(
                    "redundant (overlap {overlap:.3} >= {:.3})",
                    self.redundancy_threshold
                ),
                item.score,
            );
        }
        QualifyDecision::admit(self.route_slot(item, ccs), item.score)
    }
}

impl QualifyGate for DefaultQualifyGate {
    fn qualify<'a>(
        &'a self,
        item: &'a RecallItem,
        ccs: &'a CommittedContextState,
    ) -> BoxFuture<'a, QualifyDecision> {
        let decision = self.decide(item, ccs);
        async move { decision }.boxed()
    }
}

fn default_slot_keywords() -> Vec<(String, Vec<String>)> {
    vec![
        (
            "decision".to_string(),
            ["decid", "chose", "chosen", "will use", "agreed", "selected"]
                .iter()
                .map(|keyword| keyword.to_string())
                .collect(),
        ),
        (
            "constraint".to_string(),
            ["must", "never", "always", "require", "cannot", "do not"]
                .iter()
                .map(|keyword| keyword.to_string())
                .collect(),
        ),
        (
            "task-state".to_string(),
            ["todo", "in progress", "blocked", "next step", "remaining"]
                .iter()
                .map(|keyword| keyword.to_string())
                .collect(),
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CcsSchema;

    fn empty_ccs() -> CommittedContextState {
        CommittedContextState::new(CcsSchema::default(), 4096)
    }

    #[test]
    fn rejects_below_relevance() {
        let gate = DefaultQualifyGate::new(0.5, 0.8);
        let item = RecallItem::new("a", "some weakly relevant note", 0.1);
        let decision = gate.decide(&item, &empty_ccs());
        assert!(!decision.admitted);
        assert!(decision.reason.contains("below relevance"));
    }

    #[test]
    fn routes_decision_keyword_to_decision_slot() {
        let gate = DefaultQualifyGate::default();
        let item = RecallItem::new("a", "we chose Rust for the core crates", 1.0);
        let decision = gate.decide(&item, &empty_ccs());
        assert!(decision.admitted);
        assert_eq!(decision.slot.as_deref(), Some("decision"));
    }

    #[test]
    fn routes_unmatched_to_default_slot() {
        let gate = DefaultQualifyGate::default();
        let item = RecallItem::new("a", "the cluster has three nodes", 1.0);
        let decision = gate.decide(&item, &empty_ccs());
        assert_eq!(decision.slot.as_deref(), Some("decision")); // default_slot = first
    }

    #[test]
    fn rejects_redundant_against_committed() {
        let gate = DefaultQualifyGate::new(0.2, 0.6);
        let mut ccs = empty_ccs();
        ccs.admit(crate::CommittedEntry::new(
            "a",
            "fact",
            "the deployment runs nightly on the kubernetes cluster",
            1.0,
        ));
        let item = RecallItem::new(
            "b",
            "the deployment runs nightly on the kubernetes cluster",
            1.0,
        );
        let decision = gate.decide(&item, &ccs);
        assert!(!decision.admitted);
        assert!(decision.reason.contains("redundant"));
    }
}
