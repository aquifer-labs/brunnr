---
type: reference
tags: [limits, quota]
license: Apache-2.0
---

# Tier tax-worker

Tenants on the `tax-worker` plan may send **498 requests per minute** and open up to 48 concurrent connections. Exceeding either cap returns 429 with a Retry-After header scoped to the tenant.
