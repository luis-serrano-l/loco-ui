#!/usr/bin/env sh
# Local verification pass before a commit: the format check, clippy (zero warnings), the tests
# over the workspace (incl. the only-one-script test and the Blitz screenshots under
# tests/shots) and a source grep for stray <script> tags. The slow checks (clippy at every
# feature level, the installer test, rustdoc, the Firefox and axe check) run only in CI
# (.github/workflows/rust.yml), as parallel jobs.
# Usage: scripts/verify.sh
set -eu
cd "$(dirname "$0")/.."

echo "== format (CI runs the same check)"
cargo fmt --all --check

echo "== clippy (deny warnings)"
cargo clippy --workspace --all-targets -- -D warnings

echo "== tests (unit, doc, only-one-script, Blitz layout + screenshots)"
cargo test --workspace

echo "== no <script> outside enhance.rs"
scripts/check-script.sh

echo "== screenshots"
ls tests/shots/*.png | wc -l | xargs -I{} echo "{} PNGs under tests/shots/"
echo "OK (CI adds feature levels, install, rustdoc, Firefox)"
