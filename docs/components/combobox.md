# Combobox

`loco_ui::combobox` · demo: `/combobox`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Combobox` | `ui.combobox(name: &str, action: &str)` | stable |

## Props of `Combobox`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `options` | value | `values: impl IntoIterator<Item = &'a str>` |  |  | Suggestions with no `<optgroup>`. |
| `group` | value | `label: &'a str, values: impl IntoIterator<Item = &'a str>` |  |  | Suggestions under `<optgroup label>`. |
| `results` | value | `results: impl IntoIterator<Item = &'a str>` |  |  | The server's own results for the query. |
| `multiple` | switch |  | `off` |  | Results add to the selection instead of replacing it. |
| `create` | value | `action: &'a str` |  |  | A "Create" row posting to `action` when the query matches nothing. |
| `label` | value | `label: &'a str` | `Search` | `aria-label` | Accessible name of the input. |
| `placeholder` | value | `placeholder: &'a str` | `Type to search…` |  | Placeholder of the input. |
| `keep` | item | `name: &'a str` |  |  | A query parameter of this page (`view`) carried, with the request's value, by the search form, every result and every chip's remove link. |
| `disabled` | switch |  | `off` | `disabled` | The box and its button disabled, the chips without remove links, no results. |
| `readonly` | switch |  | `off` |  | The box `readonly`, the chips without remove links, no results. |

## Keyboard of `Combobox`

| Keys | What they do | Needs the script |
|---|---|---|
| `Tab / Shift+Tab` | Moves between the search field, the Search button, the result links and the remove links. | no |
| `ArrowDown / ArrowUp` | Opens the suggestion list under the field and moves through it. | no |
| `Enter` | Fills in the highlighted suggestion, or searches for the typed text when none is highlighted. | no |
| `Escape` | Closes the suggestion list. | no |
| `ArrowDown / ArrowUp` | Moves focus between the search field and the result links. | yes |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-combobox` | `--lui-space-2` |
| `.lui-combobox-input` |  |
| `.lui-combobox-chips` |  |
| `.lui-combobox-results` | `--lui-space-2`, `--lui-space-4` |
| `.lui-combobox-status` | `--lui-space-2`, `--lui-text-2`, `--lui-muted` |
| `.lui-combobox-list` | `--lui-space-1`, `--lui-popover`, `--lui-line`, `--lui-radius`, `--lui-shadow-md`, `--lui-highlight`, `--lui-text-2`, `--lui-space-2`, `--lui-radius-sm`, `--lui-primary`, `--lui-on-primary`, `--lui-hit` |
| `.lui-combobox-chosen` | `--lui-space-2`, `--lui-radius-sm`, `--lui-hit` |
| `.lui-combobox-picked` | `--lui-muted`, `--lui-text-1` |
| `.lui-combobox-empty` | `--lui-space-6` |
| `.lui-combobox-create` | `--lui-space-1` |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<datalist>` | 20 | 4 | 12.1 |
| `<optgroup>` | 20 | 4 | 12.1 |
| `<search>` | 118 | 118 | 17 |
| `aria-live` | 1 | 1 | 1 |

- Fallback: none needed: chips, results and the create row are links and forms
- Needs script: partly: static suggestions and per-submit results; live filtering and arrow keys into the results need script
- Responsive by: content
- Look after: shadcn Combobox

## Documentation

A text input with native suggestions, a server-filtered result list, and a selection
shown as removable chips, no script. Suggestions can be grouped, several values can be
selected, a "create" row appears when nothing matches, and the selection survives every
re-filter because it travels in the form as hidden fields.

**Platform features:**
- `<input list>` + `<datalist>` (baseline 2020) gives native type-ahead from a fixed list
  the server already knows; `<optgroup>` inside it (Chrome 20, Firefox 4, Safari 12.1)
  groups the suggestions where the browser draws them grouped.
- `<search>` element (baseline 2023) for semantics; the results are a plain list of links
  (a `listbox` of options cannot hold links, and without script it could not be operated
  as one), named "Results", with `aria-live="polite"` so a swapped list is announced.
- Submitting the form (Enter or the button) re-renders with the server's results. Every
  result is a link that adds it to the selection (or replaces it, unless `multiple`), every
  chip has a link that removes it, and the "create" row is a plain post form.

**Accessibility:** a `<search>` form: a labelled search input with a datalist, results a
plain list of links named "Results" in an `aria-live` region, chips removable by named
links. Checked by axe-core in headless Firefox on every demo route, both capability
variants, light and dark (no serious or critical violation).

**What it does not do without script:** filter as you type against the server or move
through results with arrow keys and a live `aria-activedescendant`; suggestions come from
the `<datalist>` sent with the page.

**Fallback:** none needed; `Caps` is accepted for uniformity and unused.

**Without script:** results update per round trip and the arrow keys do not move into
the list (Tab does). With the enhancement script the form is submitted as you type and
ArrowDown/ArrowUp walk the input and the results.

```rust
use loco_ui::prelude::*;
// The text is `?q=` and the selection `?sel=`, read from the request.
let ui = Ui::from_request("/langs", "q=ru&sel=Zig", "");
let m = ui.combobox("q", "/langs")
    .group("Systems", ["Rust", "Zig"])
    .options(["Ruby"])
    .multiple()
    .create("/langs/new")
    .label("Language")
    .placeholder("Type a language");
let html = m.render().into_string();
assert!(html.contains("<optgroup label=\"Systems\">"));
assert!(html.contains("name=\"sel\" value=\"Zig\""), "the selection rides along with the next search");
assert!(html.contains("href=\"/langs?q=ru&amp;sel=Zig&amp;sel=Rust\""), "a result adds itself");
assert!(html.contains("href=\"/langs?q=ru\" aria-label=\"Remove Zig\""), "a chip removes itself");
// The same in `lui!`:
let same = lui! { Combobox("q", "/langs") group=("Systems", ["Rust", "Zig"])
    options=(["Ruby"]) multiple create="/langs/new" label="Language"
    placeholder="Type a language"; };
assert_eq!(same.into_string(), html);
```
