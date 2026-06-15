---
type: decision
tags: [deployment, release]
license: Apache-2.0
---

# Deploy strategy

Releases use **blue-green** deploys with a **10% canary held for 15 minutes**.
If canary error rate exceeds baseline by more than two standard deviations the
rollout aborts and traffic stays on the previous color.
