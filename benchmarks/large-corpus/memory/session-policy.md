---
type: reference
tags: [auth, sessions]
license: Apache-2.0
---

# Session policy

Web sessions are stored in Redis and expire after **8 hours of inactivity**, with
an absolute lifetime of 7 days. Logging out deletes the session server-side; it is
not just a cookie clear.
