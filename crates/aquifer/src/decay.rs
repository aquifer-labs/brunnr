// SPDX-License-Identifier: Apache-2.0

//! Recency-aware soft-decay for retrieval ranking.
//!
//! [`retrieval_strength`] computes a per-record multiplier in `[floor, ceil]` from:
//! - **time-since-last-access** (`dt`): exponential decay `e^(-lambda * dt_days)`, bounded below
//!   by `floor` so never-accessed or very stale records remain discoverable.
//! - **access boost**: records retrieved frequently get a multiplier up to `access_boost` (≥ 1.0).
//!
//! The caller multiplies the stored similarity score by this value **without mutating** the record.
//! Backward-compat: records with `last_access = None` return `strength = 1.0` unchanged.
//!
//! ## mem0 alignment
//! - Unused records → ~0.3× (floor default 0.2 * boost cancels to ~0.3 at low count)
//! - Recently + frequently accessed → up to ~1.5× (`access_boost` default 1.5)
//! - Half-life default 30 days (`lambda ≈ ln(2)/30 ≈ 0.023`)

use chrono::{DateTime, Utc};

use crate::MemoryRecord;

/// Configuration for retrieval-time recency soft-decay.
///
/// All defaults are chosen to match mem0's soft-dampening profile
/// (unused→~0.3×, recently-accessed→~1.5×) without aggressive eviction.
#[derive(Debug, Clone, PartialEq)]
pub struct DecayConfig {
    /// Enable recency-aware re-ranking. When `false`, `retrieval_strength` always returns 1.0.
    pub decay_enabled: bool,

    /// Decay half-life in days. `e^(-lambda * dt_days)` where `lambda = ln(2) / half_life`.
    /// Default: 30 days.
    pub half_life_days: f32,

    /// Minimum retrieval strength (floor). Records well past the half-life approach this value,
    /// never zero. Default: 0.2 (stale → ~0.2× dampening).
    pub floor: f32,

    /// Maximum access-frequency boost applied on top of the decay factor.
    /// `access_boost` applies at 50 accesses; linear scale. Default: 1.5.
    pub access_boost: f32,
}

impl Default for DecayConfig {
    fn default() -> Self {
        Self {
            decay_enabled: true,
            half_life_days: 30.0,
            floor: 0.2,
            access_boost: 1.5,
        }
    }
}

impl DecayConfig {
    fn lambda(&self) -> f32 {
        // lambda = ln(2) / half_life
        std::f32::consts::LN_2 / self.half_life_days.max(1.0)
    }
}

/// Compute the retrieval strength multiplier for a record.
///
/// Returns a value in `[floor, access_boost]`.
/// Records without `last_access` return `1.0` (no signal → no dampening).
pub fn retrieval_strength(record: &MemoryRecord, config: &DecayConfig, now: DateTime<Utc>) -> f32 {
    if !config.decay_enabled {
        return 1.0;
    }
    let Some(last) = record.last_access else {
        // No access signal yet — return neutral 1.0 (backward-compat, not dampened)
        return 1.0;
    };
    let dt_days = now.signed_duration_since(last).num_seconds().max(0) as f32 / 86_400.0;
    let decay = (-config.lambda() * dt_days).exp();
    // Clamp to floor so stale records remain discoverable
    let decay_clamped = decay.max(config.floor / config.access_boost.max(1.0));

    // Access boost: 50+ accesses → full boost, linear below
    let count = record.access_count as f32;
    let boost = 1.0 + (config.access_boost - 1.0) * (count / 50.0).min(1.0);

    (decay_clamped * boost).clamp(config.floor, config.access_boost)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use chrono::{Duration, Utc};

    use super::*;
    use crate::{MemoryId, MemoryTier};

    fn make_record(access_count: u32, last_access_days_ago: Option<f32>) -> MemoryRecord {
        let mut r = MemoryRecord::new(
            MemoryId::new("test"),
            "node:test",
            "content",
            Vec::new(),
            BTreeMap::new(),
            MemoryTier::L1Atom,
        );
        r.access_count = access_count;
        r.last_access = last_access_days_ago
            .map(|days| Utc::now() - Duration::seconds((days * 86_400.0) as i64));
        r
    }

    #[test]
    fn no_last_access_returns_neutral() {
        let record = make_record(0, None);
        let strength = retrieval_strength(&record, &DecayConfig::default(), Utc::now());
        assert!(
            (strength - 1.0).abs() < 1e-6,
            "should be 1.0, got {strength}"
        );
    }

    #[test]
    fn decay_disabled_returns_one() {
        let record = make_record(0, Some(365.0));
        let config = DecayConfig {
            decay_enabled: false,
            ..DecayConfig::default()
        };
        let strength = retrieval_strength(&record, &config, Utc::now());
        assert!((strength - 1.0).abs() < 1e-6);
    }

    #[test]
    fn recently_accessed_high_count_gets_boost() {
        // 0 days ago, 50 accesses → decay ≈ 1.0, boost ≈ 1.5
        let record = make_record(50, Some(0.0));
        let strength = retrieval_strength(&record, &DecayConfig::default(), Utc::now());
        assert!(
            strength > 1.3,
            "recent + high-count should be > 1.3, got {strength}"
        );
        assert!(strength <= 1.5 + 1e-3);
    }

    #[test]
    fn stale_record_is_dampened_but_not_zeroed() {
        // 180 days ago (6 × half-life), no accesses → decayed well below 1.0 but >= floor
        let record = make_record(0, Some(180.0));
        let config = DecayConfig::default();
        let strength = retrieval_strength(&record, &config, Utc::now());
        assert!(
            strength >= config.floor,
            "strength should be >= floor={}, got {strength}",
            config.floor
        );
        assert!(
            strength < 0.5,
            "highly stale record should be dampened, got {strength}"
        );
    }

    #[test]
    fn stale_downranks_vs_fresh_same_base_score() {
        let now = Utc::now();
        let stale = make_record(0, Some(120.0)); // 4 half-lives ago
        let fresh = make_record(5, Some(1.0)); // 1 day ago, some accesses
        let config = DecayConfig::default();

        let s_stale = 0.9 * retrieval_strength(&stale, &config, now);
        let s_fresh = 0.9 * retrieval_strength(&fresh, &config, now);
        assert!(
            s_fresh > s_stale,
            "fresh record (score={s_fresh:.3}) should outrank stale (score={s_stale:.3})"
        );
    }
}
