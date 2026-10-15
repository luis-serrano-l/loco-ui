# Tag input

`loco_ui::tag_input` · demo: `/tag-input`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `TagInput` | `ui.tag_input(name: &str, label: &str)` | beta |

## Props of `TagInput`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `tags` | value | `tags: &'a [&'a str]` |  |  | The tags, instead of the query's `<name>[]`. |
| `action` | value | `action: &'a str` |  | `formaction` | The page Add sends the form back to with GET; this page by default. |
| `placeholder` | value | `placeholder: &'a str` |  | `placeholder` | Placeholder of the text box. |
| `help` | value | `help: &'a str` |  |  | Help text under the box. |
| `error` | value | `message: &'a str` |  |  | A server message under the box. |
| `id` | value | `id: &'a str` |  | `id` | The text box's id, `f-<name>` by default. |

## Keyboard of `TagInput`

| Keys | What they do | Needs the script |
|---|---|---|
| `Tab / Shift+Tab` | Moves between the remove links, the text box and the Add button. | no |
| `Enter` | Adds the typed tag when the Add button is the first submit button in its form; inside a `Form`, Enter submits the form instead. | no |
| `Enter / Space` | Adds the typed tag from the focused Add button. | no |
| `Enter` | Removes the tag from its focused remove link. | no |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-tag-input-box` | `--lui-space-1`, `--lui-control-h`, `--lui-input`, `--lui-radius-sm`, `--lui-duration-fast`, `--lui-danger` |
| `.lui-tag-input-field` | `--lui-ring`, `--lui-control-h` |
| `.lui-tag-input-chips` |  |
| `.lui-tag-input-chip` | `--lui-radius-sm`, `--lui-brand-3`, `--lui-brand-11`, `--lui-text-1`, `--lui-leading-1`, `--lui-brand-5`, `--lui-hit` |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `formmethod` | 9 | 4 | 5.1 |
| `formnovalidate` | 4 | 4 | 5 |
| `:has()` | 105 | 121 | 15.4 |

- Fallback: none needed: Add and remove are page loads that keep the tags in the URL
- Needs script: partly: a chip on Enter or a comma, and removing one in place, need script
- Responsive by: content
- Look after: Origin UI tag input, shadcn Badge

## Documentation

A field that collects several short values (labels, keywords, emails) as removable chips.
The tags travel in the form as hidden `<name>[]` fields, so the post that saves the form
sends them all; the text box beside them is a `<name>[]` too, so a typed tag rides along
with that post as well. Without script, the Add button sends the form back to the page
with GET and the tag becomes a chip; each chip's × is a link to the page without it.

After the Origin UI tag inputs and the shadcn/ui badge: one bordered box holding soft
brand chips (a remove × in each), the text box and the Add button, wrapping as it fills.
A combobox's selected values use the same chips (`.lui-tag-input-chip`).

**Platform features:** hidden inputs named `<name>[]` (every browser), a button with
`formmethod="get"`, `formaction` and `formnovalidate` (baseline 2015) for Add, plain links
for remove, and `:has()` (Chrome 105, Firefox 121, Safari 15.4) for the box's focus ring.

**Accessibility:** the text box has the field's `<label>`; the chips are a list named
"Selected", each × a link named "Remove rust"; help and error describe the text box.
Checked by axe-core in headless Firefox on every demo route, both capability variants,
light and dark (no serious or critical violation).

**What it does not do without script:** turn a tag into a chip on Enter or a comma, or
remove one without a round trip. Without script Add and × are page loads that keep the
other tags in the URL.

**Fallback:** none needed. Without `:has()` the box keeps no ring and the text box shows
its own focus outline.

```rust
use loco_ui::prelude::*;
// The tags are the query's `tags[]` (an Add round trip leaves them there), or `.tags(..)`.
let ui = Ui::from_request("/note", "tags%5B%5D=rust&tags%5B%5D=web", "");
let m = ui.tag_input("tags", "Tags").render().into_string();
assert!(m.contains(r#"<input type="hidden" name="tags[]" value="rust">"#));
assert!(m.contains(r#"href="/note?tags%5B%5D=web" aria-label="Remove rust""#));
assert!(m.contains(r#"formmethod="get" formaction="/note""#));
// The same in `lui!`:
assert_eq!(lui! { TagInput("tags", "Tags"); }.into_string(), m);
// Saved tags, a placeholder and the page the Add round trip goes to.
let m = ui.tag_input("tags", "Tags").tags(&["css"]).placeholder("Add a tag").action("/note/edit");
let html = m.render().into_string();
assert!(html.contains(r#"value="css""#) && !html.contains(r#"value="rust""#));
let same = lui! { TagInput("tags", "Tags") tags=(&["css"]) placeholder="Add a tag" action="/note/edit"; };
assert_eq!(same.into_string(), html);
```
