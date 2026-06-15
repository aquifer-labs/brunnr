---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service refunds-indexer

The `refunds-indexer` service listens on port **8504** and is owned by the refunds team. Its health check is at /healthz and it drains in-flight requests for 10 seconds on deploy before the old version exits.
