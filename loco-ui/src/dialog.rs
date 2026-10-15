//! # Dialog
//!
//! A modal opened by a button and closed by a button, with no script. It can carry a title
//! with a close control, a size (Radix's 1–4), a `danger` variant, and a footer that is a real form: a
//! confirm button posting to a URL beside a cancel button, so "Delete account?" is one round
//! trip and the server redirects back to the page that opened it. `.description(..)` adds a
//! line under the title that names the dialog's purpose to assistive tech too,
//! `.footer(..)` puts markup at the start of the button row, and `.fullscreen()` fills the
//! viewport.
//!
//! **Platform features:**
//! - `<dialog>` element (baseline 2022), its `::backdrop`, and `closedby` (Chrome 134+,
//!   Firefox 141+, Safari 26+) for Escape and light dismiss.
//! - Invoker commands: `<button command="show-modal" commandfor="id">` and `command="close"`
//!   (Chrome 135+, Firefox 144+, Safari 26.2+). Closing without a confirm form uses
//!   `<form method="dialog">` (baseline 2022).
//! - Motion: `@starting-style` (Chrome 117, Firefox 129, Safari 17.5) and
//!   `transition-behavior: allow-discrete` on `display` and `overlay` (Chrome 117,
//!   Firefox 129, Safari 17.4) fade and scale the dialog and its backdrop in and out; older
//!   browsers open and close it at once, and `prefers-reduced-motion: reduce` zeroes it.
//! - Focus: the close control sits last in the markup, so the dialog's own focusing steps land
//!   on the first field in the body, then on the confirm button.
//!
//! **Accessibility:** a native `<dialog>` opened modally: focus moves in, Escape closes, the
//! page behind is inert; named by its title through `aria-labelledby`. Checked by axe-core in
//! headless Firefox on every demo route, both capability variants, light and dark (no serious
//! or critical violation).
//!
//! **What it does not do without script:** return focus to the opener in the `:target`
//! fallback, trap focus there, or post a form and close without reloading.
//!
//! **Fallback:** when `Caps` lacks `Invokers`, the opener is a link to `#id` and a `:target`
//! rule shows the dialog as a fixed overlay; links to `#` close and cancel it. Only one variant
//! is ever in the markup.
//!
//! **Server state:** `?dialog=<id>` in the URL renders the dialog already open (non-modal, no
//! backdrop), as does `.open(true)`. The confirm form goes back to the page it came from
//! unless `returns_to` says otherwise. `returns_to` is posted
//! with the confirm form as a hidden `returns_to` field, so the handler knows where to send
//! the browser back; check it is a local path before redirecting to it.
//!
//! **Without script:** `closedby` is honoured by the real `<dialog>` only; the `:target`
//! fallback closes through its links alone. Focus is not trapped in the fallback.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::from(Caps::all());
//! let hi = ui.dialog("Say hi").body(html! { p { "Hello." } });
//! assert!(hi.render().into_string().contains(r#"commandfor="lui-dialog-say-hi""#), "the id is the trigger's slug");
//!
//! let m = ui.dialog("Delete account")
//!     .id("confirm")
//!     .title("Delete account?")
//!     .size(1)
//!     .danger()
//!     .confirm("Delete", "/account/delete")
//!     .returns_to("/settings")
//!     .cancel("Keep it")
//!     .closedby("closerequest")
//!     .open(true)
//!     .body(html! {
//!         p { "This cannot be undone." }
//!         label { "Reason " input name="reason"; }
//!     });
//! let html = m.render().into_string();
//! assert!(html.contains("<dialog id=\"confirm\" class=\"lui-dialog-sm\" closedby=\"closerequest\" aria-labelledby=\"confirm-title\" open>"));
//! assert!(html.contains("<form method=\"post\" action=\"/account/delete\""));
//! assert!(html.contains("name=\"returns_to\" value=\"/settings\""));
//! // The same in `lui!`:
//! let same = lui! { Dialog("Delete account") id="confirm" title="Delete account?" size=1 danger
//!     confirm=("Delete", "/account/delete") returns_to="/settings" cancel="Keep it"
//!     closedby="closerequest" open=(true) {
//!     p { "This cannot be undone." }
//!     label { "Reason " input name="reason"; }
//! } };
//! assert_eq!(same.into_string(), html);
//!
//! // From the request: open when the URL says `?dialog=confirm`, and the confirm form
//! // returns to this page.
//! let ui = Ui::from_request("/account", "dialog=confirm", "");
//! let html = ui.dialog("Delete account").id("confirm").confirm("Delete", "/account/delete").render().into_string();
//! assert!(html.contains(" open>") && html.contains("value=\"/account\""));
//!
//! // Radix's four sizes; `.small()` and `.large()` are sizes 1 and 3.
//! let wide = ui.dialog("Terms").size(4).render().into_string();
//! assert!(wide.contains(r#"class="lui-dialog-xl""#));
//! assert_eq!(lui! { Dialog("Terms") size=4; }.into_string(), wide);
//!
//! // A description, markup in the footer, and the whole viewport.
//! let m = ui.dialog("Edit photo").title("Edit photo").description("Crop and rotate before saving.")
//!     .footer(html! { a href="/help" { "Help" } }).fullscreen().render().into_string();
//! assert!(m.contains(r#"class="lui-dialog-md lui-dialog-full""#) && m.contains(r#"aria-describedby="lui-dialog-edit-photo-description""#));
//! assert!(m.contains(r#"<p id="lui-dialog-edit-photo-description" class="lui-dialog-description">"#) && m.contains(r#"<div class="lui-dialog-footer"><a href="/help">"#));
//! let same = lui! { Dialog("Edit photo") title="Edit photo" description="Crop and rotate before saving."
//!     footer={ a href="/help" { "Help" } } fullscreen; };
//! assert_eq!(same.into_string(), m);
//!
//! // A styled opener of your own; the dialog sets its command. Or none at all.
//! let ui = Ui::from(Caps::all());
//! let styled = ui.dialog("Delete").trigger(ui.button("Delete").danger().leading(Icon::Trash)).render().into_string();
//! assert!(styled.contains(r#"lui-button-danger""#) && styled.contains(r#"commandfor="lui-dialog-delete""#));
//! assert_eq!(lui! { Dialog("Delete") trigger=(ui.button("Delete").danger().leading(Icon::Trash)); }.into_string(), styled);
//! let bare = ui.dialog("Terms").no_trigger().render().into_string();
//! assert!(bare.starts_with(r#"<div class="lui-dialog"><dialog id="lui-dialog-terms""#));
//! ```

use maud::{Markup, Render, html};

use crate::button::Button;
use crate::i18n::Text;
use crate::props::{Key, Prop, PropKind};
use crate::{Cap, Icon, Ui, slug};

/// Width of a dialog, Radix's sizes 1–4: `max-width` of 20, 32, 42 or 56 rem.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum DialogSize {
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl DialogSize {
    fn class(self) -> &'static str {
        match self {
            DialogSize::Sm => "lui-dialog-sm",
            DialogSize::Md => "lui-dialog-md",
            DialogSize::Lg => "lui-dialog-lg",
            DialogSize::Xl => "lui-dialog-xl",
        }
    }
}

/// A modal dialog behind a button, made by [`Ui::dialog`]: medium width, untitled, closed by
/// a "Close" button, unless told otherwise.
///
/// **Setters.** Values and items: `.body(..)`, `.id(..)`, `.title(..)`, `.confirm(..)`,
/// `.returns_to(..)`, `.close(..)`, `.cancel(..)`, `.closedby(..)`, `.size(..)`, `.trigger(..)`,
/// `.description(..)`, `.footer(..)`; switches: `.small()`, `.large()`, `.danger()`, `.no_trigger()`,
/// `.fullscreen()`; from a condition: `.open(bool)`.
#[derive(Clone, Debug)]
pub struct Dialog<'a> {
    ui: &'a Ui,
    id: Option<&'a str>,
    trigger: &'a str,
    body: Markup,
    description: Option<&'a str>,
    footer: Option<Markup>,
    fullscreen: bool,
    open: bool,
    title: Option<&'a str>,
    size: DialogSize,
    danger: bool,
    confirm: Option<(&'a str, &'a str)>,
    returns_to: Option<&'a str>,
    close: &'a str,
    cancel: &'a str,
    closedby: &'a str,
    button: Option<Button<'a>>,
    no_trigger: bool,
}

impl Dialog<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [crate::props::Key] = &[
        Key::new("Enter / Space", "Opens the dialog from its button."),
        Key::new(
            "Tab / Shift+Tab",
            "Moves between the dialog's controls while the page behind it is inert.",
        ),
        Key::new(
            "Enter",
            "Presses the focused button, or sends the confirm form from a field in it.",
        ),
        Key::new(
            "Escape",
            "Closes the dialog and returns focus to its button, unless `closedby` is \"none\". Without invoker commands or the script, its Close link closes it instead.",
        ),
    ];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("body", PropKind::Value, "body: Markup")
            .doc("What the dialog says."),
        Prop::new("id", PropKind::Value, "id: &'a str").attr("id")
            .doc("The dialog's id instead of `lui-dialog-` and the trigger's slug."),
        Prop::new("open", PropKind::Condition, "open: bool").attr("open")
            .doc("Render the dialog already open (non-modal, no backdrop)."),
        Prop::new("title", PropKind::Value, "title: &'a str")
            .doc("A title in a header with a close control."),
        Prop::new("size", PropKind::Value, "size: u8")
            .default("2")
            .doc("Radix's sizes 1–4: 20, 32, 42 or 56 rem wide, roomier padding as it grows."),
        Prop::new("small", PropKind::Switch, "")
            .doc("`.size(1)`: 20 rem wide."),
        Prop::new("large", PropKind::Switch, "")
            .doc("`.size(3)`: 42 rem wide."),
        Prop::new("danger", PropKind::Switch, "")
            .doc("A red solid confirm button."),
        Prop::new("description", PropKind::Value, "text: &'a str")
            .doc("A line under the title, tied to the dialog by `aria-describedby`."),
        Prop::new("footer", PropKind::Value, "footer: Markup")
            .doc("Markup at the start of the button row (a link, a note, a checkbox)."),
        Prop::new("fullscreen", PropKind::Switch, "")
            .doc("Fill the viewport, without the rounded frame."),
        Prop::new("confirm", PropKind::Value, "label: &'a str, action: &'a str")
            .doc("A confirm button labelled `label`."),
        Prop::new("returns_to", PropKind::Value, "path: &'a str")
            .doc("Where the server should send the browser after the confirm form, posted as a hidden `returns_to` field."),
        Prop::new("close", PropKind::Value, "label: &'a str").default("Close")
            .doc("Label of the closing button when there is no confirm form."),
        Prop::new("cancel", PropKind::Value, "label: &'a str").default("Cancel")
            .doc("Label of the cancel button beside the confirm button."),
        Prop::new("closedby", PropKind::Value, "closedby: &'a str").default("any").attr("closedby")
            .doc("The `closedby` attribute."),
        Prop::new("trigger", PropKind::Value, "button: Button<'a>")
            .doc("The opener as a styled button of the caller's (`ui.button(..).danger()`); its command or link is the dialog's."),
        Prop::new("no_trigger", PropKind::Switch, "")
            .doc("No opener: a link to `?dialog=<id>` or a button elsewhere opens it."),
    ];
}

impl Ui {
    /// A dialog opened by a button labelled `trigger`. Its id is `lui-dialog-` and the
    /// trigger's slug (`"Delete account"` is `lui-dialog-delete-account`); it renders open
    /// when the URL says `?dialog=<id>`.
    pub fn dialog<'a>(&'a self, trigger: &'a str) -> Dialog<'a> {
        Dialog {
            description: None,
            footer: None,
            fullscreen: false,
            ui: self,
            id: None,
            trigger,
            body: Markup::default(),
            open: false,
            title: None,
            size: DialogSize::Md,
            danger: false,
            confirm: None,
            returns_to: None,
            close: self.text(Text::Close),
            cancel: self.text(Text::Cancel),
            closedby: "any",
            button: None,
            no_trigger: false,
        }
    }
}

impl<'a> Dialog<'a> {
    /// What the dialog says.
    pub fn body(mut self, body: Markup) -> Self {
        self.body = body;
        self
    }

    /// The dialog's id instead of `lui-dialog-` and the trigger's slug; it must be unique on
    /// the page.
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }

    /// Render the dialog already open (non-modal, no backdrop).
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// A title in a header with a close control; also the dialog's accessible name.
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    /// Radix's sizes 1–4 (clamped): 20 rem (a confirmation), 32 (the default), 42 (a long
    /// form) or 56 rem wide (a table), with roomier padding as it grows.
    pub fn size(mut self, size: u8) -> Self {
        self.size = match size {
            0 | 1 => DialogSize::Sm,
            2 => DialogSize::Md,
            3 => DialogSize::Lg,
            _ => DialogSize::Xl,
        };
        self
    }

    /// `.size(1)`: 20 rem wide, a confirmation.
    pub fn small(self) -> Self {
        self.size(1)
    }

    /// `.size(3)`: 42 rem wide, a long form or a table.
    pub fn large(self) -> Self {
        self.size(3)
    }

    /// A red solid confirm button (Radix AlertDialog's): for destructive actions.
    pub fn danger(mut self) -> Self {
        self.danger = true;
        self
    }

    /// A confirm button labelled `label`: the footer becomes `<form method="post">` to
    /// `action`, and the body's fields are posted with it.
    pub fn confirm(mut self, label: &'a str, action: &'a str) -> Self {
        self.confirm = Some((label, action));
        self
    }

    /// Where the server should send the browser after the confirm form, posted as a hidden
    /// `returns_to` field; this page by default. Check it is a local path before
    /// redirecting to it.
    pub fn returns_to(mut self, path: &'a str) -> Self {
        self.returns_to = Some(path);
        self
    }

    /// Label of the closing button when there is no confirm form.
    pub fn close(mut self, label: &'a str) -> Self {
        self.close = label;
        self
    }

    /// Label of the cancel button beside the confirm button.
    pub fn cancel(mut self, label: &'a str) -> Self {
        self.cancel = label;
        self
    }

    /// The opener as a styled button of the caller's (`ui.button("Delete").danger()`) in place
    /// of the plain one: its look, icons and size are kept, and the dialog sets what opens it
    /// (`command="show-modal"`, or a link to `#id` without invoker commands).
    pub fn trigger(mut self, button: Button<'a>) -> Self {
        self.button = Some(button);
        self
    }

    /// No opener: the dialog alone, opened by `?dialog=<id>`, a link to `#id` or a button
    /// elsewhere with `.command("show-modal", id)`.
    pub fn no_trigger(mut self) -> Self {
        self.no_trigger = true;
        self
    }

    /// The `closedby` attribute: `any` (Escape and outside click, the default),
    /// `closerequest` (Escape only) or `none`.
    pub fn closedby(mut self, closedby: &'a str) -> Self {
        self.closedby = closedby;
        self
    }

    /// A line under the title, tied to the dialog by `aria-describedby` so a screen reader
    /// says it on opening.
    pub fn description(mut self, text: &'a str) -> Self {
        self.description = Some(text);
        self
    }

    /// Markup at the start of the button row (a link, a note, a checkbox inside the confirm
    /// form); the buttons stay at the end.
    pub fn footer(mut self, footer: Markup) -> Self {
        self.footer = Some(footer);
        self
    }

    /// Fill the viewport, without the rounded frame: an editor, a long form on a phone.
    pub fn fullscreen(mut self) -> Self {
        self.fullscreen = true;
        self
    }
}

impl Render for Dialog<'_> {
    fn render(&self) -> Markup {
        let Dialog {
            ui,
            id,
            trigger,
            ref body,
            open,
            title,
            size,
            danger,
            confirm,
            returns_to,
            close,
            cancel,
            closedby,
            ref button,
            no_trigger,
            description,
            ref footer,
            fullscreen,
        } = *self;
        let id = &id.map_or_else(|| format!("lui-dialog-{}", slug(trigger)), str::to_string);
        let desc_id = format!("{id}-description");
        let footer = footer
            .as_ref()
            .map(|f| html! { div class="lui-dialog-footer" { (f) } });
        let open = open || ui.state.dialog_is(id);
        let returns_to = returns_to.unwrap_or(ui.state.path());
        let returns_to = (!returns_to.is_empty()).then_some(returns_to);
        let invokers = ui.has(Cap::Invokers);
        let title_id = format!("{id}-title");
        let open_href = format!("#{id}");
        html! {
            div class={ "lui-dialog" @if danger { " lui-dialog-danger" } } {
                @let opener = button.clone().unwrap_or_else(|| ui.button(trigger));
                @if no_trigger {
                } @else if invokers {
                    (opener.command("show-modal", id).aria_haspopup("dialog"))
                } @else {
                    (opener.linking(&open_href).part("lui-dialog-open"))
                }
                dialog id=(id) class={ (size.class()) @if fullscreen { " lui-dialog-full" } } closedby=(closedby) aria-labelledby=[title.map(|_| &title_id)]
                    aria-describedby=[description.map(|_| &desc_id)] open[open] {
                    @if let Some(t) = title { h2 id=(title_id) class="lui-dialog-title" { (t) } }
                    @if let Some(d) = description { p id=(desc_id) class="lui-dialog-description" { (d) } }
                    @if let Some((label, action)) = confirm {
                        form method="post" action=(action) class="lui-dialog-form" {
                            div class="lui-dialog-body" { (body) }
                            @if let Some(to) = returns_to { input type="hidden" name="returns_to" value=(to); }
                            div class="lui-dialog-actions" {
                                (footer.clone().unwrap_or_default())
                                @if invokers {
                                    (ui.button(cancel).command("close", id))
                                } @else {
                                    (ui.link_button(cancel, "#").class("lui-dialog-cancel").role("button"))
                                }
                                @if danger { (ui.button(label).danger()) } @else { (ui.button(label).primary()) }
                            }
                        }
                    } @else {
                        div class="lui-dialog-body" { (body) }
                        @if invokers {
                            form method="dialog" class="lui-dialog-actions" {
                                (footer.clone().unwrap_or_default())
                                (ui.button(close).primary())
                            }
                        } @else {
                            div class="lui-dialog-actions" { (footer.clone().unwrap_or_default()) (ui.link_button(close, "#").primary().role("button")) }
                        }
                    }
                    // Last in the markup so the dialog's focusing steps skip it for the first field.
                    @if title.is_some() {
                        @let x = html! { (Icon::X) };
                        @if invokers {
                            (ui.button("").ghost().small().icon_only().class("lui-dialog-close").aria_label(ui.text(Text::Close)).body(x).command("close", id))
                        } @else {
                            (ui.link_button("", "#").ghost().small().icon_only().class("lui-dialog-close").aria_label(ui.text(Text::Close)).body(x))
                        }
                    }
                }
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
.lui-dialog { display: inline-flex; flex-wrap: wrap; gap: var(--lui-space-2); align-items: center; }
/* After Radix Themes Dialog and AlertDialog: a popover surface with a large radius and shadow
   over the --lui-overlay backdrop; max width and padding by size (1–4); the title at the
   heading size, the body in --lui-gray-11, actions at the end. The width is the viewport less a
   16px gutter either side, and a tall body scrolls inside while title and actions stay. */
.lui-dialog dialog {
  --lui-dialog-pad: var(--lui-space-6);
  box-sizing: border-box; width: calc(100% - 2rem); max-height: calc(100dvh - 2rem);
  background: var(--lui-popover); color: var(--lui-fg);
  border: 1px solid var(--lui-line); border-radius: var(--lui-radius-lg);
  padding: var(--lui-dialog-pad); box-shadow: var(--lui-shadow-lg), var(--lui-highlight);
}
.lui-dialog dialog[open], .lui-dialog dialog:target { display: flex; flex-direction: column; }
.lui-dialog-form { display: flex; flex-direction: column; min-height: 0; }
/* Server-opened (non-modal) dialogs sit in the flow; positioned so the close control anchors. */
.lui-dialog dialog:not(:modal):not(:target) { position: relative; }
.lui-dialog dialog.lui-dialog-sm { max-width: 20rem; --lui-dialog-pad: var(--lui-space-4); }
.lui-dialog dialog.lui-dialog-md { max-width: 32rem; }
.lui-dialog dialog.lui-dialog-lg { max-width: 42rem; }
.lui-dialog dialog.lui-dialog-xl { max-width: 56rem; --lui-dialog-pad: var(--lui-space-8); }
.lui-dialog dialog::backdrop { background: var(--lui-overlay); }
.lui-dialog dialog h2 { margin-top: 0; }
.lui-dialog-title { flex: none; font-size: var(--lui-text-5); line-height: 1.75rem; font-weight: 700; letter-spacing: -0.01em; margin: 0 2.5rem var(--lui-space-2) 0; }
.lui-dialog-body { flex: 1 1 auto; min-height: 0; overflow-y: auto; font-size: var(--lui-text-2); line-height: 1.25rem; color: var(--lui-muted); }
.lui-dialog-body > :first-child { margin-top: 0; }
.lui-dialog-body > :last-child { margin-bottom: 0; }
.lui-dialog-body label { color: var(--lui-fg); }
.lui-dialog-actions { flex: none; display: flex; flex-wrap: wrap-reverse; justify-content: flex-end; gap: var(--lui-space-3); margin: var(--lui-space-6) 0 0; }
/* The close × is a ghost icon button in the corner (44px itself on a coarse pointer, where
   the small control height is --lui-hit), 70% opacity until hovered. */
.lui-dialog-close { position: absolute; top: var(--lui-space-3); inset-inline-end: var(--lui-space-3); opacity: 0.7; }
.lui-dialog-close:hover { opacity: 1; }
.lui-dialog-description { flex: none; margin: 0 2.5rem var(--lui-space-3) 0; font-size: var(--lui-text-2); line-height: var(--lui-leading-2); color: var(--lui-muted); }
.lui-dialog-footer { display: flex; flex-wrap: wrap; align-items: center; gap: var(--lui-space-2); margin-inline-end: auto; font-size: var(--lui-text-2); }
/* .fullscreen(): the whole viewport, square corners, no border. */
.lui-dialog dialog.lui-dialog-full { width: 100%; max-width: none; height: 100dvh; max-height: none; margin: 0; border: 0; border-radius: 0; }

/* Narrow screens: the footer stacks, full width, confirm on top. */
@media (max-width: 30rem) {
  .lui-dialog-actions { flex-direction: column-reverse; align-items: stretch; }
  .lui-dialog-actions > * { width: 100%; }
}

/* :target fallback: a dialog that is the URL fragment renders as a fixed overlay. */
.lui-dialog dialog:target {
  display: block; position: fixed; inset: 0; margin: auto; height: fit-content; z-index: var(--lui-z-dialog);
  box-shadow: var(--lui-shadow-lg), var(--lui-highlight), 0 0 0 100vmax var(--lui-overlay);
}

/* Motion: a fade and a small scale in and out, the backdrop fading with it. display and
   overlay transition as discrete properties, so a closing modal stays in the top layer until
   its fade ends; @starting-style gives an opening one (or a :target one) its first frame.
   Browsers without transition-behavior drop these transitions and open and close at once. */
.lui-dialog dialog {
  transition: opacity var(--lui-duration) var(--lui-ease-out), scale var(--lui-duration-slow) var(--lui-ease-spring),
    display var(--lui-duration) allow-discrete, overlay var(--lui-duration) allow-discrete;
}
.lui-dialog dialog:not([open]):not(:target) { opacity: 0; scale: 0.96; }
@starting-style { .lui-dialog dialog[open], .lui-dialog dialog:target { opacity: 0; scale: 0.96; } }
.lui-dialog dialog::backdrop {
  transition: opacity var(--lui-duration) var(--lui-ease-out), display var(--lui-duration) allow-discrete, overlay var(--lui-duration) allow-discrete;
}
.lui-dialog dialog:not([open])::backdrop { opacity: 0; }
@starting-style { .lui-dialog dialog[open]::backdrop { opacity: 0; } }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_opens_the_named_dialog_and_returns_to_its_page() {
        let ui = Ui::from_request("/dialog", "dialog=confirm", "");
        let with = |id: &str, to: Option<&str>| {
            let d = ui.dialog("Open").id(id).confirm("Go", "/go");
            to.map_or(d.clone(), |to| d.returns_to(to))
                .render()
                .into_string()
        };
        let named = with("confirm", None);
        assert!(named.contains(" open>") && named.contains(r#"name="returns_to" value="/dialog""#));
        assert!(!with("other", None).contains(" open>"));
        let explicit = with("confirm", Some("/home"));
        assert!(explicit.contains(r#"value="/home""#) && !explicit.contains(r#"value="/dialog""#));
        assert!(
            !Ui::default()
                .dialog("Open")
                .confirm("Go", "/go")
                .render()
                .into_string()
                .contains("returns_to"),
            "no request, no return path"
        );
    }
}
