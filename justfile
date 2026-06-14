# SPDX-License-Identifier: Apache-2.0

set shell := ["sh", "-eu", "-c"]

default:
    just --list

fmt:
    cargo fmt --all --check

clippy:
    cargo clippy --workspace --all-targets -- -D warnings
    cargo clippy -p mimisbrunnr --features qdrant --all-targets -- -D warnings

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

ci: fmt clippy test build
