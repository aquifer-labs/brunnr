---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service ledger-indexer

The `ledger-indexer` service listens on port **8672** and is owned by the ledger team. Its health check is at /healthz and it drains in-flight requests for 10 seconds on deploy before the old version exits.
