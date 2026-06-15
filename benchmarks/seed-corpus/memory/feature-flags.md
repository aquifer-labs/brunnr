---
type: decision
tags: [config, safety]
license: Apache-2.0
---

# Feature flag evaluation

Flags are evaluated server-side through the config service. If the config
service is unreachable a flag **defaults to OFF (fail-closed)** so an outage can
never silently enable an unfinished code path.
