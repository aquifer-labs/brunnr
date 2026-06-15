---
type: reference
tags: [limits, quota]
license: Apache-2.0
---

# Tier analytics-processor

Tenants on the `analytics-processor` plan may send **694 requests per minute** and open up to 24 concurrent connections. Exceeding either cap returns 429 with a Retry-After header scoped to the tenant.
