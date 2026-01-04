#!/usr/bin/env bash

_pg_debyte_versions() {
  local arg="${1:-pg17}"
  case "${arg}" in
    pg15|pg17)
      echo "${arg}"
      ;;
    all)
      echo "pg15 pg17"
      ;;
    *)
      echo "unknown pg version: ${arg}" >&2
      return 2
      ;;
  esac
}

_pg_debyte_run() {
  local arg="${1:-pg17}"
  shift
  local ver
  for ver in $(_pg_debyte_versions "${arg}"); do
    "$@" "${ver}"
  done
}

_host_tests() {
  local ver="$1"
  cargo test --workspace --exclude pg_debyte_ext --exclude readme_known_schema --exclude readme_by_id --exclude readme_envelope --features "${ver}"
  cargo pgrx test -p pg_debyte_ext --features "${ver}"
}

_host_build() {
  local ver="$1"
  cargo build --workspace --all-targets --exclude pg_debyte_ext --exclude readme_known_schema --exclude readme_by_id --exclude readme_envelope --features "${ver}"
  cargo build -p pg_debyte_ext --all-targets --features "${ver}"
}

_host_examples() {
  local ver="$1"
  cargo build -p readme_known_schema --all-targets --features "${ver}"
  cargo build -p readme_by_id --all-targets --features "${ver}"
  cargo build -p readme_envelope --all-targets --features "${ver}"
}

_docker_run() {
  local cmd="$1"
  echo "[pg-debyte] docker: ${cmd}"
  if [[ -t 1 ]]; then
    docker run --rm -it --entrypoint bash pg-debyte-ci -lc "TERM=xterm-256color CARGO_TERM_PROGRESS_WHEN=auto CARGO_TERM_COLOR=always PGRX_BUILD_VERBOSE=1 ${cmd}"
  else
    docker run --rm -i --entrypoint bash pg-debyte-ci -lc "TERM=xterm-256color CARGO_TERM_PROGRESS_WHEN=auto CARGO_TERM_COLOR=always PGRX_BUILD_VERBOSE=1 ${cmd}"
  fi
}

_docker_tests_pg_extensions() {
  local ver="$1"
  _docker_run "env -u PG_VERSION -u PG_MAJOR /usr/local/cargo/bin/cargo pgrx test -p pg_debyte_ext --features ${ver}"
}

_docker_tests_workspace() {
  local ver="$1"
  _docker_run "/usr/local/cargo/bin/cargo test --workspace --exclude pg_debyte_ext --exclude readme_known_schema --exclude readme_by_id --exclude readme_envelope --features ${ver}"
}

_docker_build_workspace() {
  local ver="$1"
  _docker_run "cargo build --workspace --all-targets --exclude pg_debyte_ext --exclude readme_known_schema --exclude readme_by_id --exclude readme_envelope --features ${ver}"
  _docker_run "cargo build -p pg_debyte_ext --all-targets --features ${ver}"
}

_docker_build_examples() {
  local ver="$1"
  _docker_run "cargo build -p readme_known_schema --all-targets --features ${ver}"
  _docker_run "cargo build -p readme_by_id --all-targets --features ${ver}"
  _docker_run "cargo build -p readme_envelope --all-targets --features ${ver}"
}

_host_lints() {
  local ver="$1"
  cargo +nightly fmt -- --check
  cargo clippy --workspace --exclude pg_debyte_ext --exclude readme_known_schema --exclude readme_by_id --exclude readme_envelope --features "${ver}"
  cargo clippy -p pg_debyte_ext --features "${ver}"
}

host_tests() {
  _pg_debyte_run "${1:-pg17}" _host_tests
}

host_build() {
  _pg_debyte_run "${1:-pg17}" _host_build
}

host_examples() {
  _pg_debyte_run "${1:-pg17}" _host_examples
}

host_lints() {
  _pg_debyte_run "${1:-pg17}" _host_lints
}

docker_tests_pg_extensions() {
  _pg_debyte_run "${1:-pg17}" _docker_tests_pg_extensions
}

docker_tests_workspace() {
  _pg_debyte_run "${1:-pg17}" _docker_tests_workspace
}

docker_build_workspace() {
  _pg_debyte_run "${1:-pg17}" _docker_build_workspace
}

docker_build_examples() {
  _pg_debyte_run "${1:-pg17}" _docker_build_examples
}

docker_build_image() {
  ./scripts/docker-ci-build-image.sh
}

pgd_host_tests() { host_tests "$@"; }
pgd_host_build() { host_build "$@"; }
pgd_host_examples() { host_examples "$@"; }
pgd_host_lints() { host_lints "$@"; }
pgd_docker_tests_pg_extensions() { docker_tests_pg_extensions "$@"; }
pgd_docker_tests_workspace() { docker_tests_workspace "$@"; }
pgd_docker_build_workspace() { docker_build_workspace "$@"; }
pgd_docker_build_examples() { docker_build_examples "$@"; }
pgd_docker_build_image() { docker_build_image "$@"; }

_pg_debyte_complete_zsh() {
  _arguments '1:pg version:(pg15 pg17 all)'
}

_pg_debyte_complete_bash() {
  local opts="pg15 pg17 all"
  COMPREPLY=($(compgen -W "${opts}" -- "${COMP_WORDS[COMP_CWORD]}"))
}

if [[ -n "${ZSH_VERSION:-}" ]]; then
  if ! typeset -f compdef >/dev/null; then
    autoload -Uz compinit
    compinit -i
  fi
  compdef _pg_debyte_complete_zsh host_tests host_build host_examples host_lints docker_tests_pg_extensions docker_tests_workspace docker_build_workspace docker_build_examples
  compdef _pg_debyte_complete_zsh pgd_host_tests pgd_host_build pgd_host_examples pgd_host_lints pgd_docker_tests_pg_extensions pgd_docker_tests_workspace pgd_docker_build_workspace pgd_docker_build_examples
elif [[ -n "${BASH_VERSION:-}" ]]; then
  complete -F _pg_debyte_complete_bash host_tests host_build host_examples host_lints docker_tests_pg_extensions docker_tests_workspace docker_build_workspace docker_build_examples
  complete -F _pg_debyte_complete_bash pgd_host_tests pgd_host_build pgd_host_examples pgd_host_lints pgd_docker_tests_pg_extensions pgd_docker_tests_workspace pgd_docker_build_workspace pgd_docker_build_examples
fi
