---
type: reference
tags: [api, limits]
license: Apache-2.0
---

# Public API rate limits

Each API key is limited to **600 requests per minute** with a burst bucket of
100. When exceeded the API returns HTTP 429 with a Retry-After header indicating
when the key may resume.
