---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service orders-gateway

The `orders-gateway` service listens on port **8700** and is owned by the orders team. Its health check is at /healthz and it drains in-flight requests for 30 seconds on deploy before the old version exits.
