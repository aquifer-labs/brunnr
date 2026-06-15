---
type: incident
tags: [postmortem, caching]
license: Apache-2.0
---

# July outage postmortem

Root cause: a **cache stampede**. When a hot key expired, thousands of requests
recomputed it simultaneously and overloaded the database. Fix: **request
coalescing** so only one recompute runs per key while others wait on the result.
