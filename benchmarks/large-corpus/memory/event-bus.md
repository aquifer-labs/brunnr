---
type: reference
tags: [messaging, architecture]
license: Apache-2.0
---

# Event bus

Domain events flow through **Kafka** with a **7-day retention** and a default of
**12 partitions** per topic. Consumers commit offsets after processing so a crash
replays from the last committed position.
