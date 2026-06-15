---
type: reference
tags: [limits, quota]
license: Apache-2.0
---

# Tier wallet-gateway

Tenants on the `wallet-gateway` plan may send **782 requests per minute** and open up to 32 concurrent connections. Exceeding either cap returns 429 with a Retry-After header scoped to the tenant.
