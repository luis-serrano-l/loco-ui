//! # Props
//!
//! What every builder accepts, as data. Each builder carries a `PROPS` constant listing its
//! setters: the name, what kind of setter it is, its arguments as written in Rust, the default
//! when it is not called, the HTML attribute it sets (if it maps to one) and the first sentence
//! of its documentation. A test reads the source and fails when a setter is missing from its
//! builder's `PROPS`, or listed with arguments that no longer match.
//!
//! [`crate::props()`] lists every builder with its constructors and `PROPS`, which the spec JSON
//! and the demo's props tables are made from.
//!
//! ```rust
//! use loco_ui::props::PropKind;
//! use loco_ui::tabs::Tabs;
//! let vertical = Tabs::PROPS.iter().find(|p| p.name == "vertical").unwrap();
//! assert_eq!(vertical.kind, PropKind::Switch);
//! assert_eq!(vertical.default, "off");
//! let badge = Tabs::PROPS.iter().find(|p| p.name == "badge").unwrap();
//! assert_eq!(badge.kind, PropKind::Modifier);
//! let size = loco_ui::button::Button::PROPS.iter().find(|p| p.name == "size").unwrap();
//! assert_eq!(size.kind, PropKind::Choice { of: &["1", "2", "3"] });
//!
//! let tabs = loco_ui::props().iter().find(|c| c.builder == "Tabs").unwrap();
//! assert_eq!(tabs.calls, ["ui.tabs(name: &str)"]);
//! assert_eq!(tabs.lui(), "Tabs");
//! ```

/// What a setter does to its builder.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PropKind {
    /// Sets a value: text, markup, a list given at once, or several arguments together.
    Value,
    /// Sets a number.
    Number,
    /// No argument: switches something on (`.required()`).
    Switch,
    /// Takes a `bool`, for a route that decides from a condition (`.open(..)`).
    Condition,
    /// Adds one item to the builder's list (`.tab(..)`, `.column(..)`).
    Item,
    /// Changes the item added last (`.badge(..)`, `.sortable()`).
    Modifier,
    /// One of a fixed set of names (`.size(..)`, `.tone(..)`): `of` lists them as written in
    /// `lui!` and in the `data-*` attribute.
    Choice {
        /// The names it takes, in order.
        of: &'static [&'static str],
    },
}

impl PropKind {
    /// The kind as one lowercase word, as in the spec JSON.
    pub const fn as_str(self) -> &'static str {
        match self {
            PropKind::Value => "value",
            PropKind::Number => "number",
            PropKind::Switch => "switch",
            PropKind::Condition => "condition",
            PropKind::Item => "item",
            PropKind::Modifier => "modifier",
            PropKind::Choice { .. } => "choice",
        }
    }
}

/// One setter of a builder.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Prop {
    /// The setter's name (`vertical` for `.vertical()`).
    pub name: &'static str,
    /// What it does to the builder.
    pub kind: PropKind,
    /// Its arguments after `self`, as written in the source; empty for a switch.
    pub args: &'static str,
    /// The value when the setter is not called: `off` for switches and conditions, empty when
    /// there is nothing to say (an unset value, an item).
    pub default: &'static str,
    /// The HTML attribute it sets, empty when it sets a class, an element or several things.
    pub attr: &'static str,
    /// The first sentence of the setter's documentation.
    pub doc: &'static str,
    /// For a modifier, the items it may follow, by the name of the setter that adds them
    /// (`["link", "action"]`); empty for every other kind. A modifier called after any
    /// other item (or before the first) panics in a debug build.
    /// For a [`PropKind::Modifier`], the kinds of item it changes (`["field"]`); empty for
    /// every other kind. A modifier called after an item it does not apply to fails a
    /// `debug_assert!` naming the builder, the method and the item it found.
    pub applies_to: &'static [&'static str],
}

impl Prop {
    /// A setter `name` of `kind` taking `args`; switches and conditions default to `off`.
    pub const fn new(name: &'static str, kind: PropKind, args: &'static str) -> Self {
        let default = match kind {
            PropKind::Switch | PropKind::Condition => "off",
            _ => "",
        };
        Prop {
            name,
            kind,
            args,
            default,
            attr: "",
            doc: "",
            applies_to: &[],
        }
    }

    /// The value when the setter is not called.
    pub const fn default(mut self, default: &'static str) -> Self {
        self.default = default;
        self
    }

    /// The HTML attribute it sets.
    pub const fn attr(mut self, attr: &'static str) -> Self {
        self.attr = attr;
        self
    }

    /// One sentence on what it does.
    pub const fn doc(mut self, doc: &'static str) -> Self {
        self.doc = doc;
        self
    }

    /// The items a modifier changes, by the name of the setter that adds them.
    pub const fn applies_to(mut self, items: &'static [&'static str]) -> Self {
        self.applies_to = items;
        self
    }
}

/// Panics in a debug build when the modifier `method` of builder `B` follows an item it does
/// not apply to: `last` is the name of the setter that added the last item, `None` before the
/// first. The items it applies to are its [`Prop::applies_to`] in `props`, so the props table
/// and the check cannot disagree. A release build keeps the old behaviour: the modifier
/// changes whatever was added last, or nothing.
#[doc(hidden)]
#[track_caller]
pub fn check_last<B>(props: &[Prop], method: &str, last: Option<&str>) {
    if cfg!(debug_assertions) {
        // `loco_ui::popover::Menu<'_>` reads `Menu`.
        let path = std::any::type_name::<B>()
            .split('<')
            .next()
            .unwrap_or_default();
        let builder = path.rsplit("::").next().unwrap_or(path);
        let applies = props
            .iter()
            .find(|p| p.name == method)
            .map_or(&[][..], |p| p.applies_to);
        if !last.is_some_and(|l| applies.contains(&l)) {
            let after = last.map_or("nothing was added yet".to_string(), |l| {
                format!("the last item was {}", a(l))
            });
            let mut what: Vec<String> = applies.iter().map(|i| a(i)).collect();
            let end = what.pop().unwrap_or_default();
            let what = if what.is_empty() {
                end
            } else {
                format!("{} or {end}", what.join(", "))
            };
            panic!("{builder}::{method}() applies to {what}, but {after}");
        }
    }
}

/// `link` as "a link", `action` as "an action".
fn a(item: &str) -> String {
    let item = item.replace('_', " ");
    if item.starts_with(['a', 'e', 'i', 'o', 'u']) {
        format!("an {item}")
    } else {
        format!("a {item}")
    }
}

/// What a builder's API promises (README "Component status").
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    /// Its constructor and setters change only in a breaking release, with a note.
    Stable,
    /// New: its setters may be renamed or reshaped in any release while it is tried.
    Beta,
}

impl Status {
    /// The status as one lowercase word, as in the spec JSON and on the demo.
    pub const fn as_str(self) -> &'static str {
        match self {
            Status::Stable => "stable",
            Status::Beta => "beta",
        }
    }
}

/// A key a component answers to: the keys, what they do, and whether the browser or the
/// enhancement script does it. Each builder lists its own in a `KEYS` constant beside
/// `PROPS`; the docs pages, the spec JSON and the llms files show them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Key {
    /// The keys: `+` joins a chord (`Shift+Tab`), ` / ` separates alternatives
    /// (`Enter / Space`).
    pub keys: &'static str,
    /// What they do, one sentence.
    pub does: &'static str,
    /// True when only the enhancement script does it; without the script the key does what
    /// the browser does.
    pub script: bool,
}

impl Key {
    /// A key the browser handles, with or without the script.
    pub const fn new(keys: &'static str, does: &'static str) -> Self {
        Key {
            keys,
            does,
            script: false,
        }
    }

    /// The same key, handled only by the enhancement script.
    pub const fn script(self) -> Self {
        Key {
            script: true,
            ..self
        }
    }
}

/// A builder and how to get one: its module, its constructors as written in Rust, its setters
/// and how settled its API is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Component {
    /// The file under `loco-ui/src/` without `.rs`, as in the spec.
    pub module: &'static str,
    /// The builder type (`Tabs`).
    pub builder: &'static str,
    /// Its constructors with their required arguments (`ui.tabs(name: &str)`).
    pub calls: &'static [&'static str],
    /// Its setters.
    pub props: &'static [Prop],
    /// The keys it answers to.
    pub keys: &'static [Key],
    /// Stable, or beta while it is new.
    pub status: Status,
}

impl Component {
    /// The name `lui!` knows it by: the first constructor's method in `UpperCamelCase`
    /// (`ui.date_picker(..)` is `DatePicker`); a type's own constructor keeps the type name.
    pub fn lui(&self) -> String {
        let call = self.calls.first().copied().unwrap_or(self.builder);
        let Some(method) = call.strip_prefix("ui.") else {
            return self.builder.to_string();
        };
        let method = &method[..method.find('(').unwrap_or(method.len())];
        method
            .split('_')
            .map(|w| {
                let mut c = w.chars();
                c.next().map_or(String::new(), |f| {
                    f.to_ascii_uppercase().to_string() + c.as_str()
                })
            })
            .collect()
    }
}

/// Every builder, by module then name.
pub(crate) const COMPONENTS: &[Component] = &[
    Component {
        module: "accordion",
        builder: "Accordion",
        calls: &["ui.accordion(group: &str)"],
        props: crate::accordion::Accordion::PROPS,
        keys: crate::accordion::Accordion::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "alert",
        builder: "Alert",
        calls: &["ui.alert(title: &str)"],
        props: crate::alert::Alert::PROPS,
        keys: crate::alert::Alert::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "avatar",
        builder: "Avatar",
        calls: &["ui.avatar(name: &str)"],
        props: crate::avatar::Avatar::PROPS,
        keys: crate::avatar::Avatar::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "avatar",
        builder: "Avatars",
        calls: &["ui.avatars(label: &str)"],
        props: crate::avatar::Avatars::PROPS,
        keys: crate::avatar::Avatars::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "badge",
        builder: "Badge",
        calls: &["ui.badge(text: impl Display)"],
        props: crate::badge::Badge::PROPS,
        keys: crate::badge::Badge::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "breadcrumbs",
        builder: "Breadcrumbs",
        calls: &["ui.breadcrumbs()"],
        props: crate::breadcrumbs::Breadcrumbs::PROPS,
        keys: crate::breadcrumbs::Breadcrumbs::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "button",
        builder: "Button",
        calls: &[
            "ui.button(text: &str)",
            "ui.link_button(text: &str, href: &str)",
        ],
        props: crate::button::Button::PROPS,
        keys: crate::button::Button::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "calendar",
        builder: "Calendar",
        calls: &["ui.calendar(name: &str)"],
        props: crate::calendar::Calendar::PROPS,
        keys: crate::calendar::Calendar::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "card",
        builder: "Card",
        calls: &["ui.card()"],
        props: crate::card::Card::PROPS,
        keys: crate::card::Card::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "chart",
        builder: "Chart",
        calls: &["ui.chart(title: &str)"],
        props: crate::chart::Chart::PROPS,
        keys: crate::chart::Chart::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "cluster",
        builder: "Cluster",
        calls: &["ui.cluster()"],
        props: crate::cluster::Cluster::PROPS,
        keys: crate::cluster::Cluster::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "color",
        builder: "Color",
        calls: &["ui.color(name: &str, label: &str)"],
        props: crate::color::Color::PROPS,
        keys: crate::color::Color::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "combobox",
        builder: "Combobox",
        calls: &["ui.combobox(name: &str, action: &str)"],
        props: crate::combobox::Combobox::PROPS,
        keys: crate::combobox::Combobox::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "counter",
        builder: "Counter",
        calls: &["ui.counter(action: &str, value: impl Into<f64>)"],
        props: crate::counter::Counter::PROPS,
        keys: crate::counter::Counter::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "date_picker",
        builder: "DatePicker",
        calls: &["ui.date_picker(name: &str, label: &str)"],
        props: crate::date_picker::DatePicker::PROPS,
        keys: crate::date_picker::DatePicker::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "dialog",
        builder: "Dialog",
        calls: &["ui.dialog(trigger: &str)"],
        props: crate::dialog::Dialog::PROPS,
        keys: crate::dialog::Dialog::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "drawer",
        builder: "Drawer",
        calls: &["ui.drawer(label: &str)"],
        props: crate::drawer::Drawer::PROPS,
        keys: crate::drawer::Drawer::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "empty_state",
        builder: "EmptyState",
        calls: &["ui.empty_state(title: &str)"],
        props: crate::empty_state::EmptyState::PROPS,
        keys: crate::empty_state::EmptyState::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "error_summary",
        builder: "ErrorSummary",
        calls: &["ui.error_summary(errors: &[(&str, &str)])"],
        props: crate::error_summary::ErrorSummary::PROPS,
        keys: crate::error_summary::ErrorSummary::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "flash",
        builder: "Flash",
        calls: &["ui.flash()"],
        props: crate::flash::Flash::PROPS,
        keys: crate::flash::Flash::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "form",
        builder: "Form",
        calls: &["ui.form(action: &str)", "ui.fields()"],
        props: crate::form::Form::PROPS,
        keys: crate::form::Form::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "grid",
        builder: "Grid",
        calls: &["ui.grid(min: &str)"],
        props: crate::grid::Grid::PROPS,
        keys: crate::grid::Grid::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "icon",
        builder: "IconMark",
        calls: &["ui.icon(icon: Icon)"],
        props: crate::icon::IconMark::PROPS,
        keys: crate::icon::IconMark::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "input",
        builder: "Input",
        calls: &[
            "ui.input(name: &str, label: &str)",
            "ui.checkbox(name: &str, label: &str)",
            "ui.switch(name: &str, label: &str)",
        ],
        props: crate::input::Input::PROPS,
        keys: crate::input::Input::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "input",
        builder: "RadioGroup",
        calls: &["ui.radio_group(name: &str, legend: &str)"],
        props: crate::input::RadioGroup::PROPS,
        keys: crate::input::RadioGroup::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "kanban",
        builder: "Kanban",
        calls: &["ui.kanban(action: &str)"],
        props: crate::kanban::Kanban::PROPS,
        keys: crate::kanban::Kanban::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "sortable",
        builder: "Sortable",
        calls: &["ui.sortable(label: &str, action: &str)"],
        props: crate::sortable::Sortable::PROPS,
        keys: crate::sortable::Sortable::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "marquee",
        builder: "Marquee",
        calls: &["ui.marquee(label: &str)"],
        props: crate::marquee::Marquee::PROPS,
        keys: crate::marquee::Marquee::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "meter",
        builder: "Meter",
        calls: &["ui.meter(value: impl Into<f64>, min: impl Into<f64>, max: impl Into<f64>)"],
        props: crate::meter::Meter::PROPS,
        keys: crate::meter::Meter::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "pager",
        builder: "Pager",
        calls: &["ui.pager(href: &str, total: usize)"],
        props: crate::pager::Pager::PROPS,
        keys: crate::pager::Pager::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "palette",
        builder: "Palette",
        calls: &["ui.palette(action: &str)"],
        props: crate::palette::Palette::PROPS,
        keys: crate::palette::Palette::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "popover",
        builder: "Menu",
        calls: &["ui.menu(label: &str)"],
        props: crate::popover::Menu::PROPS,
        keys: crate::popover::Menu::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "progress",
        builder: "Progress",
        calls: &["ui.progress(value: u64, max: u64)"],
        props: crate::progress::Progress::PROPS,
        keys: crate::progress::Progress::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "range",
        builder: "Range",
        calls: &[
            "ui.range(name: &str, label: &str)",
            "ui.range_pair(name: &str, label: &str)",
        ],
        props: crate::range::Range::PROPS,
        keys: crate::range::Range::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "select",
        builder: "Select",
        calls: &["ui.select(name: &str, label: &str)"],
        props: crate::select::Select::PROPS,
        keys: crate::select::Select::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "select",
        builder: "SelectOption",
        calls: &["SelectOption::new(value: &str, text: &str)"],
        props: crate::select::SelectOption::PROPS,
        keys: crate::select::SelectOption::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "separator",
        builder: "Separator",
        calls: &["ui.separator()"],
        props: crate::separator::Separator::PROPS,
        keys: crate::separator::Separator::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "skeleton",
        builder: "Skeleton",
        calls: &["ui.skeleton(lines: usize)"],
        props: crate::skeleton::Skeleton::PROPS,
        keys: crate::skeleton::Skeleton::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "split",
        builder: "Split",
        calls: &["ui.split(side: Markup, main: Markup)"],
        props: crate::split::Split::PROPS,
        keys: crate::split::Split::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "stack",
        builder: "Stack",
        calls: &["ui.stack()"],
        props: crate::stack::Stack::PROPS,
        keys: crate::stack::Stack::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "stat",
        builder: "Stat",
        calls: &["ui.stat(label: &str, value: &str)"],
        props: crate::stat::Stat::PROPS,
        keys: crate::stat::Stat::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "table",
        builder: "Row",
        calls: &["Row::new(cells: impl IntoIterator<Item = Markup>)"],
        props: crate::table::Row::PROPS,
        keys: crate::table::Row::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "table",
        builder: "Table",
        calls: &["ui.table(id: &str, href: &str)"],
        props: crate::table::Table::PROPS,
        keys: crate::table::Table::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "tabs",
        builder: "Tabs",
        calls: &["ui.tabs(name: &str)"],
        props: crate::tabs::Tabs::PROPS,
        keys: crate::tabs::Tabs::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "theme",
        builder: "ThemeToggle",
        calls: &["ui.theme_toggle(action: &str)"],
        props: &[],
        keys: crate::theme::ThemeToggle::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "toast",
        builder: "Toasts",
        calls: &["ui.toasts()"],
        props: crate::toast::Toasts::PROPS,
        keys: crate::toast::Toasts::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "tooltip",
        builder: "Tooltip",
        calls: &["ui.tooltip(text: &str, trigger: Markup)"],
        props: crate::tooltip::Tooltip::PROPS,
        keys: crate::tooltip::Tooltip::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "upload",
        builder: "Upload",
        calls: &["ui.upload(action: &str, name: &str)"],
        props: crate::upload::Upload::PROPS,
        keys: crate::upload::Upload::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "wizard",
        builder: "Wizard",
        calls: &["ui.wizard(id: &str, action: &str)"],
        props: crate::wizard::Wizard::PROPS,
        keys: crate::wizard::Wizard::KEYS,
        status: Status::Stable,
    },
    Component {
        module: "blocks/app_shell",
        builder: "AppShell",
        calls: &["ui.app_shell(name: &str)"],
        props: crate::blocks::app_shell::AppShell::PROPS,
        keys: crate::blocks::app_shell::AppShell::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "blocks/auth_page",
        builder: "AuthPage",
        calls: &["ui.auth_page(title: &str)"],
        props: crate::blocks::auth_page::AuthPage::PROPS,
        keys: crate::blocks::auth_page::AuthPage::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "blocks/dashboard_page",
        builder: "DashboardPage",
        calls: &["ui.dashboard_page(title: &str)"],
        props: crate::blocks::dashboard_page::DashboardPage::PROPS,
        keys: crate::blocks::dashboard_page::DashboardPage::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "blocks/error_page",
        builder: "ErrorPage",
        calls: &["ui.error_page(status: u16)"],
        props: crate::blocks::error_page::ErrorPage::PROPS,
        keys: crate::blocks::error_page::ErrorPage::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "blocks/record_page",
        builder: "RecordPage",
        calls: &["ui.record_page(title: &str)"],
        props: crate::blocks::record_page::RecordPage::PROPS,
        keys: crate::blocks::record_page::RecordPage::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "blocks/settings_page",
        builder: "SettingsPage",
        calls: &["ui.settings_page(title: &str)"],
        props: crate::blocks::settings_page::SettingsPage::PROPS,
        keys: crate::blocks::settings_page::SettingsPage::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "context_menu",
        builder: "ContextMenu",
        calls: &["ui.context_menu(label: &str)"],
        props: crate::context_menu::ContextMenu::PROPS,
        keys: crate::context_menu::ContextMenu::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "description_list",
        builder: "DescriptionList",
        calls: &["ui.description_list()"],
        props: crate::description_list::DescriptionList::PROPS,
        keys: crate::description_list::DescriptionList::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "input_otp",
        builder: "InputOtp",
        calls: &["ui.input_otp(name: &str, label: &str)"],
        props: crate::input_otp::InputOtp::PROPS,
        keys: crate::input_otp::InputOtp::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "nav_menu",
        builder: "NavMenu",
        calls: &["ui.nav_menu(label: &str)"],
        props: crate::nav_menu::NavMenu::PROPS,
        keys: crate::nav_menu::NavMenu::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "sidebar",
        builder: "Sidebar",
        calls: &["ui.sidebar(label: &str)"],
        props: crate::sidebar::Sidebar::PROPS,
        keys: crate::sidebar::Sidebar::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "toggle_group",
        builder: "ToggleGroup",
        calls: &["ui.toggle_group(name: &str, label: &str)"],
        props: crate::toggle_group::ToggleGroup::PROPS,
        keys: crate::toggle_group::ToggleGroup::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "heading",
        builder: "Heading",
        calls: &["ui.heading(text: &str)"],
        props: crate::heading::Heading::PROPS,
        keys: crate::heading::Heading::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "text",
        builder: "Text",
        calls: &["ui.text(text: &str)"],
        props: crate::text::Text::PROPS,
        keys: crate::text::Text::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "link",
        builder: "Link",
        calls: &["ui.link(text: &str, href: &str)"],
        props: crate::link::Link::PROPS,
        keys: crate::link::Link::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "kbd",
        builder: "Kbd",
        calls: &["ui.kbd(keys: &str)"],
        props: crate::kbd::Kbd::PROPS,
        keys: crate::kbd::Kbd::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "code",
        builder: "Code",
        calls: &["ui.code(text: &str)"],
        props: crate::code::Code::PROPS,
        keys: crate::code::Code::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "code",
        builder: "CodeBlock",
        calls: &["ui.code_block(text: &str)"],
        props: crate::code::CodeBlock::PROPS,
        keys: crate::code::CodeBlock::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "blockquote",
        builder: "Blockquote",
        calls: &["ui.blockquote()"],
        props: crate::blockquote::Blockquote::PROPS,
        keys: crate::blockquote::Blockquote::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "prose",
        builder: "Prose",
        calls: &["ui.prose()"],
        props: crate::prose::Prose::PROPS,
        keys: crate::prose::Prose::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "container",
        builder: "Container",
        calls: &["ui.container()"],
        props: crate::container::Container::PROPS,
        keys: crate::container::Container::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "section",
        builder: "Section",
        calls: &["ui.section()"],
        props: crate::section::Section::PROPS,
        keys: crate::section::Section::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "inset",
        builder: "Inset",
        calls: &["ui.inset()"],
        props: crate::inset::Inset::PROPS,
        keys: crate::inset::Inset::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "aspect_ratio",
        builder: "AspectRatio",
        calls: &["ui.aspect_ratio()"],
        props: crate::aspect_ratio::AspectRatio::PROPS,
        keys: crate::aspect_ratio::AspectRatio::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "scroll_area",
        builder: "ScrollArea",
        calls: &["ui.scroll_area(label: &str)"],
        props: crate::scroll_area::ScrollArea::PROPS,
        keys: crate::scroll_area::ScrollArea::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "center",
        builder: "Center",
        calls: &["ui.center()"],
        props: crate::center::Center::PROPS,
        keys: crate::center::Center::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "spacer",
        builder: "Spacer",
        calls: &["ui.spacer()"],
        props: crate::spacer::Spacer::PROPS,
        keys: crate::spacer::Spacer::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "spinner",
        builder: "Spinner",
        calls: &["ui.spinner()"],
        props: crate::spinner::Spinner::PROPS,
        keys: crate::spinner::Spinner::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "indicator",
        builder: "Indicator",
        calls: &["ui.indicator(count: u64)"],
        props: crate::indicator::Indicator::PROPS,
        keys: crate::indicator::Indicator::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "banner",
        builder: "Banner",
        calls: &["ui.banner(text: &str)"],
        props: crate::banner::Banner::PROPS,
        keys: crate::banner::Banner::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "popover_card",
        builder: "Popover",
        calls: &["ui.popover(trigger: &str, body: Markup)"],
        props: crate::popover_card::Popover::PROPS,
        keys: crate::popover_card::Popover::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "carousel",
        builder: "Carousel",
        calls: &["ui.carousel(label: &str)"],
        props: crate::carousel::Carousel::PROPS,
        keys: crate::carousel::Carousel::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "pagination",
        builder: "Pagination",
        calls: &["ui.pagination(href: &str)"],
        props: crate::pagination::Pagination::PROPS,
        keys: crate::pagination::Pagination::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "steps",
        builder: "Steps",
        calls: &["ui.steps()"],
        props: crate::steps::Steps::PROPS,
        keys: crate::steps::Steps::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "timeline",
        builder: "Timeline",
        calls: &["ui.timeline()"],
        props: crate::timeline::Timeline::PROPS,
        keys: crate::timeline::Timeline::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "tree",
        builder: "Tree",
        calls: &["ui.tree(label: &str)"],
        props: crate::tree::Tree::PROPS,
        keys: crate::tree::Tree::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "collapsible",
        builder: "Collapsible",
        calls: &["ui.collapsible(trigger: &str, body: Markup)"],
        props: crate::collapsible::Collapsible::PROPS,
        keys: crate::collapsible::Collapsible::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "hover_card",
        builder: "HoverCard",
        calls: &["ui.hover_card(trigger: &str, body: Markup)"],
        props: crate::hover_card::HoverCard::PROPS,
        keys: crate::hover_card::HoverCard::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "checkbox_group",
        builder: "CheckboxGroup",
        calls: &["ui.checkbox_group(name: &str, legend: &str)"],
        props: crate::checkbox_group::CheckboxGroup::PROPS,
        keys: crate::checkbox_group::CheckboxGroup::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "number",
        builder: "Number",
        calls: &["ui.number(name: &str, label: &str)"],
        props: crate::number::Number::PROPS,
        keys: crate::number::Number::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "search",
        builder: "Search",
        calls: &["ui.search(name: &str, label: &str)"],
        props: crate::search::Search::PROPS,
        keys: crate::search::Search::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "rating",
        builder: "Rating",
        calls: &["ui.rating(name: &str, label: &str)"],
        props: crate::rating::Rating::PROPS,
        keys: crate::rating::Rating::KEYS,
        status: Status::Beta,
    },
    Component {
        module: "tag_input",
        builder: "TagInput",
        calls: &["ui.tag_input(name: &str, label: &str)"],
        props: crate::tag_input::TagInput::PROPS,
        keys: crate::tag_input::TagInput::KEYS,
        status: Status::Beta,
    },
];

/// A modifier after the wrong item panics in a debug build, naming the builder and the method;
/// one test per builder, plus the pass case it must not trip.
#[cfg(test)]
mod misfires {
    use crate::Ui;
    use maud::html;

    #[test]
    #[should_panic(
        expected = "Menu::danger() applies to a link, an action, a submenu, a checkbox or a radio, but the last item was a separator"
    )]
    fn menu() {
        let _ = Ui::default().menu("m").link("a", "/a").separator().danger();
    }

    #[test]
    #[should_panic(
        expected = "ContextMenu::icon() applies to a link or an action, but the last item was a group"
    )]
    fn context_menu() {
        let _ = Ui::default().context_menu("c").group("g").icon("x");
    }

    #[test]
    #[should_panic(
        expected = "Kanban::description() applies to a card, but the last item was a column"
    )]
    fn kanban() {
        let _ = Ui::default().kanban("/k").column("a", "A").description("x");
    }

    #[test]
    #[should_panic(
        expected = "Sortable::description() applies to an item, but nothing was added yet"
    )]
    fn sortable() {
        let _ = Ui::default().sortable("s", "/s").description("x");
    }

    #[test]
    #[should_panic(expected = "Palette::icon() applies to a link, but the last item was a group")]
    fn palette() {
        let _ = Ui::default()
            .palette("/p")
            .link("a", "/a")
            .group("g")
            .icon("x");
    }

    #[test]
    #[should_panic(expected = "Sidebar::badge() applies to a link, but the last item was a group")]
    fn sidebar() {
        let _ = Ui::default()
            .sidebar("s")
            .link("a", "/a")
            .group("g")
            .badge(3);
    }

    #[test]
    #[should_panic(expected = "AppShell::icon() applies to a link, but nothing was added yet")]
    fn app_shell() {
        let _ = Ui::default().app_shell("App").icon("x");
    }

    #[test]
    #[should_panic(
        expected = "Tabs::badge() applies to a tab or a lazy, but nothing was added yet"
    )]
    fn tabs() {
        let _ = Ui::default().tabs("t").badge(3);
    }

    #[test]
    #[should_panic(
        expected = "Accordion::description() applies to an item, but nothing was added yet"
    )]
    fn accordion() {
        let _ = Ui::default().accordion("a").description("x");
    }

    #[test]
    #[should_panic(expected = "Avatars::src() applies to an avatar, but nothing was added yet")]
    fn avatars() {
        let _ = Ui::default().avatars("people").src("/a.png");
    }

    #[test]
    #[should_panic(expected = "Upload::preview() applies to a file, but nothing was added yet")]
    fn upload() {
        let _ = Ui::default().upload("/u", "f").preview("/a.png");
    }

    #[test]
    #[should_panic(
        expected = "Wizard::optional() applies to a step, but the last item was a review"
    )]
    fn wizard() {
        let _ = Ui::default()
            .wizard("w", "/w")
            .step("A", html! {})
            .review("R")
            .optional();
    }

    #[test]
    fn the_right_item_passes() {
        let ui = Ui::default();
        let _ = ui
            .menu("m")
            .link("a", "/a")
            .danger()
            .submenu("s", [("b", "/b")])
            .icon("x");
        let _ = ui
            .kanban("/k")
            .column("a", "A")
            .limit(2)
            .card("c", "C")
            .description("x")
            .limit(3);
        let _ = ui
            .tabs("t")
            .tab("A", html! {})
            .badge(1)
            .lazy("B", || html! {})
            .badge(2);
        let _ = ui
            .sidebar("s")
            .group("g")
            .link("a", "/a")
            .icon("x")
            .badge(1);
    }

    #[test]
    #[should_panic(expected = "Table::numeric() applies to a column, but the last item was a keep")]
    fn table() {
        let ui = Ui::default();
        let _ = ui
            .table("t", "/t")
            .column("size", "Size")
            .keep("status")
            .numeric();
    }

    #[test]
    #[should_panic(
        expected = "ToggleGroup::icon() applies to an option, but nothing was added yet"
    )]
    fn toggle_group() {
        let _ = Ui::default().toggle_group("view", "View").icon("x");
    }
}
