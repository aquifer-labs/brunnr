---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service audit-api

The `audit-api` service listens on port **8896** and is owned by the audit team. Its health check is at /healthz and it drains in-flight requests for 20 seconds on deploy before the old version exits.
