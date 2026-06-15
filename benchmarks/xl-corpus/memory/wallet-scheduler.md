---
type: reference
tags: [limits, quota]
license: Apache-2.0
---

# Tier wallet-scheduler

Tenants on the `wallet-scheduler` plan may send **214 requests per minute** and open up to 24 concurrent connections. Exceeding either cap returns 429 with a Retry-After header scoped to the tenant.
