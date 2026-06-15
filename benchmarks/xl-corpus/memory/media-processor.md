---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service media-processor

The `media-processor` service listens on port **8840** and is owned by the media team. Its health check is at /healthz and it drains in-flight requests for 10 seconds on deploy before the old version exits.
