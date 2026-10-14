//! # Link
//!
//! A styled `<a>`: in running text, in a footer, in a table cell, or out to another site with
//! an arrow and a hint that it opens a new tab. After Radix Themes Link: step 11 of the brand
//! scale, a thin underline offset below the baseline in a paler step, and the three
//! `underline` choices (always, on hover, never).
//!
//! **Platform features:** `<a href>` (baseline 1995); `text-underline-offset` and
//! `text-decoration-thickness` (Chrome 87, Firefox 70, Safari 12.1); `.external()` adds
//! `target="_blank"` with `rel="noreferrer"` (which implies `noopener`).
//!
//! **Accessibility:** a real link, so it is in the tab order and listed as one. Underlined by
//! default so it never relies on colour alone in a paragraph; an external link says "(opens
//! in a new tab)" to a screen reader and draws its arrow `aria-hidden`. Checked by axe-core
//! in headless Firefox on every demo route, both capability variants, light and dark (no
//! serious or critical violation).
//! A link in running text keeps the text's height under a coarse pointer: WCAG 2.5.8
//! exempts inline targets, and a 44px link would break the line spacing.
//!
//! **What it does not do without script:** nothing is missing.
//!
//! **Fallback:** none needed.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let a = ui.link("Pricing", "/pricing").render().into_string();
//! assert_eq!(a, r#"<a class="lui-link" href="/pricing">Pricing</a>"#);
//! // The same in `lui!`:
//! let same = lui! { Link("Pricing", "/pricing"); };
//! assert_eq!(same.into_string(), a);
//! // Out to another site, muted, underlined only under the pointer.
//! let a = ui.link("Radix", "https://www.radix-ui.com").external().muted().underline(Underline::Hover);
//! let a = a.render().into_string();
//! assert!(a.starts_with(r#"<a class="lui-link lui-link-muted" data-underline="hover" href="https://www.radix-ui.com" target="_blank" rel="noreferrer">Radix"#));
//! assert!(a.contains(r#"<span class="lui-sr"> (opens in a new tab)</span>"#));
//! let same = lui! { Link("Radix", "https://www.radix-ui.com") external muted underline="hover"; };
//! assert_eq!(same.into_string(), a);
//! ```

use maud::{Markup, Render, html};

use crate::i18n::Text;
use crate::props::{Prop, PropKind};
use crate::variant::Underline;
use crate::{Icon, Ui};

/// A link, made by [`Ui::link`].
///
/// **Setters.** Values: `.underline(..)`; switches: `.external()`, `.muted()`.
#[derive(Clone, Debug)]
pub struct Link<'a> {
    text: &'a str,
    href: &'a str,
    new_tab: &'static str,
    underline: Option<Underline>,
    external: bool,
    muted: bool,
}

impl Link<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new(
            "underline",
            PropKind::Choice {
                of: Underline::NAMES,
            },
            "underline: Underline",
        )
        .default("always")
        .doc("When the underline shows: always, on hover, or never."),
        Prop::new("external", PropKind::Switch, "")
            .doc("Opens in a new tab, with an arrow and a hint for screen readers."),
        Prop::new("muted", PropKind::Switch, "")
            .doc("The muted gray, the text colour on hover: a footer's or a caption's links."),
    ];
}

impl Ui {
    /// A link reading `text` to `href`.
    pub fn link<'a>(&self, text: &'a str, href: &'a str) -> Link<'a> {
        Link {
            text,
            href,
            new_tab: self.text(Text::OpensInNewTab),
            underline: None,
            external: false,
            muted: false,
        }
    }
}

impl<'a> Link<'a> {
    /// When the underline shows: always (the default, so a link in a paragraph never relies on
    /// colour alone), on hover, or never.
    pub fn underline(mut self, underline: Underline) -> Self {
        self.underline = Some(underline);
        self
    }

    /// Opens in a new tab (`target="_blank"`, `rel="noreferrer"`), with an arrow
    /// after the text and "(opens in a new tab)" for screen readers.
    pub fn external(mut self) -> Self {
        self.external = true;
        self
    }

    /// The muted gray, turning to the text colour on hover: a footer's or a caption's links.
    pub fn muted(mut self) -> Self {
        self.muted = true;
        self
    }
}

impl Render for Link<'_> {
    fn render(&self) -> Markup {
        let class = if self.muted {
            "lui-link lui-link-muted"
        } else {
            "lui-link"
        };
        let underline = self
            .underline
            .filter(|u| *u != Underline::Always)
            .map(Underline::as_str);
        html! {
            a class=(class) data-underline=[underline] href=(self.href)
                target=[self.external.then_some("_blank")] rel=[self.external.then_some("noreferrer")] {
                (self.text)
                @if self.external {
                    span class="lui-sr" { " " (self.new_tab) }
                    (Icon::ExternalLink)
                }
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After Radix Themes Link: brand step 11, an underline min(2px, max(1px, 0.05em)) thick and
   calc(0.025em + 2px) below the baseline in a paler step, full strength on hover. */
.lui-link {
  color: var(--lui-brand-11); text-decoration-line: underline; text-decoration-style: solid;
  text-decoration-thickness: min(2px, max(1px, 0.05em)); text-underline-offset: calc(0.025em + 2px);
  text-decoration-color: color-mix(in srgb, currentColor 40%, transparent);
  border-radius: var(--lui-radius-sm); overflow-wrap: anywhere;
}
.lui-link:hover { text-decoration-color: currentColor; }
.lui-link[data-underline=hover] { text-decoration-line: none; }
.lui-link[data-underline=hover]:is(:hover, :focus-visible) { text-decoration-line: underline; }
.lui-link[data-underline=none] { text-decoration-line: none; }
.lui-link-muted { color: var(--lui-muted); }
.lui-link-muted:hover { color: var(--lui-fg); }
.lui-link > .lui-icon { width: 0.875em; height: 0.875em; margin-inline-start: 0.2em; vertical-align: -0.05em; }
"#;
