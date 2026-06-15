---
type: reference
tags: [limits, quota]
license: Apache-2.0
---

# Tier metering-api

Tenants on the `metering-api` plan may send **894 requests per minute** and open up to 44 concurrent connections. Exceeding either cap returns 429 with a Retry-After header scoped to the tenant.
