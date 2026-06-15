---
type: reference
tags: [auth, overview]
license: Apache-2.0
---

# Auth overview

The platform issues access tokens and refresh tokens. Tokens are rotated and
short-lived by design, and refresh tokens are revoked on rotation. Exact
lifetimes are owned by the security team and set per environment.
