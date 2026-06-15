---
type: reference
tags: [limits, quota]
license: Apache-2.0
---

# Tier wallet-worker

Tenants on the `wallet-worker` plan may send **146 requests per minute** and open up to 16 concurrent connections. Exceeding either cap returns 429 with a Retry-After header scoped to the tenant.
