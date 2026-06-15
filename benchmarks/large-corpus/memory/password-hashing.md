---
type: decision
tags: [security, auth]
license: Apache-2.0
---

# Password hashing

Passwords are hashed with **Argon2id** using 64 MB of memory, 3 iterations, and
a per-user salt. Hashes are rehashed on login when the cost parameters change so
older accounts upgrade transparently.
