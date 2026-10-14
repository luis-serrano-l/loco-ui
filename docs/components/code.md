# Code

`loco_ui::code` · demo: `/otp`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Code` | `ui.code(text: &str)` | beta |
| `CodeBlock` | `ui.code_block(text: &str)` | beta |

## Props of `Code`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `tone` | choice: neutral, brand, ok, warn, danger, info | `tone: Tone` | `neutral` |  | The colour of the tint and the text. |
| `variant` | choice: solid, soft, surface, outline, ghost | `variant: Variant` | `soft` |  | A tint (soft), a fill (solid), a tint in a border (surface), a border (outline) or the text alone (ghost). |

## Props of `CodeBlock`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `language` | value | `language: &'a str` |  |  | The language, written as `class="language-…"` for a highlighter. |
| `title` | value | `title: &'a str` |  |  | A file name or caption above the code; it names the region too. |
| `id` | value | `id: &'a str` |  | `id` | Replace the id derived from the title (or the code). |
| `copy` | switch |  | `off` |  | A copy button in the corner, shown and wired by the enhancement script only. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<pre>` | 1 | 1 | 1 |
| `overflow-x: auto` | 1 | 1 | 1 |
| `tab-size` | 21 | 91 | 7 |
| `navigator.clipboard` | 66 | 63 | 13.1 |

- Fallback: select the text: the copy button is hidden until the script shows it
- Needs script: partly: the copy button needs script; without it the code is selected by hand
- Responsive by: content
- Look after: Radix Themes Code, shadcn/ui docs code block

## Documentation

Code in two shapes: `Ui::code` for a name or a command inside a sentence, and
`Ui::code_block` for lines of it, kept as written and scrolled sideways inside their
own box. After Radix Themes Code (inline: the monospace font at 0.95em on a soft tint of
the tone, or solid, surface, outline or ghost) and the shadcn/ui docs' code block (a
bordered panel, an optional file name above, a copy button in the corner).

The library does no syntax highlighting, so it stays free of dependencies: `.language("rust")`
writes `class="language-rust"` on the `<code>`, which any highlighter (a server-side one
such as syntect, as the demo uses, or highlight.js) picks up.

**Platform features:** `<code>` and `<pre>` (baseline 1995); `overflow-x: auto` so long
lines scroll inside the block, never the page; `tab-size` (Chrome 21, Firefox 91,
Safari 7). `.copy()` adds a button for the enhancement script, which writes the code with
`navigator.clipboard` (Chrome 66, Firefox 63, Safari 13.1).

**Accessibility:** the block is a `role="region"` named by its title, or by "Code", with
`tabindex="0"` so a keyboard can scroll a long line. The copy button is a real button
reading "Copy", then "Copied" for two seconds. Checked by axe-core in headless Firefox on
every demo route, both capability variants, light and dark (no serious or critical
violation).

**What it does not do without script:** copy. The button is rendered `hidden` and only the
enhancement script shows it, so a page without script has no button that does nothing.

**Fallback:** select the text: the code is plain text in the page.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let c = ui.code("cargo add loco-ui").render().into_string();
assert_eq!(c, r#"<code class="lui-code">cargo add loco-ui</code>"#);
// The same in `lui!`:
let same = lui! { Code("cargo add loco-ui"); };
assert_eq!(same.into_string(), c);
let c = ui.code("Ok(())").tone(Tone::Brand).variant(Variant::Outline).render().into_string();
assert_eq!(c, r#"<code class="lui-code" data-tone="brand" data-variant="outline">Ok(())</code>"#);
assert_eq!(lui! { Code("Ok(())") tone="brand" variant="outline"; }.into_string(), c);
// A block with a file name and a copy button (shown by the script only).
let block = ui.code_block("[dependencies]\nloco-ui = \"0.2\"").language("toml").title("Cargo.toml").copy();
let block = block.render().into_string();
assert!(block.contains(r#"<pre class="lui-code-block-pre" id="code-cargo-toml" tabindex="0" role="region" aria-label="Cargo.toml"><code class="lui-code-block-code language-toml">[dependencies]"#));
assert!(block.contains(r#"data-lui="copy" hidden"#));
let same = lui! { CodeBlock("[dependencies]\nloco-ui = \"0.2\"") language="toml" title="Cargo.toml" copy; };
assert_eq!(same.into_string(), block);
```
