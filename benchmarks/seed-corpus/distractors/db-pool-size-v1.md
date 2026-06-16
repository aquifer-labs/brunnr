---
type: decision
tags: [database, connection-pool, DbPoolSize, knowledge-update]
license: Apache-2.0
timestamp: "2023-11-20T14:00:00Z"
---

# Database connection pool size — original November 2023

The database connection pool is configured with **max=10 connections** per instance. This was
chosen conservatively at service launch to stay within the free-tier PostgreSQL instance limit.
The minimum idle connections is 2.

**Note**: This record has been superseded — see the May 2024 update.
