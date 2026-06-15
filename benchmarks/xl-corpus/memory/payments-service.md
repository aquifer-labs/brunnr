---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service payments-service

The `payments-service` service listens on port **8756** and is owned by the payments team. Its health check is at /healthz and it drains in-flight requests for 10 seconds on deploy before the old version exits.
