#!/usr/bin/env sh
# The one copy of the source grep for stray <script> tags. The enhancement tag is built in
# loco-ui/src/enhance.rs; nothing else may write one. verify.sh, the `lint` job in
# .github/workflows/rust.yml and the pre-commit hook in .claude/settings.json all run this.
# Silent and exit 0 when clean; prints the offending lines and exits 1 otherwise.
# Usage: scripts/check-script.sh
set -eu
cd "$(dirname "$0")/.."

if grep -rn '<script' loco-ui/src demo/src loco-ui-test/src examples/loco-app/src \
     | grep -v '^loco-ui/src/enhance.rs:' \
     | grep -v 'matches("<script")' \
     | grep -v '^\S*:\s*//' \
     | grep -v 'code { "<script>" }'; then
  echo "found a <script> mention outside enhance.rs and its test" >&2
  exit 1
fi
