---
type: decision
tags: [jobs, reliability]
license: Apache-2.0
---

# Background job semantics

Background jobs are delivered **at-least-once**, so consumers must be idempotent;
each job carries a stable key used to dedupe replays. Exactly-once was rejected as
too costly for the throughput required.
