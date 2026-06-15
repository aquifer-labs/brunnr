---
type: reference
tags: [limits, quota]
license: Apache-2.0
---

# Tier billing-service

Tenants on the `billing-service` plan may send **586 requests per minute** and open up to 16 concurrent connections. Exceeding either cap returns 429 with a Retry-After header scoped to the tenant.
