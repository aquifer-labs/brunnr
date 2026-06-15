---
type: reference
tags: [limits, quota]
license: Apache-2.0
---

# Tier gateway-api

Tenants on the `gateway-api` plan may send **518 requests per minute** and open up to 48 concurrent connections. Exceeding either cap returns 429 with a Retry-After header scoped to the tenant.
