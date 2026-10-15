//! # Popover menu
//!
//! A dropdown menu that opens on click, closes on outside click or Escape, no script. Items
//! are links, `<form method="post">` actions, checkboxes and radio buttons that post their
//! own form, headings, separators and submenus; each item can show an icon and a keyboard
//! shortcut and be disabled or destructive.
//!
//! **Platform features:**
//! - `popover` attribute + `popovertarget` button (Chrome 114, Firefox 125, Safari 17). Light
//!   dismiss and top-layer stacking come for free; a submenu is a nested popover, which the
//!   platform keeps open with its parent.
//! - CSS anchor positioning `anchor-name` / `position-anchor` / `position-area`
//!   (Chrome 125, Firefox 147, Safari 26) to place the menu under its button, at its end, or
//!   to its right (`.align_end()`, `.open_right()`), with `position-try-fallbacks` (Chrome
//!   128, Firefox 147, Safari 26) flipping it when there is no room; the areas are the
//!   generic [`crate::popover_card::Popover`]'s.
//! - A checkbox or radio item (`.checkbox(..)`, `.radio(..)`) is a native control with
//!   `role="menuitemcheckbox"` or `menuitemradio` in its own `<form method="post">`, with an
//!   Apply button beside it. The enhancement script submits the form on `change`
//!   (`data-lui="autosubmit"`) and then hides the Apply buttons.
//! - Motion: `@starting-style` (Chrome 117, Firefox 129, Safari 17.5) and
//!   `transition-behavior: allow-discrete` on `display` and `overlay` (Chrome 117,
//!   Firefox 129, Safari 17.4) fade the menu in and out with a 4px drop; the `<details>`
//!   fallback fades in only. Older browsers show and hide it at once.
//!
//! **Accessibility:** a `popover` menu: `aria-haspopup="menu"` on the trigger, `role="menu"`
//! and `menuitem` on the items, Escape and a click outside close it; the script adds arrow
//! keys. Checked by axe-core in headless Firefox on every demo route, both capability variants,
//! light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** position itself against the opener where anchor
//! positioning is missing; it is centred instead.
//!
//! **Fallback:** without `Caps::Anchor` the popover is UA-centred, which is still usable.
//! Without `Caps::Popover` the menu is a `<details>` dropdown (a submenu a nested one): it
//! opens and closes on click but has no light dismiss.
//!
//! A checkbox or radio item needs its Apply button pressed without the script.
//!
//! **Without script:** a shortcut shown beside an item is a label; binding the key needs
//! script the crate does not ship. Arrow keys move between items only with the enhancement
//! script; Tab always works.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::from(Caps::all());
//! // The id is `lui-menu-` and the label's slug: this menu is `#lui-menu-account`.
//! let m = ui.menu("Account").link("Profile", "/profile").link("Sign out", "/logout");
//! assert!(m.render().into_string().contains(r#"id="lui-menu-account""#));
//! // `icon`, `shortcut`, `disabled` and `danger` apply to the item just added.
//! let m = ui.menu("Account")
//!     .group("Signed in as Ada")
//!     .link("Profile", "/profile").icon("@").shortcut("g p")
//!     .link("Billing", "/billing").disabled()
//!     .separator()
//!     .submenu("Theme", [("Light", "/?t=light"), ("Dark", "/?t=dark")])
//!     .separator()
//!     .action("Sign out", "/logout").danger()
//!     .align_end();
//! let html = m.render().into_string();
//! assert!(html.contains("<form method=\"post\" action=\"/logout\""));
//! assert!(html.contains("position-area: block-end span-inline-start") && html.contains(r#"id="lui-menu-account-theme""#));
//! // The same in `lui!`:
//! let same = lui! { Menu("Account") align_end {
//!     group "Signed in as Ada";
//!     link "Profile" "/profile" icon="@" shortcut="g p";
//!     link "Billing" "/billing" disabled;
//!     separator();
//!     submenu "Theme" ([("Light", "/?t=light"), ("Dark", "/?t=dark")]);
//!     separator();
//!     action "Sign out" "/logout" danger;
//! } };
//! assert_eq!(same.into_string(), html);
//! // A styled opener of your own: the menu sets its popover target.
//! let more = ui.button("").ghost().icon_only().aria_label("More").leading(Icon::Ellipsis);
//! let m = ui.menu("More").link("Rename", "/rename").trigger(more).render().into_string();
//! assert!(m.contains(r#"class="lui-button lui-button-ghost lui-button-icon""#) && m.contains(r#"popovertarget="lui-menu-more""#));
//! // Checkbox and radio items post their own form; `checked` applies to the item just added.
//! let m = ui.menu("View")
//!     .checkbox("Show grid", "/view", "grid").checked(true)
//!     .radio("Compact", "/view", "density", "compact")
//!     .radio("Comfortable", "/view", "density", "comfortable").checked(true);
//! let html = m.render().into_string();
//! assert!(html.contains(r#"<form method="post" action="/view" data-lui="autosubmit">"#));
//! assert!(html.contains(r#"<input class="lui-popover-check" type="checkbox" role="menuitemcheckbox" name="grid" value="on" checked>"#));
//! assert!(html.contains(r#"type="radio" role="menuitemradio" name="density" value="comfortable" checked>"#));
//! let same = lui! { Menu("View") {
//!     checkbox "Show grid" "/view" "grid" checked=(true);
//!     radio "Compact" "/view" "density" "compact";
//!     radio "Comfortable" "/view" "density" "comfortable" checked=(true);
//! } };
//! assert_eq!(same.into_string(), html);
//! ```

use maud::{Markup, Render, html};

use crate::button::Button;
use crate::i18n::Text;
use crate::icon::Glyph;
use crate::popover_card::{Align, Side, area};
use crate::props::{Key, Prop, PropKind};
use crate::{Cap, Caps, Icon, Ui, slug};

/// Where the menu opens relative to its button.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum Placement {
    /// Below, left edges aligned.
    #[default]
    BottomStart,
    /// Below, right edges aligned.
    BottomEnd,
    /// To the right, top edges aligned.
    Right,
}

impl Placement {
    fn area(self) -> &'static str {
        match self {
            Placement::BottomStart => area(Side::Bottom, Align::Start),
            Placement::BottomEnd => area(Side::Bottom, Align::End),
            Placement::Right => area(Side::End, Align::Start),
        }
    }

    fn class(self) -> &'static str {
        match self {
            Placement::BottomStart => "lui-popover-start",
            Placement::BottomEnd => "lui-popover-end",
            Placement::Right => "lui-popover-right",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Kind<'a> {
    Link(&'a str),
    Action(&'a str),
    Heading,
    Separator,
    Submenu(Vec<MenuItem<'a>>),
    /// A checkbox posting `name=on` (or nothing) to the action, and its Apply button's text.
    Check(&'a str, &'a str, &'static str),
    /// A radio button posting `name=value` to the action, and its Apply button's text.
    Radio(&'a str, &'a str, &'a str, &'static str),
}

/// One entry of a menu, for the places that take a list of them (a table row's menu). A
/// [`Menu`] builds its own with [`Menu::link`] and friends.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MenuItem<'a> {
    kind: Kind<'a>,
    text: &'a str,
    pub(crate) icon: Option<Glyph<'a>>,
    pub(crate) shortcut: Option<&'a str>,
    pub(crate) disabled: bool,
    pub(crate) danger: bool,
    pub(crate) description: Option<&'a str>,
    pub(crate) checked: bool,
}

impl<'a> MenuItem<'a> {
    const fn new(kind: Kind<'a>, text: &'a str) -> Self {
        MenuItem {
            kind,
            text,
            icon: None,
            shortcut: None,
            disabled: false,
            danger: false,
            description: None,
            checked: false,
        }
    }

    /// A link.
    pub const fn link(text: &'a str, href: &'a str) -> Self {
        Self::new(Kind::Link(href), text)
    }

    /// A `<form method="post">` button posting to `action`: for things that change state.
    pub const fn action(text: &'a str, action: &'a str) -> Self {
        Self::new(Kind::Action(action), text)
    }

    /// A heading over the items after it.
    pub(crate) const fn group(text: &'a str) -> Self {
        Self::new(Kind::Heading, text)
    }

    /// A rule between groups.
    pub(crate) const fn separator() -> Self {
        Self::new(Kind::Separator, "")
    }

    /// The name of the setter that adds an item of this kind, for the modifiers' check.
    pub(crate) const fn adder(&self) -> &'static str {
        match self.kind {
            Kind::Link(_) => "link",
            Kind::Action(_) => "action",
            Kind::Heading => "group",
            Kind::Separator => "separator",
            Kind::Submenu(_) => "submenu",
            Kind::Check(..) => "checkbox",
            Kind::Radio(..) => "radio",
        }
    }

    /// Destructive: coloured with `--lui-danger`.
    pub const fn danger(mut self) -> Self {
        self.danger = true;
        self
    }

    /// A line of muted text under the item's own (a navigation menu's panel links).
    pub const fn description(mut self, text: &'a str) -> Self {
        self.description = Some(text);
        self
    }
}

/// `("Mail", "/mail", "Read and send")`: a link with a line of description under it.
impl<'a> From<(&'a str, &'a str, &'a str)> for MenuItem<'a> {
    fn from((text, href, description): (&'a str, &'a str, &'a str)) -> Self {
        MenuItem::link(text, href).description(description)
    }
}

/// `("Profile", "/profile")`: a link, its text and where it goes.
impl<'a> From<(&'a str, &'a str)> for MenuItem<'a> {
    fn from((text, href): (&'a str, &'a str)) -> Self {
        MenuItem::link(text, href)
    }
}

/// A button that opens a menu, made by [`Ui::menu`]. Items are added in order; `icon`,
/// `shortcut`, `disabled` and `danger` apply to the item added last. Opens below the button,
/// start-aligned, unless told otherwise.
///
/// **Setters.** Values and items: `.submenu(..)`, `.link(..)`, `.action(..)`, `.checkbox(..)`,
/// `.radio(..)`, `.group(..)`, `.icon(..)`, `.shortcut(..)`, `.id(..)`, `.trigger(..)`; switches:
/// `.separator()`, `.disabled()`, `.danger()`, `.align_end()`, `.open_right()`; from a
/// condition: `.checked(bool)`.
#[derive(Clone, Debug)]
pub struct Menu<'a> {
    caps: Caps,
    apply: &'static str,
    id: Option<&'a str>,
    label: &'a str,
    items: Vec<MenuItem<'a>>,
    placement: Placement,
    button: Option<Button<'a>>,
}

/// The items a menu's modifiers change: everything but a group heading and a separator.
pub(crate) const ITEMS: &[&str] = &["link", "action", "submenu", "checkbox", "radio"];

impl Menu<'_> {
    /// The keys it answers to, listed by [`crate::props()`] and shown on its docs page.
    pub const KEYS: &'static [crate::props::Key] = &[
        Key::new("Enter / Space", "Opens the menu from its button."),
        Key::new(
            "Tab / Shift+Tab",
            "Moves to the next or previous item in the open menu.",
        ),
        Key::new(
            "ArrowDown / ArrowUp",
            "Moves to the next or previous item, wrapping at the ends.",
        )
        .script(),
        Key::new(
            "Enter",
            "Follows the focused link or sends the focused action.",
        ),
        Key::new("Space", "Ticks the focused checkbox or radio item."),
        Key::new(
            "Escape",
            "Closes the menu and returns focus to its button. Without popover support the script closes it; without the script either, its button does.",
        ),
    ];

    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("link", PropKind::Item, "text: &'a str, href: &'a str").doc("A link."),
        Prop::new("action", PropKind::Item, "text: &'a str, action: &'a str")
            .attr("action")
            .doc("A `<form method=\"post\">` button posting to `action`."),
        Prop::new(
            "checkbox",
            PropKind::Item,
            "text: &'a str, action: &'a str, name: &'a str",
        )
        .doc("A checkbox in its own form posting `name=on` (or nothing) to `action`."),
        Prop::new(
            "radio",
            PropKind::Item,
            "text: &'a str, action: &'a str, name: &'a str, value: &'a str",
        )
        .doc("A radio button in its own form posting `name=value` to `action`."),
        Prop::new("checked", PropKind::Modifier, "checked: bool")
            .applies_to(&["checkbox", "radio"])
            .attr("checked")
            .doc("The checkbox or radio item is ticked."),
        Prop::new("group", PropKind::Item, "text: &'a str").doc("A section heading."),
        Prop::new("separator", PropKind::Item, "").doc("A rule between groups."),
        Prop::new(
            "submenu",
            PropKind::Item,
            "text: &'a str, items: impl IntoIterator<Item = I>",
        )
        .doc("A nested menu of `items` (`MenuItem`s or `(text, href)` links)."),
        Prop::new("icon", PropKind::Modifier, "icon: impl Into<Glyph<'a>>").applies_to(ITEMS).doc(
            "An icon, or a glyph or emoji, before the item's text (decorative, hidden from assistive tech).",
        ),
        Prop::new("shortcut", PropKind::Modifier, "keys: &'a str").applies_to(ITEMS)
            .doc("A shortcut shown after the item's text, as `<kbd>`."),
        Prop::new("disabled", PropKind::Modifier, "").applies_to(ITEMS)
            .attr("disabled")
            .doc("The item is shown but not usable."),
        Prop::new("danger", PropKind::Modifier, "").applies_to(ITEMS).doc("The item is destructive."),
        Prop::new("id", PropKind::Value, "id: &'a str")
            .attr("id")
            .doc("The menu's id instead of `lui-menu-` and the label's slug."),
        Prop::new("align_end", PropKind::Switch, "").doc(
            "Open below the button with right edges aligned, for a button at the end of a row.",
        ),
        Prop::new("open_right", PropKind::Switch, "").doc("Open to the right of the button."),
        Prop::new("trigger", PropKind::Value, "button: Button<'a>")
            .doc("The opener as a styled button of the caller's; the menu sets its `popovertarget`."),
    ];
}

impl Ui {
    /// A menu behind a button labelled `label`; its id is `lui-menu-` and the label's slug.
    pub fn menu<'a>(&self, label: &'a str) -> Menu<'a> {
        Menu {
            caps: self.caps,
            apply: self.text(Text::Apply),
            id: None,
            label,
            items: Vec::new(),
            placement: Placement::default(),
            button: None,
        }
    }
}

impl<'a> Menu<'a> {
    fn push(mut self, item: MenuItem<'a>) -> Self {
        self.items.push(item);
        self
    }

    #[track_caller]
    fn last(mut self, method: &str, change: impl FnOnce(&mut MenuItem<'a>)) -> Self {
        let last = self.items.last().map(MenuItem::adder);
        crate::props::check_last::<Self>(Menu::PROPS, method, last);
        if let Some(item) = self.items.last_mut() {
            change(item);
        }
        self
    }

    /// A link.
    pub fn link(self, text: &'a str, href: &'a str) -> Self {
        self.push(MenuItem::link(text, href))
    }

    /// A `<form method="post">` button posting to `action`: for things that change state.
    pub fn action(self, text: &'a str, action: &'a str) -> Self {
        self.push(MenuItem::action(text, action))
    }

    /// A checkbox in its own `<form method="post">` to `action`: ticked, it posts `name=on`;
    /// unticked, nothing for `name`. The script posts it on change; without it, its Apply
    /// button does.
    pub fn checkbox(self, text: &'a str, action: &'a str, name: &'a str) -> Self {
        let apply = self.apply;
        self.push(MenuItem::new(Kind::Check(action, name, apply), text))
    }

    /// A radio button in its own `<form method="post">` to `action`, posting `name=value`.
    /// The script posts it on change; without it, its Apply button does.
    pub fn radio(self, text: &'a str, action: &'a str, name: &'a str, value: &'a str) -> Self {
        let apply = self.apply;
        self.push(MenuItem::new(Kind::Radio(action, name, value, apply), text))
    }

    /// The checkbox or radio item added last is ticked.
    pub fn checked(self, checked: bool) -> Self {
        self.last("checked", |it| it.checked = checked)
    }

    /// A section heading.
    pub fn group(self, text: &'a str) -> Self {
        self.push(MenuItem::new(Kind::Heading, text))
    }

    /// A rule between groups.
    pub fn separator(self) -> Self {
        self.push(MenuItem::new(Kind::Separator, ""))
    }

    /// A nested menu of `items` ([`MenuItem`]s or `(text, href)` links); its id is this
    /// menu's id and the text's slug.
    pub fn submenu<I: Into<MenuItem<'a>>>(
        self,
        text: &'a str,
        items: impl IntoIterator<Item = I>,
    ) -> Self {
        self.push(MenuItem::new(
            Kind::Submenu(items.into_iter().map(Into::into).collect()),
            text,
        ))
    }

    /// An icon, or a glyph or emoji, before the item's text (decorative, hidden from
    /// assistive tech).
    #[track_caller]
    pub fn icon(self, icon: impl Into<Glyph<'a>>) -> Self {
        let icon = icon.into();
        self.last("icon", |it| it.icon = Some(icon))
    }

    /// A shortcut shown after the item's text, as `<kbd>`; a label only.
    #[track_caller]
    pub fn shortcut(self, keys: &'a str) -> Self {
        self.last("shortcut", |it| it.shortcut = Some(keys))
    }

    /// The item is shown but not usable: a link without `href`, a button with `disabled`.
    #[track_caller]
    pub fn disabled(self) -> Self {
        self.last("disabled", |it| it.disabled = true)
    }

    /// The item is destructive: coloured with `--lui-danger`.
    #[track_caller]
    pub fn danger(self) -> Self {
        self.last("danger", |it| it.danger = true)
    }

    /// The menu's id instead of `lui-menu-` and the label's slug.
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }

    /// Open below the button with right edges aligned, for a button at the end of a row.
    pub fn align_end(mut self) -> Self {
        self.placement = Placement::BottomEnd;
        self
    }

    /// Open to the right of the button.
    pub fn open_right(mut self) -> Self {
        self.placement = Placement::Right;
        self
    }

    /// The opener as a styled button of the caller's (`ui.button("").icon_only().leading(..)`)
    /// in place of the label and chevron: the menu sets its `popovertarget`, and without
    /// popover support its look goes on the `<summary>`.
    pub fn trigger(mut self, button: Button<'a>) -> Self {
        self.button = Some(button);
        self
    }
}

impl Render for Menu<'_> {
    fn render(&self) -> Markup {
        menu_from(
            &self.caps,
            &self
                .id
                .map_or_else(|| format!("lui-menu-{}", slug(self.label)), str::to_string),
            self.label,
            &self.items,
            self.placement,
            Trigger::Own(self.button.as_ref()),
        )
    }
}

/// A button labelled `label` that toggles a menu of `items`. `compact` is a table row's
/// menu: a small ghost icon button, `label` its accessible name.
pub(crate) fn menu(
    caps: &Caps,
    id: &str,
    label: &str,
    items: &[MenuItem],
    placement: Placement,
    compact: bool,
) -> Markup {
    let trigger = if compact {
        Trigger::Compact
    } else {
        Trigger::Own(None)
    };
    menu_from(caps, id, label, items, placement, trigger)
}

/// What opens a menu: a table row's small ghost icon button, or the label and chevron
/// unless the caller gave a styled button of their own.
#[derive(Clone, Copy)]
enum Trigger<'b, 'a> {
    Compact,
    Own(Option<&'b Button<'a>>),
}

fn menu_from(
    caps: &Caps,
    id: &str,
    label: &str,
    items: &[MenuItem],
    placement: Placement,
    trigger: Trigger,
) -> Markup {
    let compact = matches!(trigger, Trigger::Compact);
    let own = match trigger {
        Trigger::Own(b) => b,
        Trigger::Compact => None,
    };
    let popover = caps.has(Cap::Popover);
    let anchor = caps.has(Cap::Anchor);
    let list = html! { ul role="menu" { @for it in items { (item(id, it, popover, anchor)) } } };
    // A row menu repeats once per row, so its face is a text glyph rather than an SVG.
    let face = match own {
        Some(b) => b.face(),
        None if compact => html! { "\u{22ef}" },
        None => html! { (label) (Icon::ChevronDown) },
    };
    let trigger = match own {
        Some(b) => b.clone(),
        None => Button::new(*caps, label).body(face.clone()),
    };
    let trigger = trigger.popovertarget(id).aria_haspopup("menu");
    let trigger = if compact {
        trigger.ghost().small().icon_only().aria_label(label)
    } else {
        trigger
    };
    let summary_class = trigger.classes();
    html! {
        @if !popover {
            details class={ "lui-popover lui-popover-details " (placement.class()) } id=(id) {
                summary class=(summary_class) aria-haspopup="menu" aria-label=[trigger.label()] { (face) }
                nav { (list) }
            }
        } @else if anchor {
            div class={ "lui-popover lui-popover-anchored " (placement.class()) } style={ "anchor-name: --" (id) } {
                (trigger)
                nav id=(id) popover style={ "position-anchor: --" (id) "; position-area: " (placement.area()) } { (list) }
            }
        } @else {
            div class={ "lui-popover " (placement.class()) } {
                (trigger)
                nav id=(id) popover { (list) }
            }
        }
    }
}

fn item(menu_id: &str, it: &MenuItem, popover: bool, anchor: bool) -> Markup {
    let class = format!(
        "lui-popover-item{}{}",
        if it.danger { " lui-popover-danger" } else { "" },
        if it.disabled {
            " lui-popover-disabled"
        } else {
            ""
        }
    );
    let inner = html! {
        @if let Some(i) = it.icon { span class="lui-popover-icon" aria-hidden="true" { (i) } }
        span class="lui-popover-text" {
            (it.text)
            @if let Some(d) = it.description { span class="lui-popover-description" { (d) } }
        }
        @if let Some(k) = it.shortcut { kbd class="lui-popover-kbd" { (k) } }
    };
    html! {
        @match it.kind {
            Kind::Heading => li role="presentation" class="lui-popover-heading" { (it.text) },
            Kind::Separator => li role="separator" class="lui-popover-sep" {},
            Kind::Link(href) => li role="none" {
                @if it.disabled {
                    a class=(class) role="menuitem" aria-disabled="true" data-disabled { (inner) }
                } @else {
                    a class=(class) role="menuitem" href=(href) { (inner) }
                }
            },
            Kind::Action(action) => li role="none" {
                form method="post" action=(action) {
                    button type="submit" class=(class) role="menuitem" disabled[it.disabled] data-disabled[it.disabled] { (inner) }
                }
            },
            Kind::Check(action, name, apply) => li role="none" {
                form method="post" action=(action) data-lui="autosubmit" {
                    label class=(class) {
                        input class="lui-popover-check" type="checkbox" role="menuitemcheckbox" name=(name) value="on" checked[it.checked] disabled[it.disabled];
                        (inner)
                    }
                    (Button::new(Caps::NONE, apply).ghost().small().submit().class("lui-popover-apply"))
                }
            },
            Kind::Radio(action, name, value, apply) => li role="none" {
                form method="post" action=(action) data-lui="autosubmit" {
                    label class=(class) {
                        input class="lui-popover-check" type="radio" role="menuitemradio" name=(name) value=(value) checked[it.checked] disabled[it.disabled];
                        (inner)
                    }
                    (Button::new(Caps::NONE, apply).ghost().small().submit().class("lui-popover-apply"))
                }
            },
            Kind::Submenu(ref items) => li role="none" class="lui-popover-sub" {
                @let sub_id = format!("{menu_id}-{}", slug(it.text));
                @let list = html! { ul role="menu" { @for it in items { (item(&sub_id, it, popover, anchor)) } } };
                @if !popover {
                    details class="lui-popover-details lui-popover-right" id=(sub_id) {
                        summary class=(class) role="menuitem" aria-haspopup="menu" { (inner) " \u{25b8}" }
                        nav { (list) }
                    }
                } @else if anchor {
                    button type="button" class=(class) role="menuitem" aria-haspopup="menu" popovertarget=(sub_id) style={ "anchor-name: --" (sub_id) } { (inner) " \u{25b8}" }
                    nav id=(sub_id) popover class="lui-popover-subnav" style={ "position-anchor: --" (sub_id) "; position-area: right span-bottom" } { (list) }
                } @else {
                    button type="button" class=(class) role="menuitem" aria-haspopup="menu" popovertarget=(sub_id) { (inner) " \u{25b8}" }
                    nav id=(sub_id) popover class="lui-popover-subnav" { (list) }
                }
            },
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
.lui-popover { display: inline-block; position: relative; }
/* After Radix Themes DropdownMenu, ContextMenu and Popover: a popover surface, 4px inset,
   2rem items (44px on a coarse pointer) with 8px inline padding and a 1rem leading icon slot,
   the primary colour on the highlighted item (Radix's solid highlight, as the Select picker),
   the shortcut right-aligned in --lui-gray-11, small --lui-gray-11 group labels, inset hairlines. A
   top-layer menu never outgrows the viewport: it scrolls inside. */
.lui-popover nav {
  padding: var(--lui-space-1); min-width: 14rem;
  background: var(--lui-popover); color: var(--lui-fg);
  border: 1px solid var(--lui-line); border-radius: var(--lui-radius);
  box-shadow: var(--lui-shadow-md), var(--lui-highlight);
}
.lui-popover-anchored > nav, .lui-popover-details > nav, .lui-popover-subnav { margin: 0; }
.lui-popover-anchored > nav { margin-top: 4px; position-try-fallbacks: flip-block, flip-inline; }
.lui-popover-subnav { position-try-fallbacks: flip-inline, flip-block; }
/* A checkbox or radio item: the control in the icon slot, the Apply button (no script)
   beside the label; the script hides it and posts on change. */
.lui-popover form[data-lui=autosubmit] { display: flex; align-items: center; gap: var(--lui-space-1); }
.lui-popover-check { flex: none; width: 1rem; height: 1rem; margin: 0; accent-color: var(--lui-primary); }
[data-lui-script] .lui-popover-apply { display: none; }
.lui-popover nav[popover] { max-block-size: min(24rem, calc(100dvh - 2rem)); overflow-y: auto; }
/* Popover without anchor positioning: the UA centres it in the viewport; keep that. */
.lui-popover ul { list-style: none; margin: 0; padding: 0; }
.lui-popover form { margin: 0; }
.lui-popover-item {
  display: flex; align-items: center; gap: var(--lui-space-2); width: 100%; min-height: 2rem; box-sizing: border-box;
  padding: 0 0.5rem; color: inherit; text-decoration: none; text-align: start; font: inherit;
  font-size: var(--lui-text-2); line-height: 1.25rem; font-weight: 400;
  background: none; border: 0; border-radius: var(--lui-radius-sm); box-shadow: none; cursor: pointer; justify-content: flex-start;
}
@media (pointer: coarse) { .lui-popover-item { min-height: var(--lui-hit); } }
.lui-popover-item:hover, .lui-popover-item:focus-visible { background: var(--lui-primary); color: var(--lui-on-primary); outline: none; }
.lui-popover-icon { flex: none; width: 1rem; text-align: center; color: var(--lui-muted); }
.lui-popover-item:hover .lui-popover-icon, .lui-popover-item:focus-visible .lui-popover-icon,
.lui-popover-item:hover .lui-popover-kbd, .lui-popover-item:focus-visible .lui-popover-kbd { color: inherit; }
.lui-popover-text { flex: 1; }
.lui-popover-description { display: block; margin-top: 0.125rem; font-size: var(--lui-text-2); line-height: 1.125rem; color: var(--lui-muted); }
.lui-popover-item:has(.lui-popover-description) { align-items: flex-start; padding-block: var(--lui-space-2); }
.lui-popover-item:is(:hover, :focus-visible) .lui-popover-description { color: inherit; opacity: 0.85; }
.lui-popover-kbd { font: inherit; font-size: var(--lui-text-1); letter-spacing: 0.1em; color: var(--lui-muted); background: none; border: 0; padding: 0 0 0 var(--lui-space-4); margin-inline-start: auto; }
.lui-popover-danger { color: var(--lui-danger); }
.lui-popover-danger:hover, .lui-popover-danger:focus-visible { color: var(--lui-danger); background: color-mix(in srgb, var(--lui-danger) 10%, transparent); }
.lui-popover-danger .lui-popover-icon { color: inherit; }
.lui-popover-item[data-disabled] { opacity: 0.5; cursor: default; }
.lui-popover-item[data-disabled]:hover { background: none; color: inherit; }
.lui-popover-heading { display: flex; align-items: center; min-height: 1.5rem; padding: 0 0.5rem; font-size: var(--lui-text-1); font-weight: 500; color: var(--lui-muted); }
.lui-popover-sep { margin: var(--lui-space-1) 0.5rem; border-top: 1px solid var(--lui-line); }
.lui-popover-sub { position: relative; }
/* <details> fallback: the summary is a .lui-button, the menu absolutely positioned by placement. */
.lui-popover-details > summary { list-style: none; }
.lui-popover-details > summary::-webkit-details-marker { display: none; }
.lui-popover-details > nav { position: absolute; z-index: var(--lui-z-popover); }
.lui-popover-start.lui-popover-details > nav { top: 100%; inset-inline-start: 0; margin-top: 4px; }
.lui-popover-end.lui-popover-details > nav { top: 100%; inset-inline-end: 0; margin-top: 4px; }
.lui-popover-right.lui-popover-details > nav { top: 0; inset-inline-start: 100%; margin-inline-start: 4px; }
.lui-popover-sub > .lui-popover-details > summary {
  display: flex; align-items: center; min-height: 2rem; padding: 0 0.5rem; font-weight: 400; border: 0; border-radius: var(--lui-radius-sm); background: none; box-shadow: none;
}
/* Motion: menus (dropdowns, context and nav menus, submenus) fade in and drop 4px, and fade
   out as they close; display and overlay are discrete so a closing popover keeps its top-layer
   box until the fade ends. The <details> fallback animates in only: a closed <details> hides
   its content at once. */
.lui-popover nav {
  transition: opacity var(--lui-duration-fast) var(--lui-ease-out), translate var(--lui-duration) var(--lui-ease-spring),
    display var(--lui-duration-fast) allow-discrete, overlay var(--lui-duration-fast) allow-discrete;
}
.lui-popover nav[popover]:not(:popover-open) { opacity: 0; translate: 0 -4px; }
@starting-style {
  .lui-popover nav[popover]:popover-open, .lui-popover-details[open] > nav { opacity: 0; translate: 0 -4px; }
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_menu_is_a_trigger_and_a_popover_nav_with_a_menu_role_list() {
        let m = Ui::from(Caps::all())
            .menu("Account")
            .link("Profile", "/profile")
            .render()
            .into_string();
        assert!(
            m.contains(r#"popovertarget="lui-menu-account""#)
                && m.contains(r#"aria-haspopup="menu""#),
            "{m}"
        );
        assert!(m.contains(r#"<nav id="lui-menu-account" popover"#), "{m}");
        assert!(m.contains(r#"<ul role="menu">"#), "{m}");
        assert!(
            m.contains(r#"<a class="lui-popover-item" role="menuitem" href="/profile">"#),
            "{m}"
        );
        let m = Ui::from(Caps::all())
            .menu("Account")
            .id("acct")
            .render()
            .into_string();
        assert!(
            m.contains(r#"id="acct""#) && !m.contains("lui-menu-account"),
            "{m}"
        );
    }

    #[test]
    fn items_render_as_links_post_forms_headings_and_separators() {
        let m = Ui::from(Caps::all())
            .menu("M")
            .group("Section")
            .link("Gone", "/gone")
            .disabled()
            .separator()
            .action("Delete", "/del")
            .danger()
            .render()
            .into_string();
        assert!(
            m.contains(r#"<li role="presentation" class="lui-popover-heading">Section</li>"#),
            "{m}"
        );
        assert!(
            m.contains(r#"aria-disabled="true""#) && !m.contains(r#"href="/gone""#),
            "{m}"
        );
        assert!(
            m.contains(r#"<li role="separator" class="lui-popover-sep">"#),
            "{m}"
        );
        assert!(m.contains(r#"<form method="post" action="/del">"#), "{m}");
        assert!(m.contains("lui-popover-item lui-popover-danger"), "{m}");
    }

    #[test]
    fn checkbox_and_radio_items_post_their_own_form_with_an_apply_button() {
        let m = Ui::from(Caps::all())
            .menu("View")
            .checkbox("Grid", "/v", "grid")
            .checked(true)
            .radio("Compact", "/v", "density", "compact")
            .render()
            .into_string();
        assert_eq!(m.matches(r#"data-lui="autosubmit""#).count(), 2, "{m}");
        assert!(
            m.contains(
                r#"type="checkbox" role="menuitemcheckbox" name="grid" value="on" checked>"#
            ),
            "{m}"
        );
        assert!(
            m.contains(r#"type="radio" role="menuitemradio" name="density" value="compact">"#),
            "{m}"
        );
        assert_eq!(m.matches("lui-popover-apply").count(), 2, "{m}");
    }

    #[test]
    fn icon_shortcut_and_a_submenu_add_their_parts() {
        let m = Ui::from(Caps::all())
            .menu("M")
            .link("Open", "/o")
            .icon("@")
            .shortcut("g o")
            .submenu("Theme", [("Light", "/l")])
            .render()
            .into_string();
        assert!(
            m.contains(r#"<span class="lui-popover-icon" aria-hidden="true">"#),
            "{m}"
        );
        assert!(
            m.contains(r#"<kbd class="lui-popover-kbd">g o</kbd>"#),
            "{m}"
        );
        assert!(
            m.contains(r#"id="lui-menu-m-theme""#) && m.contains(r#"href="/l""#),
            "{m}"
        );
    }

    #[test]
    fn placement_follows_anchor_support_and_the_align_setters() {
        let anchored = Ui::from(Caps::all())
            .menu("M")
            .align_end()
            .render()
            .into_string();
        assert!(
            anchored.contains("lui-popover-anchored lui-popover-end"),
            "{anchored}"
        );
        assert!(
            anchored.contains("anchor-name: --lui-menu-m") && anchored.contains("position-area"),
            "{anchored}"
        );
        let right = Ui::from(Caps::all())
            .menu("M")
            .open_right()
            .render()
            .into_string();
        assert!(right.contains("lui-popover-right"), "{right}");
        let plain = Ui::from(Caps::ASSUMED).menu("M").render().into_string();
        assert!(
            plain.contains(r#"class="lui-popover lui-popover-start""#),
            "{plain}"
        );
        assert!(
            plain.contains("popover")
                && !plain.contains("anchor-name")
                && !plain.contains("position-area"),
            "{plain}"
        );
    }

    #[test]
    fn without_popover_the_menu_is_a_details_dropdown_with_nested_details() {
        let m = Ui::from(Caps::NONE)
            .menu("M")
            .link("A", "/a")
            .submenu("Sub", [("B", "/b")])
            .render()
            .into_string();
        assert!(m.contains(r#"<details class="lui-popover lui-popover-details lui-popover-start" id="lui-menu-m">"#), "{m}");
        assert_eq!(m.matches("<details").count(), 2, "{m}");
        assert!(
            !m.contains("popovertarget") && !m.contains(" popover"),
            "{m}"
        );
    }
}
