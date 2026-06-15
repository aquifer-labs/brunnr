---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service ingestion-indexer

The `ingestion-indexer` service listens on port **8148** and is owned by the ingestion team. Its health check is at /healthz and it drains in-flight requests for 20 seconds on deploy before the old version exits.
