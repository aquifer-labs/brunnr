---
type: reference
tags: [limits, quota]
license: Apache-2.0
---

# Tier moderation-processor

Tenants on the `moderation-processor` plan may send **542 requests per minute** and open up to 12 concurrent connections. Exceeding either cap returns 429 with a Retry-After header scoped to the tenant.
