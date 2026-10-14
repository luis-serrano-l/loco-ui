//! # Alert
//!
//! A callout in the page, after Radix Themes Callout: a title, a line or two of explanation
//! and an icon, in an info, danger, warning or success tone (or any
//! [`Tone`] with `.tone(..)`), soft by default or `.surface()`,
//! `.outline()` (`.variant(..)`), at Radix's sizes 1 to 3. For a message that stays where it
//! is, unlike a flash or a toast. `.action(..)` adds a button under the text, and
//! `.dismiss()` a close button that keeps it closed, as a [`crate::banner::Banner`] does.
//!
//! **Platform features:** a `role="alert"` box for the danger tone (read out when the page
//! loads) and `role="status"` for the others; the icon is decorative. The close button is a
//! `<form method="post">` to [`crate::banner::DISMISS_PATH`], which sets a `lui-dismiss-<id>`
//! cookie and redirects back; the next page renders nothing for it.
//!
//! **Accessibility:** `role="alert"` for the danger tone, `role="status"` for the others; the
//! icon is decorative. Checked by axe-core in headless Firefox on every demo route, both
//! capability variants, light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** close in place: closing is a round trip.
//!
//! **Fallback:** none needed.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let m = ui.alert("Heads up").description("You can add components to your app.").render().into_string();
//! assert!(m.contains(r#"role="status""#) && m.contains("Heads up"));
//! let m = ui.alert("Payment failed").danger().description("Your card was declined.").render().into_string();
//! assert!(m.contains(r#"class="lui-alert lui-callout lui-callout-danger" data-tone="danger" role="alert""#));
//! // The same in `lui!`:
//! let same = lui! { Alert("Payment failed") danger description="Your card was declined."; };
//! assert_eq!(same.into_string(), m);
//! // Surface and outline variants.
//! let m = ui.alert("Heads up").surface().render().into_string();
//! assert!(m.contains("lui-callout-info lui-callout-surface"));
//! assert_eq!(lui! { Alert("Heads up") surface; }.into_string(), m);
//! // The typed setters: a quiet gray note, smaller.
//! let m = ui.alert("Draft").tone(Tone::Neutral).variant(Variant::Outline).size(1).render().into_string();
//! assert!(m.contains(r#"class="lui-alert lui-callout lui-callout-neutral lui-callout-outline" data-size="1" data-tone="neutral" data-variant="outline""#));
//! assert_eq!(lui! { Alert("Draft") tone="neutral" variant="outline" size=1; }.into_string(), m);
//! // A button under the text, and a close button that keeps it closed.
//! let ui = Ui::from_request("/billing", "", "");
//! let m = ui.alert("Trial ends Friday").info().action(ui.link_button("Upgrade", "/plans").small()).dismiss().render().into_string();
//! assert!(m.contains(r#"<div class="lui-alert-actions"><a class="lui-button"#) && m.contains(r#"name="id" value="trial-ends-friday""#));
//! let same = lui! { Alert("Trial ends Friday") info action=(ui.link_button("Upgrade", "/plans").small()) dismiss; };
//! assert_eq!(same.into_string(), m);
//! let gone = Ui::from_request("/billing", "", "lui-dismiss-trial-ends-friday=1");
//! assert_eq!(gone.alert("Trial ends Friday").dismiss().render().into_string(), "");
//! ```

use maud::{Markup, Render, html};

use crate::button::Button;
use crate::icon::Glyph;
use crate::props::{Prop, PropKind};
use crate::variant::{Size, Tone, Variant};
use crate::{Icon, Ui, slug};

/// A callout, made by [`Ui::alert`].
///
/// **Setters.** Values and items: `.description(..)`, `.body(..)`, `.icon(..)`, `.size(..)`,
/// `.tone(..)`, `.variant(..)`, `.action(..)`, `.id(..)`; switches:
/// `.danger()`, `.warn()`, `.ok()`, `.info()`, `.surface()`, `.outline()`, `.dismiss()`.
#[derive(Clone, Debug)]
pub struct Alert<'a> {
    ui: &'a Ui,
    title: &'a str,
    action: Option<Button<'a>>,
    id: Option<&'a str>,
    dismiss: bool,
    description: Option<&'a str>,
    body: Option<Markup>,
    tone: Option<Tone>,
    variant: Option<Variant>,
    size: Size,
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
        Prop::new("info", PropKind::Switch, "")
            .doc("Something to know: the brand scale, the default."),
        Prop::new("surface", PropKind::Switch, "")
            .doc("A paler tint inside a border in the level's colour."),
        Prop::new("outline", PropKind::Switch, "")
            .doc("No tint: a border in the level's colour on the page."),
        Prop::new(
            "size",
            PropKind::Choice { of: Size::NAMES },
            "size: impl Into<Size>",
        )
        .default("2")
        .doc("Radix's sizes 1 to 3: the padding, gaps and text."),
        Prop::new("tone", PropKind::Choice { of: Tone::NAMES }, "tone: Tone")
            .default("info")
            .doc("The colour; danger also makes it `role=\"alert\"`."),
        Prop::new(
            "variant",
            PropKind::Choice { of: Variant::NAMES },
            "variant: Variant",
        )
        .default("soft")
        .doc("Soft, surface or outline; ghost drops the frame, and solid draws as soft."),
        Prop::new("action", PropKind::Value, "button: Button<'a>")
            .doc("A button or link button under the text."),
        Prop::new("id", PropKind::Value, "id: &'a str")
            .doc("The id its dismiss cookie is named after instead of the title's slug."),
        Prop::new("dismiss", PropKind::Switch, "")
            .doc("A close button that keeps it closed with a `lui-dismiss-<id>` cookie."),
    ];
}

impl Ui {
    /// A callout headed `title`.
    pub fn alert<'a>(&'a self, title: &'a str) -> Alert<'a> {
        Alert {
            ui: self,
            title,
            action: None,
            id: None,
            dismiss: false,
            description: None,
            body: None,
            tone: None,
            variant: None,
            size: Size::S2,
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
    pub fn danger(self) -> Self {
        self.tone(Tone::Danger)
    }

    /// Something to watch: `--lui-warn`.
    pub fn warn(self) -> Self {
        self.tone(Tone::Warn)
    }

    /// Something worked: `--lui-ok`.
    pub fn ok(self) -> Self {
        self.tone(Tone::Ok)
    }

    /// Something to know: the brand scale, as with no tone at all.
    pub fn info(self) -> Self {
        self.tone(Tone::Info)
    }

    /// A button or link button under the text (`ui.link_button("Upgrade", "/plans").small()`).
    pub fn action(mut self, button: Button<'a>) -> Self {
        self.action = Some(button);
        self
    }

    /// The id its dismiss cookie is named after (`lui-dismiss-<id>`) instead of the title's
    /// slug.
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }

    /// A close button that keeps it closed with a `lui-dismiss-<id>` cookie, set by the form
    /// at [`crate::banner::DISMISS_PATH`].
    pub fn dismiss(mut self) -> Self {
        self.dismiss = true;
        self
    }

    /// A paler tint inside a border in the level's colour (Radix's surface variant).
    pub fn surface(self) -> Self {
        self.variant(Variant::Surface)
    }

    /// No tint: a border in the level's colour on the page (Radix's outline variant).
    pub fn outline(self) -> Self {
        self.variant(Variant::Outline)
    }

    /// The colour: info (the default, the brand scale), neutral, brand, ok, warn or danger;
    /// danger also makes it `role="alert"`.
    pub fn tone(mut self, tone: Tone) -> Self {
        self.tone = Some(tone);
        self
    }

    /// Soft (the default), surface or outline, as in Radix; ghost drops the frame and fill,
    /// and solid, which Radix's callout does not have, draws as soft.
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = Some(variant);
        self
    }

    /// Radix's sizes 1 to 3 (2 is the default): the padding, gaps and text.
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }
}

impl Render for Alert<'_> {
    fn render(&self) -> Markup {
        let icon = self.icon.unwrap_or(Glyph::Icon(match self.tone {
            Some(Tone::Danger | Tone::Warn) => Icon::TriangleAlert,
            Some(Tone::Ok) => Icon::CircleCheck,
            _ => Icon::Info,
        }));
        // The brand tone is the info callout's own scale.
        let level = match self.tone {
            Some(Tone::Brand | Tone::Info) | None => "info",
            Some(tone) => tone.as_str(),
        };
        let variant = match self.variant {
            Some(v @ (Variant::Surface | Variant::Outline)) => Some(v.as_str()),
            _ => None,
        };
        let size = (self.size != Size::S2).then(|| self.size.as_str());
        let id = self.id.map_or_else(
            || slug(self.title).trim_matches('-').to_string(),
            str::to_string,
        );
        let dismiss = self.dismiss.then_some(self.ui);
        if dismiss.is_some_and(|ui| ui.dismissed(&id)) {
            return Markup::default();
        }
        html! {
            div class={ "lui-alert lui-callout lui-callout-" (level) @if let Some(v) = variant { " lui-callout-" (v) } @if dismiss.is_some() { " lui-alert-dismissible" } }
                data-size=[size] data-tone=[self.tone.map(Tone::as_str)] data-variant=[self.variant.map(Variant::as_str)]
                role=(if self.tone == Some(Tone::Danger) { "alert" } else { "status" }) {
                (icon.hidden())
                p class="lui-alert-title" { (self.title) }
                @if let Some(d) = self.description { p class="lui-alert-description" { (d) } }
                @if let Some(b) = &self.body { div class="lui-alert-description" { (b) } }
                @if let Some(b) = &self.action { div class="lui-alert-actions" { (b) } }
                @if let Some(ui) = dismiss { (crate::banner::dismiss_form(ui, &id, self.title, "lui-alert-dismiss")) }
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
.lui-callout-neutral { --lui-callout-bg: var(--lui-gray-3); --lui-callout-fg: var(--lui-gray-12); --lui-callout-line: var(--lui-gray-7); }
.lui-callout-surface { background: color-mix(in srgb, var(--lui-callout-bg) 60%, var(--lui-bg)); border-color: var(--lui-callout-line); }
.lui-callout-outline { background: transparent; border-color: var(--lui-callout-line); }
.lui-callout[data-variant=ghost] { background: transparent; border-color: transparent; padding-inline: 0; }
/* Radix's sizes: 1 is tighter, 3 roomier with larger text. */
.lui-callout[data-size="1"] { padding: var(--lui-space-2) var(--lui-space-3); column-gap: var(--lui-space-2); font-size: 0.8125rem; }
.lui-callout[data-size="3"] { padding: var(--lui-space-4) var(--lui-space-6); column-gap: var(--lui-space-4); font-size: 1rem; line-height: 1.5rem; }
.lui-callout > .lui-icon { grid-row: 1 / span 2; margin-top: 0.125rem; }
.lui-callout > :not(.lui-icon) { grid-column: 2; margin: 0; }
.lui-callout a { color: inherit; text-decoration: underline; text-underline-offset: 2px; }
.lui-callout a:hover { text-decoration-thickness: 2px; }
.lui-alert-title { font-weight: 600; }
.lui-alert-description p { margin: 0; }
.lui-alert-actions { display: flex; flex-wrap: wrap; gap: var(--lui-space-2); margin-top: var(--lui-space-2) !important; }
.lui-alert-actions a { text-decoration: none; }
.lui-alert-dismissible { grid-template-columns: 1rem minmax(0, 1fr) auto; }
.lui-alert-dismiss { grid-column: 3 !important; grid-row: 1; margin: -0.375rem -0.5rem -0.375rem 0 !important; }
.lui-alert-dismiss .lui-button { color: inherit; }
"#;
