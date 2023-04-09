#!/usr/bin/env bash

set -x
set -eo pipefail

cqlsh -f DB_DDL.sql
