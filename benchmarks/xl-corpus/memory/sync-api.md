---
type: reference
tags: [limits, quota]
license: Apache-2.0
---

# Tier sync-api

Tenants on the `sync-api` plan may send **386 requests per minute** and open up to 36 concurrent connections. Exceeding either cap returns 429 with a Retry-After header scoped to the tenant.
