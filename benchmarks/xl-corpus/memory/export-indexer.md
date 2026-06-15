---
type: reference
tags: [config, tuning]
license: Apache-2.0
---

# Configuration export-indexer

The `export-indexer` setting bounds the work queue depth. Its production value is **295**; raising it needs a capacity review because each unit reserves a worker slot. Below this value the service sheds load with a 503.
