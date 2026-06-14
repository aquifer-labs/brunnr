<!-- SPDX-License-Identifier: Apache-2.0 -->

# Brunnr Public Memory Benchmark

This directory contains an anonymized, reproducible benchmark for comparing context strategies on
the same tasks, corpus, model/provider, and verifier.

## Methodology

The harness runs four arms over the same deterministic task suite:

- **A** — baseline full-context replay without Brunnr.
- **B** — Brunnr-style `memory.find`, returning only labeled relevant context.
- **C** — built-in agent memory, modeled as a small provider-side recall set.
- **D** — no memory.

Every task has a labeled relevant set and deterministic expected phrases. The sample provider is a
local deterministic provider that reports its own token usage and deterministic sample latencies in
the same shape expected from real providers. Real-provider adapters must record provider-reported
tokens only and actual wall-clock latencies; do not estimate provider tokens in the harness.

## Reproduce

Run:

```sh
just bench
```

Outputs are written to `benchmarks/results/sample-run/`:

- `raw.jsonl` — one row per arm/task/rep, including full prompt, output, provider token counts,
  latencies, pass/fail, and retrieval trace.
- `aggregate.json` — mean, variance, CI, success rate, precision/recall, tokens per success.
- `summary.csv` — compact table for spreadsheets.
- `charts.txt` — text chart suitable for diffs.

Dataset checksums are in `benchmarks/checksums.txt`.

## How to Verify This Is Not Faked

1. Inspect `corpus/` and `tasks/tasks.json`; task labels are explicit and anonymized.
2. Delete `benchmarks/results/sample-run/` and rerun `just bench`.
3. Compare the regenerated `raw.jsonl`, `aggregate.json`, and `checksums.txt` with the committed
   versions.
4. Confirm each raw row contains the full prompt, output, provider-reported usage, retrieval trace,
   and verifier result.

## Honest Scope

Brunnr helps when tasks require durable recall of project decisions or constraints and the relevant
context is much smaller than the full history. It does not help tasks that need no prior context,
tasks where retrieval labels are poor, or tasks where the provider already has perfect cheap memory.
