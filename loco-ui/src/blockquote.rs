//! # Blockquote
//!
//! A quotation set apart from the text around it, with the person or work it comes from
//! underneath: a testimonial, a passage from the docs, a line from a review. After Radix
//! Themes Blockquote: a bar in a pale step of the brand scale down the start edge, the text
//! indented past it at the size around it, and a muted caption.
//!
//! **Platform features:** `<figure>`, `<blockquote>`, `<figcaption>` and `<cite>` (baseline
//! 2011 for figure); `border-inline-start` (Chrome 87, Firefox 66, Safari 14.1) so the bar
//! moves to the right in a right-to-left page.
//!
//! **Accessibility:** a `<blockquote>` inside a `<figure>` whose `<figcaption>` holds the
//! source, the pattern the HTML standard gives for a quotation with its attribution. Checked by
//! axe-core in headless Firefox on every demo route, both capability variants, light and dark
//! (no serious or critical violation).
//!
//! **What it does not do without script:** nothing is missing.
//!
//! **Fallback:** none needed.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let q = ui.blockquote().body(html! { p { "Simple things should be simple." } }).render().into_string();
//! assert_eq!(q, r#"<figure class="lui-blockquote"><blockquote class="lui-blockquote-quote"><p>Simple things should be simple.</p></blockquote></figure>"#);
//! // The same in `lui!`, where the block is the quotation:
//! let same = lui! { Blockquote { p { "Simple things should be simple." } } };
//! assert_eq!(same.into_string(), q);
//! // With its source, as text and as a URL.
//! let q = ui.blockquote().caption("Alan Kay").cite("https://example.com/kay").body(html! { "Simple things should be simple." });
//! let q = q.render().into_string();
//! assert_eq!(q, r#"<figure class="lui-blockquote"><blockquote class="lui-blockquote-quote" cite="https://example.com/kay">Simple things should be simple.</blockquote><figcaption class="lui-blockquote-caption"><cite>Alan Kay</cite></figcaption></figure>"#);
//! let same = lui! { Blockquote caption="Alan Kay" cite="https://example.com/kay" { "Simple things should be simple." } };
//! assert_eq!(same.into_string(), q);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::props::{Prop, PropKind};

/// A quotation, made by [`Ui::blockquote`].
///
/// **Setters.** Values: `.body(..)`, `.caption(..)`, `.cite(..)`.
#[derive(Clone, Debug)]
pub struct Blockquote<'a> {
    content: Markup,
    caption: Option<&'a str>,
    cite: Option<&'a str>,
}

impl Blockquote<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [crate::props::Key] = &[];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("body", PropKind::Value, "markup: Markup").doc("The quotation."),
        Prop::new("caption", PropKind::Value, "source: &'a str")
            .doc("Who said it, or the work it is from, in a `<cite>` under the quotation."),
        Prop::new("cite", PropKind::Value, "url: &'a str")
            .attr("cite")
            .doc("The URL of the source, written as the blockquote's `cite` attribute."),
    ];
}

impl Ui {
    /// A quotation; its words go in [`Blockquote::body`].
    pub fn blockquote<'a>(&self) -> Blockquote<'a> {
        Blockquote {
            content: Markup::default(),
            caption: None,
            cite: None,
        }
    }
}

impl<'a> Blockquote<'a> {
    /// The quotation: text, or paragraphs.
    pub fn body(mut self, markup: Markup) -> Self {
        self.content = markup;
        self
    }

    /// Who said it, or the work it is from, in a `<cite>` under the quotation.
    pub fn caption(mut self, source: &'a str) -> Self {
        self.caption = Some(source);
        self
    }

    /// The URL of the source, written as the blockquote's `cite` attribute (browsers do not
    /// show it; crawlers and reader modes read it).
    pub fn cite(mut self, url: &'a str) -> Self {
        self.cite = Some(url);
        self
    }
}

impl Render for Blockquote<'_> {
    fn render(&self) -> Markup {
        html! {
            figure class="lui-blockquote" {
                blockquote class="lui-blockquote-quote" cite=[self.cite] { (self.content) }
                @if let Some(source) = self.caption { figcaption class="lui-blockquote-caption" { cite { (source) } } }
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After Radix Themes Blockquote: a bar max(0.25rem, 0.25em) wide in brand step 6 on the start
   edge, the text min(1.25rem, max(0.75rem, 0.5em)) past it; the caption muted at --lui-text-2. */
.lui-blockquote { margin: 0; }
.lui-blockquote-quote {
  margin: 0; padding-inline-start: min(1.25rem, max(0.75rem, 0.5em));
  border-inline-start: max(0.25rem, 0.25em) solid var(--lui-brand-6); color: var(--lui-fg); overflow-wrap: break-word;
}
.lui-blockquote-quote > :last-child { margin-bottom: 0; }
.lui-blockquote-caption { margin-top: 0.5rem; padding-inline-start: calc(min(1.25rem, max(0.75rem, 0.5em)) + max(0.25rem, 0.25em)); color: var(--lui-muted); font-size: var(--lui-text-2); }
.lui-blockquote-caption::before { content: "— "; }
.lui-blockquote-caption cite { font-style: normal; }
"#;
