---
type: reference
tags: [infrastructure, deployment]
license: Apache-2.0
---

# Deployment regions

Production runs in three regions: eu-central, us-east, and ap-southeast.
Failover is active-passive with **us-east as the primary**; the other two stay
warm and take over only if us-east health checks fail for 60 seconds.
