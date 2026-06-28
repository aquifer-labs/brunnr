<!-- SPDX-License-Identifier: Apache-2.0 -->

# CLAUDE.md

Artesian is an English-only open-source Rust workspace. Keep all in-repository artifacts universal and free of local operator details.

## Project Shape

The first milestone is a working `memory` mode:

1. `aquifer::MemoryBackend` is the stable memory seam.
2. `FilesBackend` provides zero-infrastructure markdown storage.
3. `artesian-mcp` exposes `memory.find` and `memory.store`.
4. `artesian-cli` provides `init`, `memory store`, `memory find`, and role-aware `spawn`.

## Validation

Run:

```shell
cargo fmt --all --check
cargo test --workspace
cargo build --workspace
```

Do not push unless explicitly asked.

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
