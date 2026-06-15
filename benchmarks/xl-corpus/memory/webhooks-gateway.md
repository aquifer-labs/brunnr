---
type: reference
tags: [service, infrastructure]
license: Apache-2.0
---

# Service webhooks-gateway

The `webhooks-gateway` service listens on port **8784** and is owned by the webhooks team. Its health check is at /healthz and it drains in-flight requests for 30 seconds on deploy before the old version exits.
