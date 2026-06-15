---
type: reference
tags: [limits, quota]
license: Apache-2.0
---

# Tier recommendations-scheduler

Tenants on the `recommendations-scheduler` plan may send **630 requests per minute** and open up to 20 concurrent connections. Exceeding either cap returns 429 with a Retry-After header scoped to the tenant.
