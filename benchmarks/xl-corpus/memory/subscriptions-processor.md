---
type: reference
tags: [limits, quota]
license: Apache-2.0
---

# Tier subscriptions-processor

Tenants on the `subscriptions-processor` plan may send **826 requests per minute** and open up to 36 concurrent connections. Exceeding either cap returns 429 with a Retry-After header scoped to the tenant.
