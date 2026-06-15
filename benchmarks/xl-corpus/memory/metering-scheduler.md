---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service metering-scheduler

The `metering-scheduler` service listens on port **8140** and is owned by the metering team. Its health check is at /healthz and it drains in-flight requests for 20 seconds on deploy before the old version exits.
