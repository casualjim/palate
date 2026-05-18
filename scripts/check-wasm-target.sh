#!/usr/bin/env bash
set -euo pipefail

TARGET="wasm32-unknown-unknown"

if ! rustup target list --installed | grep -qx "$TARGET"; then
  rustup target add "$TARGET"
fi

cargo build -p palate --target "$TARGET"
