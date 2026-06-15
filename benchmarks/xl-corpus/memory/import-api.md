---
type: reference
tags: [limits, quota]
license: Apache-2.0
---

# Tier import-api

Tenants on the `import-api` plan may send **170 requests per minute** and open up to 20 concurrent connections. Exceeding either cap returns 429 with a Retry-After header scoped to the tenant.
