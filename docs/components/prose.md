# Prose

`loco_ui::prose` · demo: `/prose`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Prose` | `ui.prose()` | beta |

## Props of `Prose`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `body` | value | `markup: Markup` |  |  | The HTML to style: headings, paragraphs, lists, quotes, code, tables. |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-prose` | `--lui-fg`, `--lui-text-8`, `--lui-leading-8`, `--lui-text-7`, `--lui-leading-7`, `--lui-line`, `--lui-text-6`, `--lui-leading-6`, `--lui-text-5`, `--lui-leading-5`, `--lui-link`, `--lui-radius-lg`, `--lui-font-mono`, `--lui-gray-3`, `--lui-radius-sm`, `--lui-gray-2`, `--lui-text-2`, `--lui-leading-3`, `--lui-font-sans`, `--lui-gray-1`, `--lui-gray-7`, `--lui-leading-2` |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `:where()` | 88 | 78 | 14 |
| `text-wrap: pretty` | 117 | no | 26 |

- Fallback: greedy line breaks without text-wrap: pretty
- Needs script: no
- Responsive by: content
- Look after: shadcn/ui Typography

## Documentation

Styles for plain HTML you did not build from components: a blog post rendered from
Markdown, a help page, release notes. Inside it, bare `h1` to `h4`, `p`, `a`, lists,
`blockquote`, `code`, `pre`, `kbd`, `hr`, `table` and `img` get a readable rhythm; outside
it, those elements keep the browser's look and the components keep theirs. After shadcn/ui
Typography: an extra-bold `h1`, a ruled `h2`, 1.75rem lines, 1.5rem between paragraphs,
lists indented 1.5rem, an italic quote behind a 2px bar.

**Platform features:** `:where()` and `:is()` (Chrome 88, Firefox 78, Safari 14) keep
every rule at the weight of one element, so a component placed inside prose, or a class
of the app's own, wins; `text-wrap: pretty` (Chrome 117, Safari 26) where present.

**Accessibility:** styles only; the HTML it wraps keeps its own headings, lists and
tables. Checked by axe-core in headless Firefox on every demo route, both capability
variants, light and dark (no serious or critical violation).

**What it does not do without script:** nothing is missing.

**Fallback:** without `text-wrap: pretty` paragraphs break greedily.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
// Markdown rendered elsewhere goes in as `PreEscaped`, after it has been sanitised.
let m = ui.prose().body(PreEscaped("<p>From <em>Markdown</em>.</p>".to_owned())).render().into_string();
assert!(m.ends_with("<p>From <em>Markdown</em>.</p></div>"));
let doc = html! { h2 { "Install" } p { "Add the crate." } };
let m = ui.prose().body(doc.clone()).render().into_string();
assert_eq!(m, r#"<div class="lui-prose"><h2>Install</h2><p>Add the crate.</p></div>"#);
// The same in `lui!`, where the block is the content:
let same = lui! { Prose { h2 { "Install" } p { "Add the crate." } } };
assert_eq!(same.into_string(), m);
```
