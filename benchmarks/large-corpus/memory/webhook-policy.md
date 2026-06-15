---
type: decision
tags: [integrations, reliability]
license: Apache-2.0
---

# Webhook delivery

Outbound webhooks retry with exponential backoff for up to **24 hours**; after
that the event moves to a dead-letter queue for manual replay. Each delivery
carries a signature header the receiver must verify.
