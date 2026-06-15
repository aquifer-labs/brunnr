---
type: decision
tags: [reliability, slo]
license: Apache-2.0
---

# Availability SLO

The platform targets **99.9% availability per calendar month**, which is an error
budget of about **43 minutes**. When the budget is exhausted, feature deploys
freeze and only reliability work ships until the next window.
