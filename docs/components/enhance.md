# Enhancement script

`loco_ui::enhance`

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `fetch` | 42 | 39 | 10.1 |
| `history.pushState` | 5 | 4 | 5 |
| `document.startViewTransition` | 111 | 144 | 18 |
| `CustomEvent` | 15 | 11 | 6 |
| `HTML drag and drop` | 4 | 3.5 | 3.1 |

- Fallback: none needed: without the script every form and link is a normal navigation and every data-lui-* attribute is inert
- Needs script: no
- Responsive by: -
- Look after: -

## Documentation

One small optional script that makes the same markup feel instant. Every page works
identically with it blocked, disabled or not yet loaded: it only intercepts what the
browser would otherwise do as a full navigation and does it in place.

**Platform features:** `<script defer>` (baseline 2010), `fetch` (Chrome 42, Firefox 39,
Safari 10.1), `DOMParser` (baseline 2010), `history.pushState` (Chrome 5, Firefox 4,
Safari 5), `document.startViewTransition` where present (Chrome 111, Firefox 144,
Safari 18) so swapped parts morph when the answer took over 150 ms (a faster one lands at
once, a frame sooner; a root marked `data-lui-morph` always morphs), `CustomEvent` (Chrome 15, Firefox 11, Safari 6) for
`lui:swap`.

**Fallback:** none needed. Without the script every form and link is a normal navigation;
the Blitz test suite renders every route with no script engine at all.

**Contract:** an element with an `id` and `data-lui="swap"` is a swap root. Submitting a
form or following a same-origin link inside it fetches the response, parses it, and
replaces the root with the element of the same `id` from the new document. The flash
messages, the toast list, `<title>` and `data-theme` are synced too, and the URL follows the response.
A form or link anywhere may name its root instead with `data-lui-target="#id"`, and
`data-lui-swap="outer|inner|append|prepend"` (default `outer`) says how the new element
lands: replace the root, replace its children, or add them at the end or the start. The
request is the same either way, so the server may answer an enhanced request (headers
`Lui-Enhance: 1` and `Accept: text/html`) with only the fragment it needs to; without the
script it is a full navigation to the same URL. `slim` (feature `axum`) does that for every page: an
enhanced request gets the page without its inline stylesheet, which the document already
has, and every HTML answer says `Vary: Lui-Enhance, Cookie`. User actions fetch with
`priority: "high"`. Links inside an element marked `data-lui-prefetch` are fetched at low
priority on hover or focus, and a click within five seconds reuses that answer. A response may also carry elements marked `data-lui-oob`
(out of band): each replaces the element of the same `id` anywhere in the page, in the
mode the attribute names (`outer` by default), and is dropped from the main swap; the
full page without the script already shows them in place. While a request is in flight
the root and the form carry `data-lui-busy` and `aria-busy="true"`, the form's submit
buttons are disabled, and an element named by `data-lui-indicator="#id"` (authored with
`hidden`) is shown; the `--lui-busy` property sets how much the busy root fades. A request
that fails becomes the plain navigation the browser would have made, so the server's
answer is always seen. The URL follows the response (links push a history entry, forms
replace it); `data-lui-push="false"` keeps the URL as it is and `data-lui-replace` always
replaces. Every swap stores a copy of the roots in the history entry, so Back and Forward
restore them without a request. After every swap the root dispatches a bubbling `lui:swap`
event with `{ id, url, mode }` in `detail`.
Rapid actions on one root are queued, so a counter clicked five times counts five. The
script also mirrors `<input type=range>` and `type=color` values while they move, counts
characters into the `<output for>` of a field with `maxlength`, sends multipart forms as
`FormData` so files survive (through `XMLHttpRequest`, filling a `<progress data-lui-progress>`
inside the form as the upload goes out, when the form has one), opens
the `:target` dialog fallback as a real modal, moves through an open popover menu with the
arrow keys, searches a combobox as you type and walks its results with the arrow keys.
A form marked `data-lui="autosubmit"` (a menu's checkbox or radio item) is sent when one
of its controls changes, and `<html data-lui-script>` lets CSS hide the buttons that only
the script-less page needs.
After a swap, an element marked `autofocus` in the new markup takes the focus (a form sent
back with an error summary starts there, as it would on a page load); otherwise the
element that had the focus gets it back, and when that element is gone the focus moves to
the new root's `h1`, or the root itself (`tabindex="-1"`), instead of falling to `<body>`.
A swap that changed the URL or the title says the new `document.title` through a
`role="status"` region the script adds to the page, as a screen reader hears a page load.
A tab title opens its panel and slides the underline on the click itself; the server's
answer replaces the strip quietly once the slide ends (a lazy panel fills in then). A
sortable list's item drags by its grip (HTML drag and drop); the drop posts the item's
own move form with its new index. A table's "select all matching" box ticks the row
boxes on the page as well. A code block's copy button (`data-lui="copy"`, rendered
`hidden`) is shown where `navigator.clipboard` (Chrome 66, Firefox 63, Safari 13.1) can
write, and copies the block on a click. A number field's steppers (`data-lui="steppers"`)
call `stepUp()`/`stepDown()` in place instead of posting, and a password's reveal button
(`data-lui="reveal"`, `hidden` without the script: the page carries `data-lui-js` on
`<html>` once the script runs) switches the field between `password` and `text`.

`script_tag` goes at the end of `<body>`; `router` (feature `axum`) serves the file with a content
hash in the URL so it caches forever. It is compatible with `script-src 'self'`.

```rust
use loco_ui::enhance;
let tag = enhance::script_tag().into_string();
assert!(tag.starts_with("<script src=\"/lui/enhance.js?v="));
```
