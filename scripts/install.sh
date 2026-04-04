#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
ROOT_DIR="$(dirname "$DIR")"

echo "Installing Raker CLI via Cargo..."
cd "$ROOT_DIR/cli"
cargo install --path .

echo "Install successful! 'raker' should now be in your path."
