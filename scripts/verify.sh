#!/usr/bin/env sh
# Local verification pass before a commit: the format check, clippy (zero warnings), the tests
# over the workspace (incl. the only-one-script test, the HTML conformance test and the insta
# snapshots) and a source grep for stray <script> tags. The Blitz crate (layout assertions and
# the screenshots under tests/shots) is the slowest suite and runs only with --full, or in CI.
# The other slow checks (clippy at every feature level, the installer test, rustdoc, the
# Firefox and axe check) run only in CI (.github/workflows/rust.yml), as parallel jobs.
# Usage: scripts/verify.sh [--full]
set -eu
cd "$(dirname "$0")/.."

full=false
case "${1:-}" in
  --full) full=true ;;
  "") ;;
  *) echo "usage: scripts/verify.sh [--full]"; exit 2 ;;
esac

echo "== format (CI runs the same check)"
cargo fmt --all --check

echo "== clippy (deny warnings)"
cargo clippy --workspace --all-targets -- -D warnings

if $full; then
  echo "== tests (unit, doc, only-one-script, conformance, snapshots, Blitz layout + screenshots)"
  cargo test --workspace
else
  echo "== tests (unit, doc, only-one-script, conformance, snapshots; --full adds Blitz)"
  cargo test --workspace --exclude loco-ui-test
fi

echo "== no <script> outside enhance.rs"
scripts/check-script.sh

if $full; then
  echo "== screenshots"
  ls tests/shots/*.png | wc -l | xargs -I{} echo "{} PNGs under tests/shots/"
fi
echo "OK (CI adds feature levels, install, rustdoc, Firefox)"
