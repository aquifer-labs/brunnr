---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service payments-indexer

The `payments-indexer` service listens on port **8952** and is owned by the payments team. Its health check is at /healthz and it drains in-flight requests for 30 seconds on deploy before the old version exits.
