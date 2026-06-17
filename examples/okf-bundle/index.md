---
type: Index
title: Artesian Memory — Example OKF Bundle
description: A tiny, self-describing Open Knowledge Format bundle used as a docs example and test fixture.
tags: [example, okf, memory]
timestamp: 2026-06-14T00:00:00Z
okf_version: "0.1"
---

# Artesian Example OKF Bundle

This directory is a conformant [Open Knowledge Format](https://github.com/GoogleCloudPlatform/knowledge-catalog)
bundle: plain markdown files with YAML frontmatter, no vector database required. Artesian's `files`
memory backend reads and writes bundles in exactly this shape (see `docs/memory.md` §4.1).

Concepts:

- [Reciprocal Rank Fusion](concepts/rrf.md) — how hybrid retrieval is fused.
- [Embedding Model](concepts/embedding-model.md) — the pinned multilingual embedder.

Update history lives in [log.md](log.md).

> Frontmatter requires only `type`. `title`/`description`/`tags`/`timestamp` are recommended;
> Artesian adds `node_id` and `tier` as tolerated extensions. Relationships are plain markdown links.
