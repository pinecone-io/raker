#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
ROOT_DIR="$(dirname "$DIR")"

echo "Building Raker CLI..."
cd "$ROOT_DIR/cli"
cargo build --release

echo "Build successful! Binary is located at cli/target/release/raker"
