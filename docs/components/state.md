# UI state

`loco_ui::state`

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `links` | 1 | 1 | 1 |
| `cookies` | 1 | 1 | 1 |
| `303 See Other` | 1 | 1 | 1 |

- Fallback: without cookies, state still travels in links on one page
- Needs script: no
- Responsive by: -
- Look after: -

## Documentation

Where UI state lives when there is no script: in the URL and in a cookie.

`UiState` is a small string map with five kinds of keys: `tab.<name>` (open tab index),
`open.<group>` (open accordion indexes, a comma list), `step.<wizard>` (current wizard step),
`per.<table>` (rows per page of a paged table) and `dialog` (id of a dialog to render open). It is
read from the query string first and a `lui-ui` cookie second, so a link can change one key
while everything else is remembered. In Axum it is an extractor, and returning it as part
of the response writes the cookie back when the query changed something.

**State or query.** A table's other keys (`q.<id>`, `q.<id>.<col>`, `sort.<id>`, `page.<id>`,
`cols.<id>`, `edit.<id>`) are named the same way but are not state: they describe one view
and live in the URL only, read with `ui.param` by the table (`crate::table::Keys`). Only
what a visitor would expect to find again on their next visit (a tab, an open section, a
wizard step, a page size) is remembered.

**One cookie for the site.** Keys are site-wide: two tab groups with one id share a
memory, so give each its own id. Every browser tab writes the same cookie, and the last
page view wins. The value stays under `MAX_COOKIE` bytes by dropping remembered keys this
request did not set. It is `SameSite=Lax`, `Secure` over HTTPS, and readable by page
script, since it holds nothing but view preferences.

**Platform features:** links, cookies, `303 See Other`. Nothing newer than 1997.

**Fallback:** none needed. Without cookies, state still travels in links on the same page.

**Post/Redirect/Get:** `crate::Ui::redirect` answers a form POST with a redirect and a
one-shot `lui-flash` cookie; the next page renders it with `ui.flash()` and, as a
`crate::Page`, clears it.

**Any server.** The protocol is plain strings: `UiState::from_request` reads path, query
and the `Cookie:` header; `UiState::set_cookies` gives the `Set-Cookie` values to send
back. The `axum` feature adds the extractor and the `IntoResponseParts` impl on top.

```rust
use loco_ui::UiState;
let state = UiState::parse("/settings", "tab.settings=1&page.files=3", "open.faq=2");
assert_eq!(state.tab("settings"), 1);
assert_eq!(state.open("faq"), Some(2));
assert_eq!(state.link("tab.settings", "0"), "/settings?open.faq=2&tab.settings=0");

// By hand, from a raw request: the cookie header carries both state and flash.
let state = UiState::from_request("/settings", "tab.settings=1", "lui-ui=open.faq=2; lui-flash=Saved.");
assert_eq!(state.flash(), Some("Saved."));
assert_eq!(state.set_cookies().len(), 2); // remember tab.settings, clear the flash
