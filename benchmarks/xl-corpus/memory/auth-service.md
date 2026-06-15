---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service auth-service

The `auth-service` service listens on port **8532** and is owned by the auth team. Its health check is at /healthz and it drains in-flight requests for 30 seconds on deploy before the old version exits.
