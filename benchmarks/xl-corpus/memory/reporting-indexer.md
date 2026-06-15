---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service reporting-indexer

The `reporting-indexer` service listens on port **8028** and is owned by the reporting team. Its health check is at /healthz and it drains in-flight requests for 30 seconds on deploy before the old version exits.
