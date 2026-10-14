# loco-ui-macros

The `lui!` macro for [loco-ui](https://github.com/luis-serrano-l/loco-ui): Maud's `html!`
with components written like elements. Plain Maud passes through untouched; a capitalized
name is a component and expands to the builder chain a route would write by hand.

```rust,ignore
use loco_ui::prelude::*;

let page = lui! {
    h1 { "Projects" }
    Tabs("projects") vertical {
        tab "Site" badge=3 { p { "Open issues: 3" } }
        tab "App" { p { "No open issues." } }
    }
};
```

Do not depend on this crate directly: `loco-ui` re-exports the macro as `loco_ui::lui` and in
its prelude. The rules (attributes as setters, items, `@for` among items) are in the
[crate docs](https://docs.rs/loco-ui-macros).

MIT licence, see `LICENSE`.
