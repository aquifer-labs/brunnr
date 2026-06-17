// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceQuota {
    pub agent_id: Option<String>,
    pub user_id: Option<String>,
    pub max_prompt_tokens: Option<u64>,
    pub max_requests_per_minute: Option<u32>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenAccounting {
    pub agent_id: String,
    pub session_id: Option<String>,
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
}

impl TokenAccounting {
    pub fn total_tokens(&self) -> u64 {
        self.prompt_tokens + self.completion_tokens
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Barrier {
    pub id: String,
    pub waits_for: BTreeSet<String>,
}

impl Barrier {
    pub fn new(id: impl Into<String>, waits_for: impl IntoIterator<Item = String>) -> Self {
        Self {
            id: id.into(),
            waits_for: waits_for.into_iter().collect(),
        }
    }

    pub fn is_satisfied_by<'a>(&self, completed: impl IntoIterator<Item = &'a String>) -> bool {
        let completed = completed.into_iter().collect::<BTreeSet<_>>();
        self.waits_for
            .iter()
            .all(|dependency| completed.contains(dependency))
    }
}
