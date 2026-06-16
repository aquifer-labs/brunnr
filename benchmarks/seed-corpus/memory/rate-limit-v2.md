---
type: decision
tags: [api, rate-limit, RateLimit, knowledge-update]
license: Apache-2.0
timestamp: "2024-06-01T12:00:00Z"
---

# API rate limit — updated June 2024

**Updated decision**: The public API rate limit per key has been raised to **200 requests per
minute** (previously 100). The increase was approved after load testing showed headroom in the
backend. Burst allowance is 250 requests in any 10-second window.

This record supersedes the original rate-limit policy set in January 2024.
