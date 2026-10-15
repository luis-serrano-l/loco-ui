# Blockquote

`loco_ui::blockquote` · demo: `/typography`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Blockquote` | `ui.blockquote()` | beta |

## Props of `Blockquote`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `body` | value | `markup: Markup` |  |  | The quotation. |
| `caption` | value | `source: &'a str` |  |  | Who said it, or the work it is from, in a `<cite>` under the quotation. |
| `cite` | value | `url: &'a str` |  | `cite` | The URL of the source, written as the blockquote's `cite` attribute. |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-blockquote` |  |
| `.lui-blockquote-quote` | `--lui-brand-6`, `--lui-fg` |
| `.lui-blockquote-caption` | `--lui-muted`, `--lui-text-2` |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<figure>` | 1 | 1 | 1 |
| `border-inline-start` | 87 | 66 | 14.1 |

- Fallback: none needed
- Needs script: no
- Responsive by: -
- Look after: Radix Themes Blockquote

## Documentation

A quotation set apart from the text around it, with the person or work it comes from
underneath: a testimonial, a passage from the docs, a line from a review. After Radix
Themes Blockquote: a bar in a pale step of the brand scale down the start edge, the text
indented past it at the size around it, and a muted caption.

**Platform features:** `<figure>`, `<blockquote>`, `<figcaption>` and `<cite>` (baseline
2011 for figure); `border-inline-start` (Chrome 87, Firefox 66, Safari 14.1) so the bar
moves to the right in a right-to-left page.

**Accessibility:** a `<blockquote>` inside a `<figure>` whose `<figcaption>` holds the
source, the pattern the HTML standard gives for a quotation with its attribution. Checked by
axe-core in headless Firefox on every demo route, both capability variants, light and dark
(no serious or critical violation).

**What it does not do without script:** nothing is missing.

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let q = ui.blockquote().body(html! { p { "Simple things should be simple." } }).render().into_string();
assert_eq!(q, r#"<figure class="lui-blockquote"><blockquote class="lui-blockquote-quote"><p>Simple things should be simple.</p></blockquote></figure>"#);
// The same in `lui!`, where the block is the quotation:
let same = lui! { Blockquote { p { "Simple things should be simple." } } };
assert_eq!(same.into_string(), q);
// With its source, as text and as a URL.
let q = ui.blockquote().caption("Alan Kay").cite("https://example.com/kay").body(html! { "Simple things should be simple." });
let q = q.render().into_string();
assert_eq!(q, r#"<figure class="lui-blockquote"><blockquote class="lui-blockquote-quote" cite="https://example.com/kay">Simple things should be simple.</blockquote><figcaption class="lui-blockquote-caption"><cite>Alan Kay</cite></figcaption></figure>"#);
let same = lui! { Blockquote caption="Alan Kay" cite="https://example.com/kay" { "Simple things should be simple." } };
assert_eq!(same.into_string(), q);
```
