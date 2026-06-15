---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service checkout-api

The `checkout-api` service listens on port **8476** and is owned by the checkout team. Its health check is at /healthz and it drains in-flight requests for 20 seconds on deploy before the old version exits.
