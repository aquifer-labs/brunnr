---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service routing-service

The `routing-service` service listens on port **8588** and is owned by the routing team. Its health check is at /healthz and it drains in-flight requests for 10 seconds on deploy before the old version exits.
