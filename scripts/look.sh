#!/usr/bin/env bash
# Side-by-side look check (M20, M34): Firefox screenshots of every component page, light and
# dark, 1280, 768 and 420 wide, plus the reference library's page for the same component, into
# target/look/ (or target/look/<tag>/). Needs a built demo (`cargo build -p demo`) and Firefox.
# Nothing here is a test: compare by eye, then delete the PNGs.
# Usage: scripts/look.sh [--only <name>] [--tag <before|after|…>] [--no-ref]
set -eu
cd "$(dirname "$0")/.."
only="" tag="" ref=1
while [ $# -gt 0 ]; do
  case "$1" in
    --only) only="$2"; shift 2 ;;
    --tag) tag="$2"; shift 2 ;;
    --no-ref | --no-shadcn) ref=0; shift ;;
    *) echo "usage: scripts/look.sh [--only <name>] [--tag <tag>] [--no-ref]" >&2; exit 2 ;;
  esac
done
out="$PWD/target/look${tag:+/$tag}"
mkdir -p "$out/light" "$out/dark"
echo 'user_pref("layout.css.prefers-color-scheme.content-override", 1);' > "$out/light/user.js"
printf 'user_pref("layout.css.prefers-color-scheme.content-override", 0);\nuser_pref("ui.systemUsesDarkTheme", 1);\n' > "$out/dark/user.js"

# name | demo path | reference page (the M34 box's library; empty: no counterpart)
radix=https://www.radix-ui.com/themes/docs/components
shadcn=https://ui.shadcn.com/docs/components
origin=https://coss.com/origin
tremor=https://tremor.so/docs
pages="
button|/button?loading=1|$radix/button
field|/field?email=ada|$radix/text-field
inputs|/inputs|$radix/select
form|/form|$shadcn/form
form-errors|/form?errors=1|$radix/callout
combobox|/combobox?q=r&sel=Zig|$shadcn/combobox
otp|/otp|$shadcn/input-otp
upload|/upload|$origin/file-upload
calendar|/calendar?month.day=2026-09&day=2026-09-17|$shadcn/calendar
wizard|/wizard?step.signup=1|$origin/stepper
toggle-group|/toggle-group?align=center&style=bold|$radix/segmented-control
settings|/settings|$radix/switch
dialog|/dialog?dialog=confirm|$radix/dialog
popover|/popover|$radix/dropdown-menu
context-menu|/context-menu|$radix/context-menu
nav|/nav|$shadcn/sheet
toast|/toast|$shadcn/sonner
tabs|/tabs?tab.demo=1|$radix/tabs
accordion|/accordion?open.faq=0,2&open.faq-more=0|$shadcn/accordion
list|/list?page=2|$shadcn/pagination
sidebar|/sidebar|$shadcn/sidebar
nav-menu|/nav-menu|$shadcn/navigation-menu
palette|/palette?q=ta|$shadcn/command
dashboard|/dashboard|$tremor/visualizations/area-chart
chart|/chart|$tremor/visualizations/bar-chart
table|/table?q.files=a|$shadcn/data-table
description-list|/description-list|$radix/data-list
card|/card|$radix/card
kanban|/kanban|https://www.diceui.com/docs/components/kanban
marquee|/marquee|https://magicui.design/docs/components/marquee
counter|/counter|
stream|/stream|$radix/skeleton
blocks-shell|/blocks/shell|$shadcn/sidebar
blocks-auth|/blocks/auth|
blocks-record|/blocks/record|
blocks-error|/no-such-page|
"
if [ -n "$only" ] && ! echo "$pages" | grep -q "^$only|"; then
  echo "no page named $only in scripts/look.sh" >&2; exit 2
fi

PORT=3009 target/debug/demo >/dev/null 2>&1 &
server=$!
trap 'kill $server' EXIT
for _ in $(seq 20); do curl -s -o /dev/null http://127.0.0.1:3009/ && break; sleep 0.25; done

shot() { timeout 90 firefox --headless --profile "$out/$1" --window-size="$2" --screenshot "$out/$3.png" "$4" >/dev/null 2>&1 || true; }
# First visit teaches the server the browser's capabilities; screenshots come from the second.
shot light 1280,900 warm-light http://127.0.0.1:3009/
shot dark 1280,900 warm-dark http://127.0.0.1:3009/
rm -f "$out"/warm-*.png
echo "$pages" | while IFS='|' read -r name path url; do
  [ -n "$name" ] || continue
  [ -z "$only" ] || [ "$name" = "$only" ] || continue
  for scheme in light dark; do
    for width in 1280 768 420; do
      shot "$scheme" "$width,1100" "$name-$scheme-$width" "http://127.0.0.1:3009$path"
    done
  done
  if [ -n "$url" ] && [ "$ref" = 1 ]; then
    shot light 1280,1100 "$name-ref" "$url"
  fi
  echo "shot $name"
done
ls "$out"/*.png | wc -l | xargs -I{} echo "{} PNGs under ${out#$PWD/}/"
