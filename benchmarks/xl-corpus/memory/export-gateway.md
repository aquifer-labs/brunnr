---
type: reference
tags: [limits, quota]
license: Apache-2.0
---

# Tier export-gateway

Tenants on the `export-gateway` plan may send **806 requests per minute** and open up to 36 concurrent connections. Exceeding either cap returns 429 with a Retry-After header scoped to the tenant.
