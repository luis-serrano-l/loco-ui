#!/usr/bin/env sh
# Fails when a criterion bench got slower than the threshold against a saved baseline.
# Criterion reports a change but never fails on one; this reads the change it wrote.
#   cargo bench -p loco-ui --bench render -- --save-baseline base     (on the commit to compare with)
#   cargo bench -p loco-ui --bench render -- --baseline base          (on the commit under test)
#   scripts/bench-gate.sh [percent]                      (default 25)
# CI runs the three on one runner in one job, so both sides see the same machine. Two runs
# of one commit on a laptop differed by up to 52% (FINDINGS, M50), so the CI job reports and
# does not block until the runners' own noise is known.
set -eu
export LC_ALL=C
cd "$(dirname "$0")/.."
limit="${1:-25}"
found=0
failed=0
for file in target/criterion/*/change/estimates.json; do
  [ -f "$file" ] || continue
  found=1
  name="$(basename "$(dirname "$(dirname "$file")")")"
  # The median's point estimate is the relative change: 0.1 is 10% slower.
  change="$(sed -n 's/.*"median":{[^}]*}[^}]*"point_estimate":\([-0-9.e]*\).*/\1/p' "$file")"
  percent="$(awk -v c="$change" 'BEGIN { printf "%.1f", c * 100 }')"
  if awk -v c="$change" -v l="$limit" 'BEGIN { exit !(c * 100 > l) }'; then
    echo "SLOWER  $name: +$percent% (limit $limit%)"
    failed=1
  else
    echo "ok      $name: $percent%"
  fi
done
if [ "$found" = 0 ]; then
  echo "no change/estimates.json under target/criterion: run the bench with --baseline first" >&2
  exit 2
fi
exit "$failed"
