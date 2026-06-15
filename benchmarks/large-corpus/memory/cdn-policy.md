---
type: reference
tags: [cdn, performance]
license: Apache-2.0
---

# CDN policy

Static assets are cached at the edge for **7 days** and fingerprinted by content
hash. A deploy issues a targeted purge for changed assets only; a full purge is
forbidden in business hours because it stampedes the origin.
