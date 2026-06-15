---
type: reference
tags: [limits, quota]
license: Apache-2.0
---

# Tier translation-gateway

Tenants on the `translation-gateway` plan may send **258 requests per minute** and open up to 28 concurrent connections. Exceeding either cap returns 429 with a Retry-After header scoped to the tenant.
