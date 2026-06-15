---
type: reference
tags: [config, tuning]
license: Apache-2.0
---

# Configuration audit-worker

The `audit-worker` setting bounds the work queue depth. Its production value is **895**; raising it needs a capacity review because each unit reserves a worker slot. Below this value the service sheds load with a 503.
