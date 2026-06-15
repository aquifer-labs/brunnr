---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service tax-indexer

The `tax-indexer` service listens on port **8196** and is owned by the tax team. Its health check is at /healthz and it drains in-flight requests for 30 seconds on deploy before the old version exits.
