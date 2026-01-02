#!/usr/bin/env bash
set -euo pipefail

cargo test --workspace --exclude pg_debyte_ext
cargo pgrx test -p pg_debyte_ext --features pg17
