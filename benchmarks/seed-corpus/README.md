<!-- SPDX-License-Identifier: Apache-2.0 -->

# Seed corpus for the honest benchmark rework

A small, realistic, anonymized corpus to bootstrap the rebuilt benchmark. It is
deliberately designed so a **real retriever can miss** — without that, recall is
always 1.0 and the benchmark measures nothing.

## Structure

- `memory/*.md` — 8 relevant docs. Each carries exactly one specific, answerable
  fact (a number, a name, a default).
- `distractors/*.md` — plausible near-misses. They share the topic and much of
  the vocabulary of the relevant docs but **do not contain the answer**
  (`reliability-principles.md` and `glossary.md` are high-overlap traps that
  compete on the hard tasks; `caching-faq.md` / `auth-overview.md` /
  `db-migration-notes.md` discuss the right topic but withhold the number;
  `march-outage-briefing.md` names the hard incident workflow without stating
  the final root cause).
- `tasks.json` — 8 tasks (easy/medium/hard). `relevant_docs` is the ONLY ground
  truth and the retriever must never see it. `distractor_docs` records the
  competing docs for analysis.

## How the harness must use it

- Index `memory/` + `distractors/` as ONE undifferentiated corpus through the
  real backfill/import path; the retriever does not know which is which.
- For each task, run the REAL retrieval path (memory.context / memory.find) and
  measure precision/recall against `relevant_docs`.
- Expect recall < 1.0 on the hard tasks for weak retrieval strategies — that is
  the point. If every strategy scores recall 1.0, the corpus is too easy; add
  more high-overlap distractors rather than tuning the result.
- This is a SEED. Grow it with more docs/distractors as needed, but keep every
  doc natural-length (no filler) and every distractor semantically plausible.
