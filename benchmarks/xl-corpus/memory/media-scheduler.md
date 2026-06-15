---
type: reference
tags: [limits, quota]
license: Apache-2.0
---

# Tier media-scheduler

Tenants on the `media-scheduler` plan may send **342 requests per minute** and open up to 32 concurrent connections. Exceeding either cap returns 429 with a Retry-After header scoped to the tenant.
