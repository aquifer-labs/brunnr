---
type: decision
tags: [auth, security]
license: Apache-2.0
---

# Token lifetimes

Access tokens expire **15 minutes** after issuance. Refresh tokens live for 30
days and are rotated on every successful refresh; the previous refresh token is
revoked immediately to limit replay.
