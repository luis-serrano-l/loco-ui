# Dialog

`loco_ui::dialog` · demo: `/dialog`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Dialog` | `ui.dialog(trigger: &str)` | stable |

## Props of `Dialog`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `body` | value | `body: Markup` |  |  | What the dialog says. |
| `id` | value | `id: &str` |  | `id` | The dialog's id instead of the trigger's slug. |
| `open` | condition | `open: bool` | `off` | `open` | Render the dialog already open (non-modal, no backdrop). |
| `title` | value | `title: &'a str` |  |  | A title in a header with a close control. |
| `size` | value | `size: u8` | `2` |  | Radix's sizes 1–4: 20, 32, 42 or 56 rem wide, roomier padding as it grows. |
| `small` | switch |  | `off` |  | `.size(1)`: 20 rem wide. |
| `large` | switch |  | `off` |  | `.size(3)`: 42 rem wide. |
| `danger` | switch |  | `off` |  | A red solid confirm button. |
| `confirm` | value | `label: &'a str, action: &'a str` |  |  | A confirm button labelled `label`. |
| `returns_to` | value | `path: &'a str` |  |  | Where the server should send the browser after the confirm form, posted as a hidden `returns_to` field. |
| `close` | value | `label: &'a str` | `Close` |  | Label of the closing button when there is no confirm form. |
| `cancel` | value | `label: &'a str` | `Cancel` |  | Label of the cancel button beside the confirm button. |
| `closedby` | value | `closedby: &'a str` | `any` | `closedby` | The `closedby` attribute. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<dialog>` | 37 | 98 | 15.4 |
| `command="show-modal"` | 135 | 144 | 26.2 |
| `<form method="dialog">` | 37 | 98 | 15.4 |
| `closedby` | 134 | 141 | 26 |
| `@starting-style` | 117 | 129 | 17.5 |
| `transition-behavior: allow-discrete` | 117 | 129 | 17.4 |

- Fallback: link to #id opens it through a :target rule, chosen server-side; the confirm footer is a plain form either way
- Needs script: no
- Responsive by: viewport
- Look after: Radix Themes Dialog and AlertDialog

## Documentation

A modal opened by a button and closed by a button, with no script. It can carry a title
with a close control, a size (Radix's 1–4), a `danger` variant, and a footer that is a real form: a
confirm button posting to a URL beside a cancel button, so "Delete account?" is one round
trip and the server redirects back to the page that opened it.

**Platform features:**
- `<dialog>` element (baseline 2022), its `::backdrop`, and `closedby` (Chrome 134+,
  Firefox 141+, Safari 26+) for Escape and light dismiss.
- Invoker commands: `<button command="show-modal" commandfor="id">` and `command="close"`
  (Chrome 135+, Firefox 144+, Safari 26.2+). Closing without a confirm form uses
  `<form method="dialog">` (baseline 2022).
- Motion: `@starting-style` (Chrome 117, Firefox 129, Safari 17.5) and
  `transition-behavior: allow-discrete` on `display` and `overlay` (Chrome 117,
  Firefox 129, Safari 17.4) fade and scale the dialog and its backdrop in and out; older
  browsers open and close it at once, and `prefers-reduced-motion: reduce` zeroes it.
- Focus: the close control sits last in the markup, so the dialog's own focusing steps land
  on the first field in the body, then on the confirm button.

**Accessibility:** a native `<dialog>` opened modally: focus moves in, Escape closes, the
page behind is inert; named by its title through `aria-labelledby`. Checked by axe-core in
headless Firefox on every demo route, both capability variants, light and dark (no serious
or critical violation).

**What it does not do without script:** return focus to the opener in the `:target`
fallback, trap focus there, or post a form and close without reloading.

**Fallback:** when `Caps` lacks `Invokers`, the opener is a link to `#id` and a `:target`
rule shows the dialog as a fixed overlay; links to `#` close and cancel it. Only one variant
is ever in the markup.

**Server state:** `?dialog=<id>` in the URL renders the dialog already open (non-modal, no
backdrop), as does `.open(true)`. The confirm form goes back to the page it came from
unless `returns_to` says otherwise. `returns_to` is posted
with the confirm form as a hidden `returns_to` field, so the handler knows where to send
the browser back; check it is a local path before redirecting to it.

**Without script:** `closedby` is honoured by the real `<dialog>` only; the `:target`
fallback closes through its links alone. Focus is not trapped in the fallback.

```rust
use loco_ui::prelude::*;
let ui = Ui::from(Caps::all());
let hi = ui.dialog("Say hi").body(html! { p { "Hello." } });
assert!(hi.render().into_string().contains(r#"commandfor="say-hi""#), "the id is the trigger's slug");

let m = ui.dialog("Delete account")
    .id("confirm")
    .title("Delete account?")
    .size(1)
    .danger()
    .confirm("Delete", "/account/delete")
    .returns_to("/settings")
    .cancel("Keep it")
    .closedby("closerequest")
    .open(true)
    .body(html! {
        p { "This cannot be undone." }
        label { "Reason " input name="reason"; }
    });
let html = m.render().into_string();
assert!(html.contains("<dialog id=\"confirm\" class=\"lui-dialog-sm\" closedby=\"closerequest\" aria-labelledby=\"confirm-title\" open>"));
assert!(html.contains("<form method=\"post\" action=\"/account/delete\""));
assert!(html.contains("name=\"returns_to\" value=\"/settings\""));
// The same in `lui!`:
let same = lui! { Dialog("Delete account") id="confirm" title="Delete account?" size=1 danger
    confirm=("Delete", "/account/delete") returns_to="/settings" cancel="Keep it"
    closedby="closerequest" open=(true) {
    p { "This cannot be undone." }
    label { "Reason " input name="reason"; }
} };
assert_eq!(same.into_string(), html);

// From the request: open when the URL says `?dialog=confirm`, and the confirm form
// returns to this page.
let ui = Ui::from_request("/account", "dialog=confirm", "");
let html = ui.dialog("Delete account").id("confirm").confirm("Delete", "/account/delete").render().into_string();
assert!(html.contains(" open>") && html.contains("value=\"/account\""));

// Radix's four sizes; `.small()` and `.large()` are sizes 1 and 3.
let wide = ui.dialog("Terms").size(4).render().into_string();
assert!(wide.contains(r#"class="lui-dialog-xl""#));
assert_eq!(lui! { Dialog("Terms") size=4; }.into_string(), wide);
```
