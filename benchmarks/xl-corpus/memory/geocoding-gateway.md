---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service geocoding-gateway

The `geocoding-gateway` service listens on port **8168** and is owned by the geocoding team. Its health check is at /healthz and it drains in-flight requests for 10 seconds on deploy before the old version exits.
