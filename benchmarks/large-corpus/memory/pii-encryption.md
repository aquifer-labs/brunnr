---
type: decision
tags: [security, privacy]
license: Apache-2.0
---

# PII encryption

Personally identifiable fields are encrypted at rest with **AES-256-GCM** using
envelope encryption; data keys are wrapped by a KMS master key that **rotates
every 90 days**. Decryption is audit-logged.
