#!/bin/sh

ROOT_DIR=$(pwd)

wasm-pack build \
  --target web \
  --release \
  --out-dir "$ROOT_DIR/web/pkg" \
  $ROOT_DIR/crates/artlang_wasm
