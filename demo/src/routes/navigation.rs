//! Navigation: command palette, drawer, breadcrumbs, sidebar, pagination, steps, timeline,
//! tree, collapsible and hover card.

use crate::site::{COMPONENTS, page};
use axum::{
    Router,
    response::{IntoResponse, Response},
    routing::get,
};
use loco_ui::pagination::Pagination;
use loco_ui::prelude::*;

pub(crate) fn routes() -> Router {
    super::pages(PAGES)
        .route("/palette", get(palette_page))
        .route("/pagination", get(pagination_page))
        .route("/sidebar/{*rest}", get(sidebar_under))
}

/// The pages that are their component and a note (`super::pages`).
pub(crate) const PAGES: &[super::Simple] = &[
    ("/nav", drawer, ""),
    (
        "/sidebar",
        sidebar,
        "Put it in a drawer's `.sidebar()` (or the app shell block) and it becomes a drawer on narrow screens.",
    ),
    (
        "/nav-menu",
        nav_menu,
        "A panel opens on click and closes on a click outside or Escape; the link to this page is marked current.",
    ),
    (
        "/steps",
        steps,
        "The row turns into a column under 30rem; `vertical` keeps the column. The [wizard](/wizard?step.signup=1) shows its progress with it.",
    ),
    (
        "/timeline",
        timeline,
        "From 30rem the dates move into a column of their own.",
    ),
    (
        "/tree",
        tree,
        "The folder holding this page is open; `?open.files=0,2` opens others. A folder that is a link keeps its link beside the summary, never in it.",
    ),
    (
        "/collapsible",
        collapsible,
        "The two with `name` share a group: opening one closes the other.",
    ),
    (
        "/hover-card",
        hover_card,
        "Where the browser has interest invokers the cards open on hover and focus; elsewhere a click opens them. See [/caps](/caps).",
    ),
];

/// The other pages' live components, which the index shows too (`site::preview`).
pub(crate) const PREVIEWS: &[super::Preview] = &[
    ("/palette", |ui| palette(ui).render()),
    ("/pagination", |ui| pagination(ui).render()),
];

/// Every demo page as a command, plus a few deep links.
fn palette(ui: &Ui) -> loco_ui::palette::Palette<'_> {
    // code: /palette
    ui.palette("/palette")
        .id("cmd")
        .group("Components")
        .links(COMPONENTS.iter().map(|c| (c.title, c.href)))
        .group("Shortcuts")
        .link("Notification settings", "/settings?tab.settings=1")
        .keywords("email releases")
        .icon(Icon::Mail)
        .shortcut("g n")
        .link("Largest files", "/table?sort.files=-size")
        .keywords("sort size big")
        .icon(Icon::File)
        .shortcut("g f")
        .link("Open the delete dialog", "/dialog?dialog=confirm")
        .keywords("account remove")
        .icon(Icon::Trash)
    // end code
}

/// An exact command name redirects; anything else lists the matches.
async fn palette_page(ui: Ui) -> Response {
    let palette = palette(&ui);
    // code: /palette
    if let Some(href) = palette.exact() {
        return ui.redirect(href).into_response();
    }
    // end code
    page(&ui, "Command palette", html! {
        p { "Open it with the button or the access key, type, pick a suggestion and press Enter. An exact name goes straight to the page; anything else lists what matches." }
        (palette)
    }).into_response()
}

/// A sidebar on wide screens, a drawer on narrow ones, and breadcrumbs above the content.
fn drawer(ui: &Ui) -> Markup {
    lui! {
            // code: /nav
            Drawer("Menu") id="site" title="loco-ui" description="Version 0.1" sidebar
                footer={ LinkButton("Help", "/") ghost; }
                nav={ ul {
                    li { a href="/nav" aria-current="page" { "Overview" } }
                    li { a href="/table" { "Files" } } li { a href="/dashboard" { "Reports" } } li { a href="/settings" { "Settings" } }
                } } {
                    Breadcrumbs { link "Home" "/"; link "Projects" "/nav"; link "loco-ui" "/nav"; here "Docs"; }
                    p { "Wider than 60rem the navigation is a sidebar; narrower, the menu button opens it as a drawer. Escape or a click outside closes it." }
                    p { "A long trail folds its middle so both ends stay readable:" }
                    Breadcrumbs {
                        link "Home" "/"; link "Projects" "/nav"; link "loco-ui" "/nav";
                        link "Components" "/"; link "Navigation" "/nav"; here "Breadcrumbs";
                    }
                    p { "At most three before the fold, a slash between:" }
                    Breadcrumbs max=3 separator="/" {
                        link "Home" "/"; link "Projects" "/nav"; link "loco-ui" "/nav"; here "Docs";
                    }
            // end code
                    p class="lui-note" { "Server-opened: " a href="/nav?dialog=site" { "?dialog=site" } }
                }
    }
}

/// A navigation column with groups, icons and counts; the link to this page is current.
fn sidebar(ui: &Ui) -> Markup {
    lui! {
        div style="max-width: 16rem" {
            // code: /sidebar
            Sidebar("Mail") collapsible current_prefix {
                group "Mail";
                link "Inbox" "/sidebar" icon=(Icon::Mail) badge="12";
                link "Drafts" "/sidebar?box=drafts" icon=(Icon::Pencil);
                link "Sent" "/sidebar?box=sent" icon=(Icon::ArrowRight);
                group "Labels";
                link "Work" "/sidebar/work"; link "Personal" "/sidebar/personal";
            }
            // end code
        }
    }
}

/// Top navigation: plain links and buttons that open a panel of links.
fn nav_menu(ui: &Ui) -> Markup {
    lui! {
        // code: /nav-menu
        NavMenu("Main") {
            panel "Products" ([
                ("Mail", "/sidebar", "An inbox with folders and labels."),
                ("Calendar", "/calendar", "Months you page through, days you pick."),
                ("Files", "/table", "Sort, filter and page a thousand rows."),
            ]);
            panel "Resources" ([("Docs", "/"), ("Theming", "/?palette=linen")]);
            link "Pricing" "/pricing";
            link "Navigation menu" "/nav-menu";
        }
        // end code
    }
}

/// A page under a label of the sidebar (`/sidebar/work/notes`): `current_prefix` marks
/// the label's link.
async fn sidebar_under(ui: Ui) -> Page {
    page(&ui, crate::site::title("/sidebar"), sidebar(&ui))
}

/// How many posts the paged list holds.
const POSTS: usize = 240;

/// Page links for the posts: `?page.posts=n`, ten a page.
fn pagination(ui: &Ui) -> Pagination<'_> {
    // code: /pagination
    ui.pagination("/pagination")
        .id("posts")
        .total(POSTS)
        .first_last()
    // end code
}

/// The posts of this page, the links under them, and `rel=prev/next` in the head.
async fn pagination_page(ui: Ui) -> Page {
    let pages = pagination(&ui);
    let shown = pages.range();
    // code: /pagination
    let body = lui! { Stack gap=4 {
        ol class="lui-pager-list" start=(shown.start + 1) {
            @for i in shown { li { "Post " (i + 1) } }
        }
        (pages)
        // end code
        p class="lui-note" { "Under 30rem only the first, the current and the last page stay numbered. The page's head links the pages before and after it with " code { "rel=prev" } " and " code { "rel=next" } "." }
    } };
    // code: /pagination
    pages.link_rels(page(&ui, "Pagination", body))
    // end code
}

/// A checkout's steps as a row, and the same as a column.
fn steps(ui: &Ui) -> Markup {
    lui! {
        // code: /steps
        Steps {
            step "Cart" done href="/steps";
            step "Address" done href="/steps";
            step "Payment" current description="Card or bank transfer.";
            step "Review" optional;
        }
        Steps vertical {
            step "Account" done href="/steps";
            step "Profile" current invalid description="The name is missing.";
            step "Done";
        }
        // end code
    }
}

/// A release history: dates, icons and tones.
fn timeline(ui: &Ui) -> Markup {
    lui! {
        // code: /timeline
        Timeline {
            event "Version 0.2 released" when="14 October 2026" datetime="2026-10-14" icon=(Icon::Check) tone="ok"
                description="Typed variants, a layered stylesheet and eight new components.";
            event "Release candidate" when="2 October 2026" datetime="2026-10-02" tone="brand"
                description="Feature freeze; only fixes from here.";
            event "Rolled back the beta" when="20 September 2026" datetime="2026-09-20" icon=(Icon::TriangleAlert) tone="danger";
            event "Project started" when="March 2026";
        }
        // end code
    }
}

/// A source tree whose current file is this page.
fn tree(ui: &Ui) -> Markup {
    lui! {
        div style="max-width: 20rem" {
            // code: /tree
            Tree("Files") {
                node "src" icon=(Icon::Folder) link="/tree?f=src" children=(|t| t
                    .node("lib.rs").icon(Icon::File).link("/tree?f=lib.rs")
                    .node("components").icon(Icon::Folder).children(|t| t
                        .node("tree.rs").icon(Icon::File).link("/tree")
                        .node("steps.rs").icon(Icon::File).link("/tree?f=steps.rs")));
                node "tests" icon=(Icon::Folder) children=(|t| t.node("demo.rs").icon(Icon::File).link("/tree?f=demo.rs"));
                node "README.md" icon=(Icon::File) link="/tree?f=readme";
            }
            // end code
        }
    }
}

/// One collapsible on its own and two that share a group.
fn collapsible(ui: &Ui) -> Markup {
    lui! {
        // code: /collapsible
        Collapsible("Show 3 more", html! { ul { li { "Cascade layers" } li { "Container queries" } li { "Anchor positioning" } } });
        Collapsible("Shipping", html! { p { "Two to four working days." } }) ghost name="faq";
        Collapsible("Returns", html! { p { "Thirty days, no questions." } }) ghost name="faq";
        // end code
    }
}

/// A profile card on a button and a preview card on a link.
fn hover_card(ui: &Ui) -> Markup {
    lui! {
        p { "Written by" }
        // code: /hover-card
        HoverCard("@ada", html! { strong { "Ada Lovelace" } br; "Wrote the first program, for an engine that was never built." });
        HoverCard("Release notes", html! { strong { "Version 0.2" } br; "Typed variants, a layered stylesheet, eight new components." }) href="/timeline";
        // end code
    }
}
