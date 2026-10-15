---
name: new-component
description: Add a component to loco-ui end to end - the file from `loco-ui/src/_template.rs.txt`, every list it must be appended to (lib.rs, props, spec, demo PATHS, index, i18n, look.sh), the tests that fail when one is missed, and the done-criteria. Use when asked to add, port or scaffold a component, or to check that a new one is wired in everywhere. Ends with the `component-reviewer` agent.
---

# New component

The nine component conventions in `CLAUDE.md` are the rules; this is the order to apply them
in and the list of every place a component is registered. Most of these lists are checked by a
test, so a missed one shows up as a failure, but a few (look.sh, the playground, Spanish) are
not: do those by hand.

Shared lists are append-only: add one entry at the END, never reorder (other branches append
too, and merges stay clean). `pub mod` in `lib.rs` is the exception: it is alphabetical.

Below, `<name>` is the module (`toggle_group`), `<Name>` the builder (`ToggleGroup`),
`<kebab>` the CSS name (`toggle-group`).

## 1. Pick the look first

Each component copies one named free library, rebuilt with `--lui-*` tokens and never pasted:
Radix Themes by default, else Origin UI, shadcn/ui, Tremor or Dice UI. Open its page and write
down heights, paddings, radii, gaps, font sizes and weights, borders, shadows and the colour
step per state. A value with no token gets a token in `layout.rs` (owned by M39 during the
M37 to M46 push: ask, do not edit) rather than a literal.

## 2. The file

```sh
cp loco-ui/src/_template.rs.txt loco-ui/src/<name>.rs
```

Replace `widget`/`Widget`, work through every `TODO(n)` (n is the convention number in
`CLAUDE.md`), then delete the TODO lines. Keep `///` doc lines and `#[derive]` directly above
`pub struct`: the tests find builders by the `**Setters.**` line in that doc, and a `//` line
in between hides the builder from them silently.

The `//!` header has, in this order: what it does and "After <library> <Component>",
**Platform features** (with Chrome/Firefox/Safari versions), **Accessibility**, **What it does
not do without script**, **Fallback**, and a ```` ```rust ```` doctest that shows the common
call, one call with setters, and the same call in `lui!` with `assert_eq!` on the HTML.

## 3. The lists

| Where | What to add | Test that fails without it |
|---|---|---|
| `loco-ui/src/lib.rs` `pub mod` | `pub mod <name>;` (alphabetical) | nothing compiles in the demo |
| `loco-ui/src/lib.rs` `COMPONENT_CSS` | `<name>::CSS,` at the end | your demo page has no styles; `stylesheet_stays_under_its_budget` watches the total |
| `loco-ui/src/lib.rs` `layouts_respond_to_their_container` `named` | `("<name>", <name>::CSS),` at the end (and in `NO_BREAKPOINT` with a reason if one layout fits every width) | `layouts_respond_to_their_container` ("list every stylesheet here") |
| `loco-ui/src/lib.rs` `prelude` | only a type a caller names (most never do) | none |
| `loco-ui/src/props.rs` `COMPONENTS` | `Component { module, builder, calls: &["ui.<name>(label: &str)"], props: crate::<name>::<Name>::PROPS, keys: crate::<name>::<Name>::KEYS, status: Status::Beta }` | `props_lists_every_builder_and_constructor` |
| `<Name>::KEYS` | one `Key::new(keys, does)` per key its markup answers to, `.script()` when only enhance.rs does it; `keys: crate::<name>::<Name>::KEYS` in its `COMPONENTS` entry | `builders_with_controls_list_their_keys` |
| `<Name>::PROPS` | one `Prop` per setter, kind, arguments as written, `.attr(..)`, `.doc(..)` | `every_setter_is_in_props`, `every_setter_is_listed_on_its_builder`, `props_follow_the_conventions` |
| `loco-ui/src/spec.rs` `SPECS` | `ComponentSpec { name, module, look: "Radix Themes X", responsive, features, fallback, needs_js }` at the end | `every_component_has_a_spec_and_every_spec_a_file`, `every_component_header_shows_the_lui_form` |
| `loco-ui/src/spec.rs` tests `SOURCES` | `("<name>", include_str!("<name>.rs")),` at the end, same order as `SPECS` | `every_component_has_a_spec_and_every_spec_a_file`, `doc_headers_match_the_spec` (every `features` name must appear in the header), `responsive_matches_the_css` |
| `spec/components.json` and the README matrix | `cargo run -p demo -- spec write` | `json_file_is_generated_from_this_spec`, `readme_matrix_is_generated_from_this_spec` |
| `loco-ui/src/i18n.rs` | each word the component shows: a `Text` variant at the end of the enum, of `Text::ALL` (bump its length) and of `ENGLISH` | `components_write_no_english_outside_the_string_table`, `keys_are_kebab_case_and_the_table_is_complete` |
| `demo/src/spanish.rs` | `.with(Text::New, "…")` for each new key | none: do it by hand |
| `demo/src/routes/<group>.rs` | the page: a `PAGES` entry `("/<kebab>", draw, "note")` for a page that is its component, or a route plus a `PREVIEWS` entry; the snippet between `// code: /<kebab>` and `// end code`, written in `lui!`; the drawing fn takes `ui: &Ui` (`lui!` calls `ui.<name>(..)`) | `every_component_page_shows_its_code`, `component_pages_show_their_props`, `the_index_shows_every_component_and_every_page_the_sidebar` |
| `demo/src/code.rs` `SOURCES` | only for a NEW route file (bump the array length) | the snippet tests |
| `demo/src/lib.rs` `router()`, `demo/src/site.rs` `preview` | only for a new route file: `.merge(routes::<group>::routes())`, and its `PAGES` / `PREVIEWS` in the two lists in `site::preview` | the page 404s, or the index shows no live component |
| `demo/src/lib.rs` `PATHS` | `"/<kebab>"` at the end (bump the array length) | nothing fails, but `pages_ship_only_the_enhancement_script`, `every_route_is_served_under_a_strict_csp`, the Blitz shot test, the browser check and the Pages snapshot never see the page |
| `demo/src/site.rs` `COMPONENTS` | `("/<kebab>", "Title", "Group", "platform features", "what it is for")` at the end (bump the array length) | `site::title` and the index; `every_group_on_the_index_has_a_live_component` (Blitz) |
| `demo/src/playground.rs` `ENTRIES` | an `Entry` with the switches the props table can tick | `the_playground_renders_what_was_chosen` checks the ones listed; none forces an entry |
| `loco-ui-test/tests/demo.rs` | a `#[tokio::test]` with `Page::render` and `exists` / `is_visible` / `bbox` assertions for the parts that must lay out; Blitz gaps go in `FINDINGS.md` with an issue link | `every_route_renders_and_is_captured` writes `tests/shots/<kebab>-{modern,fallback}.png` |
| `scripts/look.sh` `pages` | `<kebab>\|/<kebab>?state\|<reference page URL>` at the end | none: do it by hand |
| `FINDINGS.md` | each platform or Blitz gap, with an issue link | none |
| `CHANGELOG.md` | an Unreleased bullet under the milestone heading | none |

## 4. Done when

1. `cargo fmt --all`, then `scripts/verify.sh` passes (fmt check, clippy, tests, the
   `<script>` grep).
2. `cargo test -p loco-ui-test` ran once and only your page's PNGs under `tests/shots/`
   changed (`git checkout -- tests/shots` for the rest).
3. The `look-check` skill on your page: light and dark at 1280, 768 and 420 beside the
   reference, with each difference fixed or written down in the "Done:" note. Then
   `rm -rf target/look`.
4. The page works with script off (the demo tests prove it) and nothing overflows at 320px.
5. Run the `component-reviewer` agent with the component's module name and fix every FAIL it
   lists before ticking the roadmap box (see the `roadmap-tick` skill for the note).
