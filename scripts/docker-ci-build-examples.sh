#!/usr/bin/env bash
set -euo pipefail

docker run --rm -t pg-debyte-ci bash -lc "\
  cargo build -p readme_known_schema --all-targets --features pg17 && \
  cargo build -p readme_by_id --all-targets --features pg17 && \
  cargo build -p readme_envelope --all-targets --features pg17 \
"
