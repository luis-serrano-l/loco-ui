//! Primitives: the parts every component is built from.

use crate::site::page;
use axum::{Router, routing::get};
use loco_ui::prelude::*;

pub(crate) fn routes() -> Router {
    super::pages(PAGES).route("/button", get(button_page))
}

/// The pages that are their component alone (`super::pages`).
pub(crate) const PAGES: &[super::Simple] = &[
    ("/field", fields, ""),
    ("/card", cards, ""),
    ("/layout", layouts, ""),
    ("/typography", typography, ""),
    (
        "/code-block",
        code_blocks,
        "The library writes plain `<pre><code class=\"language-…\">`; a highlighter is the app's choice. The copy button appears only with the enhancement script.",
    ),
    ("/prose", prose, ""),
    ("/container", containers, ""),
    ("/scroll-area", scroll_areas, ""),
    ("/spinner", spinners, ""),
];

/// The other pages' live components, which the index shows too (`site::preview`).
pub(crate) const PREVIEWS: &[super::Preview] = &[("/button", buttons)];

fn buttons(ui: &Ui) -> Markup {
    let loading = ui.param("loading") == Some("1");
    lui! {
        Stack {
            // code: /button
            Cluster {
                Button("Save") primary loading=(loading);
                Button("Invite") soft;
                Button("Share") surface;
                Button("Cancel");
                Button("Skip") ghost;
                Button("Delete") danger;
                Button("Archived") disabled;
            }
            Cluster {
                Button("Small") small; Button("Default"); Button("Large") size=3;
                Button("\u{2026}") icon_only ghost aria_label="More";
                Button("Bold") pressed=(true) aria_label="Bold, on";
                LinkButton("Read the docs", "/");
            }
            Cluster {
                Badge("New"); Badge("Draft") secondary; Badge("Failed") danger;
                Badge("rust") outline; Badge("Paid") ok; Badge("Pending") warn;
                Badge("Live") solid; Badge("Beta") surface; Badge("Failed") danger solid;
            }
            Cluster {
                Button("Upgrade") primary shimmer; Button("What's new") shimmer;
                Badge("New") shimmer; Badge("Beta") outline shimmer;
            }
            Cluster {
                Button("Delete") danger ghost; Button("Retry") tone="warn" variant="soft" radius="full";
                Badge("Beta") tone="info" variant="surface" size=2; Badge("Synced") ok solid radius="full";
            }
            Cluster gap=3 { @for icon in Icon::ALL { Icon(icon) label=(icon.name()); } }
            // end code
        }
    }
}

async fn button_page(ui: Ui) -> Page {
    let loading = ui.param("loading") == Some("1");
    let body = lui! { Stack {
        (buttons(&ui))
        p class="lui-note" { "The server decides a button is loading: " a href=(if loading { "/button" } else { "/button?loading=1" }) { @if loading { "stop" } @else { "start" } } "." }
    } };
    page(&ui, "Buttons and badges", body)
}

fn fields(ui: &Ui) -> Markup {
    let email = ui.param("email").unwrap_or("");
    let bad = !email.is_empty() && !email.contains('@');
    lui! {
        // code: /field
        Form("/field") get submit="Check" {
            Input("name", "Name") placeholder="Ada Lovelace" help="As it should appear on invoices.";
            Input("email", "Email") email required value=(email)
                error=(if bad { "An email address needs an @." } else { "" });
            Input("site", "Website") leading="https://" placeholder="example.com";
            Input("q", "Search") search leading=(Icon::Search) placeholder="Search the docs…";
            Input("weight", "Weight") number=(0, 500) trailing="kg";
            Input("key", "API key") gradient_border placeholder="sk-live-...";
            Checkbox("terms", "I accept the terms") required;
            Switch("digest", "Weekly digest") checked=(ui.param("digest").is_some());
            RadioGroup("plan", "Plan") cards value=(ui.param("plan").unwrap_or("free")) {
                option "free" "Free" description="One project, community support.";
                option "pro" "Pro" description="Unlimited projects and history.";
                option "team" "Team" description="Everything in Pro, plus SSO.";
            }
        }
        // end code
    }
}

fn cards(ui: &Ui) -> Markup {
    let team = [
        ("Ada Lovelace", "Owner"),
        ("Grace Hopper", "Admin"),
        ("Alan Turing", "Member"),
    ];
    lui! {
        // code: /card
        Grid("16rem") {
            Card title="Team" description="3 people can edit this project."
                header={ Badge("Pro") secondary; }
                footer={ Button("Invite") primary; Button("Manage") ghost; } {
                Stack gap=4 {
                    Avatars("Editors") max=2 { avatar "Ada Lovelace"; avatar "Grace Hopper"; avatar "Alan Turing"; }
                    Stack gap=3 { @for (name, role) in team {
                        Cluster { Avatar(name); span { (name) } Badge(role) outline; }
                    } }
                }
            }
            Card title="Storage" description="Resets on the 1st." classic footer={ LinkButton("Upgrade", "/card"); } {
                Cluster { Avatar("Acme Inc") square; p { "3.2 GB of 5 GB used." } }
            }
            Card title="Pro" description="A beam runs round the border." beam glow
                footer={ Button("Start trial") primary shimmer; } {
                p { "The light at the top grows when you point at it." }
            }
            Card title="Changelog" description="A gradient border." gradient_border reveal {
                p { "Fades in as it scrolls into view." }
            }
        }
        // end code
    }
}

fn layouts(ui: &Ui) -> Markup {
    let tile = |t: &str| html! { div class="lui-layout-tile" { (t) } };
    lui! {
        // code: /layout
        Stack gap=6 {
            Cluster between { h3 { "Cluster" } Cluster { Button("Export"); Button("New") primary; } }
            Cluster nowrap align="baseline" justify="center" padding=2 { h3 { "Plan" } span { "$9 a month" } }
            Grid("8rem") gap=2 { @for t in ["Grid", "fills", "the row", "then", "wraps"] { (tile(t)) } }
            Split(html! { (tile("Split: side")) }, html! { (tile("main, stacks under the side when narrow")) })
                side_width="12rem";
        }
        // end code
    }
}

fn typography(ui: &Ui) -> Markup {
    let file = "release-2026-10-14-final-really-final.tar.gz";
    lui! {
        // code: /typography
        Heading("The quick brown fox") as_h=2 size=8;
        Heading("A section, level 3") as_h=3 size=5;
        Heading("A trimmed label") as_h=4 size=2 weight="medium" trim;
        Text("A lede at size 4 in the muted gray.") as_p size=4 muted;
        Text("Payment failed: the card was declined.") as_p tone="danger" weight="medium";
        Grid("12rem") { Text(file) truncate; Text(file) truncate muted; }
        p {
            "Read the " Link("pricing", "/pricing"); ", the "
            Link("changelog", "/typography") underline="hover"; ", or "
            Link("Radix Themes", "https://www.radix-ui.com/themes") external; "."
        }
        p { "Press " Kbd("⌘K"); " or " Kbd("Ctrl"); " " Kbd("K"); " to search, " Kbd("Esc") size=1; " to close." }
        p { "Run " Code("cargo add loco-ui"); ", then return " Code("Ok(())") tone="brand"; " or " Code("Err(e)") tone="danger" variant="outline"; "." }
        Blockquote caption="Alan Kay" {
            p { "Simple things should be simple, complex things should be possible." }
        }
        // end code
    }
}

const CARGO_TOML: &str =
    "[dependencies]\nloco-ui = { version = \"0.2\", features = [\"axum\"] }\naxum = \"0.8\"\n";
const MAIN_RS: &str = "async fn hello(ui: Ui) -> Page {\n    ui.page(\"Hello\", lui! { Heading(\"Hello\"); Text(\"No script needed.\") muted; })\n}\n";

fn code_blocks(ui: &Ui) -> Markup {
    lui! {
        // code: /code-block
        CodeBlock(CARGO_TOML) language="toml" title="Cargo.toml" copy;
        CodeBlock(MAIN_RS) language="rust" copy;
        CodeBlock("cargo lui add heading text link kbd code blockquote prose container section inset aspect-ratio scroll-area center spacer spinner indicator");
        // end code
    }
}

fn prose(ui: &Ui) -> Markup {
    lui! {
        // code: /prose
        Prose {
            h1 { "Release notes" }
            p { "Plain HTML, from Markdown or a CMS, styled for reading. " a href="/prose" { "Links" } ", " strong { "bold" } ", " code { "code" } " and " kbd { "Esc" } " all take the page's tokens." }
            h2 { "What changed" }
            ul { li { "Sixteen components for type and layout." } li { "Stacks and clusters align and pad." } li { "A spinner of its own, which buttons reuse." } }
            blockquote { "The best interface is the one the browser already has." }
            h3 { "Upgrading" }
            ol { li { "Bump the crate." } li { "Run the tests." } }
            pre { code { "cargo update -p loco-ui" } }
            table { thead { tr { th { "Component" } th { "Look" } } } tbody { tr { td { "Heading" } td { "Radix Themes" } } tr { td { "Prose" } td { "shadcn/ui" } } } }
            hr;
            p { "Built on the platform." }
        }
        // end code
    }
}

fn containers(ui: &Ui) -> Markup {
    let tile = |t: &str| html! { div class="lui-layout-tile" { (t) } };
    lui! {
        // code: /container
        Container size=1 { (tile("Container size 1: at most 448px, centred")) }
        Section size=1 label="A section" { (tile("Section size 1: 24px above and below")) }
        Grid("14rem") {
            Card {
                Inset top { AspectRatio ratio=(16, 9) { (tile("Inset top, 16 / 9")) } }
                Text("A cover reaching the card's edges.") as_p size=2 muted;
            }
            Card { Center min_height="9rem" text_center { Heading("Centred") as_h=3 size=3; Text("both ways") muted; } }
        }
        Cluster padding=2 { Button("Back"); Spacer; Button("Next") primary; }
        Stack align="center" justify="center" padding=4 gap=2 { (tile("align=center")) (tile("padding=4")) }
        // end code
    }
}

fn scroll_areas(ui: &Ui) -> Markup {
    lui! {
        // code: /scroll-area
        Card title="Release notes" {
            ScrollArea("Release notes") height="12rem" {
                Stack gap=2 { @for v in (1..=24).rev() { p { "v0." (v) ": fixes and small things." } } }
            }
        }
        // end code
    }
}

fn spinners(ui: &Ui) -> Markup {
    lui! {
        // code: /spinner
        Cluster gap=4 { Spinner size=1; Spinner; Spinner size=3 label="Loading results"; Button("Saving") loading=(true); }
        Cluster gap=6 {
            Indicator(3) { Button("Inbox"); }
            Indicator(120) max=99 { Button("Mail") soft; }
            Indicator(1) dot tone="ok" { Avatar("Ada Lovelace"); }
            Indicator(0) { Button("Nothing new") ghost; }
        }
        // end code
    }
}
