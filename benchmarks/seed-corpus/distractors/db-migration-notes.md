---
type: reference
tags: [database, migrations]
license: Apache-2.0
---

# Database migration notes

Migrations run against PostgreSQL and must be backward compatible for one
release. Redis caches are flushed after a schema change. Nothing here declares
which store is authoritative; see the architecture decision for that.
