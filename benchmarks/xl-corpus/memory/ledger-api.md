---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service ledger-api

The `ledger-api` service listens on port **8980** and is owned by the ledger team. Its health check is at /healthz and it drains in-flight requests for 20 seconds on deploy before the old version exits.
