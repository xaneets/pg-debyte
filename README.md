# pg-debyte

[![Build](https://github.com/Xaneets/pg-debyte/actions/workflows/build.yml/badge.svg)](https://github.com/Xaneets/pg-debyte/actions/workflows/build.yml)
[![Workspace tests](https://img.shields.io/github/actions/workflow/status/Xaneets/pg-debyte/tests.yml?branch=main&job=core-tests&label=workspace%20tests)](https://github.com/Xaneets/pg-debyte/actions/workflows/tests.yml)
[![PG extension tests](https://img.shields.io/github/actions/workflow/status/Xaneets/pg-debyte/tests.yml?branch=main&job=pg-extension-tests&label=pg%20extension%20tests)](https://github.com/Xaneets/pg-debyte/actions/workflows/tests.yml)
[![Crates.io](https://img.shields.io/crates/v/pg_debyte_core.svg)](https://crates.io/crates/pg_debyte_core)

Core building blocks for PostgreSQL extensions that decode `bytea` into JSON.
This repository provides reusable Rust crates plus a small example extension.

## Workspace

- `pg_debyte_core`: envelope parser, registry, codecs/actions, limits, errors.
- `pg_debyte_macros`: helper macros for registering typed decoders.
- `pg_debyte_pgrx`: PG17-only helper glue (GUC limits and decoding helpers).
- `pg_debyte_ext`: example PG17 extension crate with a demo registry and decoder.
- `pg_debyte_tools`: helper binaries (demo payload generator).

## MVP status

- Envelope format parsing (magic + version + type_id + schema_version + codec + actions).
- Action pipeline (decode in reverse) with bounded zstd decode.
- Bincode codec with size limits.
- Static registry for decoders/codecs/actions.
- PG17-only helper for GUC limits and decoding (to be called from extension).

## Notes

- `pg_debyte_ext` is only an example implementation; you will create your own extension crate.
- PG15 support will be added later as a separate focus.

## Example usage (PG17)

Build and install the example extension:

```bash
cargo pgrx init --pg17 /path/to/pg17
cargo pgrx install -p pg_debyte_ext --features pg17 --sudo
```

Enable in Postgres:

```sql
CREATE EXTENSION pg_debyte_ext;
```

Generate a demo payload hex:

```bash
# run the helper binary
cargo run -p pg_debyte_tools --bin demo_payload
```

Decode in SQL (raw payload, no envelope):

```sql
SELECT bytea_to_json_by_id(
  decode('<hex-encoded-payload>', 'hex'),
  '11111111-1111-1111-1111-111111111111'::uuid,
  1::smallint
);
```

Generate a demo envelope hex:

```bash
#run the helper binary
cargo run -p pg_debyte_tools --bin demo_envelope
```

Decode in SQL (auto envelope):

```sql
SELECT bytea_to_json_auto(decode('<hex-encoded-envelope>', 'hex'));
```

Generate a full SQL example for auto envelope:

```bash
cargo run -p pg_debyte_tools --bin demo_auto_sql
```

## Running pg tests in Docker

If host permissions make `cargo pgrx test` difficult, use the Docker runner:

```bash
./scripts/docker-ci-test-pg-extension.sh
```
