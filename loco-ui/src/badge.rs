//! # Badge
//!
//! A short label beside something: a status, a count, a tag. After Radix Themes Badge: a tone
//! (the brand by default; `.secondary()` gray, `.danger()`, `.ok()`, `.warn()`) in a look (soft
//! by default: a tint with darker text; `.solid()`, `.surface()` or `.outline()`), at Radix's
//! size 1 with no wrapping.
//!
//! **Platform features:** a `<span>` (or an `<a>` with `.href()`); nothing interactive of its
//! own. `.shimmer()` sweeps a light across it, as on a button: an `::after` layer moved with
//! the `translate` property (Chrome 104, Firefox 72, Safari 14.1) under
//! `@supports (translate: 100%)` and `prefers-reduced-motion: no-preference`, in
//! `--lui-shimmer` over a filled badge and `--lui-shimmer-surface` over a tinted or outlined one.
//!
//! **Accessibility:** plain text in a `<span>`; the tone colours are mixed with the text colour
//! so they pass AA on their tint. Checked by axe-core in headless Firefox on every demo route,
//! both capability variants, light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** nothing is missing.
//!
//! **Fallback:** none needed. A `.shimmer()` badge without `translate`, or under
//! `prefers-reduced-motion: reduce`, is the same badge at rest.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! assert_eq!(ui.badge("New").render().into_string(), r#"<span class="lui-badge">New</span>"#);
//! // The same in `lui!`:
//! let same = lui! { Badge("New"); };
//! assert_eq!(same.into_string(), ui.badge("New").render().into_string());
//! let paid = ui.badge("Paid").ok().render().into_string();
//! assert!(paid.contains("lui-badge lui-badge-ok"));
//! let tag = ui.badge("rust").outline().href("/tags/rust").render().into_string();
//! assert!(tag.starts_with(r#"<a class="lui-badge lui-badge-outline" href="/tags/rust">"#));
//! // Opt-in motion: a light sweeps across it.
//! let new = ui.badge("New").shimmer().render().into_string();
//! assert_eq!(new, r#"<span class="lui-badge lui-badge-shimmer">New</span>"#);
//! assert_eq!(lui! { Badge("New") shimmer; }.into_string(), new);
//! // Radix's other looks, in any tone.
//! let loud = ui.badge("Failed").danger().solid().render().into_string();
//! assert_eq!(loud, r#"<span class="lui-badge lui-badge-danger lui-badge-solid">Failed</span>"#);
//! assert_eq!(lui! { Badge("Failed") danger solid; }.into_string(), loud);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::props::{Prop, PropKind};

/// A badge, made by [`Ui::badge`].
///
/// **Setters.** Values and items: `.href(..)`; switches: `.secondary()`, `.danger()`,
/// `.outline()`, `.ok()`, `.warn()`, `.solid()`, `.surface()`, `.shimmer()`.
#[derive(Clone, Debug)]
pub struct Badge<'a> {
    text: &'a str,
    tone: Option<&'static str>,
    look: Option<&'static str>,
    href: Option<&'a str>,
    shimmer: bool,
}

impl Badge<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("secondary", PropKind::Switch, "").doc("A gray tone instead of the brand."),
        Prop::new("danger", PropKind::Switch, "").doc("The `--lui-danger` tone."),
        Prop::new("outline", PropKind::Switch, "").doc("A border and no fill (gray unless toned)."),
        Prop::new("ok", PropKind::Switch, "").doc("The `--lui-ok` tone."),
        Prop::new("warn", PropKind::Switch, "").doc("The `--lui-warn` tone."),
        Prop::new("solid", PropKind::Switch, "").doc("Filled with the tone, light text on it."),
        Prop::new("surface", PropKind::Switch, "").doc("A pale tint inside a border of the tone."),
        Prop::new("href", PropKind::Value, "href: &'a str")
            .attr("href")
            .doc("Make the badge a link."),
        Prop::new("shimmer", PropKind::Switch, "")
            .doc("A light sweeps across the badge, over its own fill."),
    ];
}

impl Ui {
    /// A badge reading `text`: a soft brand tint unless told otherwise.
    pub fn badge<'a>(&self, text: &'a str) -> Badge<'a> {
        Badge {
            text,
            tone: None,
            look: None,
            href: None,
            shimmer: false,
        }
    }
}

impl<'a> Badge<'a> {
    fn tone(mut self, tone: &'static str) -> Self {
        self.tone = Some(tone);
        self
    }

    fn look(mut self, look: &'static str) -> Self {
        self.look = Some(look);
        self
    }

    /// A gray tone instead of the brand: a draft, a count.
    pub fn secondary(self) -> Self {
        self.tone("lui-badge-secondary")
    }

    /// The `--lui-danger` tone: failed, overdue.
    pub fn danger(self) -> Self {
        self.tone("lui-badge-danger")
    }

    /// A border and no fill, gray unless a tone is given (a tag).
    pub fn outline(self) -> Self {
        self.look("lui-badge-outline")
    }

    /// Filled with the tone, light text on it: the loudest look ("New").
    pub fn solid(self) -> Self {
        self.look("lui-badge-solid")
    }

    /// A pale tint inside a border of the tone.
    pub fn surface(self) -> Self {
        self.look("lui-badge-surface")
    }

    /// A tint of `--lui-ok`: done, paid, healthy.
    pub fn ok(self) -> Self {
        self.tone("lui-badge-ok")
    }

    /// A tint of `--lui-warn`: pending, degraded.
    pub fn warn(self) -> Self {
        self.tone("lui-badge-warn")
    }

    /// Make the badge a link.
    pub fn href(mut self, href: &'a str) -> Self {
        self.href = Some(href);
        self
    }

    /// A light sweeps across the badge, over its own fill: "New" beside a feature. At rest
    /// without `translate` or under `prefers-reduced-motion: reduce`.
    pub fn shimmer(mut self) -> Self {
        self.shimmer = true;
        self
    }
}

impl Render for Badge<'_> {
    fn render(&self) -> Markup {
        let mut class = String::from("lui-badge");
        for part in [self.tone, self.look].into_iter().flatten() {
            class.push(' ');
            class.push_str(part);
        }
        if self.shimmer {
            class.push_str(" lui-badge-shimmer");
        }
        html! {
            @if let Some(href) = self.href {
                a class=(class) href=(href) { (self.text) }
            } @else {
                span class=(class) { (self.text) }
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After Radix Themes Badge (size 1): 12px medium text, 2px by 8px padding, the small radius,
   no wrapping. The tone is a custom property (brand by default; secondary is --lui-gray-11) and the
   look paints with it: soft (the default) a tint with the tone mixed into the text colour, so
   it reads AA on its tint; solid the tone with light text; surface a paler tint in a border;
   outline a border only, --lui-gray-11 unless a tone is given. */
.lui-badge {
  --lui-badge-tone: var(--lui-primary); --lui-badge-on: var(--lui-on-primary);
  display: inline-flex; align-items: center; gap: 0.25rem; width: fit-content; white-space: nowrap;
  padding: 0.125rem 0.5rem; font-size: 0.75rem; line-height: 1rem; font-weight: 500; text-decoration: none;
  border: 1px solid transparent; border-radius: var(--lui-radius-sm);
  background: color-mix(in srgb, var(--lui-badge-tone) 14%, transparent); color: color-mix(in srgb, var(--lui-badge-tone) 72%, var(--lui-fg));
  transition: background-color var(--lui-duration-fast);
}
.lui-badge:where(.lui-badge-outline) { --lui-badge-tone: var(--lui-gray-11); }
.lui-badge.lui-badge-secondary { --lui-badge-tone: var(--lui-gray-11); }
.lui-badge.lui-badge-danger { --lui-badge-tone: var(--lui-danger); --lui-badge-on: var(--lui-gray-1); }
.lui-badge.lui-badge-ok { --lui-badge-tone: var(--lui-ok); --lui-badge-on: var(--lui-gray-1); }
.lui-badge.lui-badge-warn { --lui-badge-tone: var(--lui-warn); --lui-badge-on: var(--lui-gray-1); }
.lui-badge.lui-badge-solid { background: var(--lui-badge-tone); color: var(--lui-badge-on); }
.lui-badge.lui-badge-surface { background: color-mix(in srgb, var(--lui-badge-tone) 7%, transparent); border-color: color-mix(in srgb, var(--lui-badge-tone) 35%, transparent); }
.lui-badge.lui-badge-outline { background: transparent; border-color: color-mix(in srgb, var(--lui-badge-tone) 45%, transparent); }
a.lui-badge:hover { background: color-mix(in srgb, var(--lui-badge-tone) 22%, transparent); }
a.lui-badge.lui-badge-solid:hover { background: color-mix(in srgb, var(--lui-badge-tone) 88%, var(--lui-fg)); }
.lui-badge .lui-icon { width: 0.75rem; height: 0.75rem; }
/* .shimmer(): the button's sweep (@keyframes lui-shimmer in button.rs) over the badge's fill. */
.lui-badge.lui-badge-shimmer { --lui-sweep: var(--lui-shimmer-surface); }
.lui-badge.lui-badge-shimmer.lui-badge-solid { --lui-sweep: var(--lui-shimmer); }
@media (prefers-reduced-motion: no-preference) {
  @supports (translate: 100%) {
    .lui-badge.lui-badge-shimmer { position: relative; overflow: hidden; isolation: isolate; }
    .lui-badge-shimmer::after {
      content: ""; position: absolute; inset: 0; pointer-events: none;
      background: linear-gradient(110deg, transparent 25%, var(--lui-sweep) 50%, transparent 75%);
      translate: -100% 0; animation: lui-shimmer var(--lui-shimmer-duration) ease-in-out infinite;
    }
  }
}
"#;
