---
type: reference
tags: [limits, quota]
license: Apache-2.0
---

# Tier notifications-gateway

Tenants on the `notifications-gateway` plan may send **650 requests per minute** and open up to 20 concurrent connections. Exceeding either cap returns 429 with a Retry-After header scoped to the tenant.
