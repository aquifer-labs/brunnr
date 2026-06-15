---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service shipping-service

The `shipping-service` service listens on port **8232** and is owned by the shipping team. Its health check is at /healthz and it drains in-flight requests for 20 seconds on deploy before the old version exits.
