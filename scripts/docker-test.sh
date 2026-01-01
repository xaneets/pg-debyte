#!/usr/bin/env bash
set -euo pipefail

docker build -f docker/pgtest.Dockerfile -t pg-debyte-test .
docker run --rm -t pg-debyte-test
