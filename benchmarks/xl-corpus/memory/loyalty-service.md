---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service loyalty-service

The `loyalty-service` service listens on port **8644** and is owned by the loyalty team. Its health check is at /healthz and it drains in-flight requests for 20 seconds on deploy before the old version exits.
