# Carousel

`loco_ui::carousel` · demo: `/carousel`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Carousel` | `ui.carousel(label: &str)` | beta |

## Props of `Carousel`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `item` | item | `markup: Markup` |  |  | A slide. |
| `id` | value | `id: &'a str` |  |  | The prefix of the slides' ids (`<id>-<n>`) instead of the label's slug. |

## Keyboard of `Carousel`

| Keys | What they do | Needs the script |
|---|---|---|
| `Tab / Shift+Tab` | Moves focus to the slide strip, then to the slide buttons and the dots. | no |
| `ArrowLeft / ArrowRight` | Scrolls the focused strip to the previous or next slide. | no |
| `Enter` | Goes to the slide the focused button or dot names. | no |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-carousel` | `--lui-space-3` |
| `.lui-carousel-viewport` | `--lui-space-4`, `--lui-radius-lg`, `--lui-ring`, `--lui-text-3`, `--lui-fg`, `--lui-bg`, `--lui-line`, `--lui-shadow-sm`, `--lui-hit`, `--lui-space-2`, `--lui-space-3` |
| `.lui-carousel-item` | `--lui-space-8`, `--lui-gray-6`, `--lui-fg` |
| `.lui-carousel-prev` | `--lui-bg`, `--lui-space-2`, `--lui-space-4` |
| `.lui-carousel-next` | `--lui-bg`, `--lui-space-2`, `--lui-space-4` |
| `.lui-carousel-dots` | `--lui-space-2` |
| `.lui-carousel-dot` | `--lui-gray-6`, `--lui-gray-9`, `--lui-hit` |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `scroll-snap-type` | 69 | 68 | 11 |
| `::scroll-button()` | 135 | no | no |
| `::scroll-marker` | 135 | no | no |

- Fallback: fragment links to each slide's id stand in for the scroll buttons and markers
- Needs script: no
- Responsive by: container
- Look after: shadcn/ui Carousel

## Documentation

Slides side by side in a row that scrolls sideways and snaps to each one, with previous
and next buttons and a dot per slide. After shadcn/ui Carousel: full-width slides, round
outline buttons beside them, small dots under them; rebuilt on CSS scroll snapping instead
of a script.

**Platform features:**
- `scroll-snap-type` (Chrome 69, Firefox 68, Safari 11) and `scroll-behavior: smooth` on
  a scroll container that takes the keyboard (`tabindex="0"`): swipe, trackpad, arrow keys
  and Shift+wheel all move it a slide at a time.
- Where the browser has them, `::scroll-button()` (Chrome 135; not yet in Firefox or
  Safari) draws the previous and next buttons and `::scroll-marker` with
  `scroll-marker-group` (Chrome 135) the dots, both under `@supports`; their names come
  from `data-prev` and `data-next` on the scroller through `attr()`, so they are translated.
  Spec: <https://drafts.csswg.org/css-overflow-5/>.
- Elsewhere, plain links: each slide has an `id` (`<id>-<n>`) and carries a previous and
  a next link to its neighbours' fragments, and a row of links under the row is the dots.

**Accessibility:** a `<section>` with `aria-roledescription="carousel"` named by its
label; each slide a `role="group"` "slide" named "Slide n of N". The fallback links are
named the same way. Checked by axe-core in headless Firefox on every demo route, both
capability variants, light and dark (no serious or critical violation).

**What it does not do without script:** play on its own, loop from the last slide to the
first, or mark the current dot in the link fallback (the `::scroll-marker` dots show it
with `:target-current`); a fallback link scrolls the page to the row if it is out of view.

**Fallback:** without `::scroll-button()` and `::scroll-marker` (Firefox, Safari today),
the fragment links above; without scroll snapping, a row that scrolls freely.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.carousel("Photos").item(html! { "One" }).item(html! { "Two" }).render().into_string();
assert!(m.starts_with(r#"<section class="lui-carousel" aria-roledescription="carousel" aria-label="Photos">"#));
assert!(m.contains(r#"id="photos-2" class="lui-carousel-item" role="group" aria-roledescription="slide" aria-label="Slide 2 of 2""#));
assert!(m.contains(r##"href="#photos-1""##) && m.contains(r##"href="#photos-2""##));
// The same in `lui!`:
let same = lui! { Carousel("Photos") { item (html! { "One" }); item (html! { "Two" }); } };
assert_eq!(same.into_string(), m);
let m = ui.carousel("Photos").id("trip").item(html! { "One" }).render().into_string();
assert!(m.contains(r#"id="trip-1""#));
assert_eq!(lui! { Carousel("Photos") id="trip" { item (html! { "One" }); } }.into_string(), m);
```
