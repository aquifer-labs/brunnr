---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service gateway-worker

The `gateway-worker` service listens on port **8392** and is owned by the gateway team. Its health check is at /healthz and it drains in-flight requests for 20 seconds on deploy before the old version exits.
