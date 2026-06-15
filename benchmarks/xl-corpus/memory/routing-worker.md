---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service routing-worker

The `routing-worker` service listens on port **8812** and is owned by the routing team. Its health check is at /healthz and it drains in-flight requests for 20 seconds on deploy before the old version exits.
