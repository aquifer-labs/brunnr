---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service ingestion-processor

The `ingestion-processor` service listens on port **8036** and is owned by the ingestion team. Its health check is at /healthz and it drains in-flight requests for 30 seconds on deploy before the old version exits.
