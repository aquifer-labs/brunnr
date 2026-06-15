---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service messaging-gateway

The `messaging-gateway` service listens on port **8008** and is owned by the messaging team. Its health check is at /healthz and it drains in-flight requests for 10 seconds on deploy before the old version exits.
