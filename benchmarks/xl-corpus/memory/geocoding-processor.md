---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service geocoding-processor

The `geocoding-processor` service listens on port **8204** and is owned by the geocoding team. Its health check is at /healthz and it drains in-flight requests for 30 seconds on deploy before the old version exits.
