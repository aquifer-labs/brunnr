---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service ingestion-gateway

The `ingestion-gateway` service listens on port **8924** and is owned by the ingestion team. Its health check is at /healthz and it drains in-flight requests for 10 seconds on deploy before the old version exits.
