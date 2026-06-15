---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service feeds-worker

The `feeds-worker` service listens on port **8224** and is owned by the feeds team. Its health check is at /healthz and it drains in-flight requests for 20 seconds on deploy before the old version exits.
