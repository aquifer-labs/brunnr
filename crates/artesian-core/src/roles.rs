// SPDX-License-Identifier: Apache-2.0

use std::{collections::VecDeque, str::FromStr};

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum Role {
    Master,
    Worker,
    Judge,
}

impl Role {
    pub const fn canonical_alias(self) -> &'static str {
        match self {
            Self::Master => "master",
            Self::Worker => "worker",
            Self::Judge => "judge",
        }
    }

    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Master => "Master",
            Self::Worker => "Worker",
            Self::Judge => "Judge",
        }
    }

    pub const fn aliases(self) -> &'static [&'static str] {
        match self {
            Self::Master => &["master"],
            Self::Worker => &["worker"],
            Self::Judge => &["judge"],
        }
    }
}

impl FromStr for Role {
    type Err = RoleParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.trim().to_ascii_lowercase().as_str() {
            "master" => Ok(Self::Master),
            "worker" => Ok(Self::Worker),
            "judge" => Ok(Self::Judge),
            other => Err(RoleParseError {
                value: other.to_string(),
            }),
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("unknown role alias: {value}")]
pub struct RoleParseError {
    value: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum JobStatus {
    Todo,
    Doing,
    Done,
    Blocked,
}

/// A lightweight, role-tagged unit of work in the in-core queue. (The file-backed task
/// tracker with its DAG lives in the `headrace` crate; this is the minimal queue primitive.)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub title: String,
    pub role: Role,
    pub status: JobStatus,
}

/// A FIFO queue of role-tagged jobs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Queue {
    entries: VecDeque<Job>,
}

impl Queue {
    pub fn push(&mut self, job: Job) {
        self.entries.push_back(job);
    }

    pub fn pop_next(&mut self) -> Option<Job> {
        self.entries.pop_front()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// A completed job together with the commit that recorded it — the judge's accepted output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletedJob {
    pub job: Job,
    pub commit: Option<String>,
    pub completed_at: DateTime<Utc>,
}
