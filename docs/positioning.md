<!-- SPDX-License-Identifier: Apache-2.0 -->

# Why Brunnr — and how it compares

Brunnr's memory layer deliberately borrows proven ideas from
[TencentDB Agent Memory](https://github.com/TencentCloud/TencentDB-Agent-Memory) — L0–L3
progressive tiering, hybrid BM25 + vector retrieval fused with RRF, markdown white-box records,
and `node_id` drill-down for context offloading. That project is excellent and we credit it. So
why does Brunnr exist?

## What Brunnr is — and is not

Brunnr is, first, **durable, semantic memory your agents own**: the decisions, facts, and context
they accumulate across sessions, kept in portable Open Knowledge Format markdown you can read, commit,
and carry anywhere. That is the flagship — use *only* memory and nothing else about how you run your
agent changes. Optionally, the same store is also an orchestration and agent-team layer (composable
components you opt into, never required).

It is **not**:

- a **cloud memory service you rent** — Brunnr runs locally; writes are free (no per-write LLM call)
  and your data never leaves your machine;
- a **code-structure index** like [Codebase-Memory](https://github.com/DeusData/codebase-memory-mcp)
  — that is a parsed graph of *what your code is*; Brunnr stores *what your agent learns*, and the two
  compose;
- **just a conversation log** — it is consolidated, retrievable, tiered knowledge.

## How it compares

Against TencentDB Agent Memory specifically, the key differences:

| | TencentDB Agent Memory | Brunnr |
|---|---|---|
| Scope | Memory only (capture → extract → recall) | Memory **+ task tracking + master/worker/judge orchestration + sandbox** |
| Integration | OpenClaw plugin / Hermes provider (framework-coupled) | **MCP-first, agent-agnostic** (Claude Code, Codex, Zed, opencode, …) + pluggable `Agent` adapters |
| Runtime | Node ≥22.16 + TypeScript | **Rust** — single static binary, no runtime |
| Vector store | SQLite + sqlite-vec (local-first; remote on roadmap) | **Pluggable `VectorStore`**: Files(OKF) / sqlite-vec / Qdrant (+ TencentDB-style adapter possible) |
| On-disk format | bespoke markdown/JSONL layout | **Open Knowledge Format (OKF)** — vendor-neutral, portable, interop with the OKF ecosystem |
| Concurrency | single-user, local-first | **multi-project + multi-user + parallel** (collection-per-project + payload tenancy) — see [concurrency.md](concurrency.md) |
| Cross-tool memory | within its host framework | **neutral shared store both Claude Code and Codex read** (their native memories are siloed) |
| Upgrades | — | **upgrade-survivable**: OKF = source of truth, Qdrant = rebuildable index, `migrate` + version metadata ([upgrades.md](upgrades.md)) |
| Orchestration safety | n/a (not an orchestrator) | **verifiers-as-trust-boundary, judge-sole-committer, task DAG, worker workspace isolation** |

What Brunnr reuses from TencentDB (with credit): the L0–L3 tiering, hybrid+RRF retrieval, the
markdown white-box principle, `node_id` drill-down, and the benchmark-rigor mindset. A
TencentDB-style symbolic Mermaid "task canvas" for short-term memory is a natural future addition
on top of Brunnr's WorkingMemory + session anchor.

**One-line positioning:** if you want *just memory*, several good options exist (including
TencentDB). Brunnr is for teams/operators who want a **non-intrusive, Rust, MCP-first system that
spans the whole agent loop** — memory you can reuse on its own *and* optional orchestration that
shares the same store — across multiple agents, projects, and users, surviving its own upgrades.
Use as little (just `memory` mode) or as much (`full`) as you want.

## Direct competitors (general agent memory)

These solve the same core problem — durable memory for agents — and are the honest comparison set:

- **[mem0](https://github.com/mem0ai/mem0)** (Apache-2.0) — the most prominent. An LLM **extracts
  facts on every write**, stored with entity linking; hybrid semantic + BM25 + temporal retrieval;
  strong published numbers (LoCoMo 91.6, LongMemEval 94.8), broad vector-DB and LLM support, and a
  hosted cloud. **Brunnr's wedge:** writes are **free and local** (no per-write LLM call), memory is
  **white-box OKF markdown you own** (not an opaque or rented store), it runs **zero-infra**, and it
  is **MCP-first / integrate-anything** with optional orchestration on the same store. We aim to match
  mem0's retrieval quality (opt-in LLM consolidation, entity/temporal signals are on the roadmap)
  while keeping the zero-cost, own-your-data default.
- **[Zep / Graphiti](https://github.com/getzep/graphiti)** — temporal knowledge-graph memory with
  strong LongMemEval / DMR numbers; graph-centric and service-oriented. Brunnr stays files-first and
  vendor-neutral (graph relations are a roadmap addition, not a required backend).
- **[Letta / MemGPT](https://github.com/letta-ai/letta)** — an agent "memory OS" with a server and
  its own agent runtime. Brunnr is lighter and non-intrusive: memory you add to *your* agent over
  MCP, not a runtime you adopt.

Honest take: these are well-funded and benchmark-strong. Brunnr does not try to out-platform them — it
wins on **ownership, simplicity, zero-cost local writes, and freedom to integrate**, and on being a
composable memory **+** orchestration stack rather than memory alone. A standardized comparison on
LongMemEval / LoCoMo is planned (see [benchmarks](../benchmarks/README.md)).

## Adjacent / related projects

- **[open-engram](https://github.com/Open-Nucleus/open-engram)** — a brain-inspired memory
  *library* (TypeScript): sensory → working → episodic → semantic stores, a multi-stage
  consolidation pipeline, and RFR-scored demotion. Memory-only, framework-SDK (Mastra/LangChain.js),
  no MCP, no orchestration. Strongly validates the **consolidation** direction Brunnr is taking;
  Brunnr differs by being Rust + MCP-first + pluggable backends + orchestration, not a single-runtime
  library.
- **[openrelay](https://github.com/romgX/openrelay)** — a model **quota aggregator / router** with
  a web dashboard that bridges credentials and routes requests from any tool to any provider. This
  is a **different layer** (model access), not memory or orchestration; it is *complementary* —
  Brunnr could sit above an openrelay-style router. Its clean "connect any agent" UX is a good
  presentation model to learn from.
- **[h5i](https://github.com/h5i-dev/h5i)** — an **AI-aware Git sidecar** (Rust): per-commit agent
  context/reasoning in dedicated `refs/h5i/*`, **Agent Radio** typed inter-agent messaging with
  union-merge, output **token-reduction** (collapse tool output, keep recoverable raw), and
  **progressive sandbox isolation** (workspace → process → supervised → container). A **different
  layer** from Brunnr — provenance, comms, and confinement over Git, not semantic retrieval — and
  **complementary**: they could compose. We borrow ideas, with credit: its typed agent-handoff
  protocol informs Brunnr's orchestration handoffs, its isolation tiers inform `hvergelmir`, and its
  "collapse but never discard, recover by id" mirrors Brunnr's L0–L3 + `node_id` drill-down.
- **[Codebase-Memory](https://github.com/DeusData/codebase-memory-mcp)** (MIT) — a Tree-Sitter
  **structural code graph** (who-calls-what, routes, impact) over MCP, single C binary, zero-infra. A
  **different kind of memory** — *what your code is*, parsed from source — versus Brunnr's *what your
  agent learns*. Explicitly **complementary**: an agent can use Codebase-Memory for repo structure and
  Brunnr for durable knowledge. Its local-first, deterministic, single-binary, commit-the-artifact
  philosophy mirrors and validates Brunnr's own (OKF memory is likewise local and git-versionable).

## Converging evidence shaping the roadmap

Karpathy's [LLM-wiki](https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f),
TencentDB's L0–L3, and open-engram's episodic→semantic consolidation all point the same way:
**curated, consolidated memory (atomic facts + entity/concept/scenario pages + an `index.md`
read-first catalog) beats flat record dumps** — for both retrieval precision and token cost. Below
~50–100k tokens a curated wiki/index-first context can even beat vector RAG; vector retrieval wins
at larger scale. Brunnr's plan is to do **both**: index-first + targeted `memory.find`, with
consolidation populating the tiers — see the memory roadmap in [memory.md](memory.md).
