//! # Badge
//!
//! A short label beside something: a status, a count, a tag. After Radix Themes Badge: a tone
//! (the brand by default; `.secondary()` gray, `.danger()`, `.ok()`, `.warn()`, or any
//! [`Tone`] with `.tone(..)`) in a variant (soft by default: a tint with
//! darker text; `.solid()`, `.surface()`, `.outline()`, or `.variant(..)`), at Radix's size 1
//! unless `.size(..)` says 2 or 3, with no wrapping.
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
//! assert!(tag.starts_with(r#"<a class="lui-badge lui-badge-outline" data-variant="outline" href="/tags/rust">"#));
//! // Opt-in motion: a light sweeps across it.
//! let new = ui.badge("New").shimmer().render().into_string();
//! assert_eq!(new, r#"<span class="lui-badge lui-badge-shimmer">New</span>"#);
//! assert_eq!(lui! { Badge("New") shimmer; }.into_string(), new);
//! // Radix's other looks, in any tone.
//! let loud = ui.badge("Failed").danger().solid().render().into_string();
//! assert_eq!(loud, r#"<span class="lui-badge lui-badge-danger lui-badge-solid" data-tone="danger" data-variant="solid">Failed</span>"#);
//! assert_eq!(lui! { Badge("Failed") danger solid; }.into_string(), loud);
//! // The typed setters, written as strings in `lui!`.
//! let beta = ui.badge("Beta").tone(Tone::Info).variant(Variant::Surface).size(2).radius(Radius::Full);
//! let beta = beta.render().into_string();
//! assert!(beta.contains(r#"data-size="2" data-tone="info" data-variant="surface" data-radius="full""#));
//! assert_eq!(lui! { Badge("Beta") tone="info" variant="surface" size=2 radius="full"; }.into_string(), beta);
//! ```

use std::fmt::Display;

use maud::{Markup, Render, html};

use crate::Ui;
use crate::props::{Prop, PropKind};
use crate::variant::{Radius, Size, Tone, Variant};

/// A badge, made by [`Ui::badge`].
///
/// **Setters.** Values and items: `.href(..)`, `.size(..)`, `.tone(..)`, `.variant(..)`,
/// `.radius(..)`; switches: `.secondary()`, `.danger()`,
/// `.outline()`, `.ok()`, `.warn()`, `.solid()`, `.surface()`, `.shimmer()`.
#[derive(Clone, Debug)]
pub struct Badge<'a> {
    text: String,
    tone: Option<Tone>,
    variant: Option<Variant>,
    size: Size,
    radius: Option<Radius>,
    href: Option<&'a str>,
    shimmer: bool,
}

impl Badge<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [crate::props::Key] = &[];

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
        Prop::new(
            "size",
            PropKind::Choice { of: Size::NAMES },
            "size: impl Into<Size>",
        )
        .default("1")
        .doc("Radix's sizes 1 to 3: more padding, and larger text at 3."),
        Prop::new("tone", PropKind::Choice { of: Tone::NAMES }, "tone: Tone")
            .default("brand")
            .doc("The colour."),
        Prop::new(
            "variant",
            PropKind::Choice { of: Variant::NAMES },
            "variant: Variant",
        )
        .default("soft")
        .doc("How the tone is painted; ghost is the tone's text with no fill or border."),
        Prop::new(
            "radius",
            PropKind::Choice { of: Radius::NAMES },
            "radius: Radius",
        )
        .default("small")
        .doc("The corners, from square to a pill."),
    ];
}

impl Ui {
    /// A badge reading `text`, a `&str` or anything `Display` (a count): a soft brand tint
    /// unless told otherwise.
    pub fn badge<'a>(&self, text: impl Display) -> Badge<'a> {
        Badge {
            text: text.to_string(),
            tone: None,
            variant: None,
            size: Size::S1,
            radius: None,
            href: None,
            shimmer: false,
        }
    }
}

impl<'a> Badge<'a> {
    /// The colour: brand (the default), neutral, ok, warn, danger or info.
    pub fn tone(mut self, tone: Tone) -> Self {
        self.tone = Some(tone);
        self
    }

    /// How the tone is painted: soft (the default), solid, surface, outline, or ghost (the
    /// tone's text alone).
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = Some(variant);
        self
    }

    /// Radix's sizes 1 (the default) to 3: more padding, and larger text at 3.
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    /// The corners: none, small (the default), medium, large or full (a pill).
    pub fn radius(mut self, radius: Radius) -> Self {
        self.radius = Some(radius);
        self
    }

    /// A gray tone instead of the brand: a draft, a count.
    pub fn secondary(self) -> Self {
        self.tone(Tone::Neutral)
    }

    /// The `--lui-danger` tone: failed, overdue.
    pub fn danger(self) -> Self {
        self.tone(Tone::Danger)
    }

    /// A border and no fill, gray unless a tone is given (a tag).
    pub fn outline(self) -> Self {
        self.variant(Variant::Outline)
    }

    /// Filled with the tone, light text on it: the loudest look ("New").
    pub fn solid(self) -> Self {
        self.variant(Variant::Solid)
    }

    /// A pale tint inside a border of the tone.
    pub fn surface(self) -> Self {
        self.variant(Variant::Surface)
    }

    /// A tint of `--lui-ok`: done, paid, healthy.
    pub fn ok(self) -> Self {
        self.tone(Tone::Ok)
    }

    /// A tint of `--lui-warn`: pending, degraded.
    pub fn warn(self) -> Self {
        self.tone(Tone::Warn)
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
        class.push_str(match self.tone {
            Some(Tone::Neutral) => " lui-badge-secondary",
            Some(Tone::Danger) => " lui-badge-danger",
            Some(Tone::Ok) => " lui-badge-ok",
            Some(Tone::Warn) => " lui-badge-warn",
            Some(Tone::Brand | Tone::Info) | None => "",
        });
        class.push_str(match self.variant {
            Some(Variant::Solid) => " lui-badge-solid",
            Some(Variant::Surface) => " lui-badge-surface",
            Some(Variant::Outline) => " lui-badge-outline",
            Some(Variant::Soft | Variant::Ghost) | None => "",
        });
        if self.shimmer {
            class.push_str(" lui-badge-shimmer");
        }
        let size = (self.size != Size::S1).then(|| self.size.as_str());
        let tone = self.tone.map(Tone::as_str);
        let variant = self.variant.map(Variant::as_str);
        let radius = self.radius.map(Radius::as_str);
        html! {
            @if let Some(href) = self.href {
                a class=(class) data-size=[size] data-tone=[tone] data-variant=[variant] data-radius=[radius] href=(href) { (self.text) }
            } @else {
                span class=(class) data-size=[size] data-tone=[tone] data-variant=[variant] data-radius=[radius] { (self.text) }
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
  padding: 0.125rem 0.5rem; font-size: var(--lui-text-1); line-height: 1rem; font-weight: 500; text-decoration: none;
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
/* .tone(..) for the two tones without a class (brand also beats the neutral outline), the ghost
   variant (the tone's text alone), and Radix's sizes 2 and 3. */
.lui-badge:is([data-tone=brand], [data-tone=info]) { --lui-badge-tone: var(--lui-primary); --lui-badge-on: var(--lui-on-primary); }
.lui-badge[data-variant=ghost] { background: transparent; }
.lui-badge[data-size="2"] { padding: 0.25rem 0.5rem; }
.lui-badge[data-size="3"] { padding: 0.25rem 0.625rem; font-size: var(--lui-text-2); line-height: 1.25rem; }
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
