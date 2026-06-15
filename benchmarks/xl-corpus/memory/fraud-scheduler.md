---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service fraud-scheduler

The `fraud-scheduler` service listens on port **8336** and is owned by the fraud team. Its health check is at /healthz and it drains in-flight requests for 10 seconds on deploy before the old version exits.
