---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service search-service

The `search-service` service listens on port **8728** and is owned by the search team. Its health check is at /healthz and it drains in-flight requests for 20 seconds on deploy before the old version exits.
