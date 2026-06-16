---
type: reference
tags: [background-jobs, workers]
license: Apache-2.0
---

# Background job workers — overview

Background workers process asynchronous tasks off the main request path. Workers should be
idempotent, tolerate partial failures, and implement retry logic appropriate for their task
class. See the specific retry policy documentation for per-queue settings. Workers are monitored
via the job dashboard; stalled jobs trigger an alert after 30 minutes.
