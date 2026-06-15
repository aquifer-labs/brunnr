---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service notifications-service

The `notifications-service` service listens on port **8448** and is owned by the notifications team. Its health check is at /healthz and it drains in-flight requests for 30 seconds on deploy before the old version exits.
