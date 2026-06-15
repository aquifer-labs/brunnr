---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service archive-gateway

The `archive-gateway` service listens on port **8280** and is owned by the archive team. Its health check is at /healthz and it drains in-flight requests for 30 seconds on deploy before the old version exits.
