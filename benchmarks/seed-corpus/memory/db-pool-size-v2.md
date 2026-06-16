---
type: decision
tags: [database, connection-pool, DbPoolSize, knowledge-update]
license: Apache-2.0
timestamp: "2024-05-10T08:00:00Z"
---

# Database connection pool size — updated May 2024

**Updated decision**: The database connection pool size has been increased to **max=25
connections** per instance (previously 10). The change was made after profiling showed pool
exhaustion under sustained write load. The minimum stays at 2. This applies to the primary
PostgreSQL replica; read replica pools are unchanged at max=10.

This record supersedes the original pool-size decision from November 2023.
