---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service refunds-processor

The `refunds-processor` service listens on port **8308** and is owned by the refunds team. Its health check is at /healthz and it drains in-flight requests for 20 seconds on deploy before the old version exits.
