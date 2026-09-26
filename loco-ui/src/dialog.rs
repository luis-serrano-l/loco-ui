//! # Dialog
//!
//! A modal opened by a button and closed by a button, with no script. It can carry a title
//! with a close control, a size (Radix's 1–4), a `danger` variant, and a footer that is a real form: a
//! confirm button posting to a URL beside a cancel button, so "Delete account?" is one round
//! trip and the server redirects back to the page that opened it.
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
//! assert!(hi.render().into_string().contains(r#"commandfor="say-hi""#), "the id is the trigger's slug");
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
//! ```

use maud::{Markup, Render, html};

use crate::i18n::Text;
use crate::props::{Prop, PropKind};
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
/// `.returns_to(..)`, `.close(..)`, `.cancel(..)`, `.closedby(..)`, `.size(..)`; switches: `.small()`,
/// `.large()`, `.danger()`; from a condition: `.open(bool)`.
#[derive(Clone, Debug)]
pub struct Dialog<'a> {
    ui: &'a Ui,
    id: String,
    trigger: &'a str,
    body: Markup,
    open: bool,
    title: Option<&'a str>,
    size: DialogSize,
    danger: bool,
    confirm: Option<(&'a str, &'a str)>,
    returns_to: Option<&'a str>,
    close: &'a str,
    cancel: &'a str,
    closedby: &'a str,
}

impl Dialog<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("body", PropKind::Value, "body: Markup")
            .doc("What the dialog says."),
        Prop::new("id", PropKind::Value, "id: &str").attr("id")
            .doc("The dialog's id instead of the trigger's slug."),
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
    ];
}

impl Ui {
    /// A dialog opened by a button labelled `trigger`. Its id is the trigger's slug
    /// (`"Delete account"` is `delete-account`); it renders open when the URL says
    /// `?dialog=<id>`.
    pub fn dialog<'a>(&'a self, trigger: &'a str) -> Dialog<'a> {
        Dialog {
            ui: self,
            id: slug(trigger),
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
        }
    }
}

impl<'a> Dialog<'a> {
    /// What the dialog says.
    pub fn body(mut self, body: Markup) -> Self {
        self.body = body;
        self
    }

    /// The dialog's id instead of the trigger's slug; it must be unique on the page.
    pub fn id(mut self, id: &str) -> Self {
        self.id = id.to_string();
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

    /// The `closedby` attribute: `any` (Escape and outside click, the default),
    /// `closerequest` (Escape only) or `none`.
    pub fn closedby(mut self, closedby: &'a str) -> Self {
        self.closedby = closedby;
        self
    }
}

impl Render for Dialog<'_> {
    fn render(&self) -> Markup {
        let Dialog {
            ui,
            ref id,
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
        } = *self;
        let open = open || ui.state.dialog() == Some(id);
        let returns_to = returns_to.unwrap_or(ui.state.path());
        let returns_to = (!returns_to.is_empty()).then_some(returns_to);
        let invokers = ui.has(Cap::Invokers);
        let title_id = format!("{id}-title");
        let open_href = format!("#{id}");
        html! {
            div class={ "lui-dialog" @if danger { " lui-dialog-danger" } } {
                @if invokers {
                    (ui.button(trigger).command("show-modal", id).aria_haspopup("dialog"))
                } @else {
                    (ui.link_button(trigger, &open_href).class("lui-dialog-open").role("button"))
                }
                dialog id=(id) class=(size.class()) closedby=(closedby) aria-labelledby=[title.map(|_| &title_id)] open[open] {
                    @if let Some(t) = title { h2 id=(title_id) class="lui-dialog-title" { (t) } }
                    @if let Some((label, action)) = confirm {
                        form method="post" action=(action) class="lui-dialog-form" {
                            div class="lui-dialog-body" { (body) }
                            @if let Some(to) = returns_to { input type="hidden" name="returns_to" value=(to); }
                            div class="lui-dialog-actions" {
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
                                (ui.button(close).primary())
                            }
                        } @else {
                            p class="lui-dialog-actions" { (ui.link_button(close, "#").primary().role("button")) }
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
.lui-dialog-title { flex: none; font-size: 1.25rem; line-height: 1.75rem; font-weight: 700; letter-spacing: -0.01em; margin: 0 2.5rem var(--lui-space-2) 0; }
.lui-dialog-body { flex: 1 1 auto; min-height: 0; overflow-y: auto; font-size: 0.875rem; line-height: 1.25rem; color: var(--lui-muted); }
.lui-dialog-body > :first-child { margin-top: 0; }
.lui-dialog-body > :last-child { margin-bottom: 0; }
.lui-dialog-body label { color: var(--lui-fg); }
.lui-dialog-actions { flex: none; display: flex; flex-wrap: wrap-reverse; justify-content: flex-end; gap: var(--lui-space-3); margin: var(--lui-space-6) 0 0; }
/* The close × is a ghost icon button in the corner (44px itself on a coarse pointer, where
   the small control height is --lui-hit), 70% opacity until hovered. */
.lui-dialog-close { position: absolute; top: var(--lui-space-3); right: var(--lui-space-3); opacity: 0.7; }
.lui-dialog-close:hover { opacity: 1; }

/* Narrow screens: the footer stacks, full width, confirm on top. */
@media (max-width: 30rem) {
  .lui-dialog-actions { flex-direction: column-reverse; align-items: stretch; }
  .lui-dialog-actions > * { width: 100%; }
}

/* :target fallback: a dialog that is the URL fragment renders as a fixed overlay. */
.lui-dialog dialog:target {
  display: block; position: fixed; inset: 0; margin: auto; height: fit-content; z-index: 10;
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
