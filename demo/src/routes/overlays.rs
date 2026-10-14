//! Overlays: dialog, drawer, popover menu, popover and carousel.

use axum::{Form, Router, routing::post};
use loco_ui::prelude::*;
use serde::Deserialize;

pub(crate) fn routes() -> Router {
    super::pages(PAGES)
        .route("/dialog/delete", post(dialog_delete))
        .route("/popover/signout", post(popover_signout))
        .route("/popover/view", post(popover_view))
}

/// The pages that are their component and a note (`super::pages`).
pub(crate) const PAGES: &[super::Simple] = &[
    (
        "/dialog",
        dialog,
        "Opened by an invoker button; the footer is a real form posting to `/dialog/delete` with a hidden `returns_to` so the server comes back here. Server-opened: [?dialog=confirm](/dialog?dialog=confirm)",
    ),
    (
        "/popover",
        menus,
        "Links, a heading, a disabled item, a submenu that is another popover, and a `<form method=\"post\">` action. Click outside or press Escape to close; the second menu opens end-aligned.",
    ),
    (
        "/context-menu",
        context_menu,
        "A right-click cannot be caught without script, so the menu hangs on a button in the corner.",
    ),
    (
        "/popover-card",
        popover_card,
        "Each panel is a `popover` placed by anchor positioning; one that would leave the window flips to the other side. Without the popover attribute it is a `<details>` disclosure.",
    ),
    (
        "/carousel",
        carousel,
        "Swipe, scroll sideways or use the arrow keys after focusing the row. Chromium draws the buttons and dots with `::scroll-button()` and `::scroll-marker`; other browsers use links to each slide.",
    ),
];

fn dialog(ui: &Ui) -> Markup {
    lui! {
            // code: /dialog
            Dialog("Delete account") id="confirm" title="Delete account?" size=1 danger
                trigger=(ui.button("Delete account").danger().ghost().leading(Icon::Trash))
                confirm=("Delete account", "/dialog/delete") cancel="Keep it" {
                p { "This cannot be undone. Everything you wrote goes with it." }
                Input("reason", "Tell us why (optional)") placeholder="Moving on";
            }
            Dialog("Read the terms") id="terms" title="Terms of service" size=3
                description="Version 3, effective 1 October 2026." footer={ a href="/dialog" { "Download PDF" } } {
                @for n in 1..=12 {
                    p { (n) ". A long body scrolls inside the dialog; the title and the Close button stay put." }
                }
            }
            Dialog("Edit the photo") id="editor" title="Edit the photo" fullscreen {
                p { "A full-screen dialog: an editor, or a long form on a phone." }
            }
            Drawer("Filters") id="filters" title="Filters" side=(Side::End) size=1
                nav=(html! { ul { li { a href="/dialog" { "Open" } } li { a href="/dialog" { "Closed" } } } });
            // end code
    }
}

#[derive(Deserialize)]
struct DeleteForm {
    #[serde(default)]
    reason: String,
    #[serde(default)]
    returns_to: String,
}

/// The confirm form's target: only ever redirects to a local path from `returns_to`.
async fn dialog_delete(ui: Ui, Form(form): Form<DeleteForm>) -> Redirect {
    let local = form.returns_to.starts_with('/') && !form.returns_to.starts_with("//");
    let message = if form.reason.is_empty() {
        "Account deleted (not really)".to_string()
    } else {
        format!("Account deleted (not really). Reason: {}", form.reason)
    };
    ui.redirect(if local { &form.returns_to } else { "/dialog" })
        .flash(&message)
}

fn menus(ui: &Ui) -> Markup {
    lui! {
            div class="lui-popover-row" {
                // code: /popover
                Menu("Account") {
                    group "Signed in as Ada";
                    link "Profile" "/popover" icon="@" shortcut="g p";
                    link "Settings" "/settings" icon="\u{2699}" shortcut="g s";
                    link "Billing" "/popover" icon="$" disabled;
                    separator();
                    submenu "Theme" ([("Light", "/popover?theme=light"), ("Dark", "/popover?theme=dark")]) icon="\u{25d0}";
                    separator();
                    action "Sign out" "/popover/signout" icon="\u{2192}" danger;
                }
                Menu("More") align_end trigger=(ui.button("").ghost().icon_only().aria_label("More").leading(Icon::Ellipsis)) {
                    link "Documentation" "/" icon="?";
                    action "Clear cache" "/popover/signout";
                }
                Menu("View") {
                    group "Show";
                    checkbox "Grid lines" "/popover/view" "grid" checked=(ui.state.get("grid") == Some("on"));
                    separator();
                    group "Density";
                    radio "Compact" "/popover/view" "density" "compact" checked=(ui.state.get("density") == Some("compact"));
                    radio "Comfortable" "/popover/view" "density" "comfortable" checked=(ui.state.get("density") != Some("compact"));
                }
                // end code
            }
    }
}

/// A menu action: Post/Redirect/Get back to the menu page with a flash.
async fn popover_signout(ui: Ui) -> Redirect {
    ui.redirect("/popover").flash("Signed out (not really)")
}

#[derive(Deserialize)]
struct ViewForm {
    grid: Option<String>,
    density: Option<String>,
}

/// A checkbox or radio item's form: the density when the radio sent one, else the grid box
/// (absent when unticked). Back to the menu with the choice in the URL, which the `lui-ui`
/// cookie then remembers.
async fn popover_view(ui: Ui, Form(form): Form<ViewForm>) -> Redirect {
    match (form.density.as_deref(), form.grid) {
        (Some("compact"), _) => ui.redirect("/popover?density=compact"),
        (Some(_), _) => ui.redirect("/popover?density=comfortable"),
        (None, Some(_)) => ui.redirect("/popover?grid=on"),
        (None, None) => ui.redirect("/popover?grid=off"),
    }
}

/// Actions on one thing, behind a "more" button in its corner.
fn context_menu(ui: &Ui) -> Markup {
    lui! {
        div style="max-width: 24rem" {
            // code: /context-menu
            ContextMenu("report.pdf") {
                link "Open" "/table";
                link "Download" "/table.csv";
                separator();
                action "Delete" "/blocks/record/delete" danger;
                body { p { strong { "report.pdf" } } p class="lui-note" { "2.4 MB · edited yesterday" } }
            }
            // end code
        }
    }
}

/// Panels of markup on each side of their buttons, one with an arrow.
fn popover_card(ui: &Ui) -> Markup {
    lui! {
        Cluster {
            // code: /popover-card
            Popover("Share", lui! {
                p { strong { "Share this page" } }
                p { "Anyone with the link can view it." }
                Input("link", "Link") value="https://example.org/p/42";
            }) arrow;
            Popover("Help", html! { p { "Opens above, centred, and flips below when there is no room." } })
                side=(Side::Top) align=(Align::Center) arrow id="help-top";
            Popover("Details", html! { p { "Opens at the end of its button." } }) side=(Side::End);
            // end code
        }
    }
}

/// Four slides that snap; buttons and dots from CSS where the browser has them.
fn carousel(ui: &Ui) -> Markup {
    let slides = [
        ("Plan", "Draw the shape of the thing."),
        ("Build", "Write it, test it, ship it."),
        ("Measure", "Look at what people do."),
        ("Learn", "Then plan the next one."),
    ];
    lui! {
        div style="max-width: 32rem" {
            // code: /carousel
            Carousel("Steps") {
                @for (title, text) in slides {
                    item (lui! { Card { h3 { (title) } p { (text) } } });
                }
            }
            // end code
        }
    }
}
