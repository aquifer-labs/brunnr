---
type: reference
tags: [caching, faq]
license: Apache-2.0
---

# Caching FAQ

Yes, the platform caches product listings, and yes, every cache entry has a TTL.
TTLs are configurable per resource by the owning team; consult the resource's
config for its exact value. Authenticated requests may bypass the cache.
