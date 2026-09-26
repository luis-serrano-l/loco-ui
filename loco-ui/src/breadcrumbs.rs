//! # Breadcrumbs
//!
//! Where this page sits: a trail of links ending in the current page. A long trail folds its
//! middle into a disclosure so the ends stay readable on a phone.
//!
//! **Platform features:** `<nav aria-label="Breadcrumb">` around an ordered list, the last
//! item marked `aria-current="page"`; separators drawn by CSS `::before` so screen readers do
//! not read them; the folded middle is a `<details>` element.
//!
//! **Accessibility:** a `<nav>` named "Breadcrumb" with an ordered list; the current page has
//! `aria-current="page"`; folded steps sit in a named `<details>`. Checked by axe-core in
//! headless Firefox on every demo route, both capability variants, light and dark (no serious
//! or critical violation).
//!
//! The look follows shadcn Breadcrumb: muted links, chevron separators, the current page in
//! the text colour. A trail of more than four folds its middle into a `…` disclosure menu; a
//! shorter one with a middle crumb folds it only when the breadcrumbs are under 30rem wide
//! (both are rendered; a container query shows one).
//!
//! **What it does not do without script:** measure the text to fold exactly what does not fit;
//! the fold is decided by item count and the container's width.
//!
//! **Fallback:** none needed.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::from(Caps::all());
//! let m = ui.breadcrumbs().link("Home", "/").link("Projects", "/projects").here("loco-ui").render().into_string();
//! assert!(m.contains(r#"<a href="/projects">Projects</a>"#));
//! assert!(m.contains(r#"aria-current="page">loco-ui"#));
//! // More than four: the middle folds into <details>.
//! let long = ui.breadcrumbs().link("Home", "/").link("A", "/a").link("B", "/a/b").link("C", "/a/b/c").here("Here");
//! assert!(long.render().into_string().contains("<details"));
//! // One middle crumb: shown inline, and folded too for a narrow container.
//! let three = ui.breadcrumbs().link("Home", "/").link("A", "/a").link("B", "/a/b").here("Here").render().into_string();
//! assert!(three.contains("lui-breadcrumbs-mid") && three.contains("lui-breadcrumbs-fold-narrow"));
//! // The same in `lui!`:
//! let same = lui! { Breadcrumbs {
//!     link "Home" "/"; link "Projects" "/projects"; here "loco-ui";
//! } };
//! assert_eq!(same.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::i18n::{Strings, Text};
use crate::props::{Prop, PropKind};

/// A trail of links ending in the current page, made by [`Ui::breadcrumbs`].
///
/// **Setters.** Values and items: `.link(..)`, `.here(..)`.
#[derive(Clone, Debug, Default)]
pub struct Breadcrumbs<'a> {
    trail: Vec<(&'a str, &'a str)>,
    here: &'a str,
    strings: &'static Strings,
}

impl Breadcrumbs<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("link", PropKind::Item, "label: &'a str, href: &'a str")
            .doc("One step from the root towards this page."),
        Prop::new("here", PropKind::Item, "label: &'a str")
            .doc("The current page, last in the trail and not a link."),
    ];
}

impl Ui {
    /// An empty trail: add the way down with [`Breadcrumbs::link`], then the page with
    /// [`Breadcrumbs::here`].
    pub fn breadcrumbs(&self) -> Breadcrumbs<'_> {
        Breadcrumbs {
            strings: self.strings,
            ..Breadcrumbs::default()
        }
    }
}

impl<'a> Breadcrumbs<'a> {
    /// One step from the root towards this page.
    pub fn link(mut self, label: &'a str, href: &'a str) -> Self {
        self.trail.push((label, href));
        self
    }

    /// The current page, last in the trail and not a link.
    pub fn here(mut self, label: &'a str) -> Self {
        self.here = label;
        self
    }
}

impl Render for Breadcrumbs<'_> {
    fn render(&self) -> Markup {
        let before = &self.trail[..];
        let fold = before.len() > 3;
        // A trail with a middle that fits a wide box also carries its fold, shown only when
        // the breadcrumbs are narrow (a container query swaps the two).
        let narrow_fold = !fold && before.len() == 3;
        let (head, middle, tail) = if fold || narrow_fold {
            (
                &before[..1],
                &before[1..before.len() - 1],
                &before[before.len() - 1..],
            )
        } else {
            (before, &before[..0], &before[..0])
        };
        html! {
            nav class="lui-breadcrumbs" aria-label=(self.strings.get(Text::Breadcrumb)) {
                ol {
                    @for (label, href) in head { li { a href=(href) { (label) } } }
                    @if narrow_fold {
                        @for (label, href) in middle { li class="lui-breadcrumbs-mid" { a href=(href) { (label) } } }
                    }
                    @if fold || narrow_fold {
                        li class={ "lui-breadcrumbs-fold" @if narrow_fold { " lui-breadcrumbs-fold-narrow" } } {
                            details {
                                summary aria-label=(self.strings.fill(Text::ShowMore, &[&middle.len()])) { "\u{2026}" }
                                ol { @for (label, href) in middle { li { a href=(href) { (label) } } } }
                            }
                        }
                    }
                    @for (label, href) in tail { li { a href=(href) { (label) } } }
                    li { span aria-current="page" { (self.here) } }
                }
            }
        }
    }
}
/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After shadcn Breadcrumb: muted links that darken on hover, chevrons between (two borders of
   a turned square, so screen readers read nothing), the current page in the text colour.
   The breadcrumbs are their own container: under 30rem a short trail's middle crumb gives
   way to the same … menu a long trail always has. */
.lui-breadcrumbs { container: lui-breadcrumbs / inline-size; font-size: 0.875rem; color: var(--lui-muted); margin-bottom: var(--lui-space-4); }
.lui-breadcrumbs > ol { display: flex; flex-wrap: wrap; align-items: center; gap: 0.25rem 0; list-style: none; margin: 0; padding: 0; }
.lui-breadcrumbs > ol > li { display: inline-flex; align-items: center; min-width: 0; }
.lui-breadcrumbs > ol > li + li::before {
  content: ""; flex: none; width: 0.3125rem; height: 0.3125rem; margin-inline: 0.5rem 0.625rem;
  border-right: 1.5px solid currentColor; border-bottom: 1.5px solid currentColor; rotate: -45deg; opacity: 0.8;
}
.lui-breadcrumbs a { color: var(--lui-muted); text-decoration: none; transition: color var(--lui-duration-fast); }
.lui-breadcrumbs a:hover { color: var(--lui-fg); }
.lui-breadcrumbs [aria-current] { color: var(--lui-fg); font-weight: 400; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.lui-breadcrumbs-fold { position: relative; }
.lui-breadcrumbs-fold-narrow { display: none !important; }
@container lui-breadcrumbs (width < 30rem) {
  .lui-breadcrumbs-mid { display: none !important; }
  .lui-breadcrumbs-fold-narrow { display: inline-flex !important; }
}
.lui-breadcrumbs-fold details { display: inline-block; }
.lui-breadcrumbs-fold summary { display: inline-grid; place-items: center; min-width: 1.5rem; height: 1.5rem; list-style: none; cursor: pointer; border-radius: var(--lui-radius-sm); }
.lui-breadcrumbs-fold summary::-webkit-details-marker { display: none; }
.lui-breadcrumbs-fold summary:hover { background: var(--lui-accent); color: var(--lui-on-accent); }
.lui-breadcrumbs-fold ol {
  position: absolute; z-index: 5; top: 100%; left: 0; margin: var(--lui-space-1) 0 0; padding: var(--lui-space-1);
  list-style: none; min-width: 10rem; background: var(--lui-popover);
  border: 1px solid var(--lui-line); border-radius: var(--lui-radius); box-shadow: var(--lui-shadow-md), var(--lui-highlight);
}
.lui-breadcrumbs-fold ol a { display: flex; align-items: center; min-height: 2rem; padding: 0 0.5rem; border-radius: var(--lui-radius-sm); color: var(--lui-fg); }
.lui-breadcrumbs-fold ol a:hover { background: var(--lui-accent); color: var(--lui-on-accent); }
@media (pointer: coarse) { .lui-breadcrumbs-fold ol a { min-height: var(--lui-hit); } }
"#;
