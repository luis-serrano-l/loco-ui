# Command palette

`loco_ui::palette` · demo: `/palette`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Palette` | `ui.palette(action: &str)` | stable |

## Props of `Palette`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `link` | item | `label: &'a str, href: &'a str` |  |  | A destination. |
| `links` | value | `links: impl IntoIterator<Item = (&'a str, &'a str)>` |  |  | Several `(label, href)` destinations at once. |
| `keywords` | modifier | `keywords: &'a str` |  |  | Extra words that find the command added last, space-separated. |
| `icon` | modifier | `icon: impl Into<Glyph<'a>>` |  |  | An icon before the command added last. |
| `shortcut` | modifier | `keys: &'a str` |  |  | A key hint at the end of the command added last (shown, not bound). |
| `group` | item | `heading: &'a str` |  |  | List the commands added after this under a heading. |
| `label` | value | `label: &'a str` | `Search` |  | The opener's label (default "Search"). |
| `accesskey` | value | `key: &'a str` | `k` | `accesskey` | The opener's access key (default `k`). |
| `id` | value | `id: &'a str` | `palette` | `id` | The palette's id instead of `palette`. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `popover` | 114 | 125 | 17 |
| `<datalist>` | 20 | 4 | 12.1 |
| `<search>` | 118 | 118 | 17 |
| `accesskey` | 1 | 1 | 1 |

- Fallback: a <details> disclosure with the same form
- Needs script: partly: arrow keys through live results and a global Ctrl+K need script
- Responsive by: viewport
- Look after: shadcn Command

## Documentation

A search box that jumps anywhere: open it from a button or with an access key, type, pick
a suggestion, press Enter. The browser filters the suggestions as you type; the server does
the rest. An exact name goes straight to its page; anything else lands on a results list.
This is the flagship "no script needed" component.

**Platform features:**
- `popover` (Chrome 114, Firefox 125, Safari 17) opened by `popovertarget`; light dismiss
  and Escape for free; `autofocus` puts the caret in the box when it opens.
- `<datalist>` bound by `list=` for as-you-type suggestions, filtered by the browser.
- `<search>` (Chrome 118, Firefox 118, Safari 17) around a GET `<form>`; the server
  redirects an exact name (`Palette::exact`) and lists the matches of anything else.
- `accesskey` on the opener (Alt+Shift+K in Chrome and Firefox, Ctrl+Option+K in Safari),
  announced with `aria-keyshortcuts`.

**Fallback:** without `Caps` `Popover` the palette is a `<details>` disclosure with the
same form inside. A browser without `<datalist>` shows a plain search box; the results
page still works.

**Accessibility:** a labelled search field with `aria-keyshortcuts`, results in a section
named by its heading, every command a link. Checked by axe-core in headless Firefox on every
demo route, both capability variants, light and dark (no serious or critical violation).

**What it does not do without script:** arrow-key navigation through a live results list
and the global Ctrl+K shortcut (the access key stands in for it).

```rust
use loco_ui::prelude::*;
// A search for "new" that matched no command exactly.
let ui = Ui::from_request("/search", "q=new", "");
let palette = ui.palette("/search")
    .group("Go to")
    .link("Open settings", "/settings")
    .link("New invoice", "/invoices/new").keywords("bill create");
assert_eq!(palette.exact(), None, "an exact name would redirect");
let m = palette.render().into_string();
assert!(m.contains(r#"popovertarget="palette""#) && m.contains(r#"list="palette-list""#));
assert!(m.contains("lui-palette-results") && m.contains("New invoice"));
// The same in `lui!`:
let same = lui! { Palette("/search") {
    group "Go to";
    link "Open settings" "/settings";
    link "New invoice" "/invoices/new" keywords="bill create";
} };
assert_eq!(same.into_string(), m);
let ui = Ui::from_request("/search", "q=open+settings", "");
assert_eq!(ui.palette("/search").link("Open settings", "/settings").exact(), Some("/settings"));
// An icon and a key hint on a row.
let m = ui.palette("/search").link("Open settings", "/settings").icon(Icon::Pencil).shortcut("⌘ ,").render().into_string();
assert!(m.contains(r#"<kbd class="lui-palette-shortcut">⌘ ,</kbd>"#));
assert_eq!(lui! { Palette("/search") { link "Open settings" "/settings" icon=(Icon::Pencil) shortcut="⌘ ,"; } }.into_string(), m);
```
