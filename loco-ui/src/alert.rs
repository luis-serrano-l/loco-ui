//! # Alert
//!
//! A callout in the page: a title, a line or two of explanation and an icon, in a neutral,
//! danger, warning or success tone. For a message that stays where it is, unlike a flash or a
//! toast.
//!
//! **Platform features:** a `role="alert"` box for the danger tone (read out when the page
//! loads) and `role="status"` for the others; the icon is decorative.
//!
//! **Accessibility:** `role="alert"` for the danger tone, `role="status"` for the others; the
//! icon is decorative. Checked by axe-core in headless Firefox on every demo route, both
//! capability variants, light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** nothing is missing.
//!
//! **Fallback:** none needed.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let m = ui.alert("Heads up").description("You can add components to your app.").render().into_string();
//! assert!(m.contains(r#"role="status""#) && m.contains("Heads up"));
//! let m = ui.alert("Payment failed").danger().description("Your card was declined.").render().into_string();
//! assert!(m.contains(r#"class="lui-alert lui-callout lui-callout-danger" role="alert""#));
//! // The same in `lui!`:
//! let same = lui! { Alert("Payment failed") danger description="Your card was declined."; };
//! assert_eq!(same.into_string(), m);
//! // Surface and outline variants.
//! let m = ui.alert("Heads up").surface().render().into_string();
//! assert!(m.contains("lui-callout-info lui-callout-surface"));
//! assert_eq!(lui! { Alert("Heads up") surface; }.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::icon::Glyph;
use crate::props::{Prop, PropKind};
use crate::{Icon, Ui};

/// A callout, made by [`Ui::alert`].
///
/// **Setters.** Values and items: `.description(..)`, `.body(..)`, `.icon(..)`; switches:
/// `.danger()`, `.warn()`, `.ok()`, `.surface()`, `.outline()`.
#[derive(Clone, Debug)]
pub struct Alert<'a> {
    title: &'a str,
    description: Option<&'a str>,
    body: Option<Markup>,
    tone: Option<&'static str>,
    variant: Option<&'static str>,
    icon: Option<Glyph<'a>>,
}

impl Alert<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("description", PropKind::Value, "text: &'a str")
            .doc("A line of text under the title."),
        Prop::new("body", PropKind::Value, "markup: Markup")
            .doc("Markup under the title (a list, a link), after the description."),
        Prop::new("icon", PropKind::Value, "icon: impl Into<Glyph<'a>>")
            .doc("Another icon than the tone's own."),
        Prop::new("danger", PropKind::Switch, "").doc("Something went wrong."),
        Prop::new("warn", PropKind::Switch, "").doc("Something to watch."),
        Prop::new("ok", PropKind::Switch, "").doc("Something worked."),
        Prop::new("surface", PropKind::Switch, "")
            .doc("A paler tint inside a border in the level's colour."),
        Prop::new("outline", PropKind::Switch, "")
            .doc("No tint: a border in the level's colour on the page."),
    ];
}

impl Ui {
    /// A callout headed `title`.
    pub fn alert<'a>(&self, title: &'a str) -> Alert<'a> {
        Alert {
            title,
            description: None,
            body: None,
            tone: None,
            variant: None,
            icon: None,
        }
    }
}

impl<'a> Alert<'a> {
    /// A line of text under the title.
    pub fn description(mut self, text: &'a str) -> Self {
        self.description = Some(text);
        self
    }

    /// Markup under the title (a list, a link), after the description.
    pub fn body(mut self, markup: Markup) -> Self {
        self.body = Some(markup);
        self
    }

    /// Another icon than the tone's own.
    pub fn icon(mut self, icon: impl Into<Glyph<'a>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Something went wrong: `--lui-danger`, and `role="alert"`.
    pub fn danger(mut self) -> Self {
        self.tone = Some("danger");
        self
    }

    /// Something to watch: `--lui-warn`.
    pub fn warn(mut self) -> Self {
        self.tone = Some("warn");
        self
    }

    /// Something worked: `--lui-ok`.
    pub fn ok(mut self) -> Self {
        self.tone = Some("ok");
        self
    }

    /// A paler tint inside a border in the level's colour (Radix's surface variant).
    pub fn surface(mut self) -> Self {
        self.variant = Some("surface");
        self
    }

    /// No tint: a border in the level's colour on the page (Radix's outline variant).
    pub fn outline(mut self) -> Self {
        self.variant = Some("outline");
        self
    }
}

impl Render for Alert<'_> {
    fn render(&self) -> Markup {
        let icon = self.icon.unwrap_or(Glyph::Icon(match self.tone {
            Some("danger") | Some("warn") => Icon::TriangleAlert,
            Some("ok") => Icon::CircleCheck,
            _ => Icon::Info,
        }));
        html! {
            div class={ "lui-alert lui-callout lui-callout-" (self.tone.unwrap_or("info")) @if let Some(v) = self.variant { " lui-callout-" (v) } }
                role=(if self.tone == Some("danger") { "alert" } else { "status" }) {
                (icon.hidden())
                p class="lui-alert-title" { (self.title) }
                @if let Some(d) = self.description { p class="lui-alert-description" { (d) } }
                @if let Some(b) = &self.body { div class="lui-alert-description" { (b) } }
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`]. It carries the callout
/// look that [`crate::error_summary`] and [`crate::flash`] take too.
pub const CSS: &str = r#"
/* Radix Themes Callout, shared by alert, error summary and flash (class lui-callout): the
   icon in a 1rem column, the title, then the body and links, all in the level's colour.
   Soft by default (step 3 fill, step 11 text); .surface() adds a border round a paler fill,
   .outline() keeps only the border. Info is the brand scale; ok, warn and danger have one
   colour each, so their steps are mixed from it and the page background. */
.lui-callout {
  --lui-callout-bg: var(--lui-brand-3); --lui-callout-fg: var(--lui-brand-11); --lui-callout-line: var(--lui-brand-7);
  display: grid; grid-template-columns: 1rem minmax(0, 1fr); column-gap: var(--lui-space-3); row-gap: var(--lui-space-1);
  align-items: start; box-sizing: border-box; padding: var(--lui-space-3) var(--lui-space-4);
  font-size: 0.875rem; line-height: 1.25rem; color: var(--lui-callout-fg);
  background: var(--lui-callout-bg); border: 1px solid transparent; border-radius: var(--lui-radius);
}
.lui-callout-ok { --lui-callout-bg: color-mix(in srgb, var(--lui-ok) 12%, var(--lui-bg)); --lui-callout-fg: color-mix(in srgb, var(--lui-ok) 80%, var(--lui-fg)); --lui-callout-line: color-mix(in srgb, var(--lui-ok) 45%, var(--lui-bg)); }
.lui-callout-warn { --lui-callout-bg: color-mix(in srgb, var(--lui-warn) 14%, var(--lui-bg)); --lui-callout-fg: color-mix(in srgb, var(--lui-warn) 80%, var(--lui-fg)); --lui-callout-line: color-mix(in srgb, var(--lui-warn) 45%, var(--lui-bg)); }
.lui-callout-danger { --lui-callout-bg: color-mix(in srgb, var(--lui-danger) 10%, var(--lui-bg)); --lui-callout-fg: color-mix(in srgb, var(--lui-danger) 85%, var(--lui-fg)); --lui-callout-line: color-mix(in srgb, var(--lui-danger) 40%, var(--lui-bg)); }
.lui-callout-surface { background: color-mix(in srgb, var(--lui-callout-bg) 60%, var(--lui-bg)); border-color: var(--lui-callout-line); }
.lui-callout-outline { background: transparent; border-color: var(--lui-callout-line); }
.lui-callout > .lui-icon { grid-row: 1 / span 2; margin-top: 0.125rem; }
.lui-callout > :not(.lui-icon) { grid-column: 2; margin: 0; }
.lui-callout a { color: inherit; text-decoration: underline; text-underline-offset: 2px; }
.lui-callout a:hover { text-decoration-thickness: 2px; }
.lui-alert-title { font-weight: 600; }
.lui-alert-description p { margin: 0; }
"#;
