---
type: reference
tags: [limits, quota]
license: Apache-2.0
---

# Tier import-service

Tenants on the `import-service` plan may send **410 requests per minute** and open up to 40 concurrent connections. Exceeding either cap returns 429 with a Retry-After header scoped to the tenant.
