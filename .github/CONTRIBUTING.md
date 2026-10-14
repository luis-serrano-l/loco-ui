# Contributing to loco-ui

The full rules are in [`CLAUDE.md`](../CLAUDE.md), written for people and coding agents
alike. This page is the short version.

## Before you open a pull request

```sh
scripts/verify.sh        # format check, clippy, tests, the <script> grep
```

CI also runs clippy at every feature level, rustdoc, the docs.rs build, the 1.88 MSRV check,
`cargo package`, `cargo deny`, the installer test and headless Firefox. You do not need to
run those locally.

Never format Rust by hand: run `cargo fmt --all`. If a change moves a screenshot under
`tests/shots/`, commit the PNG only when the change is intended.

## The rule that matters most

Every page works with JavaScript off. The one script, `loco-ui/src/enhance.rs`, only makes
swap roots update in place; a test fails if any page ships another `<script>`.

## Component conventions, in short

1. One component, one file `loco-ui/src/<name>.rs`, registered in `lib.rs`.
2. The file starts with a `//!` header: what it does, platform features with browser
   versions, what it does not do without script, the fallback, and a runnable example.
3. CSS sits beside it as `pub const CSS` and is added to `stylesheet()`. Colours come only
   from the `--lui-*` tokens in `layout.rs`.
4. The root element has one `lui-<component>` class; parts use `lui-<component>-<part>`.
5. A builder holding `&Ui`, reached as `ui.<name>(required…)`, with one chained setter per
   option, each listed in `PROPS`. The header example ends with the same call in `lui!` and
   an `assert_eq!` on the HTML.
6. Server state travels in a cookie or the query; changes are a `<form method="post">` and
   a redirect, never a GET with side effects.
7. Update the README feature matrix (`cargo run -p demo -- spec write`) and `FINDINGS.md`
   when a component or its fallback changes.
8. Build parts from the primitives (`ui.button`, `ui.input`, `ui.badge`, the layouts),
   never a raw `<button>` or visible `<input>` with its own CSS.
9. Responsive means container queries at the three breakpoints in `layout.rs`; nothing
   overflows sideways at 320px.

Every library change also lands in the demo (`demo/src/routes/`) in the same commit, with
a new route added to `PATHS`.

## Adding a component

Follow the nine conventions above; the tests in `loco-ui/src/lib.rs` check most of them and
name the one you missed. A `new-component` skill under `.claude/skills/` (ROADMAP M44) will
walk through them as a checklist.

## Licence

Contributions are published under the MIT licence in [`LICENSE`](../LICENSE).
