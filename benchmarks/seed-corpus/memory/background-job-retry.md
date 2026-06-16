---
type: decision
tags: [retry, background-jobs, BackgroundJobRetryPolicy]
license: Apache-2.0
timestamp: "2024-02-15T10:00:00Z"
---

# Background job retry policy

The `BackgroundJobRetryPolicy` for queue workers uses **exponential backoff** with a maximum of
**3 attempts**. The initial delay is 5 seconds; each subsequent attempt doubles the delay.
Workers that exhaust retries are dead-lettered to the `failed-jobs` queue for manual inspection.
This policy applies to all Sidekiq and Celery background jobs — it is **not** the same as the
outbound HTTP call retry policy.
