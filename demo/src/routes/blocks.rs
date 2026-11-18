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
        .route("/blocks/onboarding", post(onboarded))
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
        "/blocks/list",
        list,
        "The search and the filters are links and a GET form: [open customers named a](/blocks/list?status=open&q=a) is an address.",
    ),
    (
        "/blocks/onboarding",
        onboarding,
        "Each step is a page: [the first](/blocks/onboarding?step=1), [the last](/blocks/onboarding?step=3).",
    ),
    ("/blocks/billing", billing, ""),
    (
        "/blocks/schedule",
        schedule,
        "From 30rem a day's name sits beside its events; from 48rem the month sits beside the agenda.",
    ),
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

/// The demo customers: name, plan, whether the account is open.
const CUSTOMERS: [(&str, &str, bool); 6] = [
    ("Ada Lovelace", "Team", true),
    ("Charles Babbage", "Hobby", false),
    ("Mary Somerville", "Company", true),
    ("Michael Faraday", "Team", true),
    ("Caroline Herschel", "Hobby", false),
    ("George Boole", "Team", true),
];

/// The customers, narrowed by `?status=` and `?q=`.
fn list(ui: &Ui) -> Markup {
    let query = ui.param("q").unwrap_or("").to_lowercase();
    let open = match ui.param("status") {
        Some("open") => Some(true),
        Some("closed") => Some(false),
        _ => None,
    };
    let shown: Vec<_> = (CUSTOMERS.iter())
        .filter(|(name, _, is_open)| {
            open.is_none_or(|o| o == *is_open) && name.to_lowercase().contains(&query)
        })
        .collect();
    let count = format!("{} of {} customers", shown.len(), CUSTOMERS.len());
    let rows = shown
        .iter()
        .map(|(name, plan, is_open)| (*name, *plan, if *is_open { "Open" } else { "Closed" }));
    lui! {
        // code: /blocks/list
        ListPage("Customers") description="Everyone with an account." search="Search customers" count=(&count)
            actions={ LinkButton("New customer", "/form") primary; } {
            filter "All" "/blocks/list" badge=(CUSTOMERS.len());
            filter "Open" "/blocks/list?status=open"; filter "Closed" "/blocks/list?status=closed";
            body {
                @if shown.is_empty() { EmptyState("No customer matches") { link "Show everyone" "/blocks/list"; } }
                @else { Table("customers", "") hide_search caption="Customers" {
                    column "name" "Name"; column "plan" "Plan"; column "status" "Status"; rows (rows);
                } }
            }
        }
        // end code
    }
}

/// A three-step set-up; `?step=` picks the step.
fn onboarding(ui: &Ui) -> Markup {
    let step = ui.param("step").and_then(|s| s.parse().ok());
    let at: usize = step.unwrap_or(2).clamp(1, 3);
    let next = format!("/blocks/onboarding?step={}", (at + 1).min(3));
    let (name, label) = [
        ("name", "Your name"),
        ("team", "Team name"),
        ("invite", "A colleague's email"),
    ][at - 1];
    lui! {
        // code: /blocks/onboarding
        OnboardingPage("Set up your workspace") description="Three steps, about two minutes."
            at=(at - 1) skip="/blocks/dashboard" {
            step "Profile" href="/blocks/onboarding?step=1"; step "Team" href="/blocks/onboarding?step=2"; step "Invite";
            body { Form(&next) id="onboarding" submit="Continue" { text (name) (label); } }
        }
        // end code
    }
}

/// A step's form posts to the next step's address: on to that page.
async fn onboarded(ui: Ui) -> Redirect {
    let step = ui.param("step").unwrap_or("1");
    ui.redirect(&format!("/blocks/onboarding?step={step}"))
}

fn billing(ui: &Ui) -> Markup {
    lui! {
        // code: /blocks/billing
        BillingPage("Billing") plan=("Team", "$12 per seat / month") renews="Renews on 1 December 2026."
            actions={ LinkButton("Change plan", "/blocks/pricing"); }
            method=("Visa ending 4242, expires 08/28", "/form") {
            usage "Seats" 8 10; usage "Storage (GB)" 42 100; usage "Projects" 5 5;
            invoice "INV-0042" "1 November 2026" "$96.00" href="/blocks/invoice" status=(ui.badge("Due").warn());
            invoice "INV-0041" "1 October 2026" "$96.00" href="/blocks/invoice" status=(ui.badge("Paid").ok());
            invoice "INV-0040" "1 September 2026" "$84.00" href="/blocks/invoice" status=(ui.badge("Paid").ok());
        }
        // end code
    }
}

fn schedule(ui: &Ui) -> Markup {
    lui! {
        // code: /blocks/schedule
        SchedulePage("Schedule") period="16 to 22 November 2026"
            prev="/blocks/schedule?week=46" next="/blocks/schedule?week=48" today="/blocks/schedule"
            actions={ LinkButton("New event", "/form") primary; }
            aside={ Calendar("day") today="2026-11-17" value="2026-11-17" event=("2026-11-16", "Stand-up") event=("2026-11-18", "Offsite"); } {
            day "Monday 16 November" datetime="2026-11-16";
            event "09:00" "Stand-up" description="15 minutes, room 2";
            event "14:00" "Release review" description="Version 0.3" href="/timeline" tone=(Tone::Warn);
            day "Tuesday 17 November" datetime="2026-11-17" current;
            event "11:30" "Lunch with Charles" tone=(Tone::Ok);
            day "Wednesday 18 November" datetime="2026-11-18";
            event "All day" "Offsite" description="Greenwich" tone=(Tone::Neutral);
            day "Thursday 19 November" datetime="2026-11-19";
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
