# Capability beacons

`loco_ui::caps`

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `@supports` | 28 | 22 | 9 |
| `selector()` | 83 | 69 | 14.1 |
| `background images` | 1 | 1 | 1 |
| `cookies` | 1 | 1 | 1 |

- Fallback: unknown browser gets every fallback; the first view always does
- Needs script: no
- Responsive by: -
- Look after: -

## Documentation

Server-side feature detection with no script. The page carries a few empty beacon elements;
`@supports` rules give each one a background image only when the browser understands the
feature. Loading that image hits `/lui/caps?flag=<name>`, which sets a cookie. From the next
request on, the server knows what the browser can do and every component emits only the
markup that browser needs.

**Platform features:** `@supports` (Chrome 28, Firefox 22, Safari 9), `@supports
selector()` (Chrome 83, Firefox 69, Safari 14.1), CSS background images, cookies.

**Fallback:** a browser with no cookie yet gets `Caps::ASSUMED`, the features at baseline
in every engine for over two years, and the fallback markup for everything else. The
beacons fire while that first view loads, and the second view is tailored. Browsers that
never load CSS images (`curl`, readers) stay on `ASSUMED`.

**Finding:** CSS cannot test HTML attributes, so `invokers` and `streaming_dsd` are proxies
for CSS features that shipped in the same release. See `docs/caps.md` in the loco-ui repo.

One cookie per flag (`lui-cap-<name>=1`) rather than one cookie holding a list: the beacons
fire in parallel, and parallel `Set-Cookie` headers on one name would overwrite each other.

This crate is the detection half of `loco-ui` and stands on its own: any server that can
read a `Cookie:` header and answer one tiny route can use it.

**Any server.** Three plain functions are the whole protocol, and none needs Axum:
`Caps::from_cookie_header` reads the flags out of a `Cookie:` header,
`Caps::from_query` lets `?caps=popover,anchor` force a set (tests, `curl`, clients
without cookies), and `beacon_cookie` turns the beacon route's query string into the
`Set-Cookie` value to answer with (status 204, `Cache-Control: no-store`). The `axum`
feature only wraps them: a `Caps` extractor and `router` for the beacon route.

```rust
use loco_ui_caps::{self as caps, Cap, Caps};
let caps = Caps::from_cookie_header("lui-cap-probed=1; lui-cap-popover=1; theme=dark");
assert!(caps.has(Cap::Popover));
assert!(!caps.has(Cap::Invokers));
assert_eq!(Caps::all().names().len(), Cap::ALL.len());
// A query string overrides the cookies, so any URL can be viewed as any browser.
assert!(Caps::from_query("caps=invokers,anchor").unwrap().has(Cap::Invokers));
// The beacon route, by hand: `GET /lui/caps?flag=popover` answers 204 with this cookie.
assert!(caps::beacon_cookie("flag=popover", false).unwrap().starts_with("lui-cap-popover=1"));
assert_eq!(caps::beacon_cookie("flag=nope", false), None);
// Over HTTPS (here, behind a proxy that says so) the cookie is `Secure`.
let https = caps::is_https(None, Some("https"), None);
assert!(caps::beacon_cookie("flag=popover", https).unwrap().ends_with("; Secure"));
```
