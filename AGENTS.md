<!-- SPDX-License-Identifier: Apache-2.0 -->

# AGENTS.md

These instructions apply to the entire repository.

## Memory tool (MCP)

The memory MCP server is **`artesian-memory`**; its tools are **`memory.find`** and
**`memory.store`** (plus `memory.anchor.get/set` and `tools.find`). Do **not** call
`qdrant-find` / `qdrant-store` — those name a different/removed Python `mcp-server-qdrant` server,
not Artesian. If a session still lists an old `qdrant-memory` server, it is stale: restart the
session so it loads the current config, then use `artesian-memory` / `memory.find`. The Qdrant
backend requires an API key (supplied by Artesian's config/env); keyless clients get `401`.

## Onboarding (humans and agents)

To bring Artesian up — deploy it and connect a project per the operator's requested config —
follow [docs/onboarding.md](docs/onboarding.md). It has a human Quickstart and a deterministic,
idempotent, non-destructive **AI-agent recipe** (collect mode/backend/qdrant/project, run
`artesian init`, backfill, verify, report) with hard guardrails: never delete or overwrite existing
memory or unrelated config, keep secrets out of git, never switch an existing collection's
embedding model in place (use `migrate`), and never push or take outward-facing actions without
explicit operator approval. New here? read [docs/positioning.md](docs/positioning.md) and
[docs/README.md](docs/README.md) first.

## Language

All code, documentation, commit messages, plans, and handoff notes in this repository must be written in English.

## Boundaries

- Keep the repository universal and anonymized.
- Do not commit secrets, machine-local paths, private infrastructure names, runtime logs, or generated build output.
- Keep crate boundaries strict: orchestration primitives in `artesian-core`, memory in `aquifer`, MCP in `artesian-mcp`, CLI in `artesian-cli`.
- Add SPDX license headers to source, docs, manifests, and workflow files.
- Do not push unless a maintainer explicitly asks.

## Rust

- Use stable Rust pinned by CI.
- Run `just ci` before marking work complete. If `just` is unavailable, run the equivalent cargo commands in `docs/development.md`.
- Prefer small modules and explicit public exports.
- Newly added traits must document their contract.
- Write tests for every implemented behavior.

## Contribution Hygiene

All commits must use DCO sign-off:

```text
Signed-off-by: Your Name <you@example.com>
```

Keep changes focused. Do not mix unrelated refactors into feature work.

## Memory & Context — via Artesian (project: artesian)

Long-term memory and context for this project live in **Artesian** (one `homelab` Qdrant collection,
partitioned by a `project` field). The markdown memory files are now **legacy/reference** — Artesian is the
source of truth. This project routes to **`artesian`**.

- **Before non-trivial work** — recall: `artesian memory context "<task>" --project artesian`
  (or the `memory.context` / `memory.find` MCP tool with `project: "artesian"`). Recall returns this
  project's memory **∪ `shared`**.
- **After a durable learning** (decision, invariant, incident fix, convention) — store:
  `artesian memory store "<fact>" --project artesian` (or `memory.store` with `project`). Use
  `--project shared` for cross-cutting facts (policies, user profile, build env).
- **Discover projects** — `artesian projects` / the `memory.projects` MCP tool. Never hard-code names.
- Recall responses carry a `scope_applied` block so you can verify the project∪shared filter that ran.

Projects in the `homelab` collection: `macray` (homelab network/VPN/macray-ctl), `artesian`, `ferritex`,
`shared`. A different person = a separate collection.
