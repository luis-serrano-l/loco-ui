# Banner

`loco_ui::banner` · demo: `/banner`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Banner` | `ui.banner(text: &str)` | beta |

## Props of `Banner`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `tone` | choice: neutral, brand, ok, warn, danger, info | `tone: Tone` | `neutral` |  | The bar's colour: neutral is the dark bar, the others a tint. |
| `link` | value | `label: &'a str, href: &'a str` |  |  | A link after the text, with an arrow. |
| `id` | value | `id: &'a str` |  |  | The id its dismiss cookie is named after instead of the text's slug. |
| `dismiss` | switch |  | `off` |  | A close button that keeps it closed with a `lui-dismiss-<id>` cookie. |

## Keyboard of `Banner`

| Keys | What they do | Needs the script |
|---|---|---|
| `Enter / Space` | Dismisses the banner from its close button. | no |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-banner` | `--lui-gray-12`, `--lui-gray-1`, `--lui-space-2`, `--lui-space-3`, `--lui-space-4`, `--lui-text-2`, `--lui-leading-2`, `--lui-banner-fg`, `--lui-banner-bg`, `--lui-brand-3`, `--lui-brand-11`, `--lui-ok`, `--lui-bg`, `--lui-fg`, `--lui-warn`, `--lui-danger` |
| `.lui-banner-text` |  |
| `.lui-banner-link` |  |
| `.lui-banner-dismiss` |  |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<aside>` | 1 | 1 | 1 |
| `cookie` | 1 | 1 | 1 |
| `<form method="post">` | 1 | 1 | 1 |

- Fallback: none needed
- Needs script: no
- Responsive by: container
- Look after: Origin UI Banner

## Documentation

A full-width bar across the top of the page for news that concerns the whole site: a
release, planned downtime, a trial running out. One line of text, an optional link, and an
optional close button that keeps it closed on every page from then on. After Origin UI
Banner: a dark bar by default (the neutral tone), or tinted in a `Tone`.

**Platform features:** an `<aside>` named by its text, and a `<form method="post">` (every browser) for
the close button. The form posts to `DISMISS_PATH`, which sets a `cookie`
`lui-dismiss-<id>` for a year and redirects back (Post/Redirect/Get); the next page reads
the cookie through `Ui::dismissed` and renders nothing. The `axum` feature serves that
route in `crate::enhance::router` (Loco's initializer mounts it too); another server
answers it with `dismiss_cookie`.

**Accessibility:** a complementary landmark named by its text; the close button is named
"Dismiss: &lt;text&gt;". Checked by axe-core in headless Firefox on every demo route, both
capability variants, light and dark (no serious or critical violation).

**What it does not do without script:** close in place: closing is a round trip (with the
enhancement script too, since the answer is a redirect).

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::from_request("/pricing", "", "");
let m = ui.banner("Version 2 is out.").render().into_string();
assert_eq!(m, r#"<aside class="lui-banner" aria-label="Version 2 is out."><p class="lui-banner-text">Version 2 is out.</p></aside>"#);
let m = ui.banner("Version 2 is out.").tone(Tone::Brand).link("Read the notes", "/changelog").dismiss().render().into_string();
assert!(m.contains(r#"action="/lui/dismiss""#) && m.contains(r#"name="id" value="version-2-is-out""#));
assert!(m.contains(r#"name="returns_to" value="/pricing""#) && m.contains(r#"href="/changelog""#));
// The same in `lui!`:
let same = lui! { Banner("Version 2 is out.") tone="brand" link=("Read the notes", "/changelog") dismiss; };
assert_eq!(same.into_string(), m);
// Once dismissed, the cookie hides it.
let ui = Ui::from_request("/pricing", "", "lui-dismiss-version-2-is-out=1");
assert_eq!(ui.banner("Version 2 is out.").dismiss().render().into_string(), "");
```
