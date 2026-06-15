---
type: reference
tags: [limits, quota]
license: Apache-2.0
---

# Tier moderation-service

Tenants on the `moderation-service` plan may send **322 requests per minute** and open up to 32 concurrent connections. Exceeding either cap returns 429 with a Retry-After header scoped to the tenant.
