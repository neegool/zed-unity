#!/bin/bash
# Build script for the Zed Unity extension.

set -euo pipefail

TARGET="wasm32-wasip2"

echo "Building Zed Unity extension for ${TARGET}..."

rustup target add "${TARGET}"
cargo build --release --target "${TARGET}"

WASM_PATH="target/${TARGET}/release/zed_unity.wasm"

if [ ! -f "${WASM_PATH}" ]; then
  echo "Expected WASM output was not found: ${WASM_PATH}" >&2
  exit 1
fi

echo "Build complete: ${WASM_PATH}"
echo "Install for local development with:"
echo "  zed: extensions -> install dev extension -> $(pwd)"
