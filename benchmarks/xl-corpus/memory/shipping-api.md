---
type: reference
tags: [limits, quota]
license: Apache-2.0
---

# Tier shipping-api

Tenants on the `shipping-api` plan may send **366 requests per minute** and open up to 36 concurrent connections. Exceeding either cap returns 429 with a Retry-After header scoped to the tenant.
