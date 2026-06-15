---
type: decision
tags: [reliability, networking]
license: Apache-2.0
---

# Outbound retry policy

Outbound service calls retry with exponential backoff: base delay 200 ms, full
jitter, and a hard ceiling of **5 attempts**. POST retries require an idempotency
key so a duplicated request never double-charges.
