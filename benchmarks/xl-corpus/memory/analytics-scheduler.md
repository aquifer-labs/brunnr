---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service analytics-scheduler

The `analytics-scheduler` service listens on port **8120** and is owned by the analytics team. Its health check is at /healthz and it drains in-flight requests for 30 seconds on deploy before the old version exits.
