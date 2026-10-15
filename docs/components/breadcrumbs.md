# Breadcrumbs

`loco_ui::breadcrumbs` · demo: `/nav`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Breadcrumbs` | `ui.breadcrumbs()` | stable |

## Props of `Breadcrumbs`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `link` | item | `label: &'a str, href: &'a str` |  |  | One step from the root towards this page. |
| `here` | item | `label: &'a str` |  |  | The current page, last in the trail and not a link. |
| `max` | number | `n: usize` | `4` |  | The most crumbs shown, the current page included, before the middle folds into a menu. |
| `separator` | value | `glyph: impl Into<Glyph<'a>>` |  |  | A glyph or an icon between the crumbs instead of the chevron. |

## Keyboard of `Breadcrumbs`

| Keys | What they do | Needs the script |
|---|---|---|
| `Enter / Space` | Shows or hides the folded middle crumbs from the ellipsis button. | no |
| `Enter` | Follows the focused crumb. | no |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-breadcrumbs` | `--lui-text-2`, `--lui-muted`, `--lui-space-4`, `--lui-duration-fast`, `--lui-fg` |
| `.lui-breadcrumbs-custom` |  |
| `.lui-breadcrumbs-sep` |  |
| `.lui-breadcrumbs-fold` | `--lui-radius-sm`, `--lui-accent`, `--lui-on-accent`, `--lui-z-popover`, `--lui-space-1`, `--lui-popover`, `--lui-line`, `--lui-radius`, `--lui-shadow-md`, `--lui-highlight`, `--lui-fg`, `--lui-hit` |
| `.lui-breadcrumbs-fold-narrow` |  |
| `.lui-breadcrumbs-mid` |  |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `aria-current="page"` | 1 | 1 | 1 |
| `::before` | 1 | 1 | 1 |
| `<details>` | 12 | 49 | 6 |

- Fallback: none needed
- Needs script: no
- Responsive by: container
- Look after: shadcn Breadcrumb

## Documentation

Where this page sits: a trail of links ending in the current page. A long trail folds its
middle into a disclosure so the ends stay readable on a phone.

**Platform features:** `<nav aria-label="Breadcrumb">` around an ordered list, the last
item marked `aria-current="page"`; separators drawn by CSS `::before` so screen readers do
not read them; the folded middle is a `<details>` element.

**Accessibility:** a `<nav>` named "Breadcrumb" with an ordered list; the current page has
`aria-current="page"`; folded steps sit in a named `<details>`. Checked by axe-core in
headless Firefox on every demo route, both capability variants, light and dark (no serious
or critical violation).

The look follows shadcn Breadcrumb: muted links, chevron separators, the current page in
the text colour. A trail of more than four (or `.max(n)`) folds its middle into a `…`
disclosure menu; a shorter one with a middle crumb folds it only when the breadcrumbs are
under 30rem wide (both are rendered; a container query shows one). `.separator(..)` puts
a glyph or an icon between the crumbs instead of the chevron, `aria-hidden`.

**What it does not do without script:** measure the text to fold exactly what does not fit;
the fold is decided by item count and the container's width.

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::from(Caps::all());
let m = ui.breadcrumbs().link("Home", "/").link("Projects", "/projects").here("loco-ui").render().into_string();
assert!(m.contains(r#"<a href="/projects">Projects</a>"#));
assert!(m.contains(r#"aria-current="page">loco-ui"#));
// More than four: the middle folds into <details>.
let long = ui.breadcrumbs().link("Home", "/").link("A", "/a").link("B", "/a/b").link("C", "/a/b/c").here("Here");
assert!(long.render().into_string().contains("<details"));
// One middle crumb: shown inline, and folded too for a narrow container.
let three = ui.breadcrumbs().link("Home", "/").link("A", "/a").link("B", "/a/b").here("Here").render().into_string();
assert!(three.contains("lui-breadcrumbs-mid") && three.contains("lui-breadcrumbs-fold-narrow"));
// The same in `lui!`:
let same = lui! { Breadcrumbs {
    link "Home" "/"; link "Projects" "/projects"; here "loco-ui";
} };
assert_eq!(same.into_string(), m);
// At most three crumbs before the middle folds, a slash between them.
let short = ui.breadcrumbs().max(3).separator("/").link("Home", "/").link("A", "/a").link("B", "/a/b").here("Here");
let short = short.render().into_string();
assert!(short.contains("<details") && short.contains(r#"<span class="lui-breadcrumbs-sep" aria-hidden="true">/</span>"#));
assert_eq!(lui! { Breadcrumbs max=3 separator="/" { link "Home" "/"; link "A" "/a"; link "B" "/a/b"; here "Here"; } }.into_string(), short);
```
