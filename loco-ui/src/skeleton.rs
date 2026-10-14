//! # Skeleton
//!
//! Grey bars in the shape of content that is still coming: the placeholder for a streamed
//! `Ui::slot` (feature `http`), a lazy panel, anything the server fills in later. Besides
//! lines: `.circle()` puts an avatar's circle beside them, `.rect(w, h)` a block (an image,
//! a chart) above them, and `.avatar_lines(n)` makes `n` rows of a circle and two lines, a
//! list of people.
//!
//! **Platform features:** `aria-busy="true"` and `role="status"` with a visually hidden
//! label, so assistive technology hears "Loading" once instead of reading empty boxes; a
//! shimmer drawn with `@keyframes` over a `linear-gradient` that
//! `prefers-reduced-motion: reduce` stops.
//!
//! **Fallback:** without CSS animations the bars are still; nothing else differs.
//!
//! **Accessibility:** `role="status"` with `aria-busy` and a "Loading" name; the bars are
//! `aria-hidden`. Checked by axe-core in headless Firefox on every demo route, both capability
//! variants, light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** it cannot be removed by the client; it is replaced
//! when the real content arrives (a streamed slot, a swap, the next page).
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::from(Caps::all());
//! assert_eq!(ui.skeleton(3).render().into_string().matches("lui-skeleton-line").count(), 3);
//! let m = ui.skeleton(2).label("Loading orders").heading().render().into_string();
//! assert!(m.contains("Loading orders") && m.contains("lui-skeleton-heading"));
//! // The same in `lui!`:
//! let same = lui! { Skeleton(2) label="Loading orders" heading; };
//! assert_eq!(same.into_string(), m);
//! // A card: an image block, then a circle beside two lines.
//! let m = ui.skeleton(2).rect("100%", "8rem").circle().render().into_string();
//! assert!(m.contains(r#"<span class="lui-skeleton-rect" style="width: 100%; height: 8rem" aria-hidden="true"></span>"#));
//! assert!(m.contains(r#"<span class="lui-skeleton-circle" aria-hidden="true"></span>"#));
//! assert_eq!(lui! { Skeleton(2) rect=("100%", "8rem") circle; }.into_string(), m);
//! // Three people in a list.
//! let m = ui.skeleton(0).avatar_lines(3).render().into_string();
//! assert_eq!(m.matches("lui-skeleton-circle").count(), 3);
//! assert_eq!(lui! { Skeleton(0) avatar_lines=3; }.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::i18n::Text;
use crate::props::{Prop, PropKind};

/// Placeholder bars, made by [`Ui::skeleton`]; the last one is shorter, like the end of a
/// paragraph.
///
/// **Setters.** Values and items: `.label(..)`, `.rect(..)`, `.avatar_lines(..)`; switches:
/// `.heading()`, `.circle()`.
#[derive(Clone, Debug)]
pub struct Skeleton<'a> {
    lines: usize,
    label: &'a str,
    heading: bool,
    circle: bool,
    rect: Option<(&'a str, &'a str)>,
    avatar_lines: usize,
}

impl Skeleton<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("label", PropKind::Value, "label: &'a str")
            .default("Loading")
            .doc("What screen readers hear instead of \"Loading\"."),
        Prop::new("heading", PropKind::Switch, "")
            .doc("Start with a wider, taller bar standing in for a heading."),
        Prop::new("circle", PropKind::Switch, "")
            .doc("A 2.5rem circle beside the lines, standing in for an avatar."),
        Prop::new("rect", PropKind::Value, "width: &'a str, height: &'a str")
            .doc("A block of that CSS width and height above the lines (an image, a chart)."),
        Prop::new("avatar_lines", PropKind::Number, "n: usize")
            .default("0")
            .doc("`n` rows of a circle beside two lines, after the rest: a list of people."),
    ];
}

impl Ui {
    /// `lines` bars, announced as "Loading".
    pub fn skeleton<'a>(&self, lines: usize) -> Skeleton<'a> {
        Skeleton {
            lines,
            label: self.text(Text::Loading),
            heading: false,
            circle: false,
            rect: None,
            avatar_lines: 0,
        }
    }
}

impl<'a> Skeleton<'a> {
    /// What screen readers hear instead of "Loading".
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = label;
        self
    }

    /// Start with a wider, taller bar standing in for a heading.
    pub fn heading(mut self) -> Self {
        self.heading = true;
        self
    }

    /// A 2.5rem circle beside the lines, standing in for an avatar.
    pub fn circle(mut self) -> Self {
        self.circle = true;
        self
    }

    /// A block of that CSS `width` and `height` above the lines (`"100%", "8rem"`): an
    /// image, a chart.
    pub fn rect(mut self, width: &'a str, height: &'a str) -> Self {
        self.rect = Some((width, height));
        self
    }

    /// `n` rows of a circle beside two lines, after the rest: a list of people.
    pub fn avatar_lines(mut self, n: usize) -> Self {
        self.avatar_lines = n;
        self
    }
}

impl Render for Skeleton<'_> {
    fn render(&self) -> Markup {
        let bars = |lines: usize| {
            html! {
                @for i in 0..lines {
                    span class={ "lui-skeleton-line" @if i + 1 == lines && lines > 1 { " lui-skeleton-last" } } aria-hidden="true" {}
                }
            }
        };
        let text = html! {
            @if self.heading { span class="lui-skeleton-heading" aria-hidden="true" {} }
            (bars(self.lines))
        };
        html! {
            div class="lui-skeleton" role="status" aria-busy="true" {
                span class="lui-sr" { (self.label) }
                @if let Some((w, h)) = self.rect {
                    span class="lui-skeleton-rect" style={ "width: " (w) "; height: " (h) } aria-hidden="true" {}
                }
                @if self.circle {
                    div class="lui-skeleton-media" {
                        span class="lui-skeleton-circle" aria-hidden="true" {}
                        div class="lui-skeleton-lines" { (text) }
                    }
                } @else {
                    (text)
                }
                @for _ in 0..self.avatar_lines {
                    div class="lui-skeleton-media" {
                        span class="lui-skeleton-circle" aria-hidden="true" {}
                        div class="lui-skeleton-lines" { (bars(2)) }
                    }
                }
            }
        }
    }
}
/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After Radix Themes Skeleton: --lui-gray-4 blocks in the shape of the text they stand for
   (a heading, lines, the last one short), with a light sweeping across them; still under
   reduced motion. */
.lui-skeleton { display: grid; gap: calc(var(--lui-space) * 1.25); padding-block: var(--lui-space); }
.lui-skeleton-line, .lui-skeleton-heading, .lui-skeleton-rect, .lui-skeleton-circle {
  display: block; height: 1rem; border-radius: var(--lui-radius-sm);
  background: linear-gradient(90deg, var(--lui-gray-4) 0 35%, var(--lui-gray-3) 50%, var(--lui-gray-4) 65% 100%) 0 0 / 300% 100% var(--lui-gray-4);
  animation: lui-skeleton-shimmer 1.6s ease-in-out infinite;
}
.lui-skeleton-heading { height: 1.5rem; width: 45%; margin-bottom: calc(var(--lui-space) * 0.5); }
.lui-skeleton-last { width: 60%; }
.lui-skeleton-rect { max-width: 100%; border-radius: var(--lui-radius); }
.lui-skeleton-media { display: grid; grid-template-columns: 2.5rem minmax(0, 1fr); align-items: center; gap: var(--lui-space-3); }
.lui-skeleton-circle { width: 2.5rem; height: 2.5rem; border-radius: 9999px; }
.lui-skeleton-lines { display: grid; gap: calc(var(--lui-space) * 1.25); }
@keyframes lui-skeleton-shimmer { from { background-position: 100% 0; } to { background-position: 0 0; } }
@media (prefers-reduced-motion: reduce) { .lui-skeleton-line, .lui-skeleton-heading, .lui-skeleton-rect, .lui-skeleton-circle { animation: none; } }
"#;
