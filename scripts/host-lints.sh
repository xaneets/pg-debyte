#!/usr/bin/env bash
set -euo pipefail

cargo +nightly fmt -- --check
cargo clippy --workspace --exclude pg_debyte_ext
cargo clippy -p pg_debyte_ext --features pg17
