//! The playground under a component's props table: a GET form of the props that take a
//! switch, a condition, a text or a number, which re-renders the component with the chosen
//! ones and writes the matching `lui!` line. Rust cannot call a setter by its name at run
//! time, so each builder here has a small function mapping its props to its setters; props
//! that take a list, markup or several values (and builders made mostly of those) stay in the
//! table without a control. Query keys are `pg.<Builder>.<prop>`, so a playground's state is
//! in the URL like any other; the form is a swap root, so the script re-renders it in place.

use loco_ui::prelude::*;
use loco_ui::props::{Component, Prop, PropKind};

/// A builder the playground can build: its `lui!` call as written, the props it offers, what
/// follows the attributes (a body block or `;`), and the function that builds it.
pub(crate) struct Entry {
    pub builder: &'static str,
    call: &'static str,
    props: &'static [&'static str],
    rest: &'static str,
    build: fn(&Try) -> Markup,
}

/// The query as the playground reads it, for one builder.
pub(crate) struct Try<'a> {
    pub ui: &'a Ui,
    builder: &'a str,
}

impl<'a> Try<'a> {
    fn key(&self, prop: &str) -> String {
        format!("pg.{}.{prop}", self.builder)
    }
    /// A ticked switch or condition.
    fn on(&self, prop: &str) -> bool {
        matches!(self.ui.param(&self.key(prop)), Some("on" | "true"))
    }
    /// A text, when not empty.
    fn text(&self, prop: &str) -> Option<&'a str> {
        let key = self.key(prop);
        self.ui.param(&key).filter(|v| !v.trim().is_empty())
    }
    /// A number, when it parses.
    fn num<T: std::str::FromStr>(&self, prop: &str) -> Option<T> {
        self.text(prop).and_then(|v| v.trim().parse().ok())
    }
}

/// `builder` passed through `apply` when `on`, else unchanged.
fn switch<B>(builder: B, on: bool, apply: impl FnOnce(B) -> B) -> B {
    if on { apply(builder) } else { builder }
}

/// `builder` passed through `apply` with `value` when there is one, else unchanged.
fn with<B, V>(builder: B, value: Option<V>, apply: impl FnOnce(B, V) -> B) -> B {
    match value {
        Some(value) => apply(builder, value),
        None => builder,
    }
}

pub(crate) const ENTRIES: &[Entry] = &[
    Entry {
        builder: "Button",
        call: "Button(\"Save\")",
        props: &[
            "primary", "danger", "ghost", "small", "disabled", "loading", "shimmer",
        ],
        rest: ";",
        build: |query| {
            let builder = query.ui.button("Save");
            let builder = switch(builder, query.on("shimmer"), |b| b.shimmer());
            let builder = switch(builder, query.on("primary"), |b| b.primary());
            let builder = switch(builder, query.on("danger"), |b| b.danger());
            let builder = switch(builder, query.on("ghost"), |b| b.ghost());
            let builder = switch(builder, query.on("small"), |b| b.small());
            let builder = switch(builder, query.on("disabled"), |b| b.disabled());
            builder.loading(query.on("loading")).render()
        },
    },
    Entry {
        builder: "Badge",
        call: "Badge(\"New\")",
        props: &["secondary", "danger", "outline", "ok", "warn", "shimmer"],
        rest: ";",
        build: |query| {
            let builder = query.ui.badge("New");
            let builder = switch(builder, query.on("shimmer"), |b| b.shimmer());
            let builder = switch(builder, query.on("secondary"), |b| b.secondary());
            let builder = switch(builder, query.on("danger"), |b| b.danger());
            let builder = switch(builder, query.on("outline"), |b| b.outline());
            let builder = switch(builder, query.on("ok"), |b| b.ok());
            switch(builder, query.on("warn"), |b| b.warn()).render()
        },
    },
    Entry {
        builder: "Alert",
        call: "Alert(\"Heads up\")",
        props: &["description", "danger", "warn", "ok"],
        rest: ";",
        build: |query| {
            let builder = with(
                query.ui.alert("Heads up"),
                query.text("description"),
                |b, v| b.description(v),
            );
            let builder = switch(builder, query.on("danger"), |b| b.danger());
            let builder = switch(builder, query.on("warn"), |b| b.warn());
            switch(builder, query.on("ok"), |b| b.ok()).render()
        },
    },
    Entry {
        builder: "Card",
        call: "Card",
        props: &[
            "title",
            "description",
            "beam",
            "glow",
            "gradient_border",
            "reveal",
        ],
        rest: " { p { \"The card's body.\" } }",
        build: |query| {
            let builder = with(query.ui.card(), query.text("title"), |b, v| b.title(v));
            let builder = with(builder, query.text("description"), |b, v| b.description(v));
            let builder = switch(builder, query.on("beam"), |b| b.beam());
            let builder = switch(builder, query.on("glow"), |b| b.glow());
            let builder = switch(builder, query.on("gradient_border"), |b| {
                b.gradient_border()
            });
            let builder = switch(builder, query.on("reveal"), |b| b.reveal());
            builder.body(html! { p { "The card's body." } }).render()
        },
    },
    Entry {
        builder: "Avatar",
        call: "Avatar(\"Ada Lovelace\")",
        props: &["small", "large"],
        rest: ";",
        build: |query| {
            let builder = switch(query.ui.avatar("Ada Lovelace"), query.on("small"), |b| {
                b.small()
            });
            switch(builder, query.on("large"), |b| b.large()).render()
        },
    },
    Entry {
        builder: "Progress",
        call: "Progress(40, 100)",
        props: &["label"],
        rest: ";",
        build: |query| {
            with(query.ui.progress(40, 100), query.text("label"), |b, v| {
                b.label(v)
            })
            .render()
        },
    },
    Entry {
        builder: "Meter",
        call: "Meter(83, 0, 100)",
        props: &["label", "low", "high", "optimum"],
        rest: ";",
        build: |query| {
            let builder = with(query.ui.meter(83, 0, 100), query.text("label"), |b, v| {
                b.label(v)
            });
            let builder = with(builder, query.num("low"), |b, v| b.low(v));
            let builder = with(builder, query.num("high"), |b, v| b.high(v));
            with(builder, query.num("optimum"), |b, v| b.optimum(v)).render()
        },
    },
    Entry {
        builder: "Separator",
        call: "Separator",
        props: &["label", "vertical"],
        rest: ";",
        build: |query| {
            let builder = with(query.ui.separator(), query.text("label"), |b, v| b.label(v));
            switch(builder, query.on("vertical"), |b| b.vertical()).render()
        },
    },
    Entry {
        builder: "Skeleton",
        call: "Skeleton(3)",
        props: &["heading", "label"],
        rest: ";",
        build: |query| {
            let builder = switch(query.ui.skeleton(3), query.on("heading"), |b| b.heading());
            with(builder, query.text("label"), |b, v| b.label(v)).render()
        },
    },
    Entry {
        builder: "EmptyState",
        call: "EmptyState(\"No projects yet\")",
        props: &["icon"],
        rest: ";",
        build: |query| {
            with(
                query.ui.empty_state("No projects yet"),
                query.text("icon"),
                |b, v| b.icon(v),
            )
            .render()
        },
    },
    Entry {
        builder: "Stat",
        call: "Stat(\"Revenue\", \"$48,210\")",
        props: &["delta", "description", "down_is_good", "reveal"],
        rest: ";",
        build: |query| {
            let builder = with(
                query.ui.stat("Revenue", "$48,210"),
                query.text("delta"),
                |b, v| b.delta(v),
            );
            let builder = switch(builder, query.on("reveal"), |b| b.reveal());
            let builder = with(builder, query.text("description"), |b, v| b.description(v));
            switch(builder, query.on("down_is_good"), |b| b.down_is_good()).render()
        },
    },
    Entry {
        builder: "Chart",
        call: "Chart(\"Signups\")",
        props: &["description", "unit", "bar", "line", "sparkline"],
        rest: " { point \"Mon\" 12.0; point \"Tue\" 18.0; point \"Wed\" 9.0; }",
        build: |query| {
            let builder = query
                .ui
                .chart("Signups")
                .point("Mon", 12.0)
                .point("Tue", 18.0)
                .point("Wed", 9.0);
            let builder = with(builder, query.text("description"), |b, v| b.description(v));
            let builder = with(builder, query.text("unit"), |b, v| b.unit(v));
            let builder = switch(builder, query.on("bar"), |b| b.bar());
            let builder = switch(builder, query.on("line"), |b| b.line());
            switch(builder, query.on("sparkline"), |b| b.sparkline()).render()
        },
    },
    Entry {
        builder: "Table",
        call: "Table(\"try\", \"\")",
        props: &["hide_search", "choose_columns", "loading"],
        rest: " { column \"name\" \"Name\"; column \"size\" \"Size\" numeric; rows ([(\"a.txt\", \"1 KB\"), (\"b.txt\", \"2 KB\")]); }",
        build: |query| {
            let builder = query.ui.table("try", "").column("name", "Name");
            let builder = builder.column("size", "Size").numeric();
            let builder = builder.rows([("a.txt", "1 KB"), ("b.txt", "2 KB")]);
            let builder = switch(builder, query.on("hide_search"), |b| b.hide_search());
            let builder = switch(builder, query.on("choose_columns"), |b| b.choose_columns());
            builder.loading(query.on("loading")).render()
        },
    },
    Entry {
        builder: "DescriptionList",
        call: "DescriptionList",
        props: &["stacked"],
        rest: " { item \"Plan\" \"Team\"; item \"Seats\" \"12\"; }",
        build: |query| {
            let builder = query
                .ui
                .description_list()
                .item("Plan", "Team")
                .item("Seats", "12");
            switch(builder, query.on("stacked"), |b| b.stacked()).render()
        },
    },
    Entry {
        builder: "InputOtp",
        call: "InputOtp(\"code\", \"Code\")",
        props: &["length"],
        rest: ";",
        build: |query| {
            with(
                query.ui.input_otp("code", "Code"),
                query.num("length"),
                |b, v| b.length(v),
            )
            .render()
        },
    },
    Entry {
        builder: "Input",
        call: "Input(\"name\", \"Name\")",
        props: &[
            "email",
            "password",
            "required",
            "placeholder",
            "help",
            "error",
            "maxlength",
            "hide_label",
            "gradient_border",
        ],
        rest: ";",
        build: |query| {
            let builder = query.ui.input("name", "Name");
            let builder = switch(builder, query.on("gradient_border"), |b| {
                b.gradient_border()
            });
            let builder = switch(builder, query.on("email"), |b| b.email());
            let builder = switch(builder, query.on("password"), |b| b.password());
            let builder = switch(builder, query.on("required"), |b| b.required());
            let builder = with(builder, query.text("placeholder"), |b, v| b.placeholder(v));
            let builder = with(builder, query.text("help"), |b, v| b.help(v));
            let builder = with(builder, query.text("error"), |b, v| b.error(v));
            let builder = with(builder, query.num("maxlength"), |b, v| b.maxlength(v));
            switch(builder, query.on("hide_label"), |b| b.hide_label()).render()
        },
    },
    Entry {
        builder: "Marquee",
        call: "Marquee(\"Customers\")",
        props: &["reverse", "duration"],
        rest: " { text \"Acme\"; text \"Globex\"; text \"Initech\"; text \"Umbrella\"; }",
        build: |query| {
            let builder = query.ui.marquee("Customers").text("Acme").text("Globex");
            let builder = builder.text("Initech").text("Umbrella");
            let builder = switch(builder, query.on("reverse"), |b| b.reverse());
            with(builder, query.num("duration"), |b, v| b.duration(v)).render()
        },
    },
    Entry {
        builder: "Range",
        call: "Range(\"volume\", \"Volume\")",
        props: &["value", "min", "max", "step"],
        rest: ";",
        build: |query| {
            let builder = with(
                query.ui.range("volume", "Volume"),
                query.num("value"),
                |b, v| b.value(v),
            );
            let builder = with(builder, query.num("min"), |b, v| b.min(v));
            let builder = with(builder, query.num("max"), |b, v| b.max(v));
            with(builder, query.num("step"), |b, v| b.step(v)).render()
        },
    },
    Entry {
        builder: "Form",
        call: "Form(\"/form\")",
        props: &["submit", "inline", "get"],
        rest: " { text \"nick\" \"Nickname\"; switch \"news\" \"Newsletter\"; }",
        build: |query| {
            let builder = with(query.ui.form("/form"), query.text("submit"), |b, v| {
                b.submit(v)
            });
            let builder = switch(builder, query.on("inline"), |b| b.inline());
            let builder = switch(builder, query.on("get"), |b| b.get());
            let builder = builder.id("playground").text("nick", "Nickname");
            builder.switch("news", "Newsletter").render()
        },
    },
    Entry {
        builder: "ErrorPage",
        call: "ErrorPage(404)",
        props: &["title", "description", "home"],
        rest: ";",
        build: |query| {
            let builder = with(query.ui.error_page(404), query.text("title"), |b, v| {
                b.title(v)
            });
            let builder = with(builder, query.text("description"), |b, v| b.description(v));
            with(builder, query.text("home"), |b, v| b.home(v)).render()
        },
    },
];

impl Entry {
    /// The props this entry has a control for.
    #[cfg(test)]
    pub(crate) fn offered(&self) -> &'static [&'static str] {
        self.props
    }
}

/// The playground entry for `builder`, if it has one.
pub(crate) fn entry(builder: &str) -> Option<&'static Entry> {
    ENTRIES.iter().find(|e| e.builder == builder)
}

/// Whether the query tries any of `builder`'s props.
pub(crate) fn tried(ui: &Ui, builder: &str) -> bool {
    entry(builder).is_some_and(|e| {
        e.props
            .iter()
            .any(|p| ui.param(&format!("pg.{builder}.{p}")).is_some())
    })
}

/// The `lui!` line for what the query chose.
fn snippet(query: &Try, entry: &Entry, props: &[Prop]) -> String {
    let mut line = entry.call.to_string();
    for prop in props.iter().filter(|p| entry.props.contains(&p.name)) {
        match prop.kind {
            PropKind::Switch if query.on(prop.name) => line += &format!(" {}", prop.name),
            PropKind::Condition if query.on(prop.name) => line += &format!(" {}=(true)", prop.name),
            PropKind::Number => {
                if let Some(value) = query.text(prop.name) {
                    line += &format!(" {}={}", prop.name, value.trim());
                }
            }
            PropKind::Value => {
                if let Some(value) = query.text(prop.name) {
                    line += &format!(" {}={:?}", prop.name, value);
                }
            }
            _ => {}
        }
    }
    line + entry.rest
}

/// The control for one prop in the table's "Try" column; empty for props it does not offer.
fn control(ui: &Ui, query: &Try, entry: &Entry, prop: &Prop) -> Markup {
    if !entry.props.contains(&prop.name) {
        return html! {};
    }
    let key = query.key(prop.name);
    match prop.kind {
        PropKind::Switch | PropKind::Condition => {
            html! { (ui.checkbox(&key, prop.name).checked(query.on(prop.name))) }
        }
        _ => {
            html! { (ui.input(&key, prop.name).hide_label().value(query.text(prop.name).unwrap_or(""))) }
        }
    }
}

/// The props table as a form, the preview and the `lui!` line, in one swap root.
pub(crate) fn playground(
    ui: &Ui,
    action: &str,
    component: &Component,
    table: impl Fn(&dyn Fn(&Prop) -> Markup) -> Markup,
) -> Markup {
    let Some(entry) = entry(component.builder) else {
        return table(&|_| html! {});
    };
    let query = Try {
        ui,
        builder: component.builder,
    };
    let id = format!("pg-{}", component.builder.to_lowercase());
    html! {
        div id=(id) data-lui="swap" class="lui-playground" {
            form method="get" action=(action) {
                (table(&|prop| control(ui, &query, entry, prop)))
                p { (ui.button("Try").primary().small()) " " a href=(action) { "Reset" } }
            }
            div class="lui-playground-preview" { ((entry.build)(&query)) }
            pre tabindex="0" aria-label={ (component.builder) " in lui!" } { code { (snippet(&query, entry, component.props)) } }
        }
    }
}
