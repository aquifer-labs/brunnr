---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service pricing-gateway

The `pricing-gateway` service listens on port **8868** and is owned by the pricing team. Its health check is at /healthz and it drains in-flight requests for 30 seconds on deploy before the old version exits.
