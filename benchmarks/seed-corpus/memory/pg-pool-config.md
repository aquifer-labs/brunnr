---
type: decision
tags: [database, connection-pool, PostgreSQLConnectionPool]
license: Apache-2.0
timestamp: "2024-03-01T09:00:00Z"
---

# PostgreSQL connection pool configuration

The `PostgreSQLConnectionPool` is configured with **min=5, max=20** connections per application
instance. The idle timeout is 300 seconds. PgBouncer is used in transaction-pooling mode in
front of the database to cap the total concurrent backend connections at 100. These settings
were chosen to avoid exhausting PostgreSQL's `max_connections = 200` under peak load from three
application replicas.
