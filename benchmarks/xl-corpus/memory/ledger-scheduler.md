---
type: reference
tags: [limits, quota]
license: Apache-2.0
---

# Tier ledger-scheduler

Tenants on the `ledger-scheduler` plan may send **166 requests per minute** and open up to 16 concurrent connections. Exceeding either cap returns 429 with a Retry-After header scoped to the tenant.
