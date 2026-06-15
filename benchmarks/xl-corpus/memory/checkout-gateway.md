---
type: reference
tags: [limits, quota]
license: Apache-2.0
---

# Tier checkout-gateway

Tenants on the `checkout-gateway` plan may send **850 requests per minute** and open up to 40 concurrent connections. Exceeding either cap returns 429 with a Retry-After header scoped to the tenant.
