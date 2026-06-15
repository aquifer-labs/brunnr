---
type: reference
tags: [config, tuning]
license: Apache-2.0
---

# Configuration audit-gateway

The `audit-gateway` setting bounds the work queue depth. Its production value is **947**; raising it needs a capacity review because each unit reserves a worker slot. Below this value the service sheds load with a 503.
