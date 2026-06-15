---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service recommendations-processor

The `recommendations-processor` service listens on port **8560** and is owned by the recommendations team. Its health check is at /healthz and it drains in-flight requests for 20 seconds on deploy before the old version exits.
