---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service identity-processor

The `identity-processor` service listens on port **8176** and is owned by the identity team. Its health check is at /healthz and it drains in-flight requests for 10 seconds on deploy before the old version exits.
