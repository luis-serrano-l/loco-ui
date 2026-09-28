# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

`loco-ui` is a no-JavaScript-required component library for Rust servers (Axum + Maud).
Every component is a plain `fn name(caps, ...) -> Markup`; interactivity comes from the HTML/CSS
platform (`<dialog>`, `popover`, invoker commands, `<details name>`, `<datalist>`, view
transitions) and ordinary form round trips. **Every page must work with script disabled.** One
optional script, `loco-ui/src/enhance.rs`, served at `/lui/enhance.js`, upgrades swap roots
(`id` + `data-lui="swap"`) to fetch-and-replace in place. It is the only `<script>` allowed: a
test enforces that (`demo/src/tests.rs::pages_ship_only_the_enhancement_script`), Blitz
(no script engine) proves every route works without it, and `scripts/browser-check.mjs` drives
headless Firefox to prove the script does its job. Never add inline script, handlers or a
second file; when something needs more, extend `enhance.rs` and keep the no-script path intact.

## Commands

```sh
cargo run -p demo                  # demo server at http://127.0.0.1:3000
cargo dev                          # same, rebuilt and restarted when crate source or a manifest changes (needs cargo-watch)
cargo test --workspace             # all tests, including the only-one-script test and doctests
cargo test -p demo pages_ship_only_the_enhancement_script   # the single enforcement test
node scripts/browser-check.mjs     # headless Firefox via geckodriver: the script works (needs a built demo)
cargo test -p loco-ui --doc      # component doc examples
cargo clippy --workspace --all-targets  # must be clean before a roadmap milestone counts as done
cargo test -p loco-ui-test       # Blitz layout assertions + screenshots into tests/shots/
cargo bench -p loco-ui           # criterion: stylesheet, layout, table with 1 000 rows, paged table, UiState
scripts/bench.sh                   # latency baseline: curl p50/p95 TTFB and Firefox navigation timing on 3001
node scripts/bench-swap.mjs [runs] # click-to-paint of in-place updates: the script beside htmx 2 on the same answers
scripts/look.sh [--only <name>] [--tag <tag>] [--no-ref]  # Firefox shots of every page (light/dark, 1280/768/420) beside its M34 reference, into target/look/[<tag>/]
cargo run -p demo -- spec write    # regenerate spec/components.json and the README feature matrix from loco_ui::spec::SPECS
scripts/snapshot.sh [dir]          # static GitHub Pages snapshot of every GET page into target/site/
cargo run -p loco-ui -- install <app> --dep-path "$PWD/loco-ui"   # the cargo-lui installer on a Loco app (also `auth`); tests/install.rs runs it on tests/fresh-loco-app
scripts/verify.sh                  # fmt check, build, clippy at every feature level, tests, rustdoc, <script> grep, browser check; run before committing
```

Never format Rust by hand: write the code in any layout, then run `cargo fmt --all` (verify.sh
fails on `cargo fmt --all --check`). Do not spend edits or tokens on line breaks, wrapping or
indentation that rustfmt decides.

Delete screenshots once you have looked at them: `target/look/`, `target/shots/`,
`target/readme-shot/` and any shots in the scratchpad (`rm -rf` them after the check). The
PNGs under `tests/shots/` and `docs/` are tracked: never delete those, and commit a changed one
only when the change is intended (otherwise `git checkout -- tests/shots`).

## Workspace layout

- `loco-ui-caps/` – the detection crate: `Caps`/`Cap` bitset, `@supports` beacons, cookie and
  query parsing, `beacon_cookie`, and an `axum` feature with the extractor and beacon route.
  `loco-ui` re-exports it as `loco_ui::caps`, so nothing else changes.
- `loco-ui-macros/` – the `lui!` proc macro (Maud plus components written like elements), re-exported
  as `loco_ui::lui` and in the prelude. A token walker over `proc-macro2`/`quote`, no `syn`; its
  rules are tested in `loco-ui/tests/lui.rs` and its errors pinned with `trybuild` in `tests/ui/`.
- `loco-ui/` – the library crate. Depends only on `maud`, `loco-ui-caps` and `loco-ui-macros`. Feature `http` adds
  `Redirect::into_http` and `Streamed` (a chunk stream); feature `axum` adds the `Ui` extractor,
  `IntoResponse` for `Page`/`Redirect`/`Streamed`, the `/lui/caps` beacon route, `Posted`
  (urlencoded or multipart posts by name) and `Saved<T>` (serde in a cookie); feature `loco`
  adds `loco::Initializer`, which mounts the script and beacon routes in a Loco app, `Valid<T>`
  and `Table::paged_from`. Everything else is plain functions over strings
  (`Ui::from_request`, `Page::into_string`, `Redirect::set_cookies`, `caps::beacon_cookie`).
  `src/blocks/` holds whole pages built from the components (app shell, auth, settings,
  record, dashboard, error pages), each a builder reached from `ui` like a component.
  `src/i18n.rs` holds the components' own words (`Text`, `Strings`); an app adds a language
  with `i18n::languages` (the demo's `spanish.rs` is the example). `src/bin/cargo-lui.rs` is
  the `cargo lui install` / `cargo lui auth` installer, fed by `loco-templates/`.
- `demo/` – Axum lib + binary, one route per component, one `use loco_ui::prelude::*`.
  Routes live in `demo/src/routes/<group>.rs`, one file per index group, each with a
  `routes()` that `router()` in `lib.rs` merges, plus `PAGES` (component-and-note pages) and
  `PREVIEWS` (the live component the index shows); the shell and index are in `site.rs`, the
  snippet machinery in `code.rs`, the props playground in `playground.rs`, the static Pages
  export in `snapshot.rs`, the Spanish strings in `spanish.rs`, a component written outside
  the library in `pricing.rs`, the tests in `tests.rs`. A new route file must also be
  added to `SOURCES` in `code.rs`. The binary also takes `-- spec [write]` and
  `-- snapshot <dir>`. Each component page shows the code between its `// code: <href>` and `// end code`
  markers (a test fails if a component page has none), so keep the markers around the
  component call when editing a route.
  Handlers take `ui: Ui` (plus `Saved<T>` / `Form<T>` when they need them) and return `Page`
  or `Redirect`; they only parse input and call `loco-ui`; keep each route around 15 lines. The only-one-script test lives here
  and hits every route via `tower::oneshot`, so **add new demo routes to `PATHS`** (the
  screenshot test in `loco-ui-test` uses the same list).
- `examples/loco-app/` – a Loco notes app (accounts, notebooks, tagged notes, a task board)
  on loco-ui. Notebooks and tasks were generated by `cargo loco generate scaffold` from
  `loco-ui/loco-templates/` and then edited; notes, the overview, the shell and the look
  (`src/views/look.rs`, a `layout::Look` layer) are hand-written. Development seeds a demo
  account on first start (`src/seed.rs`). After changing a template, check it on a throwaway
  model there and run its `tests/pages.rs` (one script, Blitz shots).
- `loco-ui-test/` – Blitz-based test harness: `Page::render(router, path, cookie)` then
  `exists / is_visible / bbox / text / display / screenshot`. Blitz gaps go in `FINDINGS.md`
  with an issue link, never as skipped assertions without a comment.

## Component conventions (follow exactly when adding one)

1. One component = one file `loco-ui/src/<name>.rs`, registered in `lib.rs` as `pub mod`.
   Only types a caller names go in the `prelude` (most never do: builders are reached from `ui`).
2. File starts with a `//!` header: what it does, **Platform features** (with browser baseline
   versions), **What it does not do without script**, **Fallback**, and a runnable ```` ```rust ```` usage example (these are doctests).
3. CSS lives beside the component as `pub const CSS: &str` and must be appended to the array in
   `stylesheet()` in `lib.rs`; `ui.page()` inlines that once per page. Theming only through
   `--lui-*` custom properties defined in `layout.rs`.
4. Root element carries a single `lui-<component>` class; sub-parts use `lui-<component>-<part>`.
   Output should be readable via `curl`.
5. No macros beyond `html!` and `lui!` (the `loco-ui-macros` crate: Maud plus components
   written like elements, expanding to the builder chain). A component is a builder struct holding `&Ui` (or what it needs
   from it) plus an `impl Ui { pub fn <name>(&self, required…) -> <Name> }` in the same file,
   and `impl Render for <Name>`, so a route writes `(ui.<name>(..).setter()..)` inside `html!`,
   or `Name(..) setter item "x" { .. }` inside `lui!`.
   Required arguments stay in the call (text first); everything else is a chained setter. An
   id the caller does not care about is derived from the label with `crate::slug`, with an
   `.id()` override. Components read their own input from `ui` (`ui.param`, `ui.params`,
   `ui.state`) instead of taking it as an argument. For lists (fields, menu items, tabs,
   columns, commands), an adder per item (`.text(..)`, `.link(..)`, `.tab(..)`) and
   modifiers that apply to the item added last (`.required()`, `.icon(..)`, `.badge(..)`). No
   `_with` twins and no `XOptions` structs. The doc header shows the common call first, then
   one with the setters. Setter names follow the HTML attribute or element they set
   (`.maxlength()`, `.placeholder()`, `.closedby()`). A setter with no argument switches
   something on (`.required()`, `.danger()`, `.multiple()`); one that takes a `bool` is one a
   route sets from a condition (`.open(..)`, `.loading(..)`). Branch on `caps.has(Cap::X)` and
   emit only one variant, never both.
   A root that should update in place gets `id=(enhance::swap_id(prefix, key))` and
   `data-lui="swap"`; the markup must behave identically without the script.
   Every builder with setters carries `pub const PROPS: &'static [Prop]` (name, `PropKind`,
   arguments as written, default, HTML attribute, first doc sentence) and an entry in
   `props::COMPONENTS`, so `loco_ui::props()`, the spec JSON and the demo's props tables
   list it; the doc header's main doctest ends with the same call in `lui!` and an
   `assert_eq!` on the HTML. Tests fail when a setter is missing from `PROPS`, a builder or
   constructor from `props()`, or a header lacks its `lui!` twin. Demo snippets write the
   component's markup in `lui!`; the builder chain stays only where the route calls a
   method on it afterwards, and then the `lui!` render block sits beside it. Handler logic
   in a snippet (a redirect, a toast, `ui.stream`) stays Rust.
6. Server-held state (theme, counter, active tab) travels via cookie or `?query=`; mutations use
   `<form method="post">` + redirect (Post/Redirect/Get), never GET side effects.
7. Update the README feature matrix and Findings when a component or its fallback changes.
8. A component builds its parts from the primitives: `ui.button` / `Button::new(caps, ..)`
   (or `class="lui-button"` on a `<summary>`), `ui.input` / `Input` with `.hide_label()` for a
   bare control, `ui.badge`, `ui.card`, `Icon`, and the layouts (`ui.stack`, `ui.cluster`,
   `ui.grid`, `ui.split`). Never a raw `<button>` or visible `<input>` with its own CSS: a
   component's CSS styles its parts by class (`.lui-<component>-<part>`), and a test
   (`only_the_primitives_select_bare_buttons_and_inputs`) fails if it selects a bare `button`
   or `input`. Hidden inputs, `<select>`, range sliders and menu items are the exceptions.
9. Responsive means container queries (M34). A root that changes layout sets
   `container: lui-<component> / inline-size`; the CSS writes the narrow layout as the base
   and adds wider layouts with `@container lui-<component> (width >= …)` at the three
   breakpoints in `layout.rs` only (narrow below 30rem, medium from 30rem, wide from 48rem;
   literal numbers, since custom properties cannot appear in a container condition).
   Viewport `@media` is for `prefers-*` and `pointer` only; under `pointer: coarse` every
   clickable part is at least 44px tall. Nothing may overflow sideways in a 320px container
   or viewport: text wraps or truncates, tables hide low-priority columns and scroll inside
   themselves. Each component copies the look of one named free library (Radix Themes by
   default, else Origin UI, shadcn/ui, Tremor or Dice UI) rebuilt with `--lui-*` tokens,
   never pasted; `no_colour_literal_outside_tokens` fails on a literal colour. Blitz does
   not support `@container`, so `tests/shots/` show the narrow base.

## Roadmap

`ROADMAP.md` is the work queue: work top to bottom; M1 to M34 are ticked except boxes that
wait on the owner. A milestone is done only when every box is ticked, clippy is clean, tests
pass, and README is updated. Each ticked box carries a "Done:" note (what changed, new setters,
what was kept different from the reference and why); write one when ticking. `BLOCKED.md`
holds the questions only the owner can answer (the crates.io publish, posting the launch, the
`lui!` builder mode for the `/wizard` and `/palette` snippets): never run `cargo publish`,
push, or post anywhere without an explicit yes. `CHANGELOG.md` gets an Unreleased entry per
milestone; `FINDINGS.md` gets every platform or Blitz gap with an issue link.
