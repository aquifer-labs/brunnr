---
type: reference
tags: [limits, quota]
license: Apache-2.0
---

# Tier metering-service

Tenants on the `metering-service` plan may send **674 requests per minute** and open up to 24 concurrent connections. Exceeding either cap returns 429 with a Retry-After header scoped to the tenant.
