#!/usr/bin/env bash
set -euo pipefail

docker run --rm -t pg-debyte-ci bash -lc "\
  env -u PG_VERSION -u PG_MAJOR /usr/local/cargo/bin/cargo pgrx test \
    -p pg_debyte_ext --features pg17 \
"
