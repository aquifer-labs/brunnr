<!-- SPDX-License-Identifier: Apache-2.0 -->

# Architecture Notes

Brunnr treats the OKF markdown bundle as the source of truth for durable memory. Vector stores are
derived indexes that can be rebuilt from OKF when an embedding model, vector dimension, or payload
schema changes.

The default memory mode is non-intrusive: it performs embedding retrieval and RRF without extra LLM
calls. Orchestration is active only in orchestrate or full mode.
