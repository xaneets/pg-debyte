#!/usr/bin/env bash
set -euo pipefail


crates=(
  pg_debyte_core
  pg_debyte_macros
  pg_debyte_pgrx
)

patch_args=(
  --config
  patch.crates-io.pg_debyte_core.path=\"pg_debyte_core\"
)

for crate in "${crates[@]}"; do
  if [[ "${crate}" == "pg_debyte_core" ]]; then
    echo "==> cargo package -p ${crate}"
    cargo package -p "${crate}" --allow-dirty
  else
    echo "==> cargo package -p ${crate} --no-verify"
    cargo package -p "${crate}" --no-verify "${patch_args[@]}" --allow-dirty
  fi
  echo
done
