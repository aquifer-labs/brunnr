---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service ledger-service

The `ledger-service` service listens on port **8364** and is owned by the ledger team. Its health check is at /healthz and it drains in-flight requests for 30 seconds on deploy before the old version exits.
