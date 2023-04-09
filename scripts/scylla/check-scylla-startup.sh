#!/usr/bin/env bash

set -x
set -eo pipefail

if ! [ -x "$(command -v cqlsh)" ]; then
  echo >&2 "Error: cqlsh is not installed."
  exit 1
fi

SCYLLA_HOST=${SCYLLA_HOST:=localhost:9094}

until cqlsh -e "SHOW HOST"; do
  >&2 echo "ScyllaDB is still unavailable - sleeping"
  sleep 1
done
