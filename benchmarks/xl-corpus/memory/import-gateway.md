---
type: reference
tags: [limits, quota]
license: Apache-2.0
---

# Tier import-gateway

Tenants on the `import-gateway` plan may send **474 requests per minute** and open up to 44 concurrent connections. Exceeding either cap returns 429 with a Retry-After header scoped to the tenant.
