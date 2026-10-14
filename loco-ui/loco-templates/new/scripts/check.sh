#!/usr/bin/env sh
# Before a commit: the format check, clippy with warnings as errors, the tests, and a grep
# for a <script> tag in the source (loco-ui's optional /lui/enhance.js is the only script a
# page should load). Written by `cargo lui new`.
set -eu
cd "$(dirname "$0")/.."

echo "== format"
cargo fmt --all --check

echo "== clippy"
cargo clippy --all-targets -- -D warnings

echo "== tests"
cargo test

echo "== no <script> in the source"
if grep -rn '<script' src; then
  echo "found a <script> tag: pages work without script; ask the server instead"; exit 1
fi
echo OK
