//! Blocks: whole pages built from the components, one route each. Their posts land back on
//! the page with a flash; unknown paths get the 404 block through the router's fallback.

use axum::response::IntoResponse;
use axum::{
    Router,
    extract::Path,
    routing::{get, post},
};
use loco_ui::prelude::*;

pub(crate) fn routes() -> Router {
    super::pages(PAGES)
        .route("/blocks/settings", post(saved))
        .route("/blocks/record/delete", post(deleted))
        .route("/blocks/inbox/{n}", get(inbox_open))
}

/// Every block's page: the block, and a note (`super::pages`).
pub(crate) const PAGES: &[super::Simple] = &[
    ("/blocks/shell", app_shell, ""),
    ("/blocks/auth", auth, ""),
    ("/blocks/settings", settings, ""),
    ("/blocks/record", record, ""),
    ("/blocks/dashboard", dashboard, ""),
    ("/blocks/pricing", pricing, ""),
    (
        "/blocks/inbox",
        inbox_list,
        "Each item is a page of its own, [like this one](/blocks/inbox/2). In a narrow box the open item shows alone, with a link back to the list.",
    ),
    ("/blocks/invoice", invoice, ""),
    (
        "/blocks/error",
        error,
        "Every unknown path answers with this page and a 404, [like this one](/no-such-page).",
    ),
];

fn app_shell(ui: &Ui) -> Markup {
    lui! {
        // code: /blocks/shell
        AppShell("Acme") user=("Ada Lovelace", "/app/signout") {
            link "Dashboard" "/blocks/dashboard" icon=(Icon::House);
            link "App shell" "/blocks/shell" icon=(Icon::Menu);
            link "Settings" "/blocks/settings" icon=(Icon::User);
            group "Records";
            link "Invoices" "/blocks/record" badge=42; link "Reports" "/chart" badge=7;
            body { h2 { "Welcome back" } p { "The sidebar turns into a drawer on narrow screens." } }
        }
        // end code
    }
}

fn auth(ui: &Ui) -> Markup {
    lui! {
        // code: /blocks/auth
        AuthPage("Sign in") description="Use the email you signed up with."
            footer={ "No account? " a href="/app/signin" { "Sign up" } } {
            Form("/app/signin") submit="Sign in" { email "email" "Email" required; password "password" "Password" required; }
        }
        // end code
    }
}

fn settings(ui: &Ui) -> Markup {
    lui! {
        // code: /blocks/settings
        SettingsPage("Settings") {
            section "Profile" "How other people see you." { Form("/blocks/settings") id="settings-profile" submit="Save" { text "name" "Name" value="Ada"; } }
            section "Email" "Where we send receipts." { Form("/blocks/settings") id="settings-email" submit="Save" { email "email" "Email" value="ada@example.com"; } }
        }
        // end code
    }
}

fn record(ui: &Ui) -> Markup {
    lui! {
        // code: /blocks/record
        RecordPage("Invoice 42") back="/blocks/dashboard" edit="/form" delete="/blocks/record/delete" {
            field "Customer" "Ada Lovelace"; field "Issued" "24 September 2026"; field "Total" "€1,280.00";
            field "Status" (ui.badge("Paid").ok()); field "Purchase order" "";
        }
        // end code
    }
}

fn dashboard(ui: &Ui) -> Markup {
    lui! {
        // code: /blocks/dashboard
        DashboardPage("Overview") description="The last 30 days." {
            stat (ui.stat("Revenue", "$48,210").delta("+12%").description("vs last month"));
            stat (ui.stat("Orders", "1,284").delta("+4%"));
            stat (ui.stat("Refunds", "18").delta("-3").down_is_good());
            body { p class="lui-note" { "A table or a chart goes here." } }
        }
        // end code
    }
}

fn error(ui: &Ui) -> Markup {
    lui! {
        // code: /blocks/error
        ErrorPage(ui.param("status").and_then(|s| s.parse().ok()).unwrap_or(404)) home="/";
        // end code
        p class="lui-note" { "Also: "
            @for status in ["401", "403", "422", "500", "503"] { a href={ "/blocks/error?status=" (status) } { (status) } " " }
        }
    }
}

fn pricing(ui: &Ui) -> Markup {
    lui! {
        // code: /blocks/pricing
        PricingPage("Pricing") description="Start free. Upgrade when the team grows." {
            tier "Hobby" "$0" period="/month" blurb="For one person trying it out."
                feature="One project" feature="Community help" cta=("Start free", "/app/signin");
            tier "Team" "$12" period="per seat / month" blurb="For a team shipping together."
                feature="Unlimited projects" feature="Email support" feature="Audit log" cta=("Start a trial", "/app/signin") featured;
            tier "Company" "Custom" blurb="For many teams, with a contract."
                feature="Single sign-on" feature="A named contact" cta=("Talk to us", "/form");
            body { p class="lui-note" { "Prices exclude tax. Cancel any time." } }
        }
        // end code
    }
}

/// The demo inbox: id, sender, subject, preview, when, unread.
const MESSAGES: [(&str, &str, &str, &str, bool); 4] = [
    (
        "Ada Lovelace",
        "Notes on the engine",
        "I added a table of the Bernoulli numbers.",
        "09:12",
        true,
    ),
    (
        "Charles Babbage",
        "Lunch on Friday?",
        "The usual place, if the weather holds.",
        "Mon",
        false,
    ),
    (
        "Mary Somerville",
        "Proofs are back",
        "Two small corrections on page twelve.",
        "Sun",
        true,
    ),
    (
        "Michael Faraday",
        "Lecture slides",
        "The induction demo needs a longer wire.",
        "Sep 28",
        false,
    ),
];

fn inbox_list(ui: &Ui) -> Markup {
    inbox(ui, None)
}

/// One open message: the same page, with the message beside the list.
async fn inbox_open(ui: Ui, Path(n): Path<usize>) -> axum::response::Response {
    if !(1..=MESSAGES.len()).contains(&n) {
        return loco_ui::blocks::not_found(ui).await;
    }
    crate::site::page(
        &ui,
        crate::site::title("/blocks/inbox"),
        inbox(&ui, Some(n - 1)),
    )
    .into_response()
}

fn inbox(ui: &Ui, open: Option<usize>) -> Markup {
    let hrefs: Vec<String> = (1..=MESSAGES.len())
        .map(|n| format!("/blocks/inbox/{n}"))
        .collect();
    let messages = MESSAGES.iter().zip(&hrefs);
    let detail = open.map(|i| MESSAGES[i]);
    lui! {
        // code: /blocks/inbox
        InboxPage("Inbox", "/blocks/inbox") {
            @for ((from, subject, _, when, unread), href) in messages {
                item (from) (href) description=(subject) when=(when) unread=(*unread);
            }
            body {
                @if let Some((from, subject, preview, ..)) = detail {
                    h2 { (subject) } p class="lui-note" { "From " (from) } p { (preview) }
                }
            }
        }
        // end code
    }
}

fn invoice(ui: &Ui) -> Markup {
    lui! {
        // code: /blocks/invoice
        InvoicePage("Invoice INV-0042") status=(ui.badge("Due").warn())
            from={ "Acme Ltd" br; "1 Engine Row" br; "London" } to={ "Ada Lovelace" br; "12 St James's Square" br; "London" }
            actions={ LinkButton("Pay now", "/form") primary; }
            note="Pay within 30 days by bank transfer. Thank you for your business." {
            meta "Issued" "1 October 2026"; meta "Due" "31 October 2026";
            line_item "Design review" "2" "£400.00" "£800.00";
            line_item "Hosting, October" "1" "£40.00" "£40.00";
            line_item "Support hours" "3.5" "£60.00" "£210.00";
            total "Subtotal" "£1,050.00"; total "VAT 20%" "£210.00"; total "Amount due" "£1,260.00";
        }
        // end code
    }
}

/// The settings forms post here: back to the page with a note.
async fn saved(ui: Ui) -> Redirect {
    ui.redirect("/blocks/settings").ok("Saved.")
}

/// Deleting the demo invoice deletes nothing: back with a note.
async fn deleted(ui: Ui) -> Redirect {
    ui.redirect("/blocks/record")
        .ok("Deleted (not really: this is the demo).")
}
