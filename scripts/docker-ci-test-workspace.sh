#!/usr/bin/env bash
set -euo pipefail

docker run --rm -t pg-debyte-ci bash -lc "\
  cargo test --workspace --exclude pg_debyte_ext \
"
