---
type: incident
tags: [postmortem, cost]
license: Apache-2.0
---

# Egress cost incident

Root cause: a misconfigured CDN rule **bypassed the cache and pulled every asset
from origin**, multiplying egress cost tenfold overnight. Fix: enable an **origin
shield** tier and add a budget alert on origin egress.
