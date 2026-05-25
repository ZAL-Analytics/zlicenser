#!/usr/bin/env bash
set -euo pipefail

if [ -z "${ZER_CRATES_IO_TOKEN:-}" ]; then
  echo "Error: ZER_CRATES_IO_TOKEN is not set." >&2
  exit 1
fi

cargo publish --token "$ZER_CRATES_IO_TOKEN"
