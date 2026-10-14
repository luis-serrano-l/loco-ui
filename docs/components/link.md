# Link

`loco_ui::link` · demo: `/toast`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Link` | `ui.link(text: &str, href: &str)` | beta |

## Props of `Link`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `underline` | choice: always, hover, none | `underline: Underline` | `always` |  | When the underline shows: always, on hover, or never. |
| `external` | switch |  | `off` |  | Opens in a new tab, with an arrow and a hint for screen readers. |
| `muted` | switch |  | `off` |  | The muted gray, the text colour on hover: a footer's or a caption's links. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<a href>` | 1 | 1 | 1 |
| `text-underline-offset` | 87 | 70 | 12.1 |
| `target="_blank"` | 1 | 1 | 1 |

- Fallback: none needed
- Needs script: no
- Responsive by: -
- Look after: Radix Themes Link

## Documentation

A styled `<a>`: in running text, in a footer, in a table cell, or out to another site with
an arrow and a hint that it opens a new tab. After Radix Themes Link: step 11 of the brand
scale, a thin underline offset below the baseline in a paler step, and the three
`underline` choices (always, on hover, never).

**Platform features:** `<a href>` (baseline 1995); `text-underline-offset` and
`text-decoration-thickness` (Chrome 87, Firefox 70, Safari 12.1); `.external()` adds
`target="_blank"` with `rel="noreferrer"` (which implies `noopener`).

**Accessibility:** a real link, so it is in the tab order and listed as one. Underlined by
default so it never relies on colour alone in a paragraph; an external link says "(opens
in a new tab)" to a screen reader and draws its arrow `aria-hidden`. Checked by axe-core
in headless Firefox on every demo route, both capability variants, light and dark (no
serious or critical violation).
A link in running text keeps the text's height under a coarse pointer: WCAG 2.5.8
exempts inline targets, and a 44px link would break the line spacing.

**What it does not do without script:** nothing is missing.

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let a = ui.link("Pricing", "/pricing").render().into_string();
assert_eq!(a, r#"<a class="lui-link" href="/pricing">Pricing</a>"#);
// The same in `lui!`:
let same = lui! { Link("Pricing", "/pricing"); };
assert_eq!(same.into_string(), a);
// Out to another site, muted, underlined only under the pointer.
let a = ui.link("Radix", "https://www.radix-ui.com").external().muted().underline(Underline::Hover);
let a = a.render().into_string();
assert!(a.starts_with(r#"<a class="lui-link lui-link-muted" data-underline="hover" href="https://www.radix-ui.com" target="_blank" rel="noreferrer">Radix"#));
assert!(a.contains(r#"<span class="lui-sr"> (opens in a new tab)</span>"#));
let same = lui! { Link("Radix", "https://www.radix-ui.com") external muted underline="hover"; };
assert_eq!(same.into_string(), a);
```
