---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service inventory-processor

The `inventory-processor` service listens on port **8092** and is owned by the inventory team. Its health check is at /healthz and it drains in-flight requests for 10 seconds on deploy before the old version exits.
