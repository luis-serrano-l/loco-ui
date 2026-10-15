//! # Prose
//!
//! Styles for plain HTML you did not build from components: a blog post rendered from
//! Markdown, a help page, release notes. Inside it, bare `h1` to `h4`, `p`, `a`, lists,
//! `blockquote`, `code`, `pre`, `kbd`, `hr`, `table` and `img` get a readable rhythm; outside
//! it, those elements keep the browser's look and the components keep theirs. After shadcn/ui
//! Typography: an extra-bold `h1`, a ruled `h2`, 1.75rem lines, 1.5rem between paragraphs,
//! lists indented 1.5rem, an italic quote behind a 2px bar.
//!
//! **Platform features:** `:where()` and `:is()` (Chrome 88, Firefox 78, Safari 14) keep
//! every rule at the weight of one element, so a component placed inside prose, or a class
//! of the app's own, wins; `text-wrap: pretty` (Chrome 117, Safari 26) where present.
//!
//! **Accessibility:** styles only; the HTML it wraps keeps its own headings, lists and
//! tables. Checked by axe-core in headless Firefox on every demo route, both capability
//! variants, light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** nothing is missing.
//!
//! **Fallback:** without `text-wrap: pretty` paragraphs break greedily.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! // Markdown rendered elsewhere goes in as `PreEscaped`, after it has been sanitised.
//! let m = ui.prose().body(PreEscaped("<p>From <em>Markdown</em>.</p>".to_owned())).render().into_string();
//! assert!(m.ends_with("<p>From <em>Markdown</em>.</p></div>"));
//! # use maud::PreEscaped;
//! let doc = html! { h2 { "Install" } p { "Add the crate." } };
//! let m = ui.prose().body(doc.clone()).render().into_string();
//! assert_eq!(m, r#"<div class="lui-prose"><h2>Install</h2><p>Add the crate.</p></div>"#);
//! // The same in `lui!`, where the block is the content:
//! let same = lui! { Prose { h2 { "Install" } p { "Add the crate." } } };
//! assert_eq!(same.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::props::{Prop, PropKind};

/// A block of styled plain HTML, made by [`Ui::prose`].
///
/// **Setters.** Values: `.body(..)`.
#[derive(Clone, Debug)]
pub struct Prose {
    content: Markup,
}

impl Prose {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [crate::props::Key] = &[];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[Prop::new("body", PropKind::Value, "markup: Markup")
        .doc("The HTML to style: headings, paragraphs, lists, quotes, code, tables.")];
}

impl Ui {
    /// A block whose plain HTML (headings, paragraphs, lists, code, tables) is styled for
    /// reading; the HTML goes in [`Prose::body`].
    pub fn prose(&self) -> Prose {
        Prose {
            content: Markup::default(),
        }
    }
}

impl Prose {
    /// The HTML to style: headings, paragraphs, lists, quotes, code, tables.
    pub fn body(mut self, markup: Markup) -> Self {
        self.content = markup;
        self
    }
}

impl Render for Prose {
    fn render(&self) -> Markup {
        html! { div class="lui-prose" { (self.content) } }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After shadcn/ui Typography, the element styles layout.rs used to set on the whole page,
   scoped to .lui-prose and kept at one element of weight (:where(.lui-prose) :is(..)). On the
   --lui-text ramp: h1 step 8 extrabold and tight, h2 step 7 semibold with a rule under it,
   h3 step 6, h4 step 5; p leading-7 with 1.5rem between; lists ml-6, 0.5rem between items. */
.lui-prose { max-width: 44rem; color: var(--lui-fg); line-height: 1.75rem; overflow-wrap: break-word; }
.lui-prose > :first-child { margin-top: 0; }
.lui-prose > :last-child { margin-bottom: 0; }
:where(.lui-prose) :is(h1, h2, h3, h4) { color: var(--lui-fg); text-wrap: balance; scroll-margin-top: 5rem; }
:where(.lui-prose) :is(h1) { font-size: var(--lui-text-8); line-height: var(--lui-leading-8); font-weight: 800; letter-spacing: -0.025em; margin: 0 0 1.5rem; }
:where(.lui-prose) :is(h2) { font-size: var(--lui-text-7); line-height: var(--lui-leading-7); font-weight: 600; letter-spacing: -0.025em; margin: 2.5rem 0 1rem; padding-bottom: 0.5rem; border-bottom: 1px solid var(--lui-line); }
:where(.lui-prose) :is(h3) { font-size: var(--lui-text-6); line-height: var(--lui-leading-6); font-weight: 600; letter-spacing: -0.025em; margin: 2rem 0 0.75rem; }
:where(.lui-prose) :is(h4) { font-size: var(--lui-text-5); line-height: var(--lui-leading-5); font-weight: 600; letter-spacing: -0.025em; margin: 1.5rem 0 0.5rem; }
:where(.lui-prose) :is(p) { margin: 0 0 1.5rem; text-wrap: pretty; }
:where(.lui-prose) :is(a) { color: var(--lui-link); font-weight: 500; text-decoration-line: underline; text-underline-offset: 0.25em; text-decoration-thickness: 1px; }
:where(.lui-prose) :is(strong) { font-weight: 600; }
:where(.lui-prose) :is(ul, ol) { margin: 0 0 1.5rem; padding-inline-start: 1.5rem; }
:where(.lui-prose) :is(ul) { list-style: disc; }
:where(.lui-prose) :is(ol) { list-style: decimal; }
:where(.lui-prose) :is(li + li, li > ul, li > ol) { margin-top: 0.5rem; }
:where(.lui-prose) :is(li > ul, li > ol) { margin-bottom: 0; }
:where(.lui-prose) :is(blockquote) { margin: 0 0 1.5rem; padding-inline-start: 1.5rem; border-inline-start: 2px solid var(--lui-line); font-style: italic; }
:where(.lui-prose) :is(hr) { margin: 2rem 0; border: 0; border-top: 1px solid var(--lui-line); }
:where(.lui-prose) :is(img, video) { max-width: 100%; height: auto; border-radius: var(--lui-radius-lg); }
:where(.lui-prose) :is(code) {
  font-family: var(--lui-font-mono); font-size: 0.875em; font-weight: 600;
  background: var(--lui-gray-3); border-radius: var(--lui-radius-sm); padding: 0.2em 0.3em;
}
:where(.lui-prose) :is(pre) {
  margin: 0 0 1.5rem; padding: 1rem; overflow-x: auto; border: 1px solid var(--lui-line); border-radius: var(--lui-radius-lg);
  background: var(--lui-gray-2); font-size: var(--lui-text-2); line-height: var(--lui-leading-3);
}
:where(.lui-prose) :is(pre code) { background: none; padding: 0; font-weight: 400; font-size: inherit; }
:where(.lui-prose) :is(kbd) { font-family: var(--lui-font-sans); font-size: 0.75em; padding: 0.1em 0.4em; border-radius: 0.35em; background: var(--lui-gray-1); box-shadow: inset 0 -0.05em 0.5px 0.5px var(--lui-gray-7); }
:where(.lui-prose) :is(table) { display: block; max-width: 100%; overflow-x: auto; margin: 0 0 1.5rem; border-collapse: collapse; font-size: var(--lui-text-2); line-height: var(--lui-leading-2); }
:where(.lui-prose) :is(th, td) { padding: 0.5rem 1rem; border: 1px solid var(--lui-line); text-align: start; }
:where(.lui-prose) :is(th) { font-weight: 600; }
:where(.lui-prose) :where(tr:nth-child(even)) { background: var(--lui-gray-2); }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_empty_prose_is_just_its_root() {
        let m = Ui::default().prose().render().into_string();
        assert_eq!(m, r#"<div class="lui-prose"></div>"#, "{m}");
    }

    #[test]
    fn the_body_is_wrapped_unescaped_inside_the_root() {
        let m = Ui::default()
            .prose()
            .body(html! { h2 { "Title" } p { "Text & more" } })
            .render()
            .into_string();
        assert_eq!(
            m, r#"<div class="lui-prose"><h2>Title</h2><p>Text &amp; more</p></div>"#,
            "{m}"
        );
    }

    #[test]
    fn a_later_body_replaces_the_earlier_one() {
        let m = Ui::default()
            .prose()
            .body(html! { p { "first" } })
            .body(html! { p { "second" } })
            .render()
            .into_string();
        assert!(m.contains("second") && !m.contains("first"), "{m}");
    }
}
