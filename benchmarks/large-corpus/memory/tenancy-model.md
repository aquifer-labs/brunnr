---
type: decision
tags: [architecture, multitenancy]
license: Apache-2.0
---

# Tenancy model

Tenants are isolated using **a separate schema per tenant within a shared
database**. This balances isolation against connection overhead; very large
tenants are promoted to a dedicated database on request.
