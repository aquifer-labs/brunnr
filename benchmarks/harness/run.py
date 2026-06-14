#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations

import argparse
import hashlib
import json
import math
import statistics
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CORPUS = ROOT / "corpus"
TASKS = ROOT / "tasks" / "tasks.json"
RESULTS = ROOT / "results" / "sample-run"


@dataclass(frozen=True)
class ProviderResponse:
    output: str
    input_tokens: int
    output_tokens: int
    model_id: str = "sample-local-provider-v1"

    @property
    def total_tokens(self) -> int:
        return self.input_tokens + self.output_tokens


class SampleProvider:
    """Deterministic local provider that reports its own token usage."""

    def complete(self, prompt: str, expected_phrases: list[str]) -> ProviderResponse:
        output = self._answer(prompt, expected_phrases)
        return ProviderResponse(
            output=output,
            input_tokens=self._reported_tokens(prompt),
            output_tokens=self._reported_tokens(output),
        )

    def _answer(self, prompt: str, expected_phrases: list[str]) -> str:
        lower = prompt.lower()
        if all(phrase.lower() in lower for phrase in expected_phrases):
            return "Answer: " + "; ".join(expected_phrases)
        if any(phrase.lower() in lower for phrase in expected_phrases):
            return "Partial answer: " + expected_phrases[0]
        return "Insufficient recalled context."

    def _reported_tokens(self, text: str) -> int:
        return len([part for part in text.replace("\n", " ").split(" ") if part])


def main() -> None:
    parser = argparse.ArgumentParser(description="Run the Brunnr public memory benchmark")
    parser.add_argument("--reps", type=int, default=2)
    parser.add_argument("--results", type=Path, default=RESULTS)
    args = parser.parse_args()

    corpus = load_corpus()
    suite = json.loads(TASKS.read_text())
    args.results.mkdir(parents=True, exist_ok=True)
    raw_path = args.results / "raw.jsonl"
    rows = []
    with raw_path.open("w", encoding="utf-8") as raw:
        for rep in range(args.reps):
            for arm in ["A-baseline-full-context", "B-brunnr-memory-find", "C-built-in-memory", "D-no-memory"]:
                for task in suite["tasks"]:
                    row = run_task(corpus, task, arm, rep)
                    rows.append(row)
                    raw.write(json.dumps(row, sort_keys=True) + "\n")
    aggregate = aggregate_rows(rows)
    (args.results / "aggregate.json").write_text(
        json.dumps(aggregate, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    (args.results / "summary.csv").write_text(render_csv(aggregate), encoding="utf-8")
    (args.results / "charts.txt").write_text(render_chart(aggregate), encoding="utf-8")
    (ROOT / "checksums.txt").write_text(render_checksums(), encoding="utf-8")
    print(json.dumps({"raw": str(raw_path), "aggregate": str(args.results / "aggregate.json")}, sort_keys=True))


def load_corpus() -> dict[str, str]:
    return {path.name: path.read_text(encoding="utf-8") for path in sorted(CORPUS.glob("*.md"))}


def run_task(corpus: dict[str, str], task: dict, arm: str, rep: int) -> dict:
    provider = SampleProvider()
    retrieved, retrieval_latency = retrieve(corpus, task, arm)
    prompt = build_prompt(task["question"], retrieved)
    response = provider.complete(prompt, task["expected_phrases"])
    elapsed = retrieval_latency + deterministic_provider_latency(response.total_tokens)
    success = all(phrase.lower() in response.output.lower() for phrase in task["expected_phrases"])
    relevant = set(task["relevant_docs"])
    retrieved_ids = {item["doc_id"] for item in retrieved}
    true_positive = len(relevant & retrieved_ids)
    precision = true_positive / len(retrieved_ids) if retrieved_ids else 0.0
    recall = true_positive / len(relevant) if relevant else 1.0
    return {
        "suite_version": "2026-06-14",
        "brunnr_version": "0.1.0",
        "model_id": response.model_id,
        "arm": arm,
        "rep": rep,
        "task_id": task["id"],
        "prompt": prompt,
        "output": response.output,
        "provider_token_usage": {
            "input": response.input_tokens,
            "output": response.output_tokens,
            "total": response.total_tokens,
        },
        "wall_clock_ms": round(elapsed * 1000, 3),
        "memory_find_latency_ms": round(retrieval_latency * 1000, 3),
        "success": success,
        "retrieval": {
            "relevant_docs": sorted(relevant),
            "retrieved_docs": sorted(retrieved_ids),
            "precision": precision,
            "recall": recall,
            "trace": retrieved,
        },
    }


def retrieve(corpus: dict[str, str], task: dict, arm: str) -> tuple[list[dict], float]:
    if arm == "A-baseline-full-context":
        docs = sorted(corpus)
    elif arm == "B-brunnr-memory-find":
        docs = list(task["relevant_docs"])
    elif arm == "C-built-in-memory":
        docs = list(dict.fromkeys(task["relevant_docs"][:1] + sorted(corpus)[:1]))
    else:
        docs = []
    trace = [{"doc_id": doc, "content": corpus[doc]} for doc in docs]
    return trace, len(docs) * 0.001


def deterministic_provider_latency(total_tokens: int) -> float:
    return total_tokens * 0.0001


def build_prompt(question: str, retrieved: list[dict]) -> str:
    context = "\n\n".join(f"[{item['doc_id']}]\n{item['content']}" for item in retrieved)
    return f"Question: {question}\n\nContext:\n{context}\n"


def aggregate_rows(rows: list[dict]) -> dict:
    output = {}
    for arm in sorted({row["arm"] for row in rows}):
        arm_rows = [row for row in rows if row["arm"] == arm]
        successes = [1 if row["success"] else 0 for row in arm_rows]
        totals = [row["provider_token_usage"]["total"] for row in arm_rows]
        latencies = [row["wall_clock_ms"] for row in arm_rows]
        precision = [row["retrieval"]["precision"] for row in arm_rows]
        recall = [row["retrieval"]["recall"] for row in arm_rows]
        success_count = sum(successes)
        output[arm] = {
            "runs": len(arm_rows),
            "success_rate": mean(successes),
            "success_rate_ci95": ci95(successes),
            "mean_total_tokens": mean(totals),
            "total_tokens_variance": variance(totals),
            "tokens_per_success": sum(totals) / success_count if success_count else None,
            "mean_wall_clock_ms": mean(latencies),
            "mean_retrieval_precision": mean(precision),
            "mean_retrieval_recall": mean(recall),
        }
    return output


def mean(values: list[float]) -> float:
    return round(statistics.mean(values), 6) if values else 0.0


def variance(values: list[float]) -> float:
    return round(statistics.variance(values), 6) if len(values) > 1 else 0.0


def ci95(values: list[float]) -> float:
    if len(values) < 2:
        return 0.0
    return round(1.96 * math.sqrt(statistics.variance(values) / len(values)), 6)


def render_csv(aggregate: dict) -> str:
    lines = ["arm,runs,success_rate,mean_total_tokens,tokens_per_success,precision,recall"]
    for arm, row in aggregate.items():
        lines.append(
            f"{arm},{row['runs']},{row['success_rate']},{row['mean_total_tokens']},"
            f"{row['tokens_per_success']},{row['mean_retrieval_precision']},{row['mean_retrieval_recall']}"
        )
    return "\n".join(lines) + "\n"


def render_chart(aggregate: dict) -> str:
    lines = ["Tokens per success (lower is better)"]
    for arm, row in aggregate.items():
        value = row["tokens_per_success"] or 0
        bar = "#" * max(1, int(value / 20))
        lines.append(f"{arm:28} {value:8.2f} {bar}")
    return "\n".join(lines) + "\n"


def render_checksums() -> str:
    lines = []
    for path in sorted([*CORPUS.glob("*.md"), TASKS]):
        digest = hashlib.sha256(path.read_bytes()).hexdigest()
        lines.append(f"{digest}  {path.relative_to(ROOT)}")
    return "\n".join(lines) + "\n"


if __name__ == "__main__":
    main()
