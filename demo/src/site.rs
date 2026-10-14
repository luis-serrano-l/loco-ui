//! What every page shares: the component index, the shell around each page (the sidebar of
//! every component, toolbar, title, the stage and its code), the index page (a gallery of the
//! components themselves) and the theme switch.

use crate::code::CODE;
use axum::{
    Form, Router,
    http::HeaderMap,
    routing::{get, post},
};
use loco_ui::layout::{Look, Palette, Radius, Scale, Tokens};
use loco_ui::prelude::*;
use serde::Deserialize;
use std::borrow::Cow;

pub(crate) fn routes() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/lang", post(lang_submit))
}

/// Every component in the index: path, title (what each route passes to `page`), group, the
/// platform features it is built on, and what it is for in plain words.
pub(crate) const COMPONENTS: [(&str, &str, &str, &str, &str); 48] = [
    (
        "/feedback",
        "Alerts, progress and tooltips",
        "Feedback",
        "role=alert, <progress>, <meter>, :hover/:focus-within, <hr>",
        "Callouts, bars and meters, a tooltip on hover or focus, and separators.",
    ),
    (
        "/app/signin",
        "Sign in",
        "Complete flows",
        "server validation, errors re-rendered, a session cookie, PRG",
        "A sign-in form whose mistakes come back from the server, next to the fields.",
    ),
    (
        "/app/notes",
        "Notes",
        "Complete flows",
        "create, edit in place, delete, filter, pages, flash, PRG",
        "A small app: add, rename and delete notes, with script off.",
    ),
    (
        "/pricing",
        "Pricing card",
        "Your own",
        "ui.card, ui.badge, ui.link_button, Icon, ui.grid, Page::css",
        "A component written in the demo crate, from the public primitives only.",
    ),
    (
        "/kanban",
        "Kanban",
        "Widgets",
        "form POST per move, PRG, view-transition-name, scroll-snap",
        "Cards in columns; each move is a form post the server keeps.",
    ),
    (
        "/sortable",
        "Sortable list",
        "Widgets",
        "form POST per move, PRG, view-transition-name, drag and drop with the script",
        "A list in the order the server keeps; arrows without script, a grip to drag with it.",
    ),
    (
        "/upload",
        "Upload",
        "Widgets",
        "multipart POST, <input type=file>, drop on the input, <progress>, PRG",
        "Send files; the list under the form is what the server kept.",
    ),
    (
        "/calendar",
        "Calendar",
        "Widgets",
        "<table>, links or radios, aria-current=date, :has(:checked), ?month=",
        "A month you can page through and pick a day from.",
    ),
    (
        "/marquee",
        "Marquee",
        "Widgets",
        "@keyframes, translate, :hover/:focus-within, aria-hidden + inert",
        "A row that loops sideways on its own and stops under the pointer or focus.",
    ),
    (
        "/button",
        "Buttons and badges",
        "Primitives",
        "<button>, invoker commands, popovertarget, aria-busy, inline <svg>",
        "The button every other component is built from, with badges and icons.",
    ),
    (
        "/field",
        "Fields",
        "Primitives",
        "<label>, aria-describedby, :user-invalid, role=switch, <fieldset>",
        "A labelled input, checkbox, switch and radio group, with help and errors.",
    ),
    (
        "/card",
        "Cards and avatars",
        "Primitives",
        "grid, <img alt=\"\">, loading=lazy",
        "A box with a header, body and footer, and a picture that falls back to initials.",
    ),
    (
        "/layout",
        "Layout",
        "Primitives",
        "flex gap, flex-wrap, repeat(auto-fill), custom properties",
        "Stack, cluster, grid and split: even spacing with no margins, and columns that wrap on their own.",
    ),
    (
        "/palette",
        "Command palette",
        "Navigation",
        "popover, <datalist>, <search>, accesskey, GET + 303",
        "Jump to any page by typing its name.",
    ),
    (
        "/nav",
        "Drawer and breadcrumbs",
        "Navigation",
        "<dialog>, invoker commands, closedby, @starting-style, <details>",
        "A sidebar that turns into a drawer on small screens, with a trail back up.",
    ),
    (
        "/toast",
        "Toasts",
        "Feedback",
        "position: fixed, role=alert, CSS fade, PRG",
        "Short messages in the corner after a form is sent.",
    ),
    (
        "/dashboard",
        "Stats and empty states",
        "Feedback",
        "auto-fit grid, form POST",
        "Numbers with how they changed, and what to show when there is nothing yet.",
    ),
    (
        "/dialog",
        "Dialog",
        "Overlays",
        "<dialog>, closedby, invoker commands, form footer",
        "Ask before doing something that cannot be undone.",
    ),
    (
        "/popover",
        "Popover menu",
        "Overlays",
        "popover, anchor positioning, nested popover, form actions",
        "A menu of links and actions that opens over the page.",
    ),
    (
        "/tabs",
        "Tabs",
        "Disclosure",
        "<details name>, ::details-content, view-transition-name, grid",
        "Several panels in one place, one open at a time.",
    ),
    (
        "/accordion",
        "Accordion",
        "Disclosure",
        "<details name>, ::details-content, interpolate-size",
        "Questions that open to their answers.",
    ),
    (
        "/combobox",
        "Combobox",
        "Input",
        "<datalist>, <optgroup>, <search>, aria-live",
        "Search a list and pick one item or several.",
    ),
    (
        "/form",
        "Validated form",
        "Input",
        ":user-invalid, <fieldset>, <output> counters, field-sizing, multipart, PRG, error summary with autofocus",
        "Fields the browser checks first and the server checks again.",
    ),
    (
        "/wizard",
        "Wizard",
        "Input",
        "one form per step, PRG, formnovalidate, <progress>, UiState",
        "A long form split into steps you can leave and come back to.",
    ),
    (
        "/inputs",
        "Select, range, colour",
        "Input",
        "<selectedcontent>, <optgroup>, formmethod, two-thumb range, color-mix()",
        "Pick a size, a country, a volume, a price range and a colour.",
    ),
    (
        "/counter",
        "Counter",
        "Server state",
        "form POST + cookie, type=number, disabled",
        "A number that goes up and down within limits.",
    ),
    (
        "/settings",
        "Settings",
        "Server state",
        "UiState, PRG + flash, role=alert, CSS auto-hide",
        "Tabs of settings that stay where you left them.",
    ),
    (
        "/list",
        "Load-more list",
        "Server state",
        "links + view transitions",
        "A long list shown a page at a time.",
    ),
    (
        "/table/minimal",
        "Table, minimal",
        "Server state",
        "sort links, <search> filter, ?page.<id>=n",
        "The smallest useful table: sorted, filtered and paged by the URL.",
    ),
    (
        "/table",
        "Table",
        "Server state",
        "sort links, <search> filter, form= checkboxes, ?cols.<id>=, <details> rows, sticky first column, ?page.<id>=n",
        "Sort, filter, page through and select rows of data.",
    ),
    (
        "/caps",
        "Capabilities",
        "Server state",
        "@supports beacons + cookie",
        "What the server knows this browser can do.",
    ),
    (
        "/stream",
        "Streaming",
        "Server state",
        "declarative shadow DOM slots, skeleton placeholders, aria-busy",
        "A page that sends its fast parts first.",
    ),
    (
        "/swap",
        "Swap targets",
        "Server state",
        "data-lui-target, data-lui-swap, data-lui-oob, data-lui-indicator, data-lui-push, Lui-Enhance header",
        "Update one part of the page without reloading it.",
    ),
    (
        "/blocks/shell",
        "App shell",
        "Blocks",
        "a sidebar that is a drawer on narrow screens, aria-current",
        "The frame of a signed-in app: navigation, who is signed in, the page.",
    ),
    (
        "/blocks/auth",
        "Auth page",
        "Blocks",
        "a card, a form that posts",
        "A sign-in or sign-up page, centred, with the links to the other account pages.",
    ),
    (
        "/blocks/settings",
        "Settings page",
        "Blocks",
        "fragment links, a two-column grid",
        "Settings in sections, each with its own form, and a list that jumps to each.",
    ),
    (
        "/blocks/record",
        "Record page",
        "Blocks",
        "<dl>, a delete form, PRG",
        "One record's fields and the actions on it.",
    ),
    (
        "/blocks/dashboard",
        "Dashboard page",
        "Blocks",
        "auto-fit grid",
        "A row of numbers and what goes under them.",
    ),
    (
        "/blocks/error",
        "Error page",
        "Blocks",
        "Router::fallback, HTTP status",
        "The 404 and 500 pages in the site's look; the demo's fallback.",
    ),
    (
        "/chart",
        "Charts",
        "Feedback",
        "inline <svg>, focusable marks, :focus-visible chips, a hidden data table",
        "Bars, a line and a sparkline drawn on the server, no chart library.",
    ),
    (
        "/sidebar",
        "Sidebar",
        "Navigation",
        "<nav>, aria-current",
        "An app's navigation as a column: groups, icons, counts, the current page marked.",
    ),
    (
        "/nav-menu",
        "Navigation menu",
        "Navigation",
        "popover, anchor-name, <details> fallback",
        "Top navigation whose items open panels of links.",
    ),
    (
        "/description-list",
        "Description list",
        "Feedback",
        "<dl>, @container, subgrid",
        "Terms and their details, side by side or stacked.",
    ),
    (
        "/toggle-group",
        "Toggle group",
        "Input",
        "radios and checkboxes drawn as segments, :checked",
        "Pressable options, one or several, sent with their form.",
    ),
    (
        "/otp",
        "One-time code",
        "Input",
        "autocomplete=one-time-code, inputmode=numeric, pattern",
        "The box for a code from a text message, which the phone offers to fill.",
    ),
    (
        "/context-menu",
        "Context menu",
        "Overlays",
        "popover, a menu on a secondary button",
        "Actions on one thing from a button in its corner.",
    ),
    (
        "/theme",
        "Theme builder",
        "Primitives",
        "<input type=color>, --lui-* custom properties, a GET form, a CSS download",
        "Pick the colours and the radius, see a few components in them, download theme.css.",
    ),
    (
        "/modes",
        "Modes",
        "Primitives",
        "@media (forced-colors: active), (prefers-contrast: more), print",
        "The controls drawn in CSS under forced colours and more contrast, emulated by a class.",
    ),
];

/// The index's layers, bottom up, each with the groups it holds (as in `docs/layers.svg`).
pub(crate) const LAYERS: [(&str, &str, &[&str]); 6] = [
    (
        "Primitives",
        "The parts every component is built from.",
        &["Primitives"],
    ),
    (
        "Components",
        "Built from the primitives: one change to the button restyles them all.",
        &[
            "Overlays",
            "Disclosure",
            "Navigation",
            "Input",
            "Feedback",
            "Server state",
        ],
    ),
    (
        "Widgets",
        "Larger pieces built from components and primitives.",
        &["Widgets"],
    ),
    (
        "Blocks",
        "Whole pages from the components: fill one in, or copy its file when yours differs.",
        &["Blocks"],
    ),
    (
        "Your own",
        "A component written in the demo crate, the way you would write one.",
        &["Your own"],
    ),
    (
        "Complete flows",
        "Whole tasks, end to end, with JavaScript off: the parts working together.",
        &["Complete flows"],
    ),
];

/// The second palette from `docs/theming.md`: warm paper, copper primary, amber in the dark.
const LINEN: Tokens = Tokens {
    gray: Scale::SLATE,
    brand: Scale::INDIGO,
    light: Palette {
        bg: Cow::Borrowed("#f4efe6"),
        fg: Cow::Borrowed("#1d1a17"),
        muted: Cow::Borrowed("#5d574f"),
        line: Cow::Borrowed("#d6cdbf"),
        surface: Cow::Borrowed("#fffdf9"),
        card: Cow::Borrowed("#fffdf9"),
        popover: Cow::Borrowed("#fffdf9"),
        secondary: Cow::Borrowed("#ebe3d6"),
        accent: Cow::Borrowed("#ebe3d6"),
        on_accent: Cow::Borrowed("#1d1a17"),
        primary: Cow::Borrowed("#8a3b12"),
        on_primary: Cow::Borrowed("#ffffff"),
        input: Cow::Borrowed("#d6cdbf"),
        ring: Cow::Borrowed("#b5764f"),
        link: Cow::Borrowed("#8a3b12"),
        danger: Cow::Borrowed("#a0261c"),
        on_danger: Cow::Borrowed("#ffffff"),
        ok: Cow::Borrowed("#2f6b3a"),
        warn: Cow::Borrowed("#7a5500"),
        info: Cow::Borrowed("#245f8f"),
        on_info: Cow::Borrowed("#ffffff"),
    },
    dark: Palette {
        bg: Cow::Borrowed("#161311"),
        fg: Cow::Borrowed("#ece6dc"),
        muted: Cow::Borrowed("#a59c90"),
        line: Cow::Borrowed("#3a332c"),
        surface: Cow::Borrowed("#1f1b18"),
        card: Cow::Borrowed("#1f1b18"),
        popover: Cow::Borrowed("#1f1b18"),
        secondary: Cow::Borrowed("#2b2521"),
        accent: Cow::Borrowed("#2b2521"),
        on_accent: Cow::Borrowed("#ece6dc"),
        primary: Cow::Borrowed("#e8965a"),
        on_primary: Cow::Borrowed("#1a0f06"),
        input: Cow::Borrowed("#4a4038"),
        ring: Cow::Borrowed("#a8683a"),
        link: Cow::Borrowed("#e8965a"),
        danger: Cow::Borrowed("#ff8f85"),
        on_danger: Cow::Borrowed("#1a0f06"),
        ok: Cow::Borrowed("#8fd39a"),
        warn: Cow::Borrowed("#f0c060"),
        info: Cow::Borrowed("#8cc4f0"),
        on_info: Cow::Borrowed("#1a0f06"),
    },
    radius: Radius::new("3px"),
    ..Tokens::DEFAULT
};

/// A component page's live component, from the `PAGES` or `PREVIEWS` beside its route: the
/// function its page calls between the `// code:` markers, so the index shows the same call
/// the page does.
pub(crate) fn preview(href: &str) -> Option<fn(&Ui) -> Markup> {
    use crate::routes::*;
    let pages = [
        primitives::PAGES,
        overlays::PAGES,
        disclosure::PAGES,
        navigation::PAGES,
        input::PAGES,
        feedback::PAGES,
        server_state::PAGES,
        widgets::PAGES,
        blocks::PAGES,
    ];
    let previews = [
        primitives::PREVIEWS,
        theme::PREVIEWS,
        disclosure::PREVIEWS,
        navigation::PREVIEWS,
        input::PREVIEWS,
        feedback::PREVIEWS,
        table::PREVIEWS,
        server_state::PREVIEWS,
        widgets::PREVIEWS,
        flows::PREVIEWS,
        own::PREVIEWS,
    ];
    let simple = pages.into_iter().flatten().map(|p| (p.0, p.1));
    simple
        .chain(previews.into_iter().flatten().copied())
        .find(|p| p.0 == href)
        .map(|p| p.1)
}

/// A component page's title, as `COMPONENTS` has it.
pub(crate) fn title(href: &'static str) -> &'static str {
    COMPONENTS
        .iter()
        .find(|c| c.0 == href)
        .map_or(href, |c| c.1)
}

/// A note's text as markup: `` `x` `` is code, `[text](href)` a link.
pub(crate) fn note(text: &str) -> Markup {
    let link = |part: &str| {
        let mut rest = part;
        let mut out = Vec::new();
        while let Some((before, after)) = rest.split_once('[')
            && let Some((text, after)) = after.split_once("](")
            && let Some((href, after)) = after.split_once(')')
        {
            out.push(html! { (before) a href=(href) { (text) } });
            rest = after;
        }
        html! { @for m in out { (m) } (rest) }
    };
    html! { @for (i, part) in text.split('`').enumerate() { @if i % 2 == 1 { code { (part) } } @else { (link(part)) } } }
}

/// Every component under its group, in the index's order, the current page marked. From 60rem
/// it is a sticky column beside the page; narrower, the `<details>` before it hides it until
/// opened (CSS in `layout.rs`, no script).
fn sidebar(ui: &Ui) -> Markup {
    let mut nav = ui.sidebar("Components").link("Overview", "/");
    for group in LAYERS.iter().flat_map(|l| l.2) {
        nav = nav.group(group);
        for (href, title, ..) in COMPONENTS.iter().filter(|c| c.2 == *group) {
            nav = nav.link(title, href);
        }
    }
    html! {
        div class="lui-site-nav" {
            details class="lui-site-menu" { summary class="lui-button" { "Browse components" (Icon::ChevronDown) } }
            (nav)
        }
    }
}

/// The top of every page: the way back to the index (not on the index), then the title with
/// one toolbar beside it, the language of the components' own words and the theme switch
/// (under the title when the page is narrow).
fn title_bar(ui: &Ui, back: bool, title: Markup) -> Markup {
    let languages = html! {
        form method="post" action="/lang" class="lui-lang" {
            @for (tag, name) in [("en", "English"), ("es", "Español"), ("ar", "العربية")] {
                (ui.button(name).small().ghost().name("lang").value(tag).pressed(ui.lang() == tag))
            }
        }
    };
    html! {
        @if back { a class="lui-back" href="/" { "All components" } }
        div class="lui-title-bar" {
            h1 { (title) }
            div class="lui-toolbar" { (languages) (ui.theme_toggle("/theme")) }
        }
    }
}

/// What every page shows around its body: the toolbar and the title, and on a component page
/// what it is for and built on, then the body on a stage with the code that drew it underneath.
pub(crate) fn shell(ui: &Ui, title: &str, body: Markup) -> Markup {
    let component = COMPONENTS.iter().find(|c| c.1 == title);
    let page = match component {
        Some(c) => component_page(ui, c, body),
        None => html! { (title_bar(ui, false, html! { (title) })) (body) },
    };
    html! {
        div class="lui-site" {
            (sidebar(ui))
            div class={ "lui-site-main" @if component.is_none() { " lui-site-wide" } } { (page) }
        }
    }
}

/// A component page: what it is for and built on, then the body on a stage with the code
/// that drew it underneath, and the props of the builders that code calls.
fn component_page(ui: &Ui, component: &(&str, &str, &str, &str, &str), body: Markup) -> Markup {
    html! {
        (title_bar(ui, true, html! { (component.1) @if beta(component.0) { " " (ui.badge("beta").warn()) } }))
        p class="lui-lede" { (component.4) }
        p class="lui-built" { "Built on " @for feature in component.3.split(", ") { code { (feature) } " " } }
        div class="lui-plate" {
            div class="lui-stage" { (body) }
            figure class="lui-snippet" {
                @if let Some((_, path, code, _)) = CODE.iter().find(|h| h.0 == component.0) {
                    figcaption { span { (path) } span { "The code behind the component above" } }
                    pre tabindex="0" aria-label=(path) { code { (maud::PreEscaped(code)) } }
                }
            }
        }
        @if let Some((.., builders)) = CODE.iter().find(|h| h.0 == component.0).filter(|h| !h.3.is_empty()) {
            (props(ui, builders))
        }
    }
}

/// Whether a component page shows a builder whose API is still beta.
fn beta(href: &str) -> bool {
    CODE.iter()
        .find(|h| h.0 == href)
        .is_some_and(|h| h.3.iter().any(|b| b.status == loco_ui::props::Status::Beta))
}

/// `text` with each `` `span` `` as `<code>`, as rustdoc shows it.
fn inline_code(text: &str) -> Markup {
    html! { @for (i, part) in text.split('`').enumerate() { @if i % 2 == 1 { code { (part) } } @else { (part) } } }
}

/// What each builder on the page accepts, from `loco_ui::props()`: one `<details>` per
/// builder (the first open, or the one being tried), its constructors in the summary and a
/// table of its setters inside. For a builder the playground knows, the table is a form with
/// a "Try" column, followed by the component as chosen and its `lui!` line.
fn props(ui: &Ui, builders: &[&loco_ui::props::Component]) -> Markup {
    let tried = |b: &str| crate::playground::tried(ui, b);
    let any_tried = builders.iter().any(|b| tried(b.builder));
    html! {
        section class="lui-props" {
            h2 { "Props" }
            @for (i, b) in builders.iter().enumerate() {
                @let has_try = crate::playground::entry(b.builder).is_some();
                details open[if any_tried { tried(b.builder) } else { i == 0 }] {
                    summary {
                        code { (b.lui()) }
                        @for call in b.calls { " " code { (call) } }
                        " " (ui.badge(b.status.as_str()).outline())
                        span { (b.props.len()) @if b.props.len() == 1 { " prop" } @else { " props" } }
                    }
                    @if !b.props.is_empty() {
                        (crate::playground::playground(ui, ui.state.path(), b, |cell| html! {
                            div class="lui-props-scroll" tabindex="0" role="region" aria-label={ (b.builder) " props" } { table {
                                thead { tr { th { "Prop" } th { "Kind" } th { "Arguments" } th { "Default" } th { "HTML" } th { "What it does" } @if has_try { th { "Try" } } } }
                                tbody { @for p in b.props { tr {
                                    td { code { (p.name) } }
                                    td { (p.kind.as_str()) @if let loco_ui::props::PropKind::Choice { of } = p.kind { br; small { (of.join(", ")) } } @if !p.applies_to.is_empty() { br; small { "of " (p.applies_to.join(", ")) } } }
                                    td { @if !p.args.is_empty() { code { (p.args) } } }
                                    td { @if !p.default.is_empty() { code { (p.default) } } }
                                    td { @if !p.attr.is_empty() { code { (p.attr) } } }
                                    td { (inline_code(p.doc)) }
                                    @if has_try { td { (cell(p)) } }
                                } } }
                            } }
                        }))
                    }
                }
            }
        }
    }
}

pub(crate) fn page(ui: &Ui, title: &str, body: Markup) -> Page {
    // A pending flash on the stage, over the component that sent it, unless the body shows
    // it itself (as toasts, or with setters); `ui.page` would put it above the whole shell.
    let own = ["class=\"lui-flash\"", "class=\"lui-toasts\""];
    let body = if own.iter().any(|c| body.0.contains(c)) {
        body
    } else {
        html! { (ui.flash()) (body) }
    };
    let page = ui.page(title, shell(ui, title, body));
    // `?script=off`: the same page without the enhancement script, served under
    // `script-src 'none'`, so the no-script path can be tried in any browser.
    let page = if ui.param("script") == Some("off") {
        page.without_script()
    } else {
        page
    };
    // `?css=linked`: the stylesheet as `/lui.css` instead of inline, served without
    // `'unsafe-inline'` for `<style>` (the demo's own CSS goes in by hash).
    if ui.param("css") == Some("linked") {
        page.linked_stylesheet()
    } else {
        page
    }
}

async fn index(ui: Ui) -> Page {
    let linen = ui.param("palette") == Some("linen");
    let page = page(
        &ui,
        "Components",
        html! {
            @let count = |groups: &[&str]| COMPONENTS.iter().filter(|c| groups.contains(&c.2)).count();
            p class="lui-lede" { (count(LAYERS[0].2)) " primitives, " (count(LAYERS[1].2)) " components, " (count(LAYERS[2].2)) " widgets and one of your own, for Axum and Maud, all working with JavaScript turned off. The HTML platform and plain form posts do the work. Each page loads one optional script, " code { "/lui/enhance.js" } ", which updates the same markup in place instead of reloading. Block it and every page still works. Every component below is live: the same call its page shows, and its name leads to that page." }
            @if !ui.has(Cap::Probed) { p class="lui-note" { "First visit: this page is the fallback variant. Reload and the server will know your browser." } }
            p class="lui-note" { "Theme: " @if linen { a href="/" { "neutral" } " · linen and copper" } @else { "neutral · " a href="/?palette=linen" { "linen and copper" } } ", see " code { "docs/theming.md" } }
            div class="lui-index" { @for (layer, blurb, groups) in LAYERS {
                h2 { (layer) }
                p class="lui-index-layer" { (blurb) }
                @for group in groups.iter() {
                    @if groups.len() > 1 { h3 { (group) } }
                    ul { @for (href, title, .., what) in COMPONENTS.iter().filter(|c| c.2 == *group) {
                        li class=[wide(href).then_some("lui-index-wide")] {
                            p { a href=(href) { (title) } @if beta(href) { " " (ui.badge("beta").warn()) } }
                            p class="lui-note" { (what) }
                            div class="lui-index-stage" { @if let Some(preview) = preview(href) { (preview(&ui)) } }
                        }
                    } }
                }
            } }
            // Idle-time fetch of every component page, so the click is served from cache.
            @for (href, ..) in COMPONENTS { link rel="prefetch" href=(href); }
        },
    );
    let page = page.css(crate::pricing::PRICING_CSS);
    if linen { page.tokens(&LINEN) } else { page }
}

/// Components shown across the whole row of the index: the blocks and flows (whole pages),
/// the pricing cards, and the few components as wide as the page they sit on.
fn wide(href: &str) -> bool {
    let groups = ["Blocks", "Your own", "Complete flows"];
    COMPONENTS
        .iter()
        .any(|c| c.0 == href && groups.contains(&c.2))
        || [
            "/kanban",
            "/table",
            "/table/minimal",
            "/theme",
            "/nav",
            "/inputs",
            "/chart",
        ]
        .contains(&href)
}

#[derive(Deserialize)]
pub(crate) struct ThemeForm {
    theme: String,
}

/// Keep the picked theme and go back to the page the toggle was on.
pub(crate) async fn theme_submit(
    ui: Ui,
    headers: HeaderMap,
    Form(form): Form<ThemeForm>,
) -> Redirect {
    ui.redirect(&back_to(&headers))
        .theme(Theme::parse(&form.theme))
}

#[derive(Deserialize)]
struct LangForm {
    lang: String,
}

/// Keep the picked language and go back to the page the switch was on.
async fn lang_submit(ui: Ui, headers: HeaderMap, Form(form): Form<LangForm>) -> Redirect {
    ui.redirect(&back_to(&headers)).lang(&form.lang)
}

/// The path of the page a form was posted from (same-origin `Referer`), or `/`.
fn back_to(headers: &HeaderMap) -> String {
    headers
        .get("referer")
        .and_then(|v| v.to_str().ok())
        .and_then(|url| url.splitn(4, '/').nth(3))
        .map(|path| format!("/{path}"))
        .filter(|path| !path.starts_with("//"))
        .unwrap_or_else(|| "/".to_string())
}

/// The demo's look: its header and its own CSS (the shell, the index, the snippets, the
/// props tables, the theme builder), on every page through one layer in `router()`. The
/// CSS is unlayered, so it wins over the library's layers.
pub static LOOK: Look = Look {
    tokens: Tokens::DEFAULT,
    css: &[CSS],
    header: Some(header),
};

/// The site header shown above every page.
fn header() -> Markup {
    html! {
        header class="lui-site-header" {
            a href="/" { strong { "loco-ui" } }
            span { "Interactive HTML for Rust servers, works without JavaScript" }
        }
    }
}

/// The demo's own styles, outside the library: the shell, the index, the snippets, the
/// props tables, the theme builder and the modes page.
/// [`CSS`] as pages carry it, for the markup budget in the tests.
#[cfg(test)]
pub(crate) static CSS_MINIFIED: std::sync::LazyLock<String> =
    std::sync::LazyLock::new(|| loco_ui::minify_css(CSS));

pub const CSS: &str = r#"
/* The site header above every page: the name and what it is. */
.lui-site-header {
  display: flex; flex-wrap: wrap; align-items: baseline; gap: 0.25rem 1rem;
  max-width: 52rem; margin: 0 auto; padding: calc(var(--lui-space) * 2);
  color: var(--lui-muted);
}
.lui-site-header a { color: var(--lui-fg); text-decoration: none; font-size: 1rem; letter-spacing: -0.01em; }
.lui-site-header span { font-size: 0.875rem; }
.lui-site-header strong { font-weight: 600; }
/* The dashed boxes on the layout page. */
.lui-layout-tile { padding: calc(var(--lui-space) * 1.5); border: 1px dashed var(--lui-input); border-radius: var(--lui-radius-sm); background: var(--lui-surface); font-size: 0.875rem; }
.lui-yes { color: var(--lui-ok); font-weight: 600; }
.lui-no { color: var(--lui-danger); font-weight: 600; }
/* Demo shell, after the shadcn docs: the sidebar of every component beside the page, a
   toolbar with the way back and the theme switch, the lede under a title, "built on" as
   outline badges, the plate (a --lui-gray-2 stage over a muted code block, so a card on the stage
   is the only frame), and the index as a
   gallery of the components themselves, per group. */
/* The frame: from 60rem the component list is a sticky column beside a readable page (the
   index spreads wider); narrower, it follows a closed <details> that hides it until opened,
   so it is a plain <nav> of links either way and needs no script. */
/* Wider than <main> (52rem): the frame spreads to 84rem, centred, by negative margins (not
   :has(), which Blitz lacks: FINDINGS, M34); the header follows where :has() works. */
.lui-site {
  display: grid; gap: calc(var(--lui-space) * 3); width: min(84rem, calc(100vw - 2rem));
  margin-inline: calc((100% - min(84rem, calc(100vw - 2rem))) / 2);
}
body:has(.lui-site) > .lui-site-header { max-width: 84rem; }
.lui-site-main { min-width: 0; }
.lui-site-menu > summary { display: flex; width: 100%; justify-content: space-between; list-style: none; }
.lui-site-menu > summary::-webkit-details-marker { display: none; }
.lui-site-menu > summary svg { transition: rotate var(--lui-duration-fast) ease; }
.lui-site-menu[open] > summary svg { rotate: 180deg; }
.lui-site-menu[open] > summary { margin-bottom: var(--lui-space); }
@media (max-width: 59.99rem) {
  .lui-site-menu:not([open]) + .lui-sidebar { display: none; }
}
@media (min-width: 60rem) {
  .lui-site { grid-template-columns: 14rem minmax(0, 1fr); gap: calc(var(--lui-space) * 6); align-items: start; }
  .lui-site-menu { display: none; }
  .lui-site-nav {
    position: sticky; top: calc(var(--lui-space) * 2); max-height: calc(100vh - var(--lui-space) * 4); overflow-y: auto;
    scrollbar-width: thin; scrollbar-color: var(--lui-line) transparent;
  }
  .lui-site-main:not(.lui-site-wide) { max-width: 56rem; }
}
/* The title with the toolbar (languages, theme) on its row; the toolbar goes under the title
   when they do not fit side by side. */
.lui-title-bar { display: flex; flex-wrap: wrap; justify-content: space-between; align-items: center; gap: var(--lui-space) calc(var(--lui-space) * 2); margin: 0 0 0.75rem; }
.lui-title-bar h1 { margin: 0; }
.lui-toolbar { display: flex; flex-wrap: wrap; align-items: center; gap: var(--lui-space); }
.lui-popover-row { display: flex; justify-content: space-between; gap: var(--lui-space); margin-bottom: calc(var(--lui-space) * 2); }
.lui-back { display: inline-block; margin-bottom: calc(var(--lui-space) * 2); color: var(--lui-muted); text-decoration: none; font-size: 0.875rem; font-weight: 500; }
.lui-back::before { content: "\2190"; margin-right: 0.35em; }
.lui-back:hover { color: var(--lui-fg); }
.lui-lede { font-size: 1.125rem; line-height: 1.75rem; color: var(--lui-muted); margin-bottom: 1rem; }
.lui-built { color: var(--lui-muted); font-size: 0.875rem; margin: 0 0 1.5rem; }
.lui-built code {
  display: inline-block; max-width: 100%; box-sizing: border-box; margin: 0 0.25rem 0.25rem 0; padding: 0.125rem 0.5rem; overflow-wrap: anywhere;
  font-size: 0.75rem; line-height: 1rem; font-weight: 500; color: var(--lui-fg);
  background: transparent; border: 1px solid var(--lui-line); border-radius: var(--lui-radius-sm);
}
/* A component page's plate: the live component on a stage, the code that drew it joined
   underneath. One per page; no transform, overflow or contain on the stage, so dialogs,
   drawers and toasts still escape it. */
.lui-plate { margin: 0 0 2rem; }
.lui-stage {
  padding: calc(var(--lui-space) * 5) calc(var(--lui-space) * 4); background: var(--lui-gray-2);
  border-radius: var(--lui-radius-lg) var(--lui-radius-lg) 0 0;
}
.lui-stage > :last-child { margin-bottom: 0; }
.lui-stage h2:first-child { margin-top: 0; }
@media (max-width: 40rem) { .lui-stage { padding: calc(var(--lui-space) * 3) calc(var(--lui-space) * 2); } }
.lui-snippet {
  margin: 0; max-width: none; overflow: hidden;
  border: 1px solid var(--lui-line); border-radius: 0 0 var(--lui-radius-lg) var(--lui-radius-lg);
  background: var(--lui-surface);
}
.lui-snippet figcaption {
  display: flex; flex-wrap: wrap; justify-content: space-between; gap: 0.25rem 1rem;
  padding: 0.5rem calc(var(--lui-space) * 2); border-bottom: 1px solid var(--lui-line);
  color: var(--lui-muted); font-size: 0.75rem;
}
.lui-snippet figcaption span:first-child { color: var(--lui-fg); font-weight: 500; font-family: var(--lui-font-mono); }
.lui-snippet pre { margin: 0; padding: calc(var(--lui-space) * 2); overflow-x: auto; scrollbar-color: var(--lui-line) transparent; line-height: 1.7; tab-size: 4; }
.lui-snippet pre code { background: none; border: 0; padding: 0; font-size: 0.8125rem; color: var(--lui-fg); }
/* Highlighted Rust in the status colours, as a GitHub-like theme: keywords in danger, strings
   in ok, numbers and types in warn, comments muted, macros bold. */
.lui-hl-k { color: var(--lui-danger); }
.lui-hl-s { color: var(--lui-ok); }
.lui-hl-n, .lui-hl-t { color: color-mix(in srgb, var(--lui-warn) 70%, var(--lui-fg)); }
.lui-hl-c { color: var(--lui-muted); font-style: italic; }
.lui-hl-m { color: var(--lui-fg); font-weight: 600; }
.lui-hl-f { color: var(--lui-fg); }
/* The demo's props tables under the snippet: one <details> per builder. */
.lui-theme-builder { display: grid; gap: calc(var(--lui-space) * 2); max-width: none; }
.lui-theme-builder fieldset > div { display: grid; grid-template-columns: repeat(auto-fill, minmax(min(18rem, 100%), 1fr)); gap: var(--lui-space); }
.lui-theme-builder fieldset { margin: 0; padding: calc(var(--lui-space) * 2); border: 1px solid var(--lui-line); border-radius: var(--lui-radius); }
.lui-theme-previews { display: grid; grid-template-columns: repeat(auto-fit, minmax(min(18rem, 100%), 1fr)); gap: calc(var(--lui-space) * 2); margin-block: calc(var(--lui-space) * 2); }
.lui-theme-preview { padding: calc(var(--lui-space) * 2); background: var(--lui-bg); color: var(--lui-fg); border: 1px solid var(--lui-line); border-radius: var(--lui-radius); }
/* The builder's depth row: a tile per shadow step, then the primary gradient. */
.lui-theme-depth { display: flex; flex-wrap: wrap; gap: calc(var(--lui-space) * 2); margin-top: calc(var(--lui-space) * 3); }
.lui-theme-depth span { display: grid; place-items: center; width: 3.5rem; height: 3.5rem; border-radius: var(--lui-radius); background: var(--lui-card); color: var(--lui-muted); font-size: 0.75rem; }
.lui-theme-depth .lui-theme-gradient { background: var(--lui-primary); background-image: var(--lui-gradient-primary); box-shadow: var(--lui-shadow-sm), var(--lui-highlight); }
.lui-props { max-width: none; margin: 0 0 2rem; }
.lui-props details { border: 1px solid var(--lui-line); border-radius: var(--lui-radius); margin: 0 0 0.5rem; }
.lui-props summary { display: flex; flex-wrap: wrap; align-items: baseline; gap: 0.25rem 0.5rem; padding: 0.5rem 0.75rem; cursor: pointer; }
.lui-props summary span { margin-left: auto; color: var(--lui-muted); font-size: 0.8125rem; }
.lui-props-scroll { overflow-x: auto; border-top: 1px solid var(--lui-line); }
.lui-caps-scroll { overflow-x: auto; }
.lui-playground > form > p { margin: 0; padding: 0.5rem 0.75rem; border-top: 1px solid var(--lui-line); }
.lui-playground-preview { padding: 1rem 0.75rem; border-top: 1px solid var(--lui-line); }
.lui-playground > pre { margin: 0; padding: 0.5rem 0.75rem; border-top: 1px solid var(--lui-line); overflow-x: auto; font-size: 0.8125rem; }
.lui-props table { width: 100%; border-collapse: collapse; font-size: 0.8125rem; }
.lui-props th, .lui-props td { text-align: left; vertical-align: top; padding: 0.375rem 0.75rem; border-bottom: 1px solid var(--lui-line); }
.lui-props th { color: var(--lui-muted); font-weight: 500; white-space: nowrap; }
.lui-props tbody tr:last-child td { border-bottom: 0; }
.lui-props td:nth-child(-n+2), .lui-props td:nth-child(4), .lui-props td:nth-child(5) { white-space: nowrap; }
.lui-props td:nth-child(3) { min-width: 10rem; }
.lui-props td:nth-child(6) { min-width: 16rem; }
.lui-props td:nth-child(7) { white-space: nowrap; }
.lui-index { max-width: none; }
.lui-index h2 { margin: 3rem 0 0.25rem; font-size: 1.5rem; line-height: 2rem; }
.lui-index-layer { margin: 0 0 1rem; color: var(--lui-muted); font-size: 0.875rem; }
.lui-index h3 { margin: 1.5rem 0 0.75rem; font-size: 0.875rem; font-weight: 500; color: var(--lui-muted); text-transform: uppercase; letter-spacing: 0.05em; }
.lui-index > ul { list-style: none; margin: 0; padding: 0; display: grid; grid-template-columns: repeat(auto-fill, minmax(18rem, 1fr)); gap: 1rem; }
/* Each component is a card: its name (the link to its page), what it is for, and the
   component itself, live, on a small stage. Widgets, blocks and flows take the whole row. */
.lui-index > ul > li {
  display: grid; grid-template-rows: auto auto 1fr; min-width: 0; max-width: none; padding: 1rem;
  background: var(--lui-card); border: 1px solid var(--lui-line); border-radius: var(--lui-radius-lg);
  box-shadow: var(--lui-shadow-xs);
}
.lui-index > ul > li.lui-index-wide { grid-column: 1 / -1; }
.lui-index > ul > li > p { margin: 0 0 0.25rem; }
.lui-index > ul > li > p:first-child > a { font-size: 1rem; font-weight: 600; line-height: 1.5rem; color: var(--lui-fg); text-decoration: none; }
.lui-index > ul > li > p:first-child > a:hover { text-decoration: underline; }
.lui-index-stage {
  min-width: 0; margin-top: 0.75rem; padding: calc(var(--lui-space) * 3) calc(var(--lui-space) * 2);
  background: var(--lui-bg); border: 1px solid var(--lui-line); border-radius: var(--lui-radius);
}
.lui-index-stage > :last-child { margin-bottom: 0; }
/* The theme builder's radius tiles and its table of every token, a swatch per scheme. */
.lui-theme-depth .lui-theme-corner { background: var(--lui-secondary); border: 1px solid var(--lui-line); }
.lui-theme-tokens { grid-column: 1 / -1; }
.lui-theme-tokens > div { max-height: 32rem; overflow: auto; margin-top: var(--lui-space-2); }
.lui-theme-tokens td { vertical-align: middle; font-family: var(--lui-font-mono); font-size: 0.75rem; }
.lui-theme-light, .lui-theme-dark { background: var(--lui-bg); color: var(--lui-fg); }
.lui-theme-light { color-scheme: light; }
.lui-theme-dark { color-scheme: dark; }
.lui-theme-tokens i { display: inline-block; vertical-align: middle; margin-inline-end: var(--lui-space-2); background: var(--lui-swatch); width: 2rem; height: 1.25rem; border: 1px solid var(--lui-line); border-radius: var(--lui-radius-sm); }
/* The modes page: one stage per mode, side by side when they fit. */
.lui-modes { display: grid; grid-template-columns: repeat(auto-fit, minmax(min(18rem, 100%), 1fr)); gap: var(--lui-space-4); }
.lui-modes > section { padding: var(--lui-space-4); border: 1px solid var(--lui-line); border-radius: var(--lui-radius); }
.lui-modes h2 { margin-top: 0; font-size: 1rem; }
"#;
