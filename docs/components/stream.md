# Streaming

`loco_ui::stream`

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<template shadowrootmode="open">` | 111 | 123 | 16.4 |
| `<slot name` | 53 | 63 | 10 |
| `Chunked transfer` | 1 | 1 | 1 |

- Fallback: in-order streaming with in-place splicing
- Needs script: no
- Responsive by: -
- Look after: Radix Themes Skeleton

## Documentation

Out-of-order streaming: the page is sent at once with placeholders, and each slow section
is appended to the response whenever it finishes, in any order. The browser moves each
chunk into its placeholder. No script.

**Platform features:**
- Declarative shadow DOM, `<template shadowrootmode="open">` (Chrome 111, Firefox 123,
  Safari 16.4). `<body>` becomes a shadow host whose shadow tree is the page, with a
  `<slot name="id">placeholder</slot>` per section.
- Named slots: a light-DOM child `<div slot="id">` of `<body>` replaces the placeholder the
  moment the parser sees it, wherever it arrives in the byte stream.
- Chunked transfer: the response body is a stream of HTML chunks. `Streamed::into_stream`
  is that stream for any server (`http` feature); the `axum` feature turns it into a
  response with `IntoResponse`.

**Accessibility:** the page is complete HTML in document order however the slots arrive;
placeholders are ordinary text until filled. Checked by axe-core in headless Firefox on
every demo route, both capability variants, light and dark (no serious or critical
violation).

**What it does not do without script:** fill slots out of order where declarative shadow DOM
streaming is missing; the fallback keeps document order.

**Fallback:** without `Caps::StreamingDsd`, `ui.slot` leaves an HTML comment marker and the
response is streamed *in document order*: the bytes up to the first marker go out at once,
then each section as soon as it and everything before it are ready. Every browser renders
that progressively; only the out-of-order part is lost.

**Finding:** shadow trees do not see document stylesheets, so the stylesheet is inlined
twice in DSD mode, once in `<head>` for the slotted chunks and once in the shadow tree.

```rust
use loco_ui::prelude::*;
let ui = Ui::from(Caps::all());
let page = ui.stream("Feed", html! {
    h1 { "Feed" }
    (ui.slot("news", html! { p { "Loading news…" } }))
})
.fill("news", async { html! { p { "Fresh news." } } });
assert!(page.out_of_order());

// The same slot in `lui!`:
let same = lui! { Slot("news", (html! { p { "Loading news…" } })); };
let slot = ui.slot("news", html! { p { "Loading news…" } });
assert_eq!(same.into_string(), slot.into_string());
```
