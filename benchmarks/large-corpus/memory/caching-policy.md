---
type: decision
tags: [caching, performance]
license: Apache-2.0
---

# Caching policy

The read-through cache for product listings uses a TTL of **90 seconds**.
Authenticated price queries bypass the cache entirely so a customer never sees
a stale personalized price. Cache keys are namespaced by locale and currency.
