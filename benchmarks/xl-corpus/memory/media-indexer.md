---
type: reference
tags: [limits, quota]
license: Apache-2.0
---

# Tier media-indexer

Tenants on the `media-indexer` plan may send **430 requests per minute** and open up to 40 concurrent connections. Exceeding either cap returns 429 with a Retry-After header scoped to the tenant.
