---
type: reference
tags: [limits, quota]
license: Apache-2.0
---

# Tier fraud-worker

Tenants on the `fraud-worker` plan may send **914 requests per minute** and open up to 44 concurrent connections. Exceeding either cap returns 429 with a Retry-After header scoped to the tenant.
