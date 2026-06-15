---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service catalog-worker

The `catalog-worker` service listens on port **8616** and is owned by the catalog team. Its health check is at /healthz and it drains in-flight requests for 30 seconds on deploy before the old version exits.
