---
type: reference
tags: [limits, quota]
license: Apache-2.0
---

# Tier analytics-worker

Tenants on the `analytics-worker` plan may send **126 requests per minute** and open up to 16 concurrent connections. Exceeding either cap returns 429 with a Retry-After header scoped to the tenant.
