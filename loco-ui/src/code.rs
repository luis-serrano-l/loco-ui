//! # Code
//!
//! Code in two shapes: [`Ui::code`] for a name or a command inside a sentence, and
//! [`Ui::code_block`] for lines of it, kept as written and scrolled sideways inside their
//! own box. After Radix Themes Code (inline: the monospace font at 0.95em on a soft tint of
//! the tone, or solid, surface, outline or ghost) and the shadcn/ui docs' code block (a
//! bordered panel, an optional file name above, a copy button in the corner).
//!
//! The library does no syntax highlighting, so it stays free of dependencies: `.language("rust")`
//! writes `class="language-rust"` on the `<code>`, which any highlighter (a server-side one
//! such as syntect, as the demo uses, or highlight.js) picks up.
//!
//! **Platform features:** `<code>` and `<pre>` (baseline 1995); `overflow-x: auto` so long
//! lines scroll inside the block, never the page; `tab-size` (Chrome 21, Firefox 91,
//! Safari 7). `.copy()` adds a button for the enhancement script, which writes the code with
//! `navigator.clipboard` (Chrome 66, Firefox 63, Safari 13.1).
//!
//! **Accessibility:** the block is a `role="region"` named by its title, or by "Code", with
//! `tabindex="0"` so a keyboard can scroll a long line. The copy button is a real button
//! reading "Copy", then "Copied" for two seconds. Checked by axe-core in headless Firefox on
//! every demo route, both capability variants, light and dark (no serious or critical
//! violation).
//!
//! **What it does not do without script:** copy. The button is rendered `hidden` and only the
//! enhancement script shows it, so a page without script has no button that does nothing.
//!
//! **Fallback:** select the text: the code is plain text in the page.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let c = ui.code("cargo add loco-ui").render().into_string();
//! assert_eq!(c, r#"<code class="lui-code">cargo add loco-ui</code>"#);
//! // The same in `lui!`:
//! let same = lui! { Code("cargo add loco-ui"); };
//! assert_eq!(same.into_string(), c);
//! let c = ui.code("Ok(())").tone(Tone::Brand).variant(Variant::Outline).render().into_string();
//! assert_eq!(c, r#"<code class="lui-code" data-tone="brand" data-variant="outline">Ok(())</code>"#);
//! assert_eq!(lui! { Code("Ok(())") tone="brand" variant="outline"; }.into_string(), c);
//! // A block with a file name and a copy button (shown by the script only).
//! let block = ui.code_block("[dependencies]\nloco-ui = \"0.2\"").language("toml").title("Cargo.toml").copy();
//! let block = block.render().into_string();
//! assert!(block.contains(r#"<pre class="lui-code-block-pre" id="code-cargo-toml" tabindex="0" role="region" aria-label="Cargo.toml"><code class="lui-code-block-code language-toml">[dependencies]"#));
//! assert!(block.contains(r#"data-lui="copy" hidden"#));
//! let same = lui! { CodeBlock("[dependencies]\nloco-ui = \"0.2\"") language="toml" title="Cargo.toml" copy; };
//! assert_eq!(same.into_string(), block);
//! ```

use maud::{Markup, Render, html};

use crate::i18n::Text;
use crate::props::{Key, Prop, PropKind};
use crate::variant::{Tone, Variant};
use crate::{Caps, Icon, Ui};

/// Inline code, made by [`Ui::code`].
///
/// **Setters.** Values: `.tone(..)`, `.variant(..)`.
#[derive(Clone, Debug)]
pub struct Code<'a> {
    text: &'a str,
    tone: Option<Tone>,
    variant: Option<Variant>,
}

impl Code<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [crate::props::Key] = &[];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("tone", PropKind::Choice { of: Tone::NAMES }, "tone: Tone")
            .default("neutral")
            .doc("The colour of the tint and the text."),
        Prop::new(
            "variant",
            PropKind::Choice { of: Variant::NAMES },
            "variant: Variant",
        )
        .default("soft")
        .doc("A tint (soft), a fill (solid), a tint in a border (surface), a border (outline) or the text alone (ghost)."),
    ];
}

impl Ui {
    /// Inline code reading `text`: a name, a value or a command inside a sentence.
    pub fn code<'a>(&self, text: &'a str) -> Code<'a> {
        Code {
            text,
            tone: None,
            variant: None,
        }
    }

    /// A block of code reading `text`, its lines and spaces kept as written.
    pub fn code_block<'a>(&self, text: &'a str) -> CodeBlock<'a> {
        CodeBlock {
            caps: self.caps,
            text,
            language: None,
            title: None,
            id: None,
            copy: false,
            words: [
                self.text(Text::Code),
                self.text(Text::Copy),
                self.text(Text::Copied),
            ],
        }
    }
}

impl<'a> Code<'a> {
    /// The colour of the tint and the text: neutral (the default), brand, ok, warn, danger or
    /// info.
    pub fn tone(mut self, tone: Tone) -> Self {
        self.tone = Some(tone);
        self
    }

    /// A tint (soft, the default), a fill (solid), a tint in a border (surface), a border
    /// (outline) or the text alone (ghost).
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = Some(variant);
        self
    }
}

impl Render for Code<'_> {
    fn render(&self) -> Markup {
        html! {
            code class="lui-code" data-tone=[self.tone.map(Tone::as_str)]
                data-variant=[self.variant.map(Variant::as_str)] { (self.text) }
        }
    }
}

/// A block of code, made by [`Ui::code_block`].
///
/// **Setters.** Values: `.language(..)`, `.title(..)`, `.id(..)`; switches: `.copy()`.
#[derive(Clone, Debug)]
pub struct CodeBlock<'a> {
    caps: Caps,
    text: &'a str,
    language: Option<&'a str>,
    title: Option<&'a str>,
    id: Option<&'a str>,
    copy: bool,
    /// The region's default name, the button's text, and its text once copied.
    words: [&'static str; 3],
}

impl CodeBlock<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [Key] = &[
        Key::new(
            "Tab",
            "Focuses the code block, then its copy button when the script shows one.",
        ),
        Key::new(
            "ArrowLeft / ArrowRight",
            "Scrolls a long line sideways while the block has focus.",
        ),
        Key::new("Enter / Space", "Copies the code from the copy button.").script(),
    ];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("language", PropKind::Value, "language: &'a str")
            .doc("The language, written as `class=\"language-…\"` for a highlighter."),
        Prop::new("title", PropKind::Value, "title: &'a str")
            .doc("A file name or caption above the code; it names the region too."),
        Prop::new("id", PropKind::Value, "id: &'a str")
            .attr("id")
            .doc("Replace the id derived from the title (or the code)."),
        Prop::new("copy", PropKind::Switch, "")
            .doc("A copy button in the corner, shown and wired by the enhancement script only."),
    ];
}

impl<'a> CodeBlock<'a> {
    /// The language, written as `class="language-…"` on the `<code>` so a highlighter finds
    /// it; the library itself colours nothing.
    pub fn language(mut self, language: &'a str) -> Self {
        self.language = Some(language);
        self
    }

    /// A file name or caption above the code (`Cargo.toml`); it names the region too.
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    /// Replace the id derived from the title, or from the code when there is none.
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }

    /// A copy button in the corner. It is rendered `hidden`, and the enhancement script shows
    /// it and copies the code; without script the code is selected by hand.
    pub fn copy(mut self) -> Self {
        self.copy = true;
        self
    }
}

/// `code-` and a short FNV-1a hash of `text`: an id that is the same on every render and
/// differs between blocks.
fn text_id(text: &str) -> String {
    let mut hash: u32 = 0x811c_9dc5;
    for b in text.bytes() {
        hash = (hash ^ u32::from(b)).wrapping_mul(0x0100_0193);
    }
    format!("code-{hash:08x}")
}

impl Render for CodeBlock<'_> {
    fn render(&self) -> Markup {
        let id = match (self.id, self.title) {
            (Some(id), _) => id.to_owned(),
            (None, Some(title)) => format!("code-{}", crate::slug(title)),
            (None, None) => text_id(self.text),
        };
        let [name, copy, copied] = self.words;
        let class = match self.language {
            Some(l) => format!("lui-code-block-code language-{l}"),
            None => "lui-code-block-code".to_owned(),
        };
        html! {
            div class="lui-code-block" {
                @if let Some(title) = self.title { div class="lui-code-block-title" { (title) } }
                pre class="lui-code-block-pre" id=(id) tabindex="0" role="region" aria-label=(self.title.unwrap_or(name)) {
                    code class=(class) { (self.text.trim_end_matches('\n')) }
                }
                @if self.copy {
                    // Hidden until the script shows it: a button that copies needs script.
                    span class="lui-code-block-copy" data-lui="copy" hidden data-lui-copied=(copied) {
                        (crate::button::Button::new(self.caps, copy)
                            .ghost().small().icon_only().aria_label(copy)
                            .command("--copy", &id)
                            .body(html! { (Icon::Copy) }))
                    }
                }
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After Radix Themes Code: the monospace font at 0.95em, padding 0.1em 0.25em, radius
   calc(--radius × 0.4); soft is a tint of the tone with step-11 text. */
.lui-code {
  --lui-code-tone: var(--lui-gray-11);
  font-family: var(--lui-font-mono); font-size: 0.95em; line-height: inherit; padding: 0.1em 0.25em;
  border: 0; border-radius: var(--lui-radius-sm); overflow-wrap: anywhere;
  background: color-mix(in srgb, var(--lui-code-tone) 12%, transparent); color: color-mix(in srgb, var(--lui-code-tone) 70%, var(--lui-fg));
}
.lui-code:is([data-tone=brand], [data-tone=info]) { --lui-code-tone: var(--lui-brand-11); }
.lui-code[data-tone=ok] { --lui-code-tone: var(--lui-ok); }
.lui-code[data-tone=warn] { --lui-code-tone: var(--lui-warn); }
.lui-code[data-tone=danger] { --lui-code-tone: var(--lui-danger); }
.lui-code[data-variant=solid] { background: var(--lui-code-tone); color: var(--lui-gray-1); }
.lui-code[data-variant=surface] { background: color-mix(in srgb, var(--lui-code-tone) 6%, transparent); box-shadow: inset 0 0 0 1px color-mix(in srgb, var(--lui-code-tone) 35%, transparent); }
.lui-code[data-variant=outline] { background: none; box-shadow: inset 0 0 0 1px color-mix(in srgb, var(--lui-code-tone) 45%, transparent); }
.lui-code[data-variant=ghost] { background: none; padding: 0; }
/* After the shadcn/ui docs' code block: a --lui-gray-2 panel in a 1px border, radius
   --lui-radius-lg, 1rem padding, --lui-text-2 mono; the file name sits in a strip above. */
.lui-code-block {
  position: relative; min-width: 0; max-width: 100%; margin: 0;
  border: 1px solid var(--lui-line); border-radius: var(--lui-radius-lg); background: var(--lui-gray-2); overflow: hidden;
}
.lui-code-block-title {
  padding: 0.5rem 1rem; border-bottom: 1px solid var(--lui-line); color: var(--lui-muted);
  font-family: var(--lui-font-mono); font-size: var(--lui-text-1); overflow: hidden; text-overflow: ellipsis; white-space: nowrap;
}
.lui-code-block-pre {
  margin: 0; padding: 1rem; overflow-x: auto; tab-size: 4;
  font-family: var(--lui-font-mono); font-size: var(--lui-text-2); line-height: var(--lui-leading-3); color: var(--lui-fg);
  scrollbar-width: thin; scrollbar-color: var(--lui-line) transparent;
}
.lui-code-block-code { font: inherit; background: none; border: 0; padding: 0; }
.lui-code-block-copy { position: absolute; top: 0.375rem; inset-inline-end: 0.375rem; }
.lui-code-block-copy[hidden] { display: none; }
.lui-code-block:has(.lui-code-block-copy:not([hidden])) .lui-code-block-pre { padding-inline-end: 3rem; }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_differ_by_code_and_repeat_for_the_same_code() {
        assert_eq!(text_id("fn main() {}"), text_id("fn main() {}"));
        assert_ne!(text_id("a"), text_id("b"));
        assert!(text_id("").starts_with("code-"));
    }
}
