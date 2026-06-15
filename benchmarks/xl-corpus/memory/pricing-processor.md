---
type: reference
tags: [limits, quota]
license: Apache-2.0
---

# Tier pricing-processor

Tenants on the `pricing-processor` plan may send **254 requests per minute** and open up to 24 concurrent connections. Exceeding either cap returns 429 with a Retry-After header scoped to the tenant.
