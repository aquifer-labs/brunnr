#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Generate the XL benchmark tier.

This emits a large, fully-anonymized corpus of *genuine, distinct* facts (no
filler): each memory doc states one unique answerable fact about one unique
entity, and tasks query a specific value that must be discriminated from many
similarly-shaped docs. The generator is deterministic (fixed seed) so the
corpus and the benchmark results are reproducible.

The seed/large tiers are hand-authored prose; this XL tier is procedurally
generated purely to chart the token-saving curve at scale. It is honest because
the harness still runs the real retriever, precision/recall are measured against
labels the retriever never sees, and the retriever can (and does) miss.
"""

from __future__ import annotations

import json
import random
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1] / "xl-corpus"
MEM = ROOT / "memory"
DIST = ROOT / "distractors"
SEED = 20260615
N_TASKS = 40

ADJ = [
    "orders", "billing", "catalog", "search", "inventory", "shipping", "payments",
    "auth", "notifications", "analytics", "reviews", "pricing", "checkout", "tax",
    "fraud", "loyalty", "recommendations", "media", "messaging", "scheduling",
    "reporting", "audit", "identity", "gateway", "ingestion", "export", "import",
    "webhooks", "feeds", "geocoding", "translation", "moderation", "sync", "archive",
    "ledger", "wallet", "refunds", "subscriptions", "metering", "routing",
]
NOUN = ["api", "service", "worker", "indexer", "scheduler", "gateway", "processor"]


def front(doc_type: str, tags: list[str]) -> str:
    return f"---\ntype: {doc_type}\ntags: [{', '.join(tags)}]\nlicense: Apache-2.0\n---\n\n"


def main() -> None:
    rng = random.Random(SEED)
    MEM.mkdir(parents=True, exist_ok=True)
    DIST.mkdir(parents=True, exist_ok=True)

    names = sorted({f"{a}-{n}" for a in ADJ for n in NOUN})
    rng.shuffle(names)
    names = names[:180]

    facts = []  # (doc_path, question, answer_token)
    for i, name in enumerate(names):
        kind = i % 4
        if kind == 0:  # service port
            port = 8000 + (i * 7 % 1000)
            drain = 10 + (i % 6) * 5
            body = (
                f"# Service {name}\n\n"
                f"The `{name}` service listens on port **{port}** and is owned by the "
                f"{name.split('-')[0]} team. Its health check is at /healthz and it drains "
                f"in-flight requests for {drain} seconds on deploy before the old version exits."
            )
            q = f"Which port does the {name} service listen on?"
            ans = str(port)
            tags = ["service", "infrastructure"]
        elif kind == 1:  # config value
            val = 50 + (i * 13 % 950)
            body = (
                f"# Configuration {name}\n\n"
                f"The `{name}` setting bounds the work queue depth. Its production value is "
                f"**{val}**; raising it needs a capacity review because each unit reserves a "
                f"worker slot. Below this value the service sheds load with a 503."
            )
            q = f"What is the production value of the {name} setting?"
            ans = str(val)
            tags = ["config", "tuning"]
        elif kind == 2:  # quota
            rpm = 100 + (i * 11 % 900)
            body = (
                f"# Tier {name}\n\n"
                f"Tenants on the `{name}` plan may send **{rpm} requests per minute** and open "
                f"up to {10 + i % 40} concurrent connections. Exceeding either cap returns 429 "
                f"with a Retry-After header scoped to the tenant."
            )
            q = f"How many requests per minute does the {name} plan allow?"
            ans = str(rpm)
            tags = ["limits", "quota"]
        else:  # retention
            days = 7 + (i * 3 % 358)
            body = (
                f"# Retention {name}\n\n"
                f"`{name}` records are kept for **{days} days** in primary storage, then moved "
                f"to cold archive for two years before deletion. Legal holds pause deletion for "
                f"affected records only."
            )
            q = f"For how many days are {name} records kept in primary storage?"
            ans = str(days)
            tags = ["retention", "compliance"]

        path = MEM / f"{name}.md"
        path.write_text(front(tags[0] if False else "reference", tags) + body + "\n", encoding="utf-8")
        facts.append((f"memory/{name}.md", q, ans))

    # plausible distractors: topic-level overviews with no specific values
    overviews = {
        "services-overview": "Every service listens on a port, declares an owner, exposes /healthz, and drains on deploy. Exact ports and drain times live in each service's own record, not here.",
        "config-conventions": "Settings are namespaced, reviewed before changes, and bound queue depth or concurrency. The production values are recorded per setting, not in this conventions page.",
        "quota-overview": "Plans cap requests per minute and concurrent connections and return 429 on breach. The specific numbers per plan are documented in each plan's record.",
        "retention-overview": "Records are kept in primary storage, then cold-archived, then deleted, subject to legal holds. The exact windows are per data type and live in their own records.",
        "platform-glossary": "Port: a network endpoint. Quota: a usage cap. Retention: how long data is kept. Drain: finishing in-flight work before shutdown. 429: rate-limit response.",
        "onboarding-index": "New engineers should read the service, config, quota, and retention records for the systems they own. This index only points at where those records live.",
        "naming-guide": "Services are named team-role, settings are kebab-case, plans use tier names. Names appear across many records; a name alone does not tell you a value.",
        "deploy-runbook": "Deploys roll out gradually, watch health checks, and respect per-service drain windows. The runbook describes the process, not any one service's port or drain.",
    }
    for slug, text in overviews.items():
        (DIST / f"{slug}.md").write_text(
            front("reference", ["overview"]) + f"# {slug.replace('-', ' ').title()}\n\n{text}\n",
            encoding="utf-8",
        )

    # sample tasks across the fact set
    chosen = rng.sample(facts, N_TASKS)
    tasks = []
    for j, (doc, q, _ans) in enumerate(chosen):
        difficulty = "easy" if j % 3 == 0 else ("medium" if j % 3 == 1 else "hard")
        tasks.append({
            "id": f"xl-{j:02d}-{Path(doc).stem}",
            "difficulty": difficulty,
            "question": q,
            "relevant_docs": [doc],
            "distractor_docs": [],
        })

    (ROOT / "tasks.json").write_text(
        json.dumps(
            {
                "suite": "seed-honest-v1",
                "notes": "Procedurally generated XL tier (deterministic seed). 180 distinct fact docs + 8 overview distractors. relevant_docs is the only ground truth; the retriever never sees it.",
                "tasks": tasks,
            },
            indent=2,
        ) + "\n",
        encoding="utf-8",
    )
    print(f"memory={len(facts)} distractors={len(overviews)} tasks={len(tasks)} -> {ROOT}")


if __name__ == "__main__":
    main()
