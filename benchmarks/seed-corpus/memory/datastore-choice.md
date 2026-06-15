---
type: decision
tags: [database, architecture]
license: Apache-2.0
---

# Primary datastore

The primary transactional store is **PostgreSQL 16** with logical replication to
a read replica. Analytics are offloaded to a separate columnar warehouse. Redis
holds only ephemeral session state and is never the system of record.
