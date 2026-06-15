---
type: reference
tags: [reliability, sre]
license: Apache-2.0
---

# Reliability principles

We retry transient failures with backoff and jitter, rate-limit abusive
clients, run multiple regions with failover, and fail closed on config errors.
These are principles, not configured values — the specific numbers live in each
service's own decision record.
