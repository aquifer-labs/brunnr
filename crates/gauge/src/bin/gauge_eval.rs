// SPDX-License-Identifier: Apache-2.0

//! `gauge-eval` — run the Artesian side of a competitor-comparable QA benchmark
//! (LoCoMo / LongMemEval) and print accuracy + tokens/query.
//!
//! Requires building with `--features llm`. The answering/grading LLM is reached through a
//! command (default `benchmarks/comparison/codex-complete`, which wraps `codex exec`).
//!
//! Usage:
//!   gauge-eval <locomo|longmemeval> <dataset.json> [--limit N] [--llm-command CMD] [--json]

#[cfg(not(feature = "llm"))]
fn main() {
    eprintln!("gauge-eval requires building gauge with --features llm");
    std::process::exit(2);
}

#[cfg(feature = "llm")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use gauge::{load_locomo, load_longmemeval, run_qa_eval};
    use headgate::{CommandLlmClient, HeadgateConfig};

    let mut args = std::env::args().skip(1);
    let dataset = args.next().unwrap_or_default();
    let path = args.next().unwrap_or_default();
    if dataset.is_empty() || path.is_empty() {
        eprintln!(
            "usage: gauge-eval <locomo|longmemeval> <dataset.json> [--limit N] \
[--llm-command CMD] [--json]"
        );
        std::process::exit(2);
    }

    let mut limit: Option<usize> = None;
    let mut llm_command = "benchmarks/comparison/codex-complete".to_string();
    let mut json = false;
    let rest: Vec<String> = args.collect();
    let mut iter = rest.iter();
    while let Some(flag) = iter.next() {
        match flag.as_str() {
            "--limit" => limit = iter.next().and_then(|v| v.parse().ok()),
            "--llm-command" => {
                if let Some(value) = iter.next() {
                    llm_command = value.clone();
                }
            }
            "--json" => json = true,
            other => {
                eprintln!("unknown flag: {other}");
                std::process::exit(2);
            }
        }
    }

    let raw = std::fs::read_to_string(&path)?;
    let report = match dataset.as_str() {
        "locomo" => load_locomo(&raw)?,
        "longmemeval" => load_longmemeval(&raw)?,
        other => {
            eprintln!("unknown dataset: {other} (expected locomo or longmemeval)");
            std::process::exit(2);
        }
    };

    let mut cases = report.cases;
    if let Some(limit) = limit {
        cases.truncate(limit);
    }
    eprintln!(
        "loaded {} cases ({} skipped) from {path}; running...",
        cases.len(),
        report.skipped
    );

    // The wrapper reads the prompt from stdin, so no {prompt} placeholder.
    let client = CommandLlmClient::new(llm_command, Vec::new());
    let (summary, _outcomes) =
        run_qa_eval(dataset, &cases, &client, HeadgateConfig::default()).await;

    if json {
        println!("{}", serde_json::to_string_pretty(&summary)?);
    } else {
        println!("dataset:               {}", summary.dataset);
        println!("cases:                 {}", summary.cases);
        println!("graded:                {}", summary.graded);
        println!("accuracy:              {:.3}", summary.accuracy);
        println!(
            "mean tokens/query:     {:.1}",
            summary.mean_committed_tokens
        );
        println!(
            "mean raw recall tok:   {:.1}",
            summary.mean_raw_recall_tokens
        );
        println!("footprint_ratio:       {:.3}", summary.footprint_ratio);
    }
    Ok(())
}
