# SPDX-License-Identifier: Apache-2.0

set shell := ["sh", "-eu", "-c"]

default:
    just --list

fmt:
    cargo fmt --all --check

clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
    cargo test --workspace
    cargo test --workspace --all-features

build:
    cargo build --workspace
    cargo build --workspace --all-features

diagrams:
    for source in docs/diagrams/*.mmd; do \
        output="${source%.mmd}.png"; \
        npx -y @mermaid-js/mermaid-cli \
            -i "$source" \
            -o "$output" \
            -c docs/diagrams/mermaid-config.json \
            -b transparent \
            -s 2; \
    done

bench:
    cargo run -p brunnr-bench -- --reps 2

bench-large:
    cargo run -p brunnr-bench -- --reps 2 --seed-corpus benchmarks/large-corpus --results benchmarks/results/large-corpus

bench-xl:
    cargo run -p brunnr-bench -- --reps 2 --seed-corpus benchmarks/xl-corpus --results benchmarks/results/xl-corpus

bench-check:
    just bench
    just bench-large
    just bench-xl
    git diff --exit-code -- benchmarks/results/sample-run benchmarks/results/large-corpus benchmarks/results/xl-corpus

ci: fmt clippy test build
