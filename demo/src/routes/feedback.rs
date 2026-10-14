//! Feedback: alerts, progress and tooltips, toasts, banners, stats and empty states.

use crate::site::page;
use axum::{Form, Router, routing::get};
use loco_ui::prelude::*;
use serde::Deserialize;

pub(crate) fn routes() -> Router {
    super::pages(PAGES)
        .route("/toast", axum::routing::post(toast_submit))
        .route("/banner", get(banner_page).post(banner_reset))
        .route("/dashboard", get(dashboard_page))
}

/// The pages that are their component and a note (`super::pages`).
pub(crate) const PAGES: &[super::Simple] = &[
    (
        "/feedback",
        feedback,
        "Hover or tab to the buttons for their tooltips. The meter turns amber past 60 and red past 80 because its best value is 0.",
    ),
    (
        "/toast",
        toasts,
        "Each button posts, the server redirects back, and the answer shows in the corner. Calm ones fade after five seconds (hover to keep them); errors stay until dismissed.",
    ),
    (
        "/chart",
        charts,
        "Point at or tab to a bar or a point for its value, shown by CSS alone. The numbers are also a table that screen readers read and that stays when the picture cannot load.",
    ),
    ("/description-list", description_list, ""),
];

/// The other pages' live components, which the index shows too (`site::preview`).
pub(crate) const PREVIEWS: &[super::Preview] = &[("/dashboard", stats), ("/banner", banners)];

fn feedback(ui: &Ui) -> Markup {
    lui! {
        Stack gap=6 {
            // code: /feedback
            Alert("Heads up") description="Deploys pause at 18:00 on Fridays.";
            Alert("Payment failed") danger description="The card was declined. Try another one.";
            Alert("Backups are complete") ok surface;
            Alert("Storage is 90% full") warn outline description="Old builds are removed after 30 days.";
            Alert("Draft saved") tone="neutral" variant="surface" size=1;
            Alert("Your trial ends on Friday") info description="Pick a plan to keep your projects."
                action=(ui.link_button("See plans", "/pricing").small()) dismiss;
            Progress(62, 100) label="Uploading photos" show_value;
            Progress(0, 100) label="Waiting for the server" indeterminate;
            Skeleton(2) circle avatar_lines=1;
            Meter(83, 0, 100) label="Disk used" low=60 high=80 optimum=0;
            Cluster {
                Tooltip("Copy the link", lui! { Button("") icon_only aria_label="Copy" body={ (Icon::Copy) }; });
                Separator vertical;
                Tooltip("Opens in a new tab", lui! { LinkButton("Docs", "/"); }) below;
                Tooltip("Shows after half a second", lui! { Button("Slow"); }) side=(Side::End) delay=500;
            }
            Separator label="or";
            // end code
        }
    }
}

/// The buttons that post for a toast, and the toasts that came back.
fn toasts(ui: &Ui) -> Markup {
    lui! {
        // code: /toast
        form method="post" action="/toast" class="lui-cluster" {
            Button("Send invite") primary name="kind" value="ok";
            Button("Copy link") name="kind" value="warn";
            Button("Sync now") name="kind" value="danger";
            Button("All three") name="kind" value="all";
        }
        Toasts dismiss max=3 duration=6000 position=(position(ui)) action=("View", "/toast");
        // end code
        p class="lui-note" { "Stack them at: "
            @for at in ["top-start", "top-center", "top-end", "bottom-start", "bottom-center", "bottom-end"] {
                a href={ "/toast?at=" (at) } { (at) } " "
            }
        }
    }
}

/// Where `?at=` puts the toasts; bottom-end without it.
fn position(ui: &Ui) -> Position {
    match ui.param("at") {
        Some("top-start") => Position::TopStart,
        Some("top-center") => Position::TopCenter,
        Some("top-end") => Position::TopEnd,
        Some("bottom-start") => Position::BottomStart,
        Some("bottom-center") => Position::BottomCenter,
        _ => Position::BottomEnd,
    }
}

#[derive(Deserialize)]
struct ToastForm {
    kind: String,
}

/// Toasts come back from a post like a flash: the one-shot cookie, several at once.
async fn toast_submit(ui: Ui, Form(form): Form<ToastForm>) -> Redirect {
    let wants = |k: &str| form.kind == "all" || form.kind == k;
    // code: /toast
    let mut back = ui.redirect("/toast");
    if wants("ok") {
        back = back.ok("Invite sent to ada@example.org.");
    }
    if wants("warn") {
        back = back.warn("Link copied; it expires in an hour.");
    }
    if wants("danger") {
        back = back.danger("Sync failed: the server did not answer.");
    }
    // end code
    back
}

/// Stat cards; the orders count follows `?orders=none`.
fn stats(ui: &Ui) -> Markup {
    let none = ui.param("orders") == Some("none");
    lui! {
        div class="lui-stat-grid" {
            // code: /dashboard
            Stat("Visitors", "12,480") delta="+8.2%" description="last 7 days" reveal
                sample=8.1 sample=9.4 sample=8.8 sample=10.2 sample=11.0 sample=10.6 sample=12.5;
            Stat("Orders", if none { "0" } else { "3" }) delta=(if none { "-3" } else { "0" });
            Stat("Error rate", "0.4%") delta="-0.2 pt" down_is_good href="/table";
            Stat("p95 latency", "38 ms") delta="+6 ms" down_is_good progress=(38, 50) description="of a 50 ms budget";
            // end code
        }
    }
}

/// Stat cards over a list that may be empty (`?orders=none`).
async fn dashboard_page(ui: Ui) -> Page {
    let none = ui.param("orders") == Some("none");
    let body = lui! {
        (stats(&ui))
        h2 { "Recent orders" }
        @if none {
            // code: /dashboard
            EmptyState("No orders yet") icon="\u{1f4e6}" {
                body { "Orders show up here as soon as a customer checks out." }
                action "Import orders" "/dashboard";
                link "Show sample orders" "/dashboard";
                link "Read the guide" "/";
            }
            // end code
        } @else {
            Card {
                DescriptionList {
                    item "#1042" "Ada Lovelace, 3 items"; item "#1041" "Grace Hopper, 1 item"; item "#1040" "Alan Turing, 2 items";
                }
            }
            p class="lui-note" { a href="/dashboard?orders=none" { "See the empty state" } }
        }
    };
    page(&ui, "Stats and empty states", body)
}

/// Bars, a line and a sparkline, drawn on the server as SVG with the data in a hidden table.
fn charts(ui: &Ui) -> Markup {
    lui! {
            // code: /chart
            Chart("Signups") description="New accounts per weekday, this week" {
                point "Mon" 12.0; point "Tue" 18.0; point "Wed" 9.0; point "Thu" 22.0; point "Fri" 15.0;
            }
            Chart("Latency") line unit=" ms" description="p50 response time per day" {
                point "Mon" 41.5; point "Tue" 38.0; point "Wed" 44.2; point "Thu" 36.9; point "Fri" 35.1;
            }
            p { "Revenue " strong { "$48,210" } " "
                Chart("Revenue, last 8 weeks") sparkline {
                    point "1" 30.0; point "2" 34.0; point "3" 31.0; point "4" 38.0;
                    point "5" 36.0; point "6" 41.0; point "7" 44.0; point "8" 48.0;
                }
            }
            // end code
    }
}

/// Terms beside their details, or stacked above them.
fn description_list(ui: &Ui) -> Markup {
    lui! {
        // code: /description-list
        DescriptionList {
            item "Plan" "Team"; item "Seats" "12 of 20";
            item "Renews" "1 October 2026"; item "Status" (ui.badge("Active").ok());
            item "Invoices" (ui.link_button("Download", "/description-list").small());
            item "Billing contact for receipts" "ada@example.com"; item "Notes" "";
        }
        h3 { "Stacked" }
        DescriptionList stacked {
            item "Billing email" "ada@example.com"; item "Tax id" "ES-B12345678";
        }
        // end code
    }
}

/// A banner that stays closed once closed (the `lui-dismiss-<id>` cookie), and a form that
/// brings it back.
async fn banner_page(ui: Ui) -> Page {
    let body = lui! { Stack gap=4 {
        (banners(&ui))
        Flash title="Banner restored" action=("Read the changes", "/");
        form method="post" action="/banner" { Button("Bring the first banner back") submit; }
    } };
    page(&ui, "Banner", body)
}

/// The banners: one that can be closed for good, two toned ones.
fn banners(ui: &Ui) -> Markup {
    lui! {
        Stack gap=3 {
            // code: /banner
            Banner("Version 0.2 is out: typed variants and a CLI.") id="v0-2" link=("Read the changes", "/") dismiss;
            Banner("Maintenance tonight from 22:00 to 22:30 UTC.") tone="warn";
            Banner("Your card was declined.") tone="danger" link=("Update billing", "/blocks/settings");
            // end code
        }
    }
}

/// Clears the first banner's dismiss cookie: back to the page with a note.
async fn banner_reset(ui: Ui) -> Redirect {
    let clear = loco_ui::cookie::SetCookie::clear("lui-dismiss-v0-2")
        .secure(ui.is_secure())
        .to_string();
    ui.redirect("/banner")
        .ok("It shows again until you close it.")
        .cookie(clear)
}
