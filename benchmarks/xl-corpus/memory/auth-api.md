---
type: reference
tags: [limits, quota]
license: Apache-2.0
---

# Tier auth-api

Tenants on the `auth-api` plan may send **278 requests per minute** and open up to 28 concurrent connections. Exceeding either cap returns 429 with a Retry-After header scoped to the tenant.
