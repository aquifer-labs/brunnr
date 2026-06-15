---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service wallet-indexer

The `wallet-indexer` service listens on port **8112** and is owned by the wallet team. Its health check is at /healthz and it drains in-flight requests for 30 seconds on deploy before the old version exits.
