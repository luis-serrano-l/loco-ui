//! HTML snapshots of every builder: the main call of its doc header (the one with its `lui!`
//! twin), rendered with every capability and with none, one `.snap` file per builder under
//! `tests/snapshots/`. A change to a component's markup shows up as a diff to review with
//! `cargo insta review` (or `cargo insta test --check` in CI) instead of as a surprise.

use loco_ui::select::SelectOption;
use loco_ui::table::Row;
use loco_ui::{MenuItem, prelude::*};

/// How a builder's header example is set up: the request it reads (path, query, cookie
/// header) and the call itself.
struct Example {
    builder: &'static str,
    request: (&'static str, &'static str, &'static str),
    render: fn(&Ui) -> Markup,
}

const fn example(builder: &'static str, render: fn(&Ui) -> Markup) -> Example {
    Example {
        builder,
        request: ("/", "", ""),
        render,
    }
}

const fn on(
    builder: &'static str,
    request: (&'static str, &'static str, &'static str),
    render: fn(&Ui) -> Markup,
) -> Example {
    Example {
        builder,
        request,
        render,
    }
}

/// Builders with nothing to render on their own: a row and an option go inside their table
/// and select, whose snapshots show them.
const PARTS: [&str; 2] = ["Row", "SelectOption"];

const EXAMPLES: &[Example] = &[
    on("Accordion", ("/help", "open.faq=0,2", ""), |ui| {
        ui.accordion("faq")
            .item("Install", html! { p { "cargo add" } })
            .icon("\u{1F4E6}")
            .description("One line.")
            .item("Use", html! { p { "html!" } })
            .item(
                "More",
                html! { (ui.accordion("faq-more").item("Nested", html! { p { "Own group." } })) },
            )
            .multiple()
            .controls()
            .render()
    }),
    example("Alert", |ui| {
        ui.alert("Payment failed")
            .danger()
            .description("Your card was declined.")
            .render()
    }),
    example("Avatar", |ui| ui.avatar("Ada Lovelace").render()),
    example("Avatars", |ui| {
        ui.avatars("Team")
            .max(3)
            .avatar("Ada Lovelace")
            .avatar("Grace Hopper")
            .avatar("Alan Turing")
            .avatar("Ken Thompson")
            .render()
    }),
    example("Badge", |ui| ui.badge("Failed").danger().solid().render()),
    example("Breadcrumbs", |ui| {
        ui.breadcrumbs()
            .link("Home", "/")
            .link("Projects", "/projects")
            .here("loco-ui")
            .render()
    }),
    example("Button", |ui| ui.button("Save").primary().render()),
    on("Calendar", ("/book", "month.due=2026-09", ""), |ui| {
        // Today pinned, so the snapshot does not move with the clock.
        ui.calendar("due")
            .today("2026-09-24")
            .radio()
            .required()
            .min("2026-09-10")
            .max("2026-10-31")
            .disabled_dates(&|d| d.weekday() >= 5)
            .event("2026-09-30", "Invoice due")
            .sunday_first()
            .render()
    }),
    example("Card", |ui| {
        ui.card()
            .title("Team")
            .body(html! { p { "3 members" } })
            .render()
    }),
    example("Chart", |ui| {
        ui.chart("Latency")
            .line()
            .unit(" ms")
            .description("p50 per day, last week")
            .point("Mon", 41.5)
            .point("Tue", 38.0)
            .render()
    }),
    example("Cluster", |ui| {
        ui.cluster()
            .between()
            .gap(4)
            .body(html! { h2 { "Orders" } (ui.button("New order").primary()) })
            .render()
    }),
    on("Color", ("/", "accent=%232f5bea", ""), |ui| {
        ui.color("accent", "Accent")
            .value("#2f5bea")
            .presets(&["#1f6f5f", "#b3261e"])
            .alpha(80)
            .render()
    }),
    on("Combobox", ("/langs", "q=ru&sel=Zig", ""), |ui| {
        ui.combobox("q", "/langs")
            .group("Systems", ["Rust", "Zig"])
            .options(["Ruby"])
            .multiple()
            .create("/langs/new")
            .label("Language")
            .placeholder("Type a language")
            .render()
    }),
    example("ContextMenu", |ui| {
        ui.context_menu("report.pdf")
            .link("Open", "/files/1")
            .shortcut("o")
            .separator()
            .action("Delete", "/files/1/delete")
            .danger()
            .body(html! { p { "report.pdf" } })
            .render()
    }),
    example("Counter", |ui| {
        ui.counter("/counter", 10)
            .min(0)
            .max(10)
            .step(2)
            .typed()
            .render()
    }),
    example("DatePicker", |ui| {
        ui.date_picker("due", "Due date")
            .value("2026-09-24")
            .render()
    }),
    example("DescriptionList", |ui| {
        ui.description_list()
            .stacked()
            .item("Status", ui.badge("Active").ok())
            .render()
    }),
    example("Dialog", |ui| {
        ui.dialog("Delete account")
            .id("confirm")
            .title("Delete account?")
            .size(1)
            .danger()
            .confirm("Delete", "/account/delete")
            .returns_to("/settings")
            .cancel("Keep it")
            .closedby("closerequest")
            .open(true)
            .body(html! {
                p { "This cannot be undone." }
                label { "Reason " input name="reason"; }
            })
            .render()
    }),
    on("Drawer", ("/", "dialog=menu", ""), |ui| {
        ui.drawer("Menu")
            .title("Browse")
            .sidebar()
            .nav(html! { ul { li { a href="/" { "Home" } } } })
            .render()
    }),
    example("EmptyState", |ui| {
        ui.empty_state("No results for \u{201c}zzz\u{201d}")
            .icon("\u{1f50d}")
            .body(html! { "Check the spelling or clear the filter." })
            .link("Clear the filter", "/table")
            .action("Create a file", "/files/new")
            .render()
    }),
    example("ErrorSummary", |ui| {
        let errors = [
            ("email", "Enter an email address."),
            ("password", "At least 8 characters."),
        ];
        ui.error_summary(&errors)
            .field("email", "Email")
            .title("Check your details")
            .render()
    }),
    on(
        "Flash",
        (
            "/settings",
            "",
            "lui-flash=ok%3ASaved.%0Adanger%3AAvatar%20too%20large.",
        ),
        |ui| ui.flash().dismiss().auto_hide().render(),
    ),
    example("Form", |ui| {
        ui.form("/profile")
            .group("About you")
            .description("Shown on your public page.")
            .textarea("bio", "Bio", 3)
            .maxlength(280)
            .value("Hi")
            .help("Shown on your profile.")
            .file("avatar", "Avatar", "image/png,image/jpeg")
            .date("born", "Born", "1900-01-01", "2026-12-31")
            .select("digest", "Digest", ["daily", "weekly", "never"])
            .value("weekly")
            .submit("Save profile")
            .inline()
            .render()
    }),
    example("Grid", |ui| {
        ui.grid("10rem").gap(2).body(html! { p { "x" } }).render()
    }),
    example("IconMark", |ui| {
        ui.icon(Icon::TriangleAlert).label("Warning").render()
    }),
    example("Input", |ui| {
        ui.input("email", "Email")
            .email()
            .required()
            .value("ada@x.org")
            .help("We never share it.")
            .error("Already taken.")
            .render()
    }),
    example("RadioGroup", |ui| {
        ui.radio_group("tier", "Tier")
            .cards()
            .option("hobby", "Hobby")
            .description("One project")
            .option("team", "Team")
            .description("Unlimited projects")
            .value("team")
            .render()
    }),
    example("Kanban", |ui| {
        ui.kanban("/board/move")
            .column("todo", "To do")
            .card("c1", "Write the docs")
            .column("doing", "Doing")
            .limit(2)
            .card("c2", "Calendar")
            .description("M23")
            .badge("Beta")
            .card("c3", "Upload")
            .column("done", "Done")
            .render()
    }),
    example("Sortable", |ui| {
        ui.sortable("Tasks", "/tasks/order")
            .item("a", "Write the docs")
            .description("Due Friday")
            .item("b", "Ship the release")
            .render()
    }),
    example("Marquee", |ui| {
        ui.marquee("What people say")
            .item(html! { blockquote { "Fast." } })
            .item(html! { blockquote { "No script!" } })
            .reverse()
            .duration(20)
            .render()
    }),
    example("Meter", |ui| {
        ui.meter(82, 0, 100)
            .label("Disk")
            .low(60)
            .high(80)
            .optimum(0)
            .render()
    }),
    on("Pager", ("/list", "page=2", ""), |ui| {
        ui.pager("/list", 30)
            .per_page(10)
            .rows(|i| html! { "Row " (i + 1) })
            .render()
    }),
    on("Palette", ("/search", "q=new", ""), |ui| {
        ui.palette("/search")
            .group("Go to")
            .link("Open settings", "/settings")
            .link("New invoice", "/invoices/new")
            .keywords("bill create")
            .render()
    }),
    example("Menu", |ui| {
        ui.menu("Account")
            .group("Signed in as Ada")
            .link("Profile", "/profile")
            .icon("@")
            .shortcut("g p")
            .link("Billing", "/billing")
            .disabled()
            .separator()
            .submenu("Theme", [("Light", "/?t=light"), ("Dark", "/?t=dark")])
            .separator()
            .action("Sign out", "/logout")
            .danger()
            .align_end()
            .render()
    }),
    example("Progress", |ui| {
        ui.progress(33, 100).label("Upload").render()
    }),
    on(
        "Range",
        ("/", "volume=40&price_min=80&price_max=20", ""),
        |ui| {
            html! {
                (ui.range("volume", "Volume").ticks())
                (ui.range_pair("price", "Price").step(10))
            }
        },
    ),
    on("Select", ("/shop", "food=leek&food-q=k", ""), |ui| {
        ui.select("food", "Food")
            .group(
                "Fruit",
                [
                    SelectOption::new("apple", "Apple"),
                    SelectOption::new("kiwi", "Kiwi").body(html! { b { "Kiwi" } }),
                ],
            )
            .group("Vegetables", [("leek", "Leek")])
            .search("/shop")
            .search_over(2)
            .render()
    }),
    example("Separator", |ui| ui.separator().label("or").render()),
    example("Skeleton", |ui| {
        ui.skeleton(2).label("Loading orders").heading().render()
    }),
    example("Split", |ui| {
        ui.split(html! { nav { "Settings" } }, html! { p { "Form" } })
            .render()
    }),
    example("Stack", |ui| {
        ui.stack().gap(2).body(html! { p { "Tight" } }).render()
    }),
    example("Stat", |ui| {
        ui.stat("Error rate", "0.4%")
            .delta("-0.2 pt")
            .down_is_good()
            .description("last 7 days")
            .href("/errors")
            .render()
    }),
    on(
        "Table",
        (
            "/table",
            "sort.files=name&dir.files=asc&q.files=a&cols.files=name,size",
            "",
        ),
        |ui| {
            let row = Row::new([html! { "a.txt" }, html! { "1 KB" }, html! { "—" }])
                .key("a.txt")
                .detail(html! { p { "Modified today." } })
                .menu([
                    MenuItem::link("Open", "/files/a.txt"),
                    MenuItem::action("Delete", "/files/a.txt/delete").danger(),
                ]);
            ui.table("files", "/table")
                .column("name", "Name")
                .sortable()
                .column("size", "Size")
                .sortable()
                .numeric()
                .width("6rem")
                .column("note", "Note")
                .choose_columns()
                .bulk(
                    "/files/bulk",
                    [("archive", "Archive"), ("delete", "Delete")],
                )
                .csv("/table.csv")
                .empty("No files yet.")
                .rows([row])
                .render()
        },
    ),
    on("Tabs", ("/docs", "tab.docs=1", ""), |ui| {
        ui.tabs("docs")
            .tab("Install", html! { p { "cargo add" } })
            .tab("Use", html! { p { "html!" } })
            .badge(3)
            .lazy("Changelog", || html! { p { "(long)" } })
            .vertical()
            .select_below()
            .render()
    }),
    on("ThemeToggle", ("/", "", "theme=dark"), |ui| {
        ui.theme_toggle("/theme").render()
    }),
    on(
        "Toasts",
        (
            "/toast",
            "",
            "lui-flash=ok%3AInvite%20sent.%0Adanger%3AMail%20server%20down.",
        ),
        |ui| ui.toasts().render(),
    ),
    example("Tooltip", |ui| {
        ui.tooltip(
            "Copy the link",
            html! { (ui.button("").icon_only().ghost().aria_label("Copy").body(html! { (Icon::Copy) })) },
        )
        .render()
    }),
    example("Upload", |ui| {
        ui.upload("/files", "file")
            .accept("image/*")
            .multiple()
            .help("PNG or JPEG, up to 200 KB.")
            .file("cat.png", 48_213)
            .preview("/files/cat.png")
            .file("notes.txt", 1_024)
            .href("/files/notes.txt")
            .remove("/files/remove")
            .render()
    }),
    on("Wizard", ("/wizard", "step.signup=2", ""), |ui| {
        ui.wizard("signup", "/wizard")
            .step(
                "Account",
                ui.fields()
                    .email("email", "Email")
                    .required()
                    .value("a@b.c"),
            )
            .step("Newsletter", ui.fields().text("topics", "Topics"))
            .optional()
            .review("Review")
            .finish("Create account")
            .render()
    }),
    example("AppShell", |ui| {
        ui.app_shell("Acme")
            .link("Inbox", "/")
            .icon(Icon::Mail)
            .badge(3)
            .group("Projects")
            .link("Apollo", "/p/apollo")
            .badge(12)
            .user("Ada Lovelace", "/signout")
            .body(html! { h1 { "Hello" } })
            .render()
    }),
    example("AuthPage", |ui| {
        let form = ui
            .form("/signin")
            .email("email", "Email")
            .required()
            .submit("Sign in");
        ui.auth_page("Sign in")
            .description("Welcome back.")
            .body(form.render())
            .footer(html! { a href="/signup" { "Create an account" } })
            .render()
    }),
    example("DashboardPage", |ui| {
        ui.dashboard_page("Overview")
            .description("Last 30 days.")
            .stat(ui.stat("Revenue", "$12,480").delta("+8%"))
            .stat(ui.stat("Customers", "312"))
            .body(html! { p { "Recent orders" } })
            .render()
    }),
    example("ErrorPage", |ui| {
        ui.error_page(500).home("/dashboard").render()
    }),
    example("RecordPage", |ui| {
        ui.record_page("Invoice 42")
            .field("Customer", "Ada")
            .field("Total", "€120")
            .edit("/invoices/42/edit")
            .delete("/invoices/42/delete")
            .render()
    }),
    example("PricingPage", |ui| {
        ui.pricing_page("Pricing")
            .description("Start free, upgrade when you grow.")
            .tier("Hobby", "$0")
            .period("/month")
            .feature("One project")
            .cta("Start", "/signup")
            .tier("Pro", "$12")
            .period("/month")
            .blurb("For small teams.")
            .feature("Ten projects")
            .cta("Upgrade", "/signup?plan=pro")
            .featured()
            .render()
    }),
    on("InboxPage", ("/inbox/2", "", ""), |ui| {
        ui.inbox_page("Inbox", "/inbox")
            .item("Ada Lovelace", "/inbox/1")
            .description("The notes on the engine")
            .when("09:12")
            .unread(true)
            .item("Charles Babbage", "/inbox/2")
            .description("Lunch on Friday?")
            .when("Mon")
            .body(html! { h2 { "Lunch on Friday?" } })
            .render()
    }),
    example("InvoicePage", |ui| {
        ui.invoice_page("Invoice INV-042")
            .status(ui.badge("Paid").ok())
            .from("Acme Ltd")
            .to("Ada Lovelace")
            .meta("Issued", "1 Oct 2026")
            .line_item("Design review", "2", "$400.00", "$800.00")
            .total("Total", "$800.00")
            .render()
    }),
    example("SettingsPage", |ui| {
        ui.settings_page("Settings")
            .section("Profile", "How others see you.", html! { p { "form" } })
            .section("Email", "Where we write to you.", html! { p { "form" } })
            .render()
    }),
    example("InputOtp", |ui| {
        ui.input_otp("code", "Code").length(4).render()
    }),
    on("NavMenu", ("/pricing", "", ""), |ui| {
        ui.nav_menu("Main")
            .panel("Products", [("Mail", "/mail"), ("Calendar", "/calendar")])
            .link("Pricing", "/pricing")
            .render()
    }),
    on("Sidebar", ("/inbox", "", ""), |ui| {
        ui.sidebar("Mail")
            .group("Mail")
            .link("Inbox", "/inbox")
            .icon(Icon::Mail)
            .badge("12")
            .link("Sent", "/sent")
            .group("Labels")
            .link("Work", "/labels/work")
            .render()
    }),
    on("ToggleGroup", ("/editor", "align=center", ""), |ui| {
        ui.toggle_group("style", "Text style")
            .multiple()
            .option("bold", "Bold")
            .icon(Icon::Bold)
            .option("italic", "Italic")
            .icon(Icon::Italic)
            .value("bold")
            .render()
    }),
    example("Heading", |ui| {
        ui.heading("Members")
            .size(3)
            .as_h(2)
            .weight(Weight::Medium)
            .trim()
            .render()
    }),
    example("Text", |ui| {
        ui.text("No invoices yet.").as_p().size(2).muted().render()
    }),
    example("Link", |ui| {
        ui.link("Radix", "https://www.radix-ui.com")
            .external()
            .muted()
            .underline(Underline::Hover)
            .render()
    }),
    example("Kbd", |ui| ui.kbd("Esc").size(3).render()),
    example("Code", |ui| {
        ui.code("Ok(())")
            .tone(Tone::Brand)
            .variant(Variant::Outline)
            .render()
    }),
    example("CodeBlock", |ui| {
        ui.code_block("[dependencies]\nloco-ui = \"0.2\"")
            .language("toml")
            .title("Cargo.toml")
            .copy()
            .render()
    }),
    example("Blockquote", |ui| {
        ui.blockquote()
            .caption("Alan Kay")
            .cite("https://example.com/kay")
            .body(html! { "Simple things should be simple." })
            .render()
    }),
    example("Prose", |ui| {
        ui.prose()
            .body(html! { h2 { "Install" } p { "Add the crate." } })
            .render()
    }),
    example("Container", |ui| {
        ui.container()
            .size(1)
            .body(html! { p { "Narrow" } })
            .render()
    }),
    example("Section", |ui| {
        ui.section()
            .size(4)
            .label("Pricing")
            .body(html! { h2 { "Pricing" } })
            .render()
    }),
    example("Inset", |ui| {
        ui.inset()
            .top()
            .body(html! { img src="/cover.jpg" alt=""; })
            .render()
    }),
    example("AspectRatio", |ui| {
        ui.aspect_ratio()
            .ratio(16, 9)
            .body(html! { img src="/a.jpg" alt="A lake"; })
            .render()
    }),
    example("ScrollArea", |ui| {
        ui.scroll_area("Log")
            .height("12rem")
            .body(html! { p { "…" } })
            .render()
    }),
    example("Center", |ui| {
        ui.center()
            .min_height("12rem")
            .text_center()
            .body(html! { p { "Nothing here yet" } })
            .render()
    }),
    example("Spacer", |ui| ui.spacer().min("2rem").render()),
    example("Spinner", |ui| {
        ui.spinner().size(3).label("Loading results").render()
    }),
    example("Indicator", |ui| {
        ui.indicator(120)
            .max(99)
            .body(html! { (ui.button("Mail")) })
            .render()
    }),
    on("Banner", ("/pricing", "", ""), |ui| {
        ui.banner("Version 2 is out.")
            .tone(Tone::Brand)
            .link("Read the notes", "/changelog")
            .dismiss()
            .render()
    }),
    example("Popover", |ui| {
        ui.popover("Share", html! { p { "Copied." } })
            .side(Side::Top)
            .align(Align::End)
            .arrow()
            .id("share-2")
            .render()
    }),
    example("Carousel", |ui| {
        ui.carousel("Photos")
            .item(html! { "One" })
            .item(html! { "Two" })
            .render()
    }),
    on("Pagination", ("/posts", "page.posts=3&q=rust", ""), |ui| {
        ui.pagination("/posts")
            .id("posts")
            .total(400)
            .per_page(25)
            .window(2)
            .first_last()
            .param("q", "rust")
            .render()
    }),
    example("Steps", |ui| {
        ui.steps()
            .step("Account")
            .done()
            .href("/signup?step=0")
            .step("Profile")
            .current()
            .optional()
            .description("A photo and a bio.")
            .step("Review")
            .vertical()
            .render()
    }),
    example("Timeline", |ui| {
        ui.timeline()
            .event("Deployed")
            .when("12 October")
            .datetime("2026-10-12")
            .icon(Icon::Check)
            .tone(Tone::Ok)
            .description("Version 0.2 is live.")
            .event("Rolled back")
            .when("13 October")
            .tone(Tone::Danger)
            .render()
    }),
    on("Tree", ("/files", "open.files=1", ""), |ui| {
        ui.tree("Files")
            .node("src")
            .icon(Icon::Folder)
            .link("/files/src")
            .children(|t| {
                t.node("lib.rs")
                    .icon(Icon::File)
                    .link("/files/src/lib.rs")
                    .node("bin")
                    .icon(Icon::Folder)
                    .children(|t| t.node("main.rs").icon(Icon::File))
            })
            .node("README.md")
            .icon(Icon::File)
            .render()
    }),
    example("Collapsible", |ui| {
        ui.collapsible("Advanced", html! { p { "Options." } })
            .open(true)
            .ghost()
            .name("settings")
            .render()
    }),
    example("HoverCard", |ui| {
        ui.hover_card("Release notes", html! { "What changed in 0.2." })
            .href("/changelog")
            .id("notes")
            .render()
    }),
    on("CheckboxGroup", ("/news", "topics%5B%5D=rust", ""), |ui| {
        ui.checkbox_group("topics", "Topics")
            .option("rust", "Rust")
            .description("Weekly")
            .option("css", "CSS")
            .option("old", "Old news")
            .disabled()
            .help("Pick any.")
            .render()
    }),
    on("Number", ("/cart", "qty=2", ""), |ui| {
        ui.number("qty", "Quantity")
            .min(1)
            .max(10)
            .suffix("kg")
            .steppers()
            .render()
    }),
    on("Search", ("/notes", "q=rust&view=list", ""), |ui| {
        ui.search("q", "Search notes")
            .keep("view")
            .results_id("notes")
            .placeholder("Search notes")
            .render()
    }),
    on("Rating", ("/review", "stars=4", ""), |ui| {
        ui.rating("stars", "Your rating").required().render()
    }),
    on(
        "TagInput",
        ("/note", "tags%5B%5D=rust&tags%5B%5D=web", ""),
        |ui| {
            ui.tag_input("tags", "Tags")
                .placeholder("Add a tag")
                .help("Up to five.")
                .render()
        },
    ),
];

/// One tag per line, so a snapshot diff points at the element that changed.
fn lines(html: &str) -> String {
    html.replace("><", ">\n<")
}

/// `DatePicker` is `date_picker`.
fn snake(name: &str) -> String {
    let mut out = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_ascii_uppercase() && i > 0 {
            out.push('_');
        }
        out.push(c.to_ascii_lowercase());
    }
    out
}

#[test]
fn every_builder_has_an_example() {
    let missing: Vec<_> = loco_ui::props()
        .iter()
        .map(|c| c.builder)
        .filter(|b| !PARTS.contains(b) && !EXAMPLES.iter().any(|e| e.builder == *b))
        .collect();
    assert!(
        missing.is_empty(),
        "add the header example of these to EXAMPLES: {missing:?}"
    );
}

#[test]
fn components_match_their_snapshots() {
    let mut settings = insta::Settings::clone_current();
    settings.set_prepend_module_to_snapshot(false);
    settings.set_omit_expression(true);
    settings.bind(|| {
        for example in EXAMPLES {
            let (path, query, cookie) = example.request;
            let render = |caps: Caps| {
                let mut ui = Ui::from_request(path, query, cookie);
                ui.caps = caps;
                lines(&(example.render)(&ui).into_string())
            };
            let snapshot = format!(
                "caps: all\n{}\n\ncaps: none\n{}\n",
                render(Caps::all()),
                render(Caps::NONE)
            );
            insta::assert_snapshot!(snake(example.builder), snapshot);
        }
    });
}

/// Builders whose example holds controls only because it passes other builders in (a card
/// with a button, a page block with a form): the keys are the parts' own.
const HOLDS_OTHERS: &[&str] = &["Cluster", "Indicator", "AuthPage"];

/// The names a key may have in `KEYS`, besides a single character (an `accesskey`).
const KEY_NAMES: &[&str] = &[
    "Tab",
    "Shift",
    "Enter",
    "Space",
    "Escape",
    "ArrowUp",
    "ArrowDown",
    "ArrowLeft",
    "ArrowRight",
    "Home",
    "End",
    "PageUp",
    "PageDown",
    "Alt",
    "Ctrl",
    "Backspace",
    "Delete",
];

#[test]
fn builders_with_controls_list_their_keys() {
    // A control other than a plain link: links need no keys explained.
    let controls = [
        "<button",
        "<select",
        "<textarea",
        "<summary",
        "tabindex=\"0\"",
        "<input",
    ];
    let mut wrong = Vec::new();
    for example in EXAMPLES {
        let c = loco_ui::props()
            .iter()
            .find(|c| c.builder == example.builder)
            .expect("in props()");
        let (path, query, cookie) = example.request;
        let mut ui = Ui::from_request(path, query, cookie);
        ui.caps = Caps::all();
        let html = (example.render)(&ui).into_string();
        let has = controls.iter().any(|t| {
            html.match_indices(t).any(|(i, _)| {
                *t != "<input"
                    || !html[i..html[i..].find('>').map_or(html.len(), |e| i + e)]
                        .contains("type=\"hidden\"")
            })
        });
        let exempt = HOLDS_OTHERS.contains(&c.builder);
        match (has && !exempt, c.keys.is_empty()) {
            (true, true) => wrong.push(format!(
                "{} renders a control but its KEYS is empty",
                c.builder
            )),
            (false, false) if exempt => {
                wrong.push(format!("{} has KEYS: take it off HOLDS_OTHERS", c.builder))
            }
            _ => {}
        }
        for k in c.keys {
            for key in k.keys.split(" / ").flat_map(|a| a.split('+')) {
                if key.chars().count() != 1 && !KEY_NAMES.contains(&key) {
                    wrong.push(format!(
                        "{}: `{key}` in `{}` is not a key name",
                        c.builder, k.keys
                    ));
                }
            }
            if k.does.is_empty() || !k.does.ends_with('.') {
                wrong.push(format!(
                    "{}: `{}` needs a sentence ending in a full stop",
                    c.builder, k.keys
                ));
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}
