---
type: decision
tags: [api, rate-limit, RateLimit, knowledge-update]
license: Apache-2.0
timestamp: "2024-01-15T09:00:00Z"
---

# API rate limit — original January 2024

The public API rate limit per key is **100 requests per minute**. Requests above the limit
receive HTTP 429. The limit applies per API key; unauthenticated calls share a 20 req/min
global pool. This was the initial rate-limit decision at product launch.

**Note**: This record has been superseded — see the June 2024 update.
