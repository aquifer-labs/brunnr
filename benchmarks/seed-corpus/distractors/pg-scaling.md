---
type: reference
tags: [database, postgresql, scaling]
license: Apache-2.0
---

# PostgreSQL scaling notes

PostgreSQL scales writes vertically and reads horizontally via streaming replication. When
connection counts become a bottleneck, use a connection pooler such as PgBouncer or pgpool-II.
Setting `max_connections` too high wastes shared memory; tune it with the number of active
application instances in mind. Autovacuum and checkpoint tuning matter more for write-heavy
workloads than raw connection counts.
