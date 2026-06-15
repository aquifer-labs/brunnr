---
type: reference
tags: [database, backups]
license: Apache-2.0
---

# Backup policy

The database takes a **nightly full backup plus 5-minute WAL archiving**, giving
a recovery point objective of five minutes. Backups are retained for 35 days and
a restore drill runs monthly.
