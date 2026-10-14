//! # Aspect ratio
//!
//! A box that keeps its width-to-height ratio at every width: a video, a map, a picture whose
//! file may not have loaded yet, so nothing below it jumps when it does. After Radix Themes
//! AspectRatio: the ratio as a number (16 / 9, 4 / 3, 1), square by default, the child
//! stretched to fill it.
//!
//! **Platform features:** the `aspect-ratio` property (Chrome 88, Firefox 89, Safari 15),
//! set in a `style` attribute; `object-fit: cover` (Chrome 32, Firefox 36, Safari 10) crops a
//! picture to the box.
//!
//! **Accessibility:** layout only: a `<div>` with no role; an `<img>` inside keeps its own
//! `alt`. Checked by axe-core in headless Firefox on every demo route, both capability
//! variants, light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** nothing is missing.
//!
//! **Fallback:** a browser without `aspect-ratio` leaves the content in the flow (the child
//! is stretched over the box only under `@supports (aspect-ratio: 1 / 1)`), so the box takes
//! its content's height.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let m = ui.aspect_ratio().body(html! { img src="/a.jpg" alt="A lake"; }).render().into_string();
//! assert_eq!(m, r#"<div class="lui-aspect-ratio"><img src="/a.jpg" alt="A lake"></div>"#);
//! // The same in `lui!`, where the block is the body:
//! let same = lui! { AspectRatio { img src="/a.jpg" alt="A lake"; } };
//! assert_eq!(same.into_string(), m);
//! // Widescreen.
//! let m = ui.aspect_ratio().ratio(16, 9).body(html! { img src="/a.jpg" alt="A lake"; }).render().into_string();
//! assert!(m.starts_with(r#"<div class="lui-aspect-ratio" style="aspect-ratio: 16 / 9">"#));
//! assert_eq!(lui! { AspectRatio ratio=(16, 9) { img src="/a.jpg" alt="A lake"; } }.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::props::{Prop, PropKind};

/// A box of a fixed ratio, made by [`Ui::aspect_ratio`].
///
/// **Setters.** Values: `.body(..)`, `.ratio(..)`.
#[derive(Clone, Debug)]
pub struct AspectRatio {
    content: Markup,
    ratio: Option<(u32, u32)>,
}

impl AspectRatio {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("body", PropKind::Value, "markup: Markup")
            .doc("What fills the box: a picture, a video, a map."),
        Prop::new("ratio", PropKind::Value, "width: u32, height: u32")
            .default("1 / 1")
            .doc("Width to height: `(16, 9)`, `(4, 3)`."),
    ];
}

impl Ui {
    /// A square box whose content fills it; the content goes in [`AspectRatio::body`] and
    /// another shape comes from [`AspectRatio::ratio`].
    pub fn aspect_ratio(&self) -> AspectRatio {
        AspectRatio {
            content: Markup::default(),
            ratio: None,
        }
    }
}

impl AspectRatio {
    /// What fills the box: a picture, a video, a map.
    pub fn body(mut self, markup: Markup) -> Self {
        self.content = markup;
        self
    }

    /// Width to height: `(16, 9)`, `(4, 3)`; a zero is read as one.
    pub fn ratio(mut self, width: u32, height: u32) -> Self {
        self.ratio = Some((width.max(1), height.max(1)));
        self
    }
}

impl Render for AspectRatio {
    fn render(&self) -> Markup {
        let style = self.ratio.map(|(w, h)| format!("aspect-ratio: {w} / {h}"));
        html! { div class="lui-aspect-ratio" style=[style] { (self.content) } }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After Radix Themes AspectRatio: the ratio on the box, its child absolutely filling it. */
.lui-aspect-ratio { position: relative; width: 100%; aspect-ratio: 1 / 1; overflow: hidden; }
.lui-aspect-ratio > :is(img, video) { display: block; width: 100%; object-fit: cover; }
@supports (aspect-ratio: 1 / 1) {
  .lui-aspect-ratio > * { position: absolute; inset: 0; width: 100%; height: 100%; }
}
"#;
