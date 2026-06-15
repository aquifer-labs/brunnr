---
type: reference
tags: [search, performance]
license: Apache-2.0
---

# Search index

The product search index is **rebuilt nightly** and updated incrementally during
the day. The query path targets a **p95 latency of 200 ms**; slower queries fall
back to a narrowed filter set rather than timing out.
