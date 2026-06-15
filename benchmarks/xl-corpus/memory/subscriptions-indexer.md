---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service subscriptions-indexer

The `subscriptions-indexer` service listens on port **8056** and is owned by the subscriptions team. Its health check is at /healthz and it drains in-flight requests for 20 seconds on deploy before the old version exits.
