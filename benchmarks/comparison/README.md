<!-- SPDX-License-Identifier: Apache-2.0 -->

# Competitor-comparable QA benchmark (LoCoMo / LongMemEval, vs mem0)

This harness produces the **Artesian** side of a head-to-head on the two public agent-memory
QA datasets the literature reports — **LoCoMo** and **LongMemEval** — in the same shape mem0
publishes (answer accuracy via LLM-as-judge, and **tokens/query**). It is intentionally *not* a
re-run of mem0: mem0 is a Python system that needs cloud LLM calls and its own infra, so its
numbers are cited from the published paper and compared against, rather than reproduced here.

## What it measures

For each question the runner stores the conversation as recall candidates, runs one **ACC
cycle** to build a bounded committed context, asks the LLM to answer **only from that context**,
then grades the answer against gold with the same LLM (LLM-as-judge — the standard protocol):

- **accuracy** — graded-correct fraction (comparable to LoCoMo / LongMemEval "J" scores).
- **mean tokens/query** — committed-context tokens fed to the answerer; the token-efficiency
  number directly comparable to mem0's reported per-query token budget.
- **footprint_ratio** — committed tokens ÷ raw recall dump.

Retrieval defaults to lexical (term-overlap) recall over the case facts: deterministic and
dependency-free. Swap in a vector backend for a production-grade retrieval run.

## Honesty notes

- mem0 numbers must be quoted from its **paper** (Chhikara et al., *"Mem0: Building
  Production-Ready AI Agents with Scalable Long-Term Memory"*, arXiv:2504.19413, 2025) and read
  off its tables — do **not** trust second-hand figures (including any earlier draft numbers in
  `docs/positioning.md`, which should be re-verified against the source before publication).
- Cross-system comparison is only fair under a matched protocol: same dataset split, same judge
  model, same retrieval budget. State the judge model and budget with any published result.
- The Artesian numbers below were produced by this harness; the mem0 column is left to be filled
  from the cited paper under the same judge/budget you run Artesian with.

## Datasets (download separately)

Not vendored here (size + licensing). Fetch into `benchmarks/comparison/data/`:

- **LoCoMo** — <https://github.com/snap-research/locomo> (`locomo10.json`).
- **LongMemEval** — <https://github.com/xiaowu0162/LongMemEval> (`longmemeval_s.json` /
  `longmemeval_oracle.json`; also on Hugging Face).

The loaders are tolerant of the public schemas (numbered `session_N` turns for LoCoMo;
`haystack_sessions` for LongMemEval) and report how many malformed entries were skipped.

## Running

The answering/grading LLM is reached through a command. The default wraps `codex exec`
(`benchmarks/comparison/codex-complete`, model `gpt-5.5`, reasoning `xhigh`; override with
`CODEX_MODEL` / `CODEX_REASONING`). Any OpenAI-compatible endpoint works too — point
`--llm-command` at your own wrapper.

```shell
cargo build -p gauge --features llm --bin gauge-eval

# Start small; scale up with --limit.
./target/debug/gauge-eval locomo      benchmarks/comparison/data/locomo10.json     --limit 50
./target/debug/gauge-eval longmemeval benchmarks/comparison/data/longmemeval_s.json --limit 50 --json
```

## Results

| dataset | system | judge | accuracy | tokens/query | source |
|---|---|---|---|---|---|
| LoCoMo | Artesian | gpt-5.5 (xhigh) | _run it_ | _run it_ | this harness |
| LoCoMo | mem0 | (paper) | _from paper_ | _from paper_ | arXiv:2504.19413 |
| LongMemEval | Artesian | gpt-5.5 (xhigh) | _run it_ | _run it_ | this harness |
| LongMemEval | mem0 | (paper) | _from paper_ | _from paper_ | (mem0 materials) |

### Pipeline smoke (not a benchmark result)

A 2-question hand-written LoCoMo-shaped fixture (`samples/locomo-smoke.json`), graded by
`codex` gpt-5.5 at `low` reasoning, validates the full load → ACC → answer → grade path:

```
dataset:             locomo
cases:               2
graded:              2
accuracy:            1.000
mean tokens/query:   59.0
footprint_ratio:     0.797
```

This only proves the harness runs end to end; real numbers come from the full datasets above.
