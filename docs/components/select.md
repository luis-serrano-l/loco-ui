# Select

`loco_ui::select` · demo: `/inputs`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Select` | `ui.select(name: &str, label: &str)` | stable |
| `SelectOption` | `SelectOption::new(value: &str, text: &str)` | stable |

## Props of `Select`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `options` | value | `options: impl IntoIterator<Item = O>` |  |  | Options with no `<optgroup>`. |
| `group` | value | `label: &'a str, options: impl IntoIterator<Item = O>` |  |  | Options under `<optgroup label>`. |
| `groups` | value | `groups: impl IntoIterator<Item = (&'a str, G)>` |  |  | Several labelled groups at once. |
| `search` | value | `action: &'a str` |  |  | A filter box submitting `<name>-q` to `action` with GET. |
| `search_over` | number | `n: usize` | `15` |  | Show the filter box above this many options (default 15). |
| `value` | value | `value: &'a str` |  |  | The chosen option's value, instead of the query's `name` (once more per pick with `.multiple()`). |
| `error` | value | `message: &'a str` |  |  | A server message under the select, which gets `aria-invalid`. |
| `help` | value | `help: &'a str` |  |  | Help text under the select. |
| `placeholder` | value | `placeholder: &'a str` |  |  | A first option posting nothing ("Choose…"), picked until another is. |
| `required` | switch |  | `off` | `required` | An option other than the placeholder must be picked. |
| `disabled` | switch |  | `off` | `disabled` | `disabled` and `aria-disabled`: shown greyed and not posted. |
| `readonly` | switch |  | `off` |  | Shown and posted, not changeable: a disabled select and a hidden input per picked value. |
| `multiple` | switch |  | `off` | `multiple` | Several options may be picked (a list box; no rich trigger). |
| `hide_label` | switch |  | `off` |  | Only the select, the label kept as its `aria-label`. |
| `size` | choice: 1, 2, 3 | `size: impl Into<Size>` | `2` |  | The trigger's height and text: Radix's sizes 1 to 3. |
| `variant` | choice: solid, soft, surface, outline, ghost | `variant: Variant` | `surface` |  | The trigger's look: surface (a border), soft (a tint) or ghost (neither). |

## Props of `SelectOption`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `icon` | value | `icon: impl Into<Glyph<'a>>` |  |  | An icon before the label. |
| `body` | value | `content: Markup` |  |  | Rich markup instead of the text (with `Caps::BaseSelect` only). |
| `disabled` | switch |  | `off` | `disabled` | Shown, but cannot be picked. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<selectedcontent>` | 135 | no | 27 |
| `appearance: base-select` | 135 | no | 27 |
| `<optgroup label>` | 1 | 1 | 1 |
| `formmethod` | 9 | 4 | 5.1 |

- Fallback: plain <select>, chosen server-side
- Needs script: no
- Responsive by: content
- Look after: Radix Themes Select

## Documentation

A `<select>` whose closed state shows the chosen option's full content, no script. Options
come in labelled groups and can carry an icon; a long list gets a filter box.

**Platform features:**
- `<select>` with a `<button>` first child holding `<selectedcontent>` (Chrome 135, Safari
  27; Firefox behind flags): the button is the closed control and `<selectedcontent>`
  mirrors the picked option's markup into it, so options can carry a swatch or an icon.
- `appearance: base-select` on the select and its `::picker(select)` pseudo-element hands
  both to author CSS.
- `<optgroup label>` (baseline) for `Select::group`.
- A filter box when there are more than `Select::search_over` options: an
  `<input type="search" name="<name>-q">` and a button with `formmethod="get"` and
  `formaction` (baseline 2015), so filtering re-requests the page through the enclosing
  form without saving it. The server renders only the options whose text contains the
  query, and always the selected one.

**Accessibility:** a native `<select>` with its `<label>`; the filter box is a labelled
search field. Checked by axe-core in headless Firefox on every demo route, both capability
variants, light and dark (no serious or critical violation).

**What it does not do without script:** type-ahead search beyond what the browser offers;
long lists get a server-side filter box instead.

**Fallback:** without `Caps::BaseSelect` a plain `<select>` with plain options (icons as
text before the label). Older parsers also drop a `<button>` inside `<select>`, so the
enhanced markup is only emitted when the browser is known to want it.

**Enhanced:** the enhancement script filters as you type, through the same GET.

```rust
use loco_ui::{prelude::*, select::SelectOption};
// The chosen value is the query's `food`, as a form posted back by GET leaves it.
let mut ui = Ui::from_request("/shop", "food=leek&food-q=k", "");
ui.caps = Caps::all();
// Plain tuples of (value, label) or (value, label, icon) are options.
let m = ui.select("size", "Size").options([("s", "Small", "🐭"), ("l", "Large", "🐘")]).value("l");
let m = m.render().into_string();
assert!(m.contains("<selectedcontent>") && m.contains(r#"<label for="f-size">Size</label>"#));
assert!(m.contains(r#"<option value="l" selected>"#));
// A server error, linked from the error summary like a form field's.
let bad = ui.select("size", "Size").options([("s", "Small")]).error("Pick a size.");
assert!(bad.render().into_string().contains(r#"aria-describedby="f-size-error""#));

let m = ui.select("food", "Food")
    .group("Fruit", [SelectOption::new("apple", "Apple"), SelectOption::new("kiwi", "Kiwi").body(html! { b { "Kiwi" } })])
    .group("Vegetables", [("leek", "Leek")])
    .search("/shop")
    .search_over(2);
let m = m.render().into_string();
assert!(m.contains("<optgroup label=\"Fruit\">") && !m.contains("Apple"), "filtered to 'k'");
assert!(m.contains("formmethod=\"get\" formaction=\"/shop\""));

// The same in `lui!`:
let same = lui! { Select("food", "Food") search="/shop" search_over=2 {
    group "Fruit" ([
        SelectOption::new("apple", "Apple"),
        SelectOption::new("kiwi", "Kiwi").body(html! { b { "Kiwi" } }),
    ]);
    group "Vegetables" ([("leek", "Leek")]);
} };
assert_eq!(same.into_string(), m);
// A small soft trigger.
let sort = ui.select("sort", "Sort").options([("new", "Newest")]).size(1).variant(Variant::Soft);
let sort = sort.render().into_string();
assert!(sort.contains(r#"<span class="lui-select" data-size="1" data-variant="soft">"#));
let same = lui! { Select("sort", "Sort") options=([("new", "Newest")]) size=1 variant="soft"; };
assert_eq!(same.into_string(), sort);
```
