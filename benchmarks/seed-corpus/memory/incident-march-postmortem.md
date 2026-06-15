---
type: incident
tags: [postmortem, database]
license: Apache-2.0
---

# March outage postmortem

Root cause: **connection-pool exhaustion**. A deploy doubled the application max
pool size without raising the database max_connections, so new pods could not
acquire connections. Fix: pin the total pool size to 80% of database capacity
and alert when utilization crosses 70%.
