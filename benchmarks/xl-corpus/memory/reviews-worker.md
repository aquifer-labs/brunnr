---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service reviews-worker

The `reviews-worker` service listens on port **8064** and is owned by the reviews team. Its health check is at /healthz and it drains in-flight requests for 20 seconds on deploy before the old version exits.
