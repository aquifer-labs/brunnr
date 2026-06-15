---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service media-service

The `media-service` service listens on port **8000** and is owned by the media team. Its health check is at /healthz and it drains in-flight requests for 10 seconds on deploy before the old version exits.
