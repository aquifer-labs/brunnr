<!-- SPDX-License-Identifier: Apache-2.0 -->

# Architecture Overview

Brunnr is a multi-agent context orchestration system: pluggable **memory**, optional
**orchestration** (master/worker/judge), optional **task tracking**, and optional **sandboxing** —
all **non-intrusive**. It integrates with agents over **MCP**, so any MCP-capable tool (Claude
Code, Codex, Zed, opencode, …) gains Brunnr's capabilities without changing how it is driven. You
adopt only what you want via [modes](modes.md).

This page is the map; each concern has its own doc.

## System map

```mermaid
flowchart TD
  subgraph agents["Your agents — unchanged workflow"]
    direction LR
    CC[Claude Code]:::agent
    CX[Codex]:::agent
    ZD[Zed]:::agent
    OC[opencode]:::agent
  end
  agents -->|MCP| MCP

  subgraph surfaces["Integration surfaces"]
    direction LR
    MCP[brunnr-mcp]:::surface
    CLI["brunnr-cli / brunnrd"]:::surface
    TUI["bifrost — TUI"]:::surface
  end

  subgraph core["Core"]
    CORE[brunnr-core]:::core
  end
  MCP --> CORE
  CLI --> CORE
  TUI --> CORE

  subgraph caps["Capabilities — opt-in"]
    direction LR
    MEM[mimisbrunnr — memory]:::cap
    TASKS[thingr — task tracking]:::cap
    SBX[hvergelmir — sandbox]:::cap
  end
  CORE --> MEM
  CORE --> TASKS
  CORE --> SBX

  subgraph stores["Memory stores"]
    direction LR
    VS[("VectorStore: qdrant / sqlite-vec")]:::store
    FB[("Files: OKF bundle")]:::store
  end
  MEM --> VS
  MEM --> FB

  classDef agent fill:#eef2ff,stroke:#6366f1,color:#1e1b4b;
  classDef surface fill:#ecfeff,stroke:#06b6d4,color:#083344;
  classDef core fill:#fef9c3,stroke:#ca8a04,color:#422006;
  classDef cap fill:#dcfce7,stroke:#16a34a,color:#052e16;
  classDef store fill:#f1f5f9,stroke:#64748b,color:#0f172a;
```

## Crates (strict boundaries, trait seams)

| Crate | Responsibility |
|---|---|
| `brunnr-core` | roles (Óðinn/Þórr/Týr + master/worker/judge), task-queue types (Erindi/Þing/Galdr), config, modes, the `Agent` adapter trait, the event envelope |
| `mimisbrunnr` | memory: `MemoryBackend`, the `VectorStore` seam, `VectorMemoryBackend<V>`, RRF, tiers, OKF files |
| `thingr` | task tracking: `TaskStore` (Files/Vector/External), the task DAG |
| `brunnr-mcp` | exposes tools over MCP (`memory.*`, `tools.find`, task tools); the agent integration point |
| `brunnr-cli` / `brunnrd` | user entrypoint + optional daemon (init, memory ops, spawn, pooling) |
| `bifrost` | TUI control surface · `hvergelmir` optional Docker sandbox · `huginn` optional macOS tray |

Engine/agent/tracker specifics live behind traits (`VectorStore`, `Agent`, `TaskStore`,
`MemoryBackend`) so adding a backend, agent, or tracker is a small adapter, never a core change.

## Cross-cutting concerns (read the focused docs)

- **Memory** — short/long-term, retrieval math (cosine, RRF k=60), tiers L0–L3, OKF on-disk
  format, optional rerank/HyDE/consolidation. → [memory.md](memory.md)
- **Concurrency & multi-tenancy** — many agents/users in parallel; append-mostly idempotent
  writes, project-per-collection + payload tenancy, backend-by-concurrency. → [concurrency.md](concurrency.md)
- **Orchestration & coordination** — roles, topologies, router (agent + semantic tool selection),
  event envelope, coordination mechanisms, worker workspace isolation, verifiers, observability.
  → [orchestration.md](orchestration.md)
- **Task tracking** — DAG with dependencies, hierarchical decomposition, md/vector/external
  (Jira/Linear). → [task-tracking.md](task-tracking.md)
- **Self-repair** — surviving auto-compaction via a deterministic anchor + recall. → [self-repair.md](self-repair.md)
- **Modes** — `memory` | `orchestrate` | `full` | `advanced` (BYO). → [modes.md](modes.md)
- **Context tree** — layered, priority-ordered AGENTS/CLAUDE md. → [yggdrasil.md](yggdrasil.md)
- **Build & contribute** — [development.md](development.md)

## Design invariants

1. **Non-intrusive default.** `memory` mode adds only `memory.find`/`memory.store` over MCP; the
   agent workflow is unchanged. Anything costing an LLM call or latency is opt-in, off by default.
2. **MCP-first.** One universal integration surface for every agent tool.
3. **Trait seams, thin adapters.** Backends/agents/trackers are pluggable; the core is engine-
   agnostic.
4. **Append-mostly, idempotent memory.** No read-modify-write on points ⇒ safe concurrency.
5. **Single mutation authority + blackboard.** Agents coordinate indirectly through shared
   memory + the task DAG, not token-heavy chatter; one authority serializes state changes.
6. **Verifiers define trust.** The judge commits only when configured verifiers pass.
