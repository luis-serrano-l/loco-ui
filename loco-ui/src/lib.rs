//! # loco-ui
//!
//! Interactive HTML components for Rust servers that work with JavaScript turned off.
//!
//! A handler takes one [`Ui`] (what the browser supports, its theme, the page's UI state and
//! query) and describes the page with it. Every component starts from `ui`, reads what it
//! needs from the request itself, and renders where `html!` splices it. Interactivity comes
//! from the HTML and CSS platform (dialog, popover, invokers, `<details name>`, datalist, view
//! transitions) and from ordinary form round trips. No page produced by this crate needs a
//! `<script>` tag; the one optional script in [`enhance`] only makes the same markup update
//! in place.
//!
//! ```rust
//! use loco_ui::prelude::*;
//!
//! // In Axum, `ui: Ui` is an extractor; anywhere else, build it from the request.
//! let ui = Ui::from_request("/", "", "");
//! let page = ui.page("Hello", html! {
//!     (ui.dialog("Say hi").body(html! { p { "Hello from a <dialog>." } }))
//!     (ui.tabs("intro")
//!         .tab("One", html! { p { "First." } })
//!         .tab("Two", html! { p { "Second." } }))
//! });
//! // The only script is the optional enhancement tag; the page works without it.
//! assert_eq!(page.into_string().matches("<script").count(), 1);
//! ```
//!
//! Each component emits only the markup that browser needs: the modern variant or the
//! fallback, never both. See [`caps`] (the `loco-ui-caps` crate) for how the server learns
//! it. One component lives in one file; each file starts with a doc header that lists the
//! platform features it relies on, the browser baseline, and the fallback.
//!
//! ## Without Maud templates
//!
//! A component renders to [`maud::Markup`], which is `PreEscaped<String>`: `.render()` then
//! `.into_string()` hands the HTML to anything else (another template engine, a plain
//! `String` body in any server, a file). [`stylesheet`] is a `String` too.
//!
//! ```rust
//! use loco_ui::{prelude::*, stylesheet};
//!
//! let ui = Ui::from_request("/", "", "lui-flash=Saved.");
//! let body = ui.flash().render().into_string() + &ui.theme_toggle("/theme").render().into_string();
//! let page = format!("<!DOCTYPE html><style>{}</style><main>{body}</main>", stylesheet());
//! assert!(page.contains("class=\"lui-theme\"") && page.contains("Saved."));
//! assert!(!page.contains("<script"));
//! ```

// docs.rs builds with nightly and `--cfg docsrs`: feature-gated items get a "requires feature" badge.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod accordion;
pub mod alert;
pub mod aspect_ratio;
pub mod avatar;
pub mod badge;
pub mod banner;
pub mod blockquote;
pub mod blocks;
pub mod breadcrumbs;
pub mod button;
pub mod calendar;
pub mod card;
pub mod carousel;
pub mod center;
pub mod chart;
pub mod checkbox_group;
pub mod cluster;
pub mod code;
pub mod collapsible;
pub mod color;
pub mod combobox;
pub mod container;
pub mod context_menu;
pub mod cookie;
pub mod counter;
pub mod date_picker;
pub mod description_list;
pub mod dialog;
pub mod drawer;
pub mod empty_state;
pub mod enhance;
pub mod error_summary;
pub mod flash;
pub mod form;
pub mod grid;
pub mod heading;
pub mod hover_card;
pub mod i18n;
pub mod icon;
pub mod indicator;
pub mod input;
pub mod input_otp;
pub mod inset;
pub mod kanban;
pub mod kbd;
pub mod layout;
pub mod link;
#[cfg(feature = "loco")]
pub mod loco;
pub mod marquee;
pub mod meter;
pub mod nav_menu;
pub mod number;
mod oklch;
pub mod paged_table;
pub mod pager;
pub mod pagination;
pub mod palette;
pub mod popover;
pub mod popover_card;
pub mod posted;
pub mod progress;
pub mod props;
pub mod prose;
pub mod range;
pub mod rating;
#[cfg(feature = "axum")]
pub mod saved;
pub mod scroll_area;
pub mod search;
pub mod section;
pub mod select;
pub mod separator;
pub mod sidebar;
pub mod skeleton;
pub mod sortable;
pub mod spacer;
pub mod spec;
pub mod spinner;
pub mod split;
pub mod stack;
pub mod stat;
pub mod state;
pub mod steps;
#[cfg(feature = "http")]
pub mod stream;
pub mod table;
pub mod tabs;
pub mod tag_input;
pub mod text;
pub mod theme;
pub mod timeline;
pub mod toast;
pub mod toggle_group;
pub mod tooltip;
pub mod tree;
pub mod ui;
pub mod upload;
pub mod variant;
pub mod wizard;

/// Server-side feature detection: the [`loco_ui_caps`] crate, re-exported so `loco_ui::caps`
/// keeps working.
pub use loco_ui_caps as caps;

pub use loco_ui_caps::{Cap, Caps};
/// Every builder with its constructors and setters: see [`props`](mod@props).
pub fn props() -> &'static [props::Component] {
    props::COMPONENTS
}

pub use icon::Icon;
/// Maud's `html!` with components written like elements: see [`loco_ui_macros`].
pub use loco_ui_macros::lui;
pub use popover::MenuItem;
pub use posted::Posted;
#[cfg(feature = "axum")]
pub use saved::Saved;
pub use state::UiState;
#[cfg(feature = "http")]
pub use stream::Streamed;
pub use table::{Row, TableQuery};
pub use theme::Theme;
pub use ui::{Page, Redirect, Ui};

/// `docs/components.md`, whose code runs as a doctest so the guide keeps compiling.
#[cfg(doctest)]
#[doc = include_str!("../../docs/components.md")]
pub struct ComponentsGuide;

/// `docs/api.md`, whose examples run as doctests so the rules keep compiling.
#[cfg(doctest)]
#[doc = include_str!("../../docs/api.md")]
pub struct ApiGuide;

/// `docs/start/any-server.md`, the getting-started page without a framework, run as doctests.
#[cfg(doctest)]
#[doc = include_str!("../../docs/start/any-server.md")]
pub struct StartAnyServerGuide;

/// `docs/start/axum.md`, the Axum getting-started page, run as doctests.
#[cfg(all(doctest, feature = "axum"))]
#[doc = include_str!("../../docs/start/axum.md")]
pub struct StartAxumGuide;

/// Everything a handler needs, in one import: `use loco_ui::prelude::*;`.
pub mod prelude {
    #[cfg(feature = "axum")]
    pub use crate::Saved;
    pub use crate::lui;
    pub use crate::popover_card::Side;
    pub use crate::toast::Position;
    pub use crate::variant::{Align, Justify, Radius, Size, Tone, Underline, Variant, Weight};
    pub use crate::{
        Cap, Caps, Icon, MenuItem, Page, Posted, Redirect, Row, TableQuery, Theme, Ui,
    };
    pub use maud::{Markup, Render, html};
}

/// A key made safe for an `id`: anything but letters, digits, `-` and `_` becomes `-`, and
/// letters are lowercased, so `"Account"` gives `account`. Letters and digits are Unicode
/// ones (`char::is_alphanumeric`), so a label in any script keeps its words: two Japanese
/// labels give two ids, not two runs of dashes. Combining marks are not stripped. Components
/// derive the ids a caller does not care about from a label this way; so can yours.
///
/// ```rust
/// assert_eq!(loco_ui::slug("Billing & plans"), "billing---plans");
/// assert_eq!(loco_ui::slug("Configuración"), "configuración");
/// assert_ne!(loco_ui::slug("設定"), loco_ui::slug("請求"));
/// ```
pub fn slug(key: &str) -> String {
    // Lowercasing can add a combining mark (`İ` is `i` and U+0307), which becomes `-` too,
    // so a slug of a slug is the same slug.
    key.chars()
        .flat_map(char::to_lowercase)
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect()
}

/// `path` with `pairs` as its query, each key and value percent-encoded as the components'
/// own links are: a link to another page and its parameters. [`Ui::link_with`] changes one
/// parameter of this page instead.
///
/// ```rust
/// assert_eq!(loco_ui::href("/notes", [("sel", "c#")]), "/notes?sel=c%23");
/// assert_eq!(loco_ui::href("/search", [("q", "r&d"), ("page", "2")]), "/search?q=r%26d&page=2");
/// assert_eq!(loco_ui::href("/notes", []), "/notes");
/// ```
pub fn href<'a>(path: &str, pairs: impl IntoIterator<Item = (&'a str, &'a str)>) -> String {
    use state::encode;
    let mut out = path.to_string();
    for (i, (k, v)) in pairs.into_iter().enumerate() {
        out.push(if i == 0 { '?' } else { '&' });
        out.push_str(&encode(k));
        out.push('=');
        out.push_str(&encode(v));
    }
    out
}

/// A control under its label in a `div.lui-field`, as the form component lays out its
/// fields; the control alone when there is no label. `id` is the control's id.
pub(crate) fn labelled(label: Option<&str>, id: &str, control: maud::Markup) -> maud::Markup {
    match label {
        Some(text) => {
            maud::html! { div class="lui-field" { label for=(id) { (text) } (control) } }
        }
        None => control,
    }
}

/// The `lui-gap-<n>` class for a layout primitive's `.gap(n)`: the step of the
/// `--lui-space-*` scale (0, 1, 2, 3, 4, 6, 8) nearest `n`, rounding down between two.
pub(crate) fn gap_class(n: u8) -> &'static str {
    match n {
        0 => "lui-gap-0",
        1 => "lui-gap-1",
        2 => "lui-gap-2",
        3 => "lui-gap-3",
        4 | 5 => "lui-gap-4",
        6 | 7 => "lui-gap-6",
        _ => "lui-gap-8",
    }
}

/// The cascade layers of [`stylesheet`], in order. An app's own CSS is unlayered, so it wins
/// over every one of them without `!important`.
pub const LAYERS: &str = "@layer lui.reset, lui.tokens, lui.components, lui.blocks, lui.modes;";

/// The stylesheet from its parts: the layer order, `@view-transition`, then each layer.
fn layered(components: &[&str], blocks: &[&str]) -> String {
    let mut css = String::with_capacity(160 * 1024);
    for part in [
        LAYERS,
        layout::VIEW_TRANSITION,
        "\n@layer lui.reset{",
        layout::RESET,
    ] {
        css.push_str(part);
    }
    css.push_str("}\n@layer lui.tokens{");
    css.push_str(&layout::Tokens::DEFAULT.css());
    css.push_str("}\n@layer lui.components{");
    for part in components {
        css.push_str(part);
    }
    css.push_str(&caps::beacon_css());
    css.push_str("}\n@layer lui.blocks{");
    for part in blocks {
        css.push_str(part);
    }
    css.push_str("}\n@layer lui.modes{");
    css.push_str(layout::MODES);
    css.push('}');
    minify_css(&css)
}

/// All component stylesheets, in their cascade layers, built once per process. `ui.page`
/// inlines this once per page, or links it as `/lui.css` with `Page::linked_stylesheet`.
pub fn stylesheet() -> &'static str {
    static CSS: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    CSS.get_or_init(|| layered(COMPONENT_CSS, &blocks::CSS))
}

/// The stylesheet with only `components`' CSS, plus the reset, the tokens, the layout and
/// the primitives every component builds on ([`Component::BASE`]), in the same layers.
/// Built once per distinct list and then kept; `Page::subset` inlines it instead of the
/// whole sheet.
///
/// ```rust
/// use loco_ui::{Component, stylesheet, stylesheet_for};
/// let css = stylesheet_for(&[Component::Dialog]);
/// assert!(css.contains(".lui-dialog") && !css.contains(".lui-kanban"));
/// assert!(css.len() < stylesheet().len() / 2);
/// ```
pub fn stylesheet_for(components: &[Component]) -> &'static str {
    subset_sheet(components)
}

/// [`stylesheet_for`] behind one more reference, so a `Page` holds it in 8 bytes.
pub(crate) fn subset_sheet(components: &[Component]) -> &'static &'static str {
    use std::collections::HashMap;
    use std::sync::{Mutex, OnceLock};
    static BUILT: OnceLock<Mutex<HashMap<u128, &'static &'static str>>> = OnceLock::new();
    let bit = |c: &Component| 1u128 << (*c as u32);
    let key = components
        .iter()
        .chain(Component::BASE)
        .fold(0, |bits, c| bits | bit(c));
    let mut built = BUILT
        .get_or_init(Mutex::default)
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    built.entry(key).or_insert_with(|| {
        let parts: Vec<&str> = (Component::ALL.iter())
            .filter(|c| key & bit(c) != 0)
            .map(|c| c.css())
            .collect();
        let blocks: &[&str] = if key & bit(&Component::Blocks) != 0 {
            &blocks::CSS
        } else {
            &[]
        };
        let css: &'static str = Box::leak(layered(&parts, blocks).into_boxed_str());
        Box::leak(Box::new(css))
    })
}

/// One component's stylesheet, for [`stylesheet_for`]: a variant per entry of
/// [`COMPONENT_CSS`], in its order, and `Blocks` for every block. A test fails when the two
/// lists drift apart, so a new component adds its variant at the end.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Component {
    Layout,
    Button,
    Input,
    Badge,
    Card,
    Icon,
    Avatar,
    Stack,
    Cluster,
    Grid,
    Split,
    Calendar,
    DatePicker,
    Upload,
    Kanban,
    Sortable,
    Tooltip,
    Alert,
    Progress,
    Meter,
    Separator,
    Dialog,
    Popover,
    Tabs,
    Accordion,
    Combobox,
    Pager,
    Form,
    ErrorSummary,
    Counter,
    Theme,
    Flash,
    Select,
    Range,
    Color,
    Stream,
    Table,
    PagedTable,
    Wizard,
    Toast,
    Breadcrumbs,
    Skeleton,
    EmptyState,
    Stat,
    Chart,
    Sidebar,
    NavMenu,
    DescriptionList,
    ToggleGroup,
    ContextMenu,
    InputOtp,
    Drawer,
    Palette,
    Marquee,
    /// The shared `data-radius` rules of [`variant`].
    Variant,
    Heading,
    Text,
    Link,
    Kbd,
    Code,
    Blockquote,
    Prose,
    Container,
    Section,
    Inset,
    AspectRatio,
    ScrollArea,
    Center,
    Spacer,
    Spinner,
    Indicator,
    Banner,
    PopoverCard,
    Carousel,
    Pagination,
    Steps,
    Timeline,
    Tree,
    Collapsible,
    HoverCard,
    CheckboxGroup,
    Number,
    Search,
    Rating,
    TagInput,
    Blocks,
}

impl Component {
    /// Every variant, in [`COMPONENT_CSS`] order, `Blocks` included.
    pub const ALL: &'static [Component] = &[
        Component::Layout,
        Component::Button,
        Component::Input,
        Component::Badge,
        Component::Card,
        Component::Icon,
        Component::Avatar,
        Component::Stack,
        Component::Cluster,
        Component::Grid,
        Component::Split,
        Component::Calendar,
        Component::DatePicker,
        Component::Upload,
        Component::Kanban,
        Component::Sortable,
        Component::Tooltip,
        Component::Alert,
        Component::Progress,
        Component::Meter,
        Component::Separator,
        Component::Dialog,
        Component::Popover,
        Component::Tabs,
        Component::Accordion,
        Component::Combobox,
        Component::Pager,
        Component::Form,
        Component::ErrorSummary,
        Component::Counter,
        Component::Theme,
        Component::Flash,
        Component::Select,
        Component::Range,
        Component::Color,
        Component::Stream,
        Component::Table,
        Component::PagedTable,
        Component::Wizard,
        Component::Toast,
        Component::Breadcrumbs,
        Component::Skeleton,
        Component::EmptyState,
        Component::Stat,
        Component::Chart,
        Component::Sidebar,
        Component::NavMenu,
        Component::DescriptionList,
        Component::ToggleGroup,
        Component::ContextMenu,
        Component::InputOtp,
        Component::Drawer,
        Component::Palette,
        Component::Marquee,
        Component::Variant,
        Component::Heading,
        Component::Text,
        Component::Link,
        Component::Kbd,
        Component::Code,
        Component::Blockquote,
        Component::Prose,
        Component::Container,
        Component::Section,
        Component::Inset,
        Component::AspectRatio,
        Component::ScrollArea,
        Component::Center,
        Component::Spacer,
        Component::Spinner,
        Component::Indicator,
        Component::Banner,
        Component::PopoverCard,
        Component::Carousel,
        Component::Pagination,
        Component::Steps,
        Component::Timeline,
        Component::Tree,
        Component::Collapsible,
        Component::HoverCard,
        Component::CheckboxGroup,
        Component::Number,
        Component::Search,
        Component::Rating,
        Component::TagInput,
        Component::Blocks,
    ];

    /// What [`stylesheet_for`] always includes: the layout and the primitives.
    pub const BASE: &'static [Component] = &[
        Component::Layout,
        Component::Button,
        Component::Input,
        Component::Badge,
        Component::Card,
        Component::Icon,
        Component::Stack,
        Component::Cluster,
        Component::Grid,
        Component::Split,
        // The busy button's ring.
        Component::Spinner,
    ];

    /// This component's `CSS`: empty for `Blocks` (its parts are `blocks::CSS`) and for
    /// `Stream` without the `http` feature.
    pub fn css(self) -> &'static str {
        match self {
            Component::Layout => layout::CSS,
            Component::Button => button::CSS,
            Component::Input => input::CSS,
            Component::Badge => badge::CSS,
            Component::Card => card::CSS,
            Component::Icon => icon::CSS,
            Component::Avatar => avatar::CSS,
            Component::Stack => stack::CSS,
            Component::Cluster => cluster::CSS,
            Component::Grid => grid::CSS,
            Component::Split => split::CSS,
            Component::Calendar => calendar::CSS,
            Component::DatePicker => date_picker::CSS,
            Component::Upload => upload::CSS,
            Component::Kanban => kanban::CSS,
            Component::Sortable => sortable::CSS,
            Component::Tooltip => tooltip::CSS,
            Component::Alert => alert::CSS,
            Component::Progress => progress::CSS,
            Component::Meter => meter::CSS,
            Component::Separator => separator::CSS,
            Component::Dialog => dialog::CSS,
            Component::Popover => popover::CSS,
            Component::Tabs => tabs::CSS,
            Component::Accordion => accordion::CSS,
            Component::Combobox => combobox::CSS,
            Component::Pager => pager::CSS,
            Component::Form => form::CSS,
            Component::ErrorSummary => error_summary::CSS,
            Component::Counter => counter::CSS,
            Component::Theme => theme::CSS,
            Component::Flash => flash::CSS,
            Component::Select => select::CSS,
            Component::Range => range::CSS,
            Component::Color => color::CSS,
            #[cfg(feature = "http")]
            Component::Stream => stream::CSS,
            #[cfg(not(feature = "http"))]
            Component::Stream => "",
            Component::Table => table::CSS,
            Component::PagedTable => paged_table::CSS,
            Component::Wizard => wizard::CSS,
            Component::Toast => toast::CSS,
            Component::Breadcrumbs => breadcrumbs::CSS,
            Component::Skeleton => skeleton::CSS,
            Component::EmptyState => empty_state::CSS,
            Component::Stat => stat::CSS,
            Component::Chart => chart::CSS,
            Component::Sidebar => sidebar::CSS,
            Component::NavMenu => nav_menu::CSS,
            Component::DescriptionList => description_list::CSS,
            Component::ToggleGroup => toggle_group::CSS,
            Component::ContextMenu => context_menu::CSS,
            Component::InputOtp => input_otp::CSS,
            Component::Drawer => drawer::CSS,
            Component::Palette => palette::CSS,
            Component::Marquee => marquee::CSS,
            Component::Variant => variant::CSS,
            Component::Heading => heading::CSS,
            Component::Text => text::CSS,
            Component::Link => link::CSS,
            Component::Kbd => kbd::CSS,
            Component::Code => code::CSS,
            Component::Blockquote => blockquote::CSS,
            Component::Prose => prose::CSS,
            Component::Container => container::CSS,
            Component::Section => section::CSS,
            Component::Inset => inset::CSS,
            Component::AspectRatio => aspect_ratio::CSS,
            Component::ScrollArea => scroll_area::CSS,
            Component::Center => center::CSS,
            Component::Spacer => spacer::CSS,
            Component::Spinner => spinner::CSS,
            Component::Indicator => indicator::CSS,
            Component::Banner => banner::CSS,
            Component::PopoverCard => popover_card::CSS,
            Component::Carousel => carousel::CSS,
            Component::Pagination => pagination::CSS,
            Component::Steps => steps::CSS,
            Component::Timeline => timeline::CSS,
            Component::Tree => tree::CSS,
            Component::Collapsible => collapsible::CSS,
            Component::HoverCard => hover_card::CSS,
            Component::CheckboxGroup => checkbox_group::CSS,
            Component::Number => number::CSS,
            Component::Search => search::CSS,
            Component::Rating => rating::CSS,
            Component::TagInput => tag_input::CSS,
            Component::Blocks => "",
        }
    }
}

/// Strip comments and collapse whitespace in a stylesheet, leaving quoted strings alone. A
/// space survives only where CSS needs one: between two words (`0 8px`, `.a .b`, `and (`),
/// never next to `{ } ; , >`, never after `(` or `:` and never before `)`. A space before
/// `:` stays, since `.a :focus` and `.a:focus` differ. [`stylesheet`] applies it once.
pub fn minify_css(css: &str) -> String {
    let mut out = String::with_capacity(css.len());
    let mut chars = css.chars().peekable();
    let mut space = false;
    while let Some(c) = chars.next() {
        match c {
            '/' if chars.peek() == Some(&'*') => {
                chars.next();
                let mut prev = ' ';
                for c in chars.by_ref() {
                    if prev == '*' && c == '/' {
                        break;
                    }
                    prev = c;
                }
                space = true;
            }
            c if c.is_whitespace() => space = true,
            '"' | '\'' => {
                if space && !out.is_empty() && !out.ends_with(|p: char| "{};,>(:".contains(p)) {
                    out.push(' ');
                }
                space = false;
                out.push(c);
                let mut escaped = false;
                for d in chars.by_ref() {
                    out.push(d);
                    if d == c && !escaped {
                        break;
                    }
                    escaped = d == '\\' && !escaped;
                }
            }
            _ => {
                let tight = |p: char| "{};,>".contains(p);
                if space
                    && !out.is_empty()
                    && !tight(c)
                    && c != ')'
                    && !out.ends_with(|p: char| tight(p) || p == '(' || p == ':')
                {
                    out.push(' ');
                }
                space = false;
                if c == '}' {
                    while out.ends_with(';') {
                        out.pop();
                    }
                }
                out.push(c);
            }
        }
    }
    out
}

/// Every component's `CSS`, in the order the stylesheet includes them. Colours in here are
/// `var(--lui-*)` only; a test below checks that no literal slips in.
pub const COMPONENT_CSS: &[&str] = &[
    layout::CSS,
    button::CSS,
    input::CSS,
    badge::CSS,
    card::CSS,
    icon::CSS,
    avatar::CSS,
    stack::CSS,
    cluster::CSS,
    grid::CSS,
    split::CSS,
    calendar::CSS,
    date_picker::CSS,
    upload::CSS,
    kanban::CSS,
    sortable::CSS,
    tooltip::CSS,
    alert::CSS,
    progress::CSS,
    meter::CSS,
    separator::CSS,
    dialog::CSS,
    popover::CSS,
    tabs::CSS,
    accordion::CSS,
    combobox::CSS,
    pager::CSS,
    form::CSS,
    error_summary::CSS,
    counter::CSS,
    theme::CSS,
    flash::CSS,
    select::CSS,
    range::CSS,
    color::CSS,
    #[cfg(feature = "http")]
    stream::CSS,
    table::CSS,
    paged_table::CSS,
    wizard::CSS,
    toast::CSS,
    breadcrumbs::CSS,
    skeleton::CSS,
    empty_state::CSS,
    stat::CSS,
    chart::CSS,
    sidebar::CSS,
    nav_menu::CSS,
    description_list::CSS,
    toggle_group::CSS,
    context_menu::CSS,
    input_otp::CSS,
    drawer::CSS,
    palette::CSS,
    marquee::CSS,
    variant::CSS,
    heading::CSS,
    text::CSS,
    link::CSS,
    kbd::CSS,
    code::CSS,
    blockquote::CSS,
    prose::CSS,
    container::CSS,
    section::CSS,
    inset::CSS,
    aspect_ratio::CSS,
    scroll_area::CSS,
    center::CSS,
    spacer::CSS,
    spinner::CSS,
    indicator::CSS,
    banner::CSS,
    popover_card::CSS,
    carousel::CSS,
    pagination::CSS,
    steps::CSS,
    timeline::CSS,
    tree::CSS,
    collapsible::CSS,
    hover_card::CSS,
    checkbox_group::CSS,
    number::CSS,
    search::CSS,
    rating::CSS,
    tag_input::CSS,
];

#[cfg(test)]
mod tests {
    use super::*;
    use maud::{Render, html};

    #[test]
    fn tuples_build_the_same_items_as_the_constructors() {
        let ui = Ui::from(Caps::all());
        let render = |r: &dyn Render| r.render().into_string();
        assert_eq!(
            MenuItem::from(("Profile", "/p")),
            MenuItem::link("Profile", "/p")
        );
        let sizes = [("s", "Small", "🐭")];
        assert_eq!(
            render(&ui.select("size", "Size").value("s").options(sizes)),
            render(
                &ui.select("size", "Size")
                    .value("s")
                    .options([select::SelectOption::new("s", "Small").icon("🐭")])
            )
        );
        assert_eq!(
            render(&ui.menu("M").submenu("Sub", [("A", "/a")])),
            render(&ui.menu("M").submenu("Sub", [MenuItem::link("A", "/a")]))
        );
    }

    #[test]
    fn ids_come_from_labels() {
        let ui = Ui::from(Caps::all());
        let html = |r: &dyn Render| r.render().into_string();
        assert!(html(&ui.menu("My account")).contains(r#"id="lui-menu-my-account""#));
        assert!(html(&ui.drawer("Menu")).contains(r#"id="lui-drawer-menu""#));
        assert!(html(&ui.dialog("Delete account")).contains(r#"id="lui-dialog-delete-account""#));
        assert!(html(&ui.palette("/go")).contains(r#"id="lui-palette""#));
        assert!(html(&ui.dialog("Delete account").id("confirm")).contains(r#"id="confirm""#));
    }

    #[test]
    fn minified_stylesheet_keeps_every_rule() {
        let css = minify_css(
            "/* note */ .a  .b > p ,\n a:hover { margin: 0  8px ; content: \"  ← \" }\n@media (min-width: 60rem) and (x) { .c { top: calc(1px + 2px); } }",
        );
        assert_eq!(
            css,
            r#".a .b>p,a:hover{margin:0 8px;content:"  ← "}@media (min-width:60rem) and (x){.c{top:calc(1px + 2px)}}"#
        );
        let full = [
            layout::RESET,
            layout::Tokens::default().css().as_str(),
            &COMPONENT_CSS.concat(),
            &caps::beacon_css(),
            &blocks::CSS.concat(),
            layout::MODES,
        ]
        .concat();
        let min = stylesheet();
        assert!(
            min.len() * 10 < full.len() * 9,
            "at least a tenth smaller: {} of {}",
            min.len(),
            full.len()
        );
        for pair in [('{', '}'), ('(', ')')] {
            assert_eq!(
                min.matches(pair.0).count(),
                min.matches(pair.1).count(),
                "balanced {pair:?}"
            );
        }
        assert!(!min.contains("/*"), "no comments left");
        assert_eq!(minify_css(min), min, "minifying twice changes nothing");
    }

    /// Theming is tokens only: every colour in component CSS is a `var(--lui-*)`, so a palette
    /// passed to `Page::tokens` reaches everything. Literals live in `layout::Tokens` alone.
    #[test]
    fn no_colour_literal_outside_tokens() {
        for css in COMPONENT_CSS.iter().chain(&[layout::RESET, layout::MODES]) {
            for line in css.lines() {
                let hex = line.char_indices().any(|(i, c)| {
                    c == '#'
                        && line[i + 1..]
                            .chars()
                            .take_while(|c| c.is_ascii_hexdigit())
                            .count()
                            >= 3
                });
                let func = ["rgb(", "rgba(", "hsl(", "hsla(", "oklch(", "light-dark("]
                    .iter()
                    .any(|f| line.contains(f));
                // `white-space` is a property and `--lui-gray-4` a token, not colours.
                let named = line
                    .replace("white-space", "")
                    .replace("--lui-gray-", "")
                    .split(|c: char| !c.is_ascii_alphabetic())
                    .any(|w| {
                        ["white", "black", "gray", "grey", "red", "blue", "green"].contains(&w)
                    });
                assert!(
                    !hex && !func && !named,
                    "colour literal in component CSS: {line}"
                );
            }
        }
    }

    /// Builders are plain data: every public struct derives or implements `Clone` and `Debug`,
    /// so a route can keep one in a variable, build it in a loop or print it. `Streamed` owns
    /// futures and is `Debug` only; the doctest markers `ComponentsGuide` and `ApiGuide` are
    /// not types anyone holds.
    #[test]
    fn every_builder_is_clone_and_debug() {
        let mut missing = Vec::new();
        for path in library_sources() {
            let source = std::fs::read_to_string(&path).unwrap();
            let mut attrs = String::new();
            for line in source.lines().map(str::trim) {
                if line.starts_with("#[") {
                    attrs.push_str(line);
                } else if let Some(rest) = line.strip_prefix("pub struct ") {
                    let name: String = rest
                        .chars()
                        .take_while(|c| c.is_alphanumeric() || *c == '_')
                        .collect();
                    let has = |t: &str| {
                        attrs.contains(t)
                            || source.contains(&format!("impl {t} for {name}"))
                            || source.contains(&format!("impl std::fmt::{t} for {name}"))
                            || source.contains(&format!("impl fmt::{t} for {name}"))
                    };
                    // The doctest holders for `docs/*.md` (`ComponentsGuide`, `StartAxumGuide`..).
                    let guide = name.ends_with("Guide");
                    let exempt_clone = name == "Streamed" || guide;
                    if (!has("Clone") && !exempt_clone) || (!has("Debug") && !guide) {
                        missing.push(format!("{}: {name}", path.display()));
                    }
                    attrs.clear();
                } else if !line.starts_with("///") && !line.starts_with("//") {
                    attrs.clear();
                }
            }
        }
        assert!(missing.is_empty(), "not Clone + Debug: {missing:?}");
    }

    /// A builder with a `**Setters.**` paragraph, read from the source: its name, doc, the
    /// file's source and every setter (`pub fn` taking `self` and returning `Self`) of its
    /// `impl` blocks as (name, arguments after `self`).
    /// Every word a component writes by itself comes from `i18n::Strings`, so a translated
    /// table reaches every part. This reads each component's code (not its docs, tests, CSS or
    /// `PROPS`) for string literals that look like text for people: a capitalised word, or two
    /// words with a space. What is left is markup syntax, attribute values and keys.
    #[test]
    fn components_write_no_english_outside_the_string_table() {
        // Files that are not components, or whose text is not for the visitor: the table
        // itself, the spec and props lists, the script, Loco glue (its form messages are
        // `FieldErrors`' own, overridable by the app), icons (SVG paths), the site header in
        // `layout` (the demo's brand line) and debug output.
        const SKIP: [&str; 8] = [
            "i18n.rs",
            "spec.rs",
            "props.rs",
            "lib.rs",
            "enhance.rs",
            "loco.rs",
            "icon.rs",
            "layout.rs",
        ];
        let mut found = Vec::new();
        for path in library_sources() {
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            if SKIP.contains(&name.as_str()) {
                continue;
            }
            let source = std::fs::read_to_string(&path).unwrap();
            let code = source.split("#[cfg(test)]\nmod tests").next().unwrap();
            // A debug assertion speaks to the developer, not the visitor.
            let mut in_assert = false;
            let (mut in_css, mut in_props) = (false, false);
            for (n, line) in code.lines().enumerate() {
                let t = line.trim();
                if t.starts_with("pub const CSS") {
                    in_css = true;
                }
                if in_css {
                    in_css = !t.starts_with("\"#;");
                    continue;
                }
                if t.starts_with("debug_assert!(") {
                    in_assert = true;
                }
                if in_assert {
                    in_assert = !t.ends_with(");");
                    continue;
                }
                // `PROPS` and `KEYS` are documentation for the docs site, in English like
                // the rustdoc; an empty `KEYS` ends on its own line.
                if t.starts_with("pub const PROPS") || t.starts_with("pub const KEYS") {
                    in_props = !t.ends_with("];");
                    continue;
                }
                if in_props {
                    in_props = t != "];";
                    continue;
                }
                let skip = t.starts_with("//")
                    || t.starts_with("#[deprecated")
                    || t.starts_with("Prop::new")
                    || t.starts_with(".doc(")
                    || t.starts_with(".default(")
                    || t.contains("expect(")
                    || t.contains("debug_struct(")
                    || t.contains("write_str(")
                    || t.contains("debug_tuple(")
                    || t.contains("r#\"") // raw strings are SVG or CSS written by hand
                    || t.contains("write!(f");
                if skip {
                    continue;
                }
                for literal in literals(t) {
                    if looks_like_text(literal) {
                        found.push(format!("{name}:{}: \"{literal}\"", n + 1));
                    }
                }
            }
        }
        assert!(
            found.is_empty(),
            "English outside i18n::Strings:\n{}",
            found.join("\n")
        );
    }

    /// The string literals on one line (not raw strings; escapes kept as written).
    fn literals(line: &str) -> Vec<&str> {
        let mut out = Vec::new();
        let mut rest = line;
        while let Some(start) = rest.find('"') {
            let body = &rest[start + 1..];
            let mut end = None;
            let mut escaped = false;
            for (i, c) in body.char_indices() {
                match c {
                    '\\' if !escaped => escaped = true,
                    '"' if !escaped => {
                        end = Some(i);
                        break;
                    }
                    _ => escaped = false,
                }
            }
            let Some(end) = end else { break };
            out.push(&body[..end]);
            rest = &body[end + 1..];
        }
        out
    }

    /// A capitalised word followed by lowercase ("Next"), or two words with a space
    /// ("Load more", " of "), once format placeholders are taken out.
    fn looks_like_text(literal: &str) -> bool {
        let mut plain = String::new();
        let mut depth = 0;
        for c in literal.chars() {
            match c {
                '{' => depth += 1,
                '}' => depth -= 1,
                _ if depth == 0 => plain.push(c),
                _ => {}
            }
        }
        // A class, inline CSS, markup, a cookie or header value, a key shortcut.
        let syntax = plain.trim_start().starts_with("lui-")
            || plain.contains("--")
            || plain.contains('<')
            || plain.contains('>')
            || plain.contains("Path=/")
            || plain.contains("charset")
            || plain.contains("Alt+")
            || plain.contains("span-")
            || plain.contains("position-area")
            || plain.split_once(": ").is_some_and(|(k, _)| {
                !k.is_empty() && k.chars().all(|c| c.is_ascii_lowercase() || c == '-')
            });
        if syntax {
            return false;
        }
        let words: Vec<&str> = plain
            .split(|c: char| !c.is_ascii_alphabetic())
            .filter(|w| w.len() >= 2)
            .collect();
        let capitalised = words.iter().any(|w| {
            let mut cs = w.chars();
            cs.next().is_some_and(|c| c.is_ascii_uppercase())
                && cs.next().is_some_and(|c| c.is_ascii_lowercase())
        });
        let sentence = plain.contains(' ')
            && words
                .iter()
                .any(|w| w.chars().all(|c| c.is_ascii_lowercase()));
        let aside = plain.starts_with('(') && plain.ends_with(')') && words.len() == 1; // "(skipped)"
        capitalised || sentence || aside
    }

    /// Every component header says what it offers assistive tech and that axe checked it.
    #[test]
    fn every_component_header_has_an_accessibility_line() {
        let mut missing = Vec::new();
        for path in library_sources() {
            if path.ends_with("loco.rs") {
                continue; // Loco glue, not a component
            }
            let source = std::fs::read_to_string(&path).unwrap();
            let header: String = source
                .lines()
                .take_while(|l| l.starts_with("//!"))
                .collect();
            if header.contains("**What it does not do without script:**")
                && !header.contains("**Accessibility:**")
            {
                missing.push(path.file_name().unwrap().to_string_lossy().to_string());
            }
        }
        assert!(
            missing.is_empty(),
            "no **Accessibility:** line: {missing:?}"
        );
    }

    /// The library's own files, `src/*.rs` and `src/blocks/*.rs` (not `src/bin/`, the
    /// installer).
    fn library_sources() -> Vec<std::path::PathBuf> {
        let rs = |dir: &str| {
            let entries = std::fs::read_dir(dir).unwrap().map(|e| e.unwrap().path());
            entries
                .filter(|p| p.extension().is_some_and(|x| x == "rs"))
                .collect::<Vec<_>>()
        };
        let mut files = rs(concat!(env!("CARGO_MANIFEST_DIR"), "/src"));
        files.extend(rs(concat!(env!("CARGO_MANIFEST_DIR"), "/src/blocks")));
        files.extend(rs(concat!(env!("CARGO_MANIFEST_DIR"), "/src/table")));
        files
    }

    /// `source` with every `#[deprecated]` function made private, so the scrapers below see
    /// only the names a caller should use: an old name kept for one release is neither a
    /// setter to list in `PROPS` nor a constructor.
    fn undeprecated(source: &str) -> String {
        let mut out = String::with_capacity(source.len());
        let mut deprecated = false;
        for line in source.lines() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("#[deprecated") {
                deprecated = true;
            }
            if deprecated
                && (trimmed.starts_with("pub fn ") || trimmed.starts_with("pub const fn "))
            {
                out.push_str(&line.replacen("pub ", "", 1));
                deprecated = false;
            } else {
                out.push_str(line);
            }
            out.push('\n');
        }
        out
    }

    /// A library file with its deprecated functions made private (see [`undeprecated`]).
    fn read_source(path: impl AsRef<std::path::Path>) -> String {
        undeprecated(&std::fs::read_to_string(path).unwrap())
    }

    struct Builder {
        name: String,
        doc: String,
        source: String,
        setters: Vec<(String, String)>,
    }

    fn builders() -> Vec<Builder> {
        let mut found = Vec::new();
        for path in library_sources() {
            let source = read_source(path);
            let lines: Vec<&str> = source.lines().collect();
            for (i, line) in lines.iter().enumerate() {
                let Some(rest) = line.strip_prefix("pub struct ") else {
                    continue;
                };
                let name: String = rest
                    .chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '_')
                    .collect();
                let doc: String = lines[..i]
                    .iter()
                    .rev()
                    .take_while(|l| l.starts_with("///") || l.starts_with("#["))
                    .copied()
                    .collect();
                if !doc.contains("**Setters.**") {
                    continue;
                }
                // Each `impl Name` / `impl<'a> Name<'a>` block up to its closing `}`.
                let mut in_impl = false;
                let mut body = String::new();
                for l in &lines {
                    if l.starts_with("impl")
                        && !l.contains(" for ")
                        && (l.contains(&format!(" {name} ")) || l.contains(&format!(" {name}<")))
                    {
                        in_impl = true;
                    } else if in_impl && *l == "}" {
                        in_impl = false;
                    } else if in_impl {
                        body.push_str(l);
                        body.push('\n');
                    }
                }
                let mut setters = Vec::new();
                for (at, _) in body
                    .match_indices("pub fn ")
                    .chain(body.match_indices("pub const fn "))
                {
                    let sig = &body[at..body[at..].find('{').map_or(body.len(), |e| at + e)];
                    let setter: String = sig[sig.find("fn ").unwrap() + "fn ".len()..]
                        .chars()
                        .take_while(|c| c.is_alphanumeric() || *c == '_')
                        .collect();
                    if !(sig.contains("self") && sig.contains("-> Self")) {
                        continue;
                    }
                    // The arguments after `self`, on one line and without a trailing comma.
                    let open = sig.find('(').unwrap();
                    let close = sig.rfind(") ->").unwrap();
                    let args = sig[open + 1..close]
                        .split_whitespace()
                        .collect::<Vec<_>>()
                        .join(" ");
                    let args = args
                        .trim_start_matches("mut ")
                        .trim_start_matches("self")
                        .trim_start_matches(',')
                        .trim()
                        .trim_end_matches(',')
                        .replace("( ", "(")
                        .replace(" )", ")");
                    setters.push((setter, args));
                }
                found.push(Builder {
                    name,
                    doc,
                    source: source.clone(),
                    setters,
                });
            }
        }
        assert!(found.len() > 30, "found only {} builders", found.len());
        found
    }

    /// Every option discoverable in one place: a builder whose doc has a `**Setters.**`
    /// paragraph names every setter of its `impl` blocks there (grouped by the convention:
    /// values and items, no-argument switches, `bool` conditions).
    #[test]
    fn every_setter_is_listed_on_its_builder() {
        let mut missing = Vec::new();
        for b in builders() {
            for (setter, _) in &b.setters {
                if !b.doc.contains(&format!("`.{setter}(")) {
                    missing.push(format!("{}::{setter}", b.name));
                }
            }
        }
        assert!(
            missing.is_empty(),
            "setters missing from their builder's **Setters.** list: {missing:?}"
        );
    }

    /// `props()` lists every builder that has setters, and every `ui.<name>(..)` that returns
    /// a listed builder is among its constructors.
    #[test]
    fn props_lists_every_builder_and_constructor() {
        let listed = |name: &str| crate::props().iter().find(|c| c.builder == name);
        let mut missing = Vec::new();
        for b in builders() {
            if listed(&b.name).is_none() {
                missing.push(b.name);
            }
        }
        for path in library_sources() {
            let source = read_source(path);
            for block in source.split("\nimpl Ui {\n").skip(1) {
                let block = &block[..block.find("\n}\n").unwrap_or(block.len())];
                for sig in block.split("pub fn ").skip(1) {
                    let sig = &sig[..sig.find('{').unwrap_or(sig.len())];
                    let method: String = sig
                        .chars()
                        .take_while(|c| c.is_alphanumeric() || *c == '_')
                        .collect();
                    let ret: String = sig
                        .rsplit("-> ")
                        .next()
                        .unwrap()
                        .chars()
                        .take_while(|c| c.is_alphanumeric())
                        .collect();
                    if let Some(c) = listed(&ret)
                        && !c
                            .calls
                            .iter()
                            .any(|call| call.starts_with(&format!("ui.{method}(")))
                    {
                        missing.push(format!("ui.{method} -> {ret}"));
                    }
                }
            }
        }
        assert!(missing.is_empty(), "not in props(): {missing:?}");
    }

    /// Every component's header shows it both ways: its doctest builds it with the dot form and
    /// again with `lui!`, and asserts the two render the same HTML.
    #[test]
    fn every_component_header_shows_the_lui_form() {
        // Spec entries with no `ui.<component>(..)` of their own.
        let without_builder = ["enhance", "layout", "caps", "state", "paged_table"];
        let mut missing = Vec::new();
        for spec in crate::spec::SPECS {
            if without_builder.contains(&spec.module) {
                continue;
            }
            let path = format!("{}/src/{}.rs", env!("CARGO_MANIFEST_DIR"), spec.module);
            let source = std::fs::read_to_string(path).unwrap();
            let header: String = source
                .lines()
                .take_while(|l| l.starts_with("//!") || l.is_empty())
                .collect::<Vec<_>>()
                .join("\n");
            if !(header.contains("lui!") && header.contains("assert_eq!(")) {
                missing.push(spec.module);
            }
        }
        assert!(
            missing.is_empty(),
            "headers without a `lui!` twin: {missing:?}"
        );
    }

    /// Every setter is in its builder's `PROPS` with the arguments it takes, nothing else is,
    /// and the kind agrees with the arguments (a switch takes none, a condition one `bool`).
    #[test]
    fn every_setter_is_in_props() {
        let mut wrong = Vec::new();
        for b in builders() {
            // The builder's own `PROPS` block: an `impl Name` holding `pub const PROPS`.
            let head = [
                format!("\nimpl {} {{", b.name),
                format!("\nimpl {}<'_> {{", b.name),
            ];
            let block = head.iter().find_map(|h| {
                b.source.match_indices(h.as_str()).find_map(|(at, _)| {
                    let block = &b.source[at..at + b.source[at..].find("\n}").unwrap()];
                    block.contains("pub const PROPS").then_some(block)
                })
            });
            let Some(block) = block else {
                wrong.push(format!("{}: no PROPS", b.name));
                continue;
            };
            // The string literals of `text`, in order, with `\"` unescaped.
            let literals = |text: &str| {
                let mut found = Vec::new();
                let mut chars = text.chars();
                while let Some(c) = chars.next() {
                    if c != '"' {
                        continue;
                    }
                    let mut lit = String::new();
                    while let Some(c) = chars.next() {
                        match c {
                            '\\' => lit.push(chars.next().unwrap()),
                            '"' => break,
                            c => lit.push(c),
                        }
                    }
                    found.push(lit);
                }
                found
            };
            let mut listed = Vec::new();
            for entry in block.split("Prop::new(").skip(1) {
                let lits = literals(entry);
                let (name, args) = (lits[0].clone(), lits[1].clone());
                let kind = entry.split("PropKind::").nth(1).unwrap();
                let kind = &kind[..kind.find(',').unwrap()];
                listed.push((name.clone(), args.clone()));
                let fits = match kind {
                    "Switch" => args.is_empty(),
                    "Condition" => args.ends_with(": bool") && !args.contains(','),
                    _ => true,
                };
                if !fits {
                    wrong.push(format!("{}::{name}: {kind} does not fit `{args}`", b.name));
                }
            }
            for setter in &b.setters {
                if !listed.contains(setter) {
                    wrong.push(format!(
                        "{}::{}({}) not in PROPS",
                        b.name, setter.0, setter.1
                    ));
                }
            }
            for prop in &listed {
                if !b.setters.contains(prop) {
                    wrong.push(format!(
                        "{}::{}({}) in PROPS but not a setter",
                        b.name, prop.0, prop.1
                    ));
                }
            }
        }
        assert!(
            wrong.is_empty(),
            "PROPS out of step with the setters: {wrong:#?}"
        );
    }

    /// The arguments of a setter or constructor as `(name, type)` pairs, split at the commas
    /// outside `<..>`, `(..)` and `[..]`, with lifetimes dropped from the types.
    fn arguments(args: &str) -> Vec<(String, String)> {
        let mut parts = Vec::new();
        let (mut depth, mut start) = (0i32, 0);
        for (i, c) in args.char_indices() {
            match c {
                '<' | '(' | '[' => depth += 1,
                '>' if !args[..i].ends_with('-') => depth -= 1,
                ')' | ']' => depth -= 1,
                ',' if depth == 0 => {
                    parts.push(&args[start..i]);
                    start = i + 1;
                }
                _ => {}
            }
        }
        parts.push(&args[start..]);
        parts
            .into_iter()
            .map(str::trim)
            .filter(|p| !p.is_empty())
            .map(|p| {
                let (name, ty) = p.split_once(": ").unwrap_or(("", p));
                let ty = ty
                    .trim()
                    .replace("'a ", "")
                    .replace("'_ ", "")
                    .replace(" + 'a", "")
                    .replace("<'a>", "")
                    .replace("<'_>", "");
                (name.trim().to_string(), ty)
            })
            .collect()
    }

    /// Names two or more builders use for setters of a different shape, each with the reason.
    const SAME_NAME_OTHER_SHAPE: &[(&str, &str)] = &[
        (
            "total",
            "Pagination takes the row count; Table adds a totals cell under a column",
        ),
        (
            "range",
            "Form adds a slider (name, label, min, max); Calendar and DatePicker pick a range into a second name",
        ),
        (
            "radio",
            "Form adds a radio group (name, legend, options); Calendar's days become radios",
        ),
        (
            "tel",
            "Form adds a phone field (name, label); Input switches its own type",
        ),
        (
            "url",
            "Form adds a URL field (name, label); Input switches its own type",
        ),
        (
            "width",
            "Input picks a size, Form's modifier sets it on the field added last; Table's is a column width",
        ),
        (
            "footer",
            "Card's footer is markup; Table adds a `<tfoot>` row of cells, like its rows",
        ),
        (
            "checkbox",
            "Form adds a checkbox field (name, label); Menu a checkbox item posting to its action",
        ),
        (
            "radio",
            "Calendar's switch picks one day; Menu adds a radio item posting to its action",
        ),
        (
            "text",
            "Form adds a text field (name, label); Marquee adds a line of text",
        ),
        (
            "email",
            "Form's field adders take the field's name and label, then Input's arguments",
        ),
        (
            "password",
            "Form's field adders take the field's name and label",
        ),
        (
            "number",
            "Form's field adders take the field's name and label, then min and max",
        ),
        (
            "pattern",
            "Form's field adders take the field's name and label, then the pattern",
        ),
        (
            "textarea",
            "Form's field adders take the field's name and label, then the rows",
        ),
        (
            "date",
            "Form's field adders take the field's name and label, then min and max",
        ),
        (
            "time",
            "Form's field adders take the field's name and label, then min and max",
        ),
        (
            "datetime",
            "Form's field adders take the field's name and label, then min and max",
        ),
        (
            "file",
            "a file field (Input, Form) or an uploaded file with its size (Upload)",
        ),
        (
            "size",
            "a `Size` (1 to 3) everywhere but Avatar, Avatars and Dialog, which keep Radix's five and four sizes as a `u8`",
        ),
        (
            "submit",
            "Button's switch sets `type=\"submit\"`; Form's value is its button's text",
        ),
        (
            "search",
            "Input's switch sets `type=\"search\"`; Select's value is its filter's action",
        ),
        (
            "min",
            "a date (`YYYY-MM-DD`) on the date controls, a number on the numeric ones",
        ),
        (
            "max",
            "a date (`YYYY-MM-DD`) on the date controls, a number on the numeric ones",
        ),
        (
            "step",
            "a wizard's step (title and body), or the `step` of a numeric control",
        ),
        (
            "rows",
            "Table's rows from data; Pager's closure renders the rows of a page",
        ),
        (
            "values",
            "Form and Wizard take the POST body, a table Row its cells as text, a range pair its two numbers",
        ),
        ("value", "text on the text controls, a number on the slider"),
        (
            "field",
            "RecordPage adds a field (label, value); ErrorSummary names one (name, label)",
        ),
        (
            "item",
            "an item is what its component lists: a titled section, a term, a marquee entry",
        ),
        (
            "group",
            "a heading before the items that follow, or Select's and Combobox's `<optgroup>`",
        ),
        (
            "options",
            "Select takes rich options or tuples; Combobox plain suggestions",
        ),
        (
            "link",
            "EmptyState holds one link, the list builders add one per call",
        ),
        (
            "action",
            "EmptyState holds one action, the menus add one per call",
        ),
        (
            "body",
            "a component's main markup; Form's is an item, markup placed among its fields",
        ),
        (
            "underline",
            "Tabs' switch draws the underline style; Link's choice says when a link is underlined",
        ),
        (
            "current",
            "Button marks the link to this page from a condition; Steps marks the step added last",
        ),
        (
            "event",
            "Calendar adds an event on a date (date, title); Timeline an event by its title, its date a modifier",
        ),
        (
            "separator",
            "the menus add a rule between items; Breadcrumbs sets the glyph between crumbs",
        ),
    ];

    /// Setters that take a whole list in one call instead of an adder per item, each with the
    /// reason.
    const LIST_IN_ONE_CALL: &[(&str, &str)] = &[
        (
            "radio",
            "Form adds a radio group with its options, from data as `.select(..)` does",
        ),
        (
            "checkboxes",
            "Form adds a checkbox group with its options, from data as `.select(..)` does",
        ),
        (
            "value",
            "the checked values of a checkbox group: a saved set, from data",
        ),
        ("tags", "a tag input's saved tags come from data"),
        (
            "options",
            "a select's options come from data (a slice, a query), tuples welcome",
        ),
        ("group", "an `<optgroup>` is given with its options"),
        (
            "groups",
            "Select's groups from data, as (label, options) pairs",
        ),
        (
            "select",
            "Form's select field takes its options like `ui.select`",
        ),
        (
            "results",
            "the server's matches for the query, a slice by nature",
        ),
        ("presets", "a fixed palette of colours"),
        (
            "values",
            "Form and Wizard: the POST body is not in `Ui`, so the route passes it in",
        ),
        (
            "errors",
            "Form and Wizard: the validation errors come from the POST handler",
        ),
        ("rows", "a table's rows come from a slice of records"),
        (
            "filter_select",
            "a table's filter `<select>` is given with its options, like `ui.select`",
        ),
        ("points", "a chart's values from data, a slice or a query"),
        (
            "bulk",
            "the buttons of one bulk-action form, as (label, value) pairs",
        ),
        (
            "menu",
            "a table row's menu, given whole (the row is itself an item of the table)",
        ),
        (
            "submenu",
            "a nested menu is given whole, as an item of its parent",
        ),
        (
            "panel",
            "a navigation panel is given whole, as an item of the bar",
        ),
        (
            "links",
            "a palette's destinations from data (every page of an app)",
        ),
    ];

    /// `x` beside `xs` on one builder: the plural feeds a slice the caller already holds.
    const TWINS: &[(&str, &str)] = &[
        (
            "Range::values",
            "a pair's two numbers beside `.value(..)` for one",
        ),
        (
            "Form::values",
            "the POST body beside `.value(..)` for one field",
        ),
        (
            "Form::errors",
            "the POST errors beside `.error(..)` for one field",
        ),
        (
            "Select::groups",
            "groups from data beside `.group(..)` for one",
        ),
        (
            "Chart::points",
            "values from data beside `.point(..)` for one",
        ),
        (
            "Palette::links",
            "destinations from data beside `.link(..)` for one",
        ),
    ];

    /// Setters whose name is not the HTML attribute they set, each with the reason.
    const NAMED_OTHERWISE: &[(&str, &str)] = &[
        ("Input::tel", "the switch is named after the `type` it sets"),
        ("Input::url", "the switch is named after the `type` it sets"),
        (
            "CheckboxGroup::value",
            "the values to check, as every field's `.value(..)`",
        ),
        (
            "Rating::value",
            "the score to check, as every field's `.value(..)`",
        ),
        (
            "Search::results_id",
            "names the results by their id; `aria-controls` is how it says so",
        ),
        (
            "TagInput::action",
            "the page the Add round trip goes to, as a form's `action`",
        ),
        (
            "Button::submit",
            "the switch is named after the `type` it sets",
        ),
        (
            "Button::reset",
            "the switch is named after the `type` it sets",
        ),
        (
            "Input::email",
            "the switch is named after the `type` it sets",
        ),
        (
            "Input::password",
            "the switch is named after the `type` it sets",
        ),
        (
            "Input::search",
            "the switch is named after the `type` it sets",
        ),
        (
            "ToggleGroup::multiple",
            "checkboxes instead of radios: the inputs' `type`",
        ),
        (
            "Button::pressed",
            "`.pressed(on)` for a toggle, as `aria-pressed` says",
        ),
        (
            "Button::current",
            "`.current(on)` for the page's own link, as `aria-current` says",
        ),
        (
            "Combobox::label",
            "a form control's name is its label; here visually hidden",
        ),
        (
            "Section::label",
            "a region's name, as `.label(..)` names a button",
        ),
        ("Marquee::duration", "a custom property in `style`"),
        (
            "InputOtp::length",
            "the number of digits, which sets `maxlength`",
        ),
        ("NavMenu::link", "an adder of a link: text and `href`"),
        ("Sidebar::link", "an adder of a link: text and `href`"),
        ("ErrorPage::home", "the way home, a link's `href`"),
        ("RecordPage::edit", "the edit link's `href`"),
        ("RecordPage::back", "the back link's `href`"),
        ("RecordPage::delete", "the delete form's `action`"),
        (
            "ToggleGroup::option",
            "an adder: the option's `value` and text",
        ),
        (
            "ToggleGroup::value",
            "the option with this value is `checked`",
        ),
    ];

    /// Constructors whose arguments break the order (text first, or `name` then its label;
    /// an id is a setter), each with the reason.
    const CONSTRUCTOR_ORDER: &[(&str, &str)] = &[
        (
            "ui.upload",
            "the action first, like every form-posting widget; the name of its field second",
        ),
        (
            "ui.table",
            "the id keys the table's query parameters, so two tables never share them",
        ),
        (
            "ui.wizard",
            "the id keys the wizard's step parameter, so two wizards never share it",
        ),
        (
            "SelectOption::new",
            "value then text, as in `<option value>` and the `(value, text)` tuples",
        ),
    ];

    /// Modules whose modifiers do not yet check the item they follow, each with the reason.
    const MODIFIERS_UNCHECKED: &[(&str, &str)] = &[
        (
            "form",
            "M40's controls work owns form.rs; Form::required() is the pinned case",
        ),
        (
            "table",
            "M41 owns table.rs; Table::column(..).keep(..).numeric() is the pinned case",
        ),
        ("toggle_group", "M40's controls work owns toggle_group.rs"),
    ];

    /// Modules that derive ids with `slug` yet need no `.id(..)`, each with the reason.
    const SLUG_WITHOUT_ID: &[(&str, &str)] = &[
        (
            "table",
            "the table's id is its constructor's first argument",
        ),
        (
            "kanban",
            "the root's id comes from the action, the columns' from their keys",
        ),
        (
            "sortable",
            "the root's id comes from the action, the items' transition names from their keys",
        ),
        (
            "blocks/settings_page",
            "section anchors from their titles, linked from its own nav",
        ),
    ];

    /// Public `_with` functions that are not twins of a plainer one, each with the reason.
    const WITH_NAMES: &[(&str, &str)] = &[(
        "link_with",
        "this page's URL with one parameter set, the pair of `link_without`",
    )];

    /// Every attribute a `PROPS` entry may name.
    const ATTRIBUTES: &[&str] = &[
        "accept",
        "accesskey",
        "action",
        "cite",
        "aria-controls",
        "aria-current",
        "aria-haspopup",
        "aria-keyshortcuts",
        "aria-label",
        "aria-pressed",
        "autocomplete",
        "autofocus",
        "checked",
        "class",
        "closedby",
        "command",
        "datetime",
        "disabled",
        "form",
        "formaction",
        "formmethod",
        "formnovalidate",
        "high",
        "href",
        "id",
        "inputmode",
        "interestfor",
        "list",
        "low",
        "max",
        "maxlength",
        "min",
        "multiple",
        "name",
        "open",
        "optimum",
        "pattern",
        "placeholder",
        "popovertarget",
        "rel",
        "required",
        "role",
        "src",
        "step",
        "style",
        "title",
        "type",
        "value",
    ];

    /// The conventions of CLAUDE.md that `props()` can show, checked over every builder: what
    /// each kind of setter takes, one shape per name, adders rather than lists, an `.id()`
    /// wherever an id is derived, setters named after their attribute, and the order of a
    /// constructor's arguments. A deviation is fixed, or allow-listed above with its reason
    /// (and in docs/api.md, "Kept, because").
    #[test]
    fn props_follow_the_conventions() {
        use crate::props::PropKind::{self, *};
        let listed = |list: &[(&str, &str)], key: &str| list.iter().any(|(k, _)| *k == key);
        let number = |t: &str| ["i64", "u64", "usize", "u8", "u32", "f64"].contains(&t);
        // A setter's shape: its kind (a modifier counted as the kind its arguments make it)
        // and its argument types.
        let shape = |p: &crate::props::Prop| {
            let types: Vec<String> = arguments(p.args).into_iter().map(|(_, t)| t).collect();
            let kind = match p.kind {
                Modifier if types.is_empty() => Switch,
                Modifier if types == ["bool"] => Condition,
                Modifier if types.len() == 1 && number(&types[0]) => Number,
                Modifier => Value,
                k => k,
            };
            (kind, types)
        };
        let mut wrong = Vec::new();
        type Shape = (PropKind, Vec<String>);
        let mut shapes: Vec<(&str, &str, Shape)> = Vec::new();
        for c in crate::props() {
            let names: Vec<&str> = c.props.iter().map(|p| p.name).collect();
            for p in c.props {
                let at = format!("{}::{}", c.builder, p.name);
                let args = arguments(p.args);
                // 1. A switch takes nothing, a condition one `bool`.
                match p.kind {
                    Switch if !args.is_empty() => {
                        wrong.push(format!("{at}: a switch with arguments"))
                    }
                    Condition if args.len() != 1 || args[0].1 != "bool" => {
                        wrong.push(format!("{at}: a condition takes one bool"))
                    }
                    _ => {}
                }
                // 2. A `bool` argument only where a route decides from a condition.
                if !matches!(p.kind, Condition | Modifier) && args.iter().any(|(_, t)| t == "bool")
                {
                    wrong.push(format!("{at}: a bool outside a condition"));
                }
                // 3. Off until called.
                if matches!(p.kind, Switch | Condition) && p.default != "off" {
                    wrong.push(format!("{at}: on by default"));
                }
                // 6. No `_with` names, no `x` beside `xs` unless listed.
                if p.name.ends_with("_with") {
                    wrong.push(format!("{at}: a `_with` name"));
                }
                if let Some(one) = p.name.strip_suffix('s')
                    && names.contains(&one)
                    && !listed(TWINS, &at)
                {
                    wrong.push(format!("{at}: beside `.{one}(..)`"));
                }
                // A list given at once only where listed.
                let list = args
                    .iter()
                    .any(|(_, t)| t.contains("IntoIterator") || t.starts_with("&["));
                if list && !listed(LIST_IN_ONE_CALL, p.name) {
                    wrong.push(format!("{at}: a list in one call"));
                }
                // 8. A real attribute, and the setter's own name unless listed.
                if !p.attr.is_empty() {
                    if !ATTRIBUTES.contains(&p.attr) {
                        wrong.push(format!("{at}: `{}` is not an attribute", p.attr));
                    }
                    if p.attr != p.name.replace('_', "-") && !listed(NAMED_OTHERWISE, &at) {
                        wrong.push(format!("{at}: sets `{}`", p.attr));
                    }
                }
                shapes.push((c.builder, p.name, shape(p)));
            }
            // 4. A modifier needs an item to modify.
            let has = |k| c.props.iter().any(|p| p.kind == k);
            if has(Modifier) && !has(Item) {
                wrong.push(format!("{}: modifiers without an item", c.builder));
            }
            // 8. A modifier says which items it may follow, each one a setter of this
            // builder that adds an item, so its debug check and its props row agree.
            for p in c.props.iter().filter(|p| p.kind == Modifier) {
                if listed(MODIFIERS_UNCHECKED, c.module) {
                    continue;
                }
                if p.applies_to.is_empty() {
                    wrong.push(format!(
                        "{}::{}: a modifier without `applies_to`",
                        c.builder, p.name
                    ));
                }
                for item in p.applies_to {
                    let adds = c
                        .props
                        .iter()
                        .any(|q| q.name == *item && matches!(q.kind, Item | Modifier));
                    if !adds {
                        wrong.push(format!(
                            "{}::{}: applies to `{item}`, not an item setter",
                            c.builder, p.name
                        ));
                    }
                }
            }
            // Constructors: text first, or `name` then its label; an id is a setter.
            for call in c.calls {
                let open = call.find('(').unwrap();
                if listed(CONSTRUCTOR_ORDER, &call[..open]) {
                    continue;
                }
                let args = arguments(&call[open + 1..call.rfind(')').unwrap()]);
                let names: Vec<&str> = args.iter().map(|(n, _)| n.as_str()).collect();
                let text = ["text", "label", "title", "legend", "trigger"];
                if let Some(at) = names.iter().position(|n| text.contains(n))
                    && !(at == 0 || (at == 1 && names[0] == "name"))
                {
                    wrong.push(format!("{call}: its text is not first"));
                }
                if names.iter().skip(1).any(|n| *n == "name") {
                    wrong.push(format!("{call}: `name` is not first"));
                }
                if names.contains(&"id") {
                    wrong.push(format!("{call}: an id in the call, not a setter"));
                }
            }
        }
        // 5. One name, one shape, across builders.
        for (i, (builder, name, s)) in shapes.iter().enumerate() {
            let first = shapes[..i].iter().find(|(_, n, _)| n == name);
            if let Some((other, _, t)) = first
                && s != t
                && !listed(SAME_NAME_OTHER_SHAPE, name)
            {
                wrong.push(format!(
                    "`.{name}`: {builder} takes {:?} {:?}, {other} {:?} {:?}",
                    s.0, s.1, t.0, t.1
                ));
            }
        }
        // 7. A module that derives an id with `slug` lets the caller set it.
        let mut modules: Vec<&str> = crate::props().iter().map(|c| c.module).collect();
        modules.dedup();
        for module in modules {
            let path = format!("{}/src/{module}.rs", env!("CARGO_MANIFEST_DIR"));
            let source = std::fs::read_to_string(path).unwrap();
            let code = source.split("#[cfg(test)]\nmod tests").next().unwrap();
            let mut props = crate::props().iter().filter(|c| c.module == module);
            if code.contains("slug(")
                && !listed(SLUG_WITHOUT_ID, module)
                && !props.any(|c| c.props.iter().any(|p| p.name == "id"))
            {
                wrong.push(format!(
                    "{module}: derives an id with `slug` but has no `.id(..)`"
                ));
            }
        }
        // 6. No public `_with` function or `Options` struct.
        for path in library_sources() {
            for line in read_source(&path).lines() {
                let line = line.trim_start();
                let name =
                    |rest: &str| rest.split(['(', '<', ' ', '{']).next().unwrap().to_string();
                let with = line
                    .strip_prefix("pub fn ")
                    .is_some_and(|r| name(r).ends_with("_with") && !listed(WITH_NAMES, &name(r)));
                let options = line
                    .strip_prefix("pub struct ")
                    .is_some_and(|r| name(r).ends_with("Options"));
                if with || options {
                    wrong.push(format!("{}: `{line}`", path.display()));
                }
            }
        }
        assert!(
            wrong.is_empty(),
            "deviations from the conventions: {wrong:#?}"
        );
    }

    /// The stylesheet is five cascade layers in a fixed order, the minifier keeps them, and
    /// nothing of the demo's own site is in it.
    #[test]
    fn the_stylesheet_is_layered() {
        let css = stylesheet();
        assert!(
            css.starts_with("@layer lui.reset,lui.tokens,lui.components,lui.blocks,lui.modes;")
        );
        for layer in ["reset", "tokens", "components", "blocks", "modes"] {
            assert_eq!(
                css.matches(&format!("@layer lui.{layer}{{")).count(),
                1,
                "{layer}"
            );
        }
        assert_eq!(
            minify_css("@layer a, b;\n@layer a { p { margin: 0 } }"),
            "@layer a,b;@layer a{p{margin:0}}"
        );
        assert!(
            !css.contains(".lui-site")
                && !css.contains(".lui-index")
                && !css.contains(".lui-props")
        );
    }

    /// `Component` has one variant per `COMPONENT_CSS` entry, in its order: a component
    /// appended there needs its variant appended too.
    #[test]
    fn every_component_css_has_a_variant() {
        let parts: Vec<&str> = (Component::ALL.iter())
            .filter(|c| **c != Component::Blocks)
            .map(|c| c.css())
            .filter(|css| !css.is_empty())
            .collect();
        assert_eq!(
            parts.len(),
            COMPONENT_CSS.len(),
            "a Component variant is missing"
        );
        for (i, (a, b)) in parts.iter().zip(COMPONENT_CSS).enumerate() {
            assert!(a == b, "Component::ALL[{i}] is out of COMPONENT_CSS order");
        }
        for (i, c) in Component::ALL.iter().enumerate() {
            assert_eq!(*c as usize, i, "{c:?} is out of place in Component::ALL");
        }
        let dialog = stylesheet_for(&[Component::Dialog]);
        assert!(dialog.contains(".lui-dialog") && dialog.contains(".lui-button"));
        assert!(!dialog.contains(".lui-kanban") && !dialog.contains(".lui-auth"));
        assert!(
            std::ptr::eq(dialog, stylesheet_for(&[Component::Dialog])),
            "built once"
        );
    }

    /// The text of every CSS const in the library's sources, per file name.
    fn css_by_file() -> Vec<(String, String)> {
        library_sources()
            .into_iter()
            .map(|path| {
                let source = std::fs::read_to_string(&path).unwrap();
                let mut css = String::new();
                for start in source.match_indices(": &str = r#\"").map(|(i, _)| i) {
                    let head = &source[..start];
                    let name = head.rsplit("pub const ").next().unwrap_or("");
                    if !matches!(name.trim(), "CSS" | "RESET" | "MODES") {
                        continue;
                    }
                    let body = &source[start..];
                    css.push_str(&body[..body.find("\"#;").unwrap()]);
                }
                let name = path.file_name().unwrap().to_string_lossy().to_string();
                (name, css)
            })
            .filter(|(_, css)| !css.is_empty())
            .collect()
    }

    /// Every `property: value` declaration in `css`.
    fn declarations(css: &str) -> Vec<(String, String)> {
        css.split(['{', '}', ';'])
            .filter_map(|d| d.split_once(':'))
            .map(|(p, v)| (p.trim().to_string(), v.trim().to_string()))
            .collect()
    }

    /// Type sizes and stacking come from tokens: `font-size` is a `var(--lui-text-n)` (or
    /// relative: `em`, `%`, `inherit`) and `z-index` a `var(--lui-z-*)` (or a local order
    /// from -1 to 3 inside one component).
    #[test]
    fn no_literal_font_size_or_z_outside_tokens() {
        let mut failed = Vec::new();
        for (file, css) in css_by_file() {
            let bad: Vec<String> = declarations(&css)
                .into_iter()
                .filter(|(p, v)| match p.as_str() {
                    "font-size" => {
                        !(v.starts_with("var(--lui-text-")
                            || v == "inherit"
                            || (v.ends_with("em") && !v.ends_with("rem"))
                            || v.ends_with('%'))
                    }
                    "z-index" => {
                        !(v.starts_with("var(--lui-z-")
                            || matches!(v.as_str(), "auto" | "-1" | "0" | "1" | "2" | "3"))
                    }
                    _ => false,
                })
                .map(|(p, v)| format!("{p}: {v}"))
                .collect();
            if !bad.is_empty() {
                failed.push(format!("{file}: {bad:?}"));
            }
        }
        assert!(failed.is_empty(), "{failed:#?}");
    }

    /// Component CSS uses logical properties (`margin-inline-start`, `inset-inline-end`,
    /// `text-align: start`), so a right-to-left page mirrors. Transforms, `translate` and the
    /// `inset` shorthand are fine.
    #[test]
    fn no_physical_property_in_component_css() {
        let physical = |p: &str, v: &str| {
            matches!(
                p,
                "left"
                    | "right"
                    | "margin-left"
                    | "margin-right"
                    | "padding-left"
                    | "padding-right"
            ) || p.starts_with("border-left")
                || p.starts_with("border-right")
                || (matches!(p, "text-align" | "float" | "clear") && matches!(v, "left" | "right"))
        };
        let mut failed = Vec::new();
        for (file, css) in css_by_file() {
            let bad: Vec<String> = declarations(&css)
                .into_iter()
                .filter(|(p, v)| physical(p, v))
                .map(|(p, v)| format!("{p}: {v}"))
                .collect();
            if !bad.is_empty() {
                failed.push(format!("{file}: {bad:?}"));
            }
        }
        assert!(failed.is_empty(), "{failed:#?}");
    }

    /// Every component's CSS together, inlined once per page, stays under 64 KB (57.6 KB and
    /// 10.3 KB gzipped at M26; README "What a page weighs").
    #[test]
    fn stylesheet_stays_under_its_budget() {
        // 64 KB until M29 added blocks, a chart and six components, 72 KB until M30's colour
        // scales, depth tokens and motion, 88 KB until M34 restyled every component after its
        // reference; M34's real limit is +10 KB gzipped over 14.4 KB, checked at its wrap-up
        // (README: "What a page weighs"). 128 KB until M39 folded the dark palette into
        // light-dark() and moved the demo's CSS out: 117 240 bytes then, 111 358 after; 124 KB
        // once M38's data-* variant rules joined, until M39's sweep moves the literals to tokens.
        // 136 KB once M40a's sixteen typography and layout components joined (about 14.5 KB
        // minified: 133 620 bytes in all).
        // 136 KB from M40d: a banner, a popover, a carousel and the new setters of the
        // feedback components.
        // 132 KB once M40b's six navigation components joined; 168 KB with M40a, M40b and
        // M40d together, until M39's sweep trims the literals.
        // M40c added the number, search, rating, tag input and checkbox group components and
        // the input group (addons, steppers, reveal): 176 KB with every M40 branch merged.
        assert!(
            stylesheet().len() < 176 * 1024,
            "stylesheet() is {} bytes",
            stylesheet().len()
        );
    }

    /// Buttons and inputs are styled in one place each: no other component's CSS selects a
    /// bare `button` or `input` (anywhere in a selector, `:is()` and `:where()` included), so
    /// a change to the primitive reaches every component. A component styles its own parts
    /// by class (`.lui-counter-input`, `.lui-dialog-close`).
    #[test]
    fn only_the_primitives_select_bare_buttons_and_inputs() {
        fn preludes(css: &str) -> Vec<String> {
            let (mut out, mut buf) = (Vec::new(), String::new());
            for c in minify_css(css).chars() {
                match c {
                    '{' => out.push(std::mem::take(&mut buf)),
                    '}' | ';' => buf.clear(),
                    c => buf.push(c),
                }
            }
            out.retain(|p| !p.starts_with('@'));
            out
        }
        fn selects_bare(selector: &str, element: &str) -> bool {
            selector.match_indices(element).any(|(i, _)| {
                let before = selector[..i].chars().next_back();
                let after = selector[i + element.len()..].chars().next();
                let starts = before.is_none_or(|c| " ,>+~(".contains(c));
                let ends =
                    after.is_none_or(|c| !(c.is_ascii_alphanumeric() || c == '-' || c == '_'));
                starts && ends
            })
        }
        assert!(
            selects_bare(".x :is(button, a)", "button")
                && selects_bare("input[type=range]", "input")
        );
        assert!(!selects_bare(".lui-button", "button") && !selects_bare("[type=button]", "button"));
        for css in COMPONENT_CSS {
            if *css == button::CSS || *css == input::CSS {
                continue;
            }
            for p in preludes(css) {
                for element in ["button", "input"] {
                    assert!(
                        !selects_bare(&p, element),
                        "`{p}` styles a bare {element}: style the part by class, or change {element}.rs"
                    );
                }
            }
        }
    }

    /// M38: the state attributes of `docs/api.md` ("State attributes") are a styling API, so
    /// a component that writes `data-state`, `data-disabled`, `data-invalid` or
    /// `data-orientation` in its markup selects the same attribute in its own CSS: the class
    /// and the attribute cannot drift apart.
    #[test]
    fn state_attributes_are_styled_where_they_are_set() {
        const STATE: [&str; 4] = [
            "data-state",
            "data-disabled",
            "data-invalid",
            "data-orientation",
        ];
        let mut drift = Vec::new();
        for path in library_sources() {
            let source = std::fs::read_to_string(&path).unwrap();
            let (code, css) = source
                .split_once("pub const CSS: &str")
                .unwrap_or((source.as_str(), ""));
            let code: String = code
                .lines()
                .filter(|l| !l.trim_start().starts_with("//"))
                .collect::<Vec<_>>()
                .join("\n");
            for attr in STATE {
                let set = [format!("{attr}="), format!("{attr}["), format!("{attr} ")]
                    .iter()
                    .any(|s| code.contains(s.as_str()));
                if set && !css.contains(&format!("[{attr}")) {
                    let name = path.file_name().unwrap().to_string_lossy().to_string();
                    drift.push(format!("{name} sets {attr} but its CSS never selects it"));
                }
            }
        }
        assert!(drift.is_empty(), "{drift:#?}");
    }

    /// M34: components respond to the box they sit in, not the viewport (`layout.rs` header).
    /// A component whose CSS sets a layout (`display: grid`, wrapping flex, `grid-template-*`)
    /// needs an `@container` rule, and no component CSS asks the viewport's size: media
    /// queries are for `prefers-*` and `pointer`.
    #[test]
    fn layouts_respond_to_their_container() {
        let named: &[(&str, &str)] = &[
            ("layout", layout::CSS),
            ("button", button::CSS),
            ("input", input::CSS),
            ("badge", badge::CSS),
            ("card", card::CSS),
            ("icon", icon::CSS),
            ("avatar", avatar::CSS),
            ("stack", stack::CSS),
            ("cluster", cluster::CSS),
            ("grid", grid::CSS),
            ("split", split::CSS),
            ("calendar", calendar::CSS),
            ("date_picker", date_picker::CSS),
            ("upload", upload::CSS),
            ("kanban", kanban::CSS),
            ("sortable", sortable::CSS),
            ("tooltip", tooltip::CSS),
            ("alert", alert::CSS),
            ("progress", progress::CSS),
            ("meter", meter::CSS),
            ("separator", separator::CSS),
            ("dialog", dialog::CSS),
            ("popover", popover::CSS),
            ("tabs", tabs::CSS),
            ("accordion", accordion::CSS),
            ("combobox", combobox::CSS),
            ("pager", pager::CSS),
            ("form", form::CSS),
            ("error_summary", error_summary::CSS),
            ("counter", counter::CSS),
            ("theme", theme::CSS),
            ("flash", flash::CSS),
            ("select", select::CSS),
            ("range", range::CSS),
            ("color", color::CSS),
            ("table", table::CSS),
            ("paged_table", paged_table::CSS),
            ("wizard", wizard::CSS),
            ("toast", toast::CSS),
            ("breadcrumbs", breadcrumbs::CSS),
            ("skeleton", skeleton::CSS),
            ("empty_state", empty_state::CSS),
            ("stat", stat::CSS),
            ("chart", chart::CSS),
            ("sidebar", sidebar::CSS),
            ("nav_menu", nav_menu::CSS),
            ("description_list", description_list::CSS),
            ("toggle_group", toggle_group::CSS),
            ("context_menu", context_menu::CSS),
            ("input_otp", input_otp::CSS),
            ("drawer", drawer::CSS),
            ("palette", palette::CSS),
            ("marquee", marquee::CSS),
            ("variant", variant::CSS),
            ("heading", heading::CSS),
            ("text", text::CSS),
            ("link", link::CSS),
            ("kbd", kbd::CSS),
            ("code", code::CSS),
            ("blockquote", blockquote::CSS),
            ("prose", prose::CSS),
            ("container", container::CSS),
            ("section", section::CSS),
            ("inset", inset::CSS),
            ("aspect_ratio", aspect_ratio::CSS),
            ("scroll_area", scroll_area::CSS),
            ("center", center::CSS),
            ("spacer", spacer::CSS),
            ("spinner", spinner::CSS),
            ("indicator", indicator::CSS),
            ("banner", banner::CSS),
            ("popover_card", popover_card::CSS),
            ("carousel", carousel::CSS),
            ("pagination", pagination::CSS),
            ("steps", steps::CSS),
            ("timeline", timeline::CSS),
            ("tree", tree::CSS),
            ("collapsible", collapsible::CSS),
            ("hover_card", hover_card::CSS),
            ("checkbox_group", checkbox_group::CSS),
            ("number", number::CSS),
            ("search", search::CSS),
            ("rating", rating::CSS),
            ("tag_input", tag_input::CSS),
            ("app_shell", blocks::app_shell::CSS),
            ("auth_page", blocks::auth_page::CSS),
            ("settings_page", blocks::settings_page::CSS),
            ("record_page", blocks::record_page::CSS),
            ("dashboard_page", blocks::dashboard_page::CSS),
            ("error_page", blocks::error_page::CSS),
        ];
        // `stream::CSS` exists only with the `http` feature and sets no layout.
        let streamed = usize::from(cfg!(feature = "http"));
        assert_eq!(
            named.len() + streamed,
            COMPONENT_CSS.len() + blocks::CSS.len(),
            "list every stylesheet here"
        );
        // One layout at every width, or one that adapts by itself: no breakpoint to add.
        const NO_BREAKPOINT: &[(&str, &str)] = &[
            ("layout", "the page shell: its container is the viewport"),
            (
                "app_shell",
                "the page frame: its container is the viewport (the drawer decides)",
            ),
            ("stack", "one column"),
            ("center", "one centred column"),
            (
                "marquee",
                "one looping row, or at rest a row that wraps by its own content",
            ),
            ("sortable", "one column of rows at every width"),
            ("cluster", "wraps by its own content"),
            ("split", "wraps by flex-basis, no breakpoint"),
            ("grid", "auto-fill columns capped at min(…, 100%)"),
            ("avatar", "a grid only centres the initials"),
            ("counter", "one row that wraps: − value +"),
            ("auth_page", "one centred column of width min(24rem, 100%)"),
            ("error_page", "one centred column"),
            (
                "select",
                "a column of the filter row and the select; the row wraps",
            ),
            (
                "combobox",
                "one row of chips, search box and button that wraps",
            ),
            ("range", "one row: the slider and its value"),
            (
                "card",
                "a column; the header's action wraps under the title",
            ),
            ("skeleton", "a column of blocks"),
            ("empty_state", "one centred column"),
            (
                "description_list",
                "each pair a wrapping row: the value drops under its label",
            ),
            (
                "record_page",
                "a head row that wraps over fields laid out as the description list",
            ),
            ("progress", "a label row over a full-width bar"),
            (
                "stat",
                "auto-fit tiles of min(12rem, 100%): 4, 3, 2 or 1 across by width",
            ),
            (
                "sidebar",
                "one column; the icon rail is a toggle, not a width",
            ),
            (
                "alert",
                "a callout: the icon column beside the text at any width",
            ),
            ("error_summary", "a callout, as alert"),
            ("flash", "callouts, as alert, the dismiss in a third column"),
            (
                "toggle_group",
                "one row of equal items that scrolls inside itself when narrow",
            ),
            (
                "calendar",
                "width min(100%, 17.5rem): fills a narrow box with square cells",
            ),
            (
                "color",
                "a row of picker, value chip and presets that wraps",
            ),
            (
                "tree",
                "one column of rows; the depth is an indent, not a width",
            ),
            ("number", "one row: the field and its addons, as input"),
            (
                "date_picker",
                "a range's two dates are a row that wraps; the calendar adapts by itself",
            ),
            ("rating", "one row of stars that wraps"),
            (
                "tag_input",
                "one box of chips and a text box that wraps as it fills",
            ),
        ];

        // Pinned to the viewport in the top layer, so the viewport is the right question, for
        // their layout as for their media queries.
        const VIEWPORT: &[(&str, &str)] = &[
            ("layout", "the page shell and demo site"),
            ("dialog", "a modal sized against the viewport"),
            (
                "drawer",
                "a sheet over the viewport, a column of the page above 60rem",
            ),
            ("toast", "the stack is fixed to a corner of the viewport"),
            (
                "palette",
                "the panel is a top-layer popover, full width on a phone",
            ),
        ];
        // Not responsive yet, each until its M34 box lands. The test fails once one passes,
        // so the entry leaves with the fix. Empty by the M34 wrap-up.
        const PENDING: &[(&str, &str)] = &[];
        let listed = |list: &[(&str, &str)], name: &str| list.iter().any(|(n, _)| *n == name);
        let mut wrong = Vec::new();
        for (name, css) in named {
            let css = minify_css(css);
            let lays_out = [
                "display:grid",
                "display:inline-grid",
                "flex-wrap:wrap",
                "grid-template",
            ]
            .iter()
            .any(|d| css.contains(d));
            let responds = css.contains("@container");
            let asks_viewport = css.match_indices("@media").any(|(i, _)| {
                let query = &css[i..css[i..].find('{').map_or(css.len(), |e| i + e)];
                ["width", "height"].iter().any(|w| query.contains(w))
            });
            let exempt = listed(NO_BREAKPOINT, name) || listed(VIEWPORT, name);
            let fine =
                (!lays_out || responds || exempt) && (!asks_viewport || listed(VIEWPORT, name));
            match (fine, listed(PENDING, name)) {
                (true, true) => wrong.push(format!("{name} responds now: take it off PENDING")),
                (false, false) if lays_out && !responds && !exempt => {
                    wrong.push(format!("{name} sets a layout but has no @container rule"))
                }
                (false, false) => wrong.push(format!("{name} asks the viewport's width in @media")),
                _ => {}
            }
        }
        assert!(wrong.is_empty(), "{}", wrong.join("\n"));
    }

    /// Every component nests in `html!` and converts to a plain `String`.
    #[test]
    fn components_render_and_stringify() {
        let ui = Ui::from_request("/", "", "lui-flash=hi");
        let parts: [&dyn Render; 3] = [
            &ui.flash(),
            &ui.counter("/counter", 3),
            &ui.theme_toggle("/theme"),
        ];
        for part in parts {
            let nested = html! { (part) }.into_string();
            assert!(nested.starts_with('<') && nested == part.render().into_string());
        }
    }

    use maud::Markup;

    /// The example calls behind [`every_attr_prop_sets_its_attribute`]: for each builder and
    /// setter whose `PROPS` entry names an HTML attribute, the builder without the setter and
    /// with it, in that order.
    type AttrExample = (&'static str, &'static str, fn(&Ui) -> [Markup; 2]);

    const ATTR_EXAMPLES: &[AttrExample] = &[
        ("Button", "interestfor", |ui| {
            [
                ui.button("Go").render(),
                ui.button("Go").interestfor("card").render(),
            ]
        }),
        ("Steps", "href", |ui| {
            [
                ui.steps().step("A").render(),
                ui.steps().step("A").href("/a").render(),
            ]
        }),
        ("Timeline", "datetime", |ui| {
            [
                ui.timeline().event("A").when("May").render(),
                ui.timeline()
                    .event("A")
                    .when("May")
                    .datetime("2026-05")
                    .render(),
            ]
        }),
        ("Tree", "open", |ui| {
            [
                ui.tree("T").node("A").children(|t| t.node("B")).render(),
                ui.tree("T")
                    .node("A")
                    .children(|t| t.node("B"))
                    .open(true)
                    .render(),
            ]
        }),
        ("Collapsible", "open", |ui| {
            [
                ui.collapsible("More", maud::html! {}).render(),
                ui.collapsible("More", maud::html! {}).open(true).render(),
            ]
        }),
        ("Collapsible", "name", |ui| {
            [
                ui.collapsible("More", maud::html! {}).render(),
                ui.collapsible("More", maud::html! {}).name("g").render(),
            ]
        }),
        ("HoverCard", "href", |ui| {
            [
                ui.hover_card("Ada", maud::html! {}).render(),
                ui.hover_card("Ada", maud::html! {}).href("/ada").render(),
            ]
        }),
        ("HoverCard", "id", |ui| {
            [
                ui.hover_card("Ada", maud::html! {}).render(),
                ui.hover_card("Ada", maud::html! {}).id("card").render(),
            ]
        }),
        ("Avatar", "src", |ui| {
            [
                ui.avatar("Ada").render(),
                ui.avatar("Ada").src("/a.png").render(),
            ]
        }),
        ("Badge", "href", |ui| {
            [
                ui.badge("New").render(),
                ui.badge("New").href("/new").render(),
            ]
        }),
        ("Button", "submit", |ui| {
            [
                ui.button("Go").command("close", "d").render(),
                ui.button("Go").command("close", "d").submit().render(),
            ]
        }),
        ("Button", "reset", |ui| {
            [ui.button("Go").render(), ui.button("Go").reset().render()]
        }),
        ("Button", "command", |ui| {
            [
                ui.button("Go").render(),
                ui.button("Go").command("show-modal", "d").render(),
            ]
        }),
        ("Button", "popovertarget", |ui| {
            [
                ui.button("Go").render(),
                ui.button("Go").popovertarget("p").render(),
            ]
        }),
        ("Button", "form", |ui| {
            [ui.button("Go").render(), ui.button("Go").form("f").render()]
        }),
        ("Button", "name", |ui| {
            [
                ui.button("Go").render(),
                ui.button("Go").name("op").render(),
            ]
        }),
        ("Button", "value", |ui| {
            [
                ui.button("Go").render(),
                ui.button("Go").value("inc").render(),
            ]
        }),
        ("Button", "aria_label", |ui| {
            [
                ui.button("Go").render(),
                ui.button("Go").aria_label("Go on").render(),
            ]
        }),
        ("Button", "class", |ui| {
            [
                ui.button("Go").render(),
                ui.button("Go").class("x").render(),
            ]
        }),
        ("Button", "disabled", |ui| {
            [
                ui.button("Go").render(),
                ui.button("Go").disabled().render(),
            ]
        }),
        ("Button", "id", |ui| {
            [ui.button("Go").render(), ui.button("Go").id("go").render()]
        }),
        ("Button", "role", |ui| {
            [
                ui.button("Go").render(),
                ui.button("Go").role("tab").render(),
            ]
        }),
        ("Button", "title", |ui| {
            [
                ui.button("Go").render(),
                ui.button("Go").title("Go on").render(),
            ]
        }),
        ("Button", "style", |ui| {
            [
                ui.button("Go").render(),
                ui.button("Go").style("order: 1").render(),
            ]
        }),
        ("Button", "aria_haspopup", |ui| {
            [
                ui.button("Go").render(),
                ui.button("Go").aria_haspopup("menu").render(),
            ]
        }),
        ("Button", "pressed", |ui| {
            [
                ui.button("Go").render(),
                ui.button("Go").pressed(true).render(),
            ]
        }),
        ("Button", "accesskey", |ui| {
            [
                ui.button("Go").render(),
                ui.button("Go").accesskey("g").render(),
            ]
        }),
        ("Button", "aria_keyshortcuts", |ui| {
            [
                ui.button("Go").render(),
                ui.button("Go").aria_keyshortcuts("Alt+G").render(),
            ]
        }),
        ("Button", "formmethod", |ui| {
            [
                ui.button("Go").render(),
                ui.button("Go").formmethod("get").render(),
            ]
        }),
        ("Button", "formaction", |ui| {
            [
                ui.button("Go").render(),
                ui.button("Go").formaction("/go").render(),
            ]
        }),
        ("Button", "formnovalidate", |ui| {
            [
                ui.button("Go").render(),
                ui.button("Go").formnovalidate().render(),
            ]
        }),
        ("Button", "rel", |ui| {
            [
                ui.link_button("Go", "/go").render(),
                ui.link_button("Go", "/go").rel("next").render(),
            ]
        }),
        ("Button", "current", |ui| {
            [
                ui.link_button("Go", "/go").render(),
                ui.link_button("Go", "/go").current(true).render(),
            ]
        }),
        ("Calendar", "value", |ui| {
            [
                ui.calendar("due").radio().render(),
                ui.calendar("due").radio().value("2026-09-24").render(),
            ]
        }),
        ("Calendar", "required", |ui| {
            [
                ui.calendar("due").radio().render(),
                ui.calendar("due").radio().required().render(),
            ]
        }),
        ("Card", "id", |ui| {
            [ui.card().render(), ui.card().id("plan").render()]
        }),
        ("Chart", "id", |ui| {
            [
                ui.chart("Signups").render(),
                ui.chart("Signups").id("signups").render(),
            ]
        }),
        ("Blockquote", "cite", |ui| {
            [
                ui.blockquote().render(),
                ui.blockquote().cite("https://example.com").render(),
            ]
        }),
        ("CodeBlock", "id", |ui| {
            [
                ui.code_block("x").render(),
                ui.code_block("x").id("snippet").render(),
            ]
        }),
        ("Color", "value", |ui| {
            [
                ui.color("accent", "Accent").render(),
                ui.color("accent", "Accent").value("#2f5bea").render(),
            ]
        }),
        ("Combobox", "label", |ui| {
            [
                ui.combobox("q", "/s").render(),
                ui.combobox("q", "/s").label("Language").render(),
            ]
        }),
        ("Counter", "min", |ui| {
            [
                ui.counter("/c", 3).typed().render(),
                ui.counter("/c", 3).typed().min(0).render(),
            ]
        }),
        ("Counter", "max", |ui| {
            [
                ui.counter("/c", 3).typed().render(),
                ui.counter("/c", 3).typed().max(9).render(),
            ]
        }),
        ("Counter", "step", |ui| {
            [
                ui.counter("/c", 3).typed().render(),
                ui.counter("/c", 3).typed().step(2).render(),
            ]
        }),
        ("DatePicker", "value", |ui| {
            [
                ui.date_picker("due", "Due").native().render(),
                ui.date_picker("due", "Due")
                    .native()
                    .value("2026-09-24")
                    .render(),
            ]
        }),
        ("DatePicker", "min", |ui| {
            [
                ui.date_picker("due", "Due").native().render(),
                ui.date_picker("due", "Due")
                    .native()
                    .min("2026-01-01")
                    .render(),
            ]
        }),
        ("DatePicker", "max", |ui| {
            [
                ui.date_picker("due", "Due").native().render(),
                ui.date_picker("due", "Due")
                    .native()
                    .max("2026-12-31")
                    .render(),
            ]
        }),
        ("DatePicker", "required", |ui| {
            [
                ui.date_picker("due", "Due").native().render(),
                ui.date_picker("due", "Due").native().required().render(),
            ]
        }),
        ("Dialog", "id", |ui| {
            [
                ui.dialog("Hi").render(),
                ui.dialog("Hi").id("hello").render(),
            ]
        }),
        ("Dialog", "open", |ui| {
            [
                ui.dialog("Hi").render(),
                ui.dialog("Hi").open(true).render(),
            ]
        }),
        ("Dialog", "closedby", |ui| {
            [
                ui.dialog("Hi").render(),
                ui.dialog("Hi").closedby("closerequest").render(),
            ]
        }),
        ("Drawer", "id", |ui| {
            [
                ui.drawer("Menu").render(),
                ui.drawer("Menu").id("nav").render(),
            ]
        }),
        ("Drawer", "open", |ui| {
            [
                ui.drawer("Menu").render(),
                ui.drawer("Menu").open(true).render(),
            ]
        }),
        ("Form", "value", |ui| {
            [
                ui.form("/f").text("n", "N").render(),
                ui.form("/f").text("n", "N").value("Ada").render(),
            ]
        }),
        ("Form", "id", |ui| {
            [ui.form("/f").render(), ui.form("/f").id("profile").render()]
        }),
        ("Input", "email", |ui| {
            [
                ui.input("n", "N").render(),
                ui.input("n", "N").email().render(),
            ]
        }),
        ("Input", "password", |ui| {
            [
                ui.input("n", "N").render(),
                ui.input("n", "N").password().render(),
            ]
        }),
        ("Input", "pattern", |ui| {
            [
                ui.input("n", "N").render(),
                ui.input("n", "N").pattern("[0-9]+", "Digits").render(),
            ]
        }),
        ("Input", "multiple", |ui| {
            [
                ui.input("n", "N").file(".pdf").render(),
                ui.input("n", "N").file(".pdf").multiple().render(),
            ]
        }),
        ("Input", "required", |ui| {
            [
                ui.input("n", "N").render(),
                ui.input("n", "N").required().render(),
            ]
        }),
        ("Input", "maxlength", |ui| {
            [
                ui.input("n", "N").render(),
                ui.input("n", "N").maxlength(8).render(),
            ]
        }),
        ("Input", "value", |ui| {
            [
                ui.input("n", "N").render(),
                ui.input("n", "N").value("Ada").render(),
            ]
        }),
        ("Input", "checked", |ui| {
            [
                ui.checkbox("n", "N").render(),
                ui.checkbox("n", "N").checked(true).render(),
            ]
        }),
        ("Input", "placeholder", |ui| {
            [
                ui.input("n", "N").render(),
                ui.input("n", "N").placeholder("Ada").render(),
            ]
        }),
        ("Input", "search", |ui| {
            [
                ui.input("n", "N").render(),
                ui.input("n", "N").search().render(),
            ]
        }),
        ("Input", "list", |ui| {
            [
                ui.input("n", "N").render(),
                ui.input("n", "N").list("names").render(),
            ]
        }),
        ("Input", "autocomplete", |ui| {
            [
                ui.input("n", "N").render(),
                ui.input("n", "N").autocomplete("name").render(),
            ]
        }),
        ("Input", "autofocus", |ui| {
            [
                ui.input("n", "N").render(),
                ui.input("n", "N").autofocus().render(),
            ]
        }),
        ("Input", "inputmode", |ui| {
            [
                ui.input("n", "N").render(),
                ui.input("n", "N").inputmode("numeric").render(),
            ]
        }),
        ("Input", "step", |ui| {
            [
                ui.input("n", "N").number(0, 9).render(),
                ui.input("n", "N").number(0, 9).step(3).render(),
            ]
        }),
        ("Input", "aria_controls", |ui| {
            [
                ui.input("n", "N").render(),
                ui.input("n", "N").aria_controls("results").render(),
            ]
        }),
        ("Input", "form", |ui| {
            [
                ui.input("n", "N").render(),
                ui.input("n", "N").form("f").render(),
            ]
        }),
        ("Input", "class", |ui| {
            [
                ui.input("n", "N").render(),
                ui.input("n", "N").class("x").render(),
            ]
        }),
        ("Input", "id", |ui| {
            [
                ui.input("n", "N").render(),
                ui.input("n", "N").id("name").render(),
            ]
        }),
        ("RadioGroup", "value", |ui| {
            [
                ui.radio_group("p", "P").option("a", "A").render(),
                ui.radio_group("p", "P")
                    .option("a", "A")
                    .value("a")
                    .render(),
            ]
        }),
        ("RadioGroup", "required", |ui| {
            [
                ui.radio_group("p", "P").option("a", "A").render(),
                ui.radio_group("p", "P")
                    .option("a", "A")
                    .required()
                    .render(),
            ]
        }),
        ("Marquee", "duration", |ui| {
            [
                ui.marquee("M").text("a").render(),
                ui.marquee("M").text("a").duration(20).render(),
            ]
        }),
        ("Meter", "low", |ui| {
            [
                ui.meter(5, 0, 10).render(),
                ui.meter(5, 0, 10).low(2).render(),
            ]
        }),
        ("Meter", "high", |ui| {
            [
                ui.meter(5, 0, 10).render(),
                ui.meter(5, 0, 10).high(8).render(),
            ]
        }),
        ("Meter", "optimum", |ui| {
            [
                ui.meter(5, 0, 10).render(),
                ui.meter(5, 0, 10).optimum(0).render(),
            ]
        }),
        ("Meter", "id", |ui| {
            [
                ui.meter(5, 0, 10).render(),
                ui.meter(5, 0, 10).id("disk").render(),
            ]
        }),
        ("Palette", "accesskey", |ui| {
            [
                ui.palette("/s").render(),
                ui.palette("/s").accesskey("p").render(),
            ]
        }),
        ("Palette", "id", |ui| {
            [
                ui.palette("/s").render(),
                ui.palette("/s").id("commands").render(),
            ]
        }),
        ("Menu", "action", |ui| {
            [
                ui.menu("M").link("A", "/a").render(),
                ui.menu("M").link("A", "/a").action("Go", "/go").render(),
            ]
        }),
        ("Menu", "disabled", |ui| {
            [
                ui.menu("M").action("Go", "/go").render(),
                ui.menu("M").action("Go", "/go").disabled().render(),
            ]
        }),
        ("Menu", "id", |ui| {
            [ui.menu("M").render(), ui.menu("M").id("more").render()]
        }),
        ("Progress", "id", |ui| {
            [
                ui.progress(1, 2).render(),
                ui.progress(1, 2).id("upload").render(),
            ]
        }),
        ("Range", "value", |ui| {
            [
                ui.range("v", "V").render(),
                ui.range("v", "V").value(70).render(),
            ]
        }),
        ("Range", "min", |ui| {
            [
                ui.range("v", "V").render(),
                ui.range("v", "V").min(10).render(),
            ]
        }),
        ("Range", "max", |ui| {
            [
                ui.range("v", "V").render(),
                ui.range("v", "V").max(50).render(),
            ]
        }),
        ("Range", "step", |ui| {
            [
                ui.range("v", "V").render(),
                ui.range("v", "V").step(5).render(),
            ]
        }),
        ("Section", "label", |ui| {
            [
                ui.section().render(),
                ui.section().label("Pricing").render(),
            ]
        }),
        ("Stat", "href", |ui| {
            [
                ui.stat("A", "1").render(),
                ui.stat("A", "1").href("/a").render(),
            ]
        }),
        ("Menu", "checked", |ui| {
            [
                ui.menu("View").checkbox("Grid", "/v", "grid").render(),
                ui.menu("View")
                    .checkbox("Grid", "/v", "grid")
                    .checked(true)
                    .render(),
            ]
        }),
        ("Popover", "id", |ui| {
            [
                ui.popover("Share", html! { "x" }).render(),
                ui.popover("Share", html! { "x" }).id("share-2").render(),
            ]
        }),
        ("Tooltip", "id", |ui| {
            [
                ui.tooltip("Tip", html! { "x" }).render(),
                ui.tooltip("Tip", html! { "x" }).id("tip").render(),
            ]
        }),
        ("Upload", "accept", |ui| {
            [
                ui.upload("/u", "f").render(),
                ui.upload("/u", "f").accept("image/*").render(),
            ]
        }),
        ("Upload", "multiple", |ui| {
            [
                ui.upload("/u", "f").render(),
                ui.upload("/u", "f").multiple().render(),
            ]
        }),
        ("Upload", "href", |ui| {
            [
                ui.upload("/u", "f").file("a.txt", 1).render(),
                ui.upload("/u", "f")
                    .file("a.txt", 1)
                    .href("/a.txt")
                    .render(),
            ]
        }),
        ("ErrorPage", "home", |ui| {
            [
                ui.error_page(404).render(),
                ui.error_page(404).home("/dashboard").render(),
            ]
        }),
        ("RecordPage", "edit", |ui| {
            [
                ui.record_page("R").render(),
                ui.record_page("R").edit("/r/edit").render(),
            ]
        }),
        ("RecordPage", "delete", |ui| {
            [
                ui.record_page("R").render(),
                ui.record_page("R").delete("/r/delete").render(),
            ]
        }),
        ("RecordPage", "back", |ui| {
            [
                ui.record_page("R").render(),
                ui.record_page("R").back("/rs").render(),
            ]
        }),
        ("ContextMenu", "action", |ui| {
            [
                ui.context_menu("C").link("A", "/a").render(),
                ui.context_menu("C")
                    .link("A", "/a")
                    .action("Go", "/go")
                    .render(),
            ]
        }),
        ("ContextMenu", "disabled", |ui| {
            [
                ui.context_menu("C").action("Go", "/go").render(),
                ui.context_menu("C").action("Go", "/go").disabled().render(),
            ]
        }),
        ("ContextMenu", "id", |ui| {
            [
                ui.context_menu("C").render(),
                ui.context_menu("C").id("more").render(),
            ]
        }),
        ("InputOtp", "length", |ui| {
            [
                ui.input_otp("code", "Code").render(),
                ui.input_otp("code", "Code").length(4).render(),
            ]
        }),
        ("NavMenu", "link", |ui| {
            [
                ui.nav_menu("Main").render(),
                ui.nav_menu("Main").link("A", "/a").render(),
            ]
        }),
        ("NavMenu", "id", |ui| {
            [
                ui.nav_menu("Main").panel("P", [("A", "/a")]).render(),
                ui.nav_menu("Main")
                    .panel("P", [("A", "/a")])
                    .id("top")
                    .render(),
            ]
        }),
        ("Sidebar", "link", |ui| {
            [
                ui.sidebar("Mail").render(),
                ui.sidebar("Mail").link("A", "/a").render(),
            ]
        }),
        ("Sidebar", "id", |ui| {
            [
                ui.sidebar("Mail").group("G").link("A", "/a").render(),
                ui.sidebar("Mail")
                    .group("G")
                    .link("A", "/a")
                    .id("side")
                    .render(),
            ]
        }),
        ("ToggleGroup", "option", |ui| {
            [
                ui.toggle_group("a", "A").render(),
                ui.toggle_group("a", "A").option("l", "Left").render(),
            ]
        }),
        ("ToggleGroup", "value", |ui| {
            [
                ui.toggle_group("a", "A").option("l", "Left").render(),
                ui.toggle_group("a", "A")
                    .option("l", "Left")
                    .value("l")
                    .render(),
            ]
        }),
        ("ToggleGroup", "multiple", |ui| {
            [
                ui.toggle_group("a", "A").option("l", "Left").render(),
                ui.toggle_group("a", "A")
                    .option("l", "Left")
                    .multiple()
                    .render(),
            ]
        }),
        ("Color", "disabled", |ui| {
            [
                ui.color("c", "C").render(),
                ui.color("c", "C").disabled().render(),
            ]
        }),
        ("Counter", "disabled", |ui| {
            [
                ui.counter("/c", 3).render(),
                ui.counter("/c", 3).disabled().render(),
            ]
        }),
        ("Form", "disabled", |ui| {
            [
                ui.form("/f").text("n", "N").render(),
                ui.form("/f").text("n", "N").disabled().render(),
            ]
        }),
        ("Form", "autocomplete", |ui| {
            [
                ui.form("/f").text("n", "N").render(),
                ui.form("/f").text("n", "N").autocomplete("name").render(),
            ]
        }),
        ("Input", "tel", |ui| {
            [
                ui.input("n", "N").render(),
                ui.input("n", "N").tel().render(),
            ]
        }),
        ("Input", "url", |ui| {
            [
                ui.input("n", "N").render(),
                ui.input("n", "N").url().render(),
            ]
        }),
        ("Input", "disabled", |ui| {
            [
                ui.input("n", "N").render(),
                ui.input("n", "N").disabled().render(),
            ]
        }),
        ("RadioGroup", "disabled", |ui| {
            [
                ui.radio_group("r", "R").option("a", "A").render(),
                ui.radio_group("r", "R")
                    .option("a", "A")
                    .disabled()
                    .render(),
            ]
        }),
        ("Range", "disabled", |ui| {
            [
                ui.range("v", "V").render(),
                ui.range("v", "V").disabled().render(),
            ]
        }),
        ("Select", "required", |ui| {
            [
                ui.select("s", "S").options([("a", "A")]).render(),
                ui.select("s", "S")
                    .options([("a", "A")])
                    .required()
                    .render(),
            ]
        }),
        ("Select", "disabled", |ui| {
            [
                ui.select("s", "S").options([("a", "A")]).render(),
                ui.select("s", "S")
                    .options([("a", "A")])
                    .disabled()
                    .render(),
            ]
        }),
        ("Select", "multiple", |ui| {
            [
                ui.select("s", "S").options([("a", "A")]).render(),
                ui.select("s", "S")
                    .options([("a", "A")])
                    .multiple()
                    .render(),
            ]
        }),
        ("SelectOption", "disabled", |ui| {
            [
                (ui.select("s", "S"))
                    .options([select::SelectOption::new("a", "A")])
                    .render(),
                (ui.select("s", "S"))
                    .options([select::SelectOption::new("a", "A").disabled()])
                    .render(),
            ]
        }),
        ("Range", "id", |ui| {
            [
                ui.range("v", "V").render(),
                ui.range("v", "V").id("x").render(),
            ]
        }),
        ("InputOtp", "id", |ui| {
            [
                ui.input_otp("c", "C").render(),
                ui.input_otp("c", "C").id("x").render(),
            ]
        }),
        ("Combobox", "disabled", |ui| {
            [
                ui.combobox("q", "/s").render(),
                ui.combobox("q", "/s").disabled().render(),
            ]
        }),
        ("DatePicker", "disabled", |ui| {
            [
                ui.date_picker("d", "D").render(),
                ui.date_picker("d", "D").disabled().render(),
            ]
        }),
        ("ToggleGroup", "disabled", |ui| {
            [
                ui.toggle_group("a", "A").option("l", "Left").render(),
                ui.toggle_group("a", "A")
                    .option("l", "Left")
                    .disabled()
                    .render(),
            ]
        }),
        ("CheckboxGroup", "disabled", |ui| {
            [
                ui.checkbox_group("c", "C").option("a", "A").render(),
                ui.checkbox_group("c", "C")
                    .option("a", "A")
                    .disabled()
                    .render(),
            ]
        }),
        ("CheckboxGroup", "value", |ui| {
            [
                ui.checkbox_group("c", "C").option("a", "A").render(),
                ui.checkbox_group("c", "C")
                    .option("a", "A")
                    .value(&["a"])
                    .render(),
            ]
        }),
        ("CheckboxGroup", "id", |ui| {
            [
                ui.checkbox_group("c", "C").option("a", "A").render(),
                ui.checkbox_group("c", "C")
                    .option("a", "A")
                    .id("x")
                    .render(),
            ]
        }),
        ("Number", "min", |ui| {
            [
                ui.number("n", "N").render(),
                ui.number("n", "N").min(0).render(),
            ]
        }),
        ("Number", "max", |ui| {
            [
                ui.number("n", "N").render(),
                ui.number("n", "N").max(9).render(),
            ]
        }),
        ("Number", "step", |ui| {
            [
                ui.number("n", "N").render(),
                ui.number("n", "N").step(0.5).render(),
            ]
        }),
        ("Number", "value", |ui| {
            [
                ui.number("n", "N").render(),
                ui.number("n", "N").value(2).render(),
            ]
        }),
        ("Number", "required", |ui| {
            [
                ui.number("n", "N").render(),
                ui.number("n", "N").required().render(),
            ]
        }),
        ("Number", "disabled", |ui| {
            [
                ui.number("n", "N").render(),
                ui.number("n", "N").disabled().render(),
            ]
        }),
        ("Number", "placeholder", |ui| {
            [
                ui.number("n", "N").render(),
                ui.number("n", "N").placeholder("1").render(),
            ]
        }),
        ("Number", "id", |ui| {
            [
                ui.number("n", "N").render(),
                ui.number("n", "N").id("x").render(),
            ]
        }),
        ("Search", "action", |ui| {
            [
                ui.search("q", "Q").render(),
                ui.search("q", "Q").action("/find").render(),
            ]
        }),
        ("Search", "placeholder", |ui| {
            [
                ui.search("q", "Q").render(),
                ui.search("q", "Q").placeholder("Find").render(),
            ]
        }),
        ("Search", "results_id", |ui| {
            [
                ui.search("q", "Q").render(),
                ui.search("q", "Q").results_id("r").render(),
            ]
        }),
        ("Search", "id", |ui| {
            [
                ui.search("q", "Q").render(),
                ui.search("q", "Q").id("x").render(),
            ]
        }),
        ("Rating", "value", |ui| {
            [
                ui.rating("r", "R").render(),
                ui.rating("r", "R").value(3).render(),
            ]
        }),
        ("Rating", "required", |ui| {
            [
                ui.rating("r", "R").render(),
                ui.rating("r", "R").required().render(),
            ]
        }),
        ("Rating", "disabled", |ui| {
            [
                ui.rating("r", "R").render(),
                ui.rating("r", "R").disabled().render(),
            ]
        }),
        ("Rating", "id", |ui| {
            [
                ui.rating("r", "R").render(),
                ui.rating("r", "R").id("x").render(),
            ]
        }),
        ("TagInput", "action", |ui| {
            [
                ui.tag_input("t", "T").render(),
                ui.tag_input("t", "T").action("/e").render(),
            ]
        }),
        ("TagInput", "placeholder", |ui| {
            [
                ui.tag_input("t", "T").render(),
                ui.tag_input("t", "T").placeholder("Add").render(),
            ]
        }),
        ("TagInput", "id", |ui| {
            [
                ui.tag_input("t", "T").render(),
                ui.tag_input("t", "T").id("x").render(),
            ]
        }),
    ];

    /// Whether some tag in `html` carries the attribute `name`.
    fn has_attribute(html: &str, name: &str) -> bool {
        let needle = format!(" {name}");
        html.split('<').skip(1).any(|tag| {
            let tag = tag.split('>').next().unwrap_or("");
            tag.match_indices(&needle).any(|(at, found)| {
                matches!(
                    tag[at + found.len()..].chars().next(),
                    None | Some('=' | ' ' | '/')
                )
            })
        })
    }

    /// Each setter whose `PROPS` entry names an HTML attribute sets that attribute, under
    /// every capability and under none: the output changes and some tag carries it. Every
    /// such setter needs an entry in [`ATTR_EXAMPLES`].
    #[test]
    fn every_attr_prop_sets_its_attribute() {
        let mut wrong = Vec::new();
        for component in props() {
            for prop in component.props.iter().filter(|p| !p.attr.is_empty()) {
                let name = format!("{}::{}", component.builder, prop.name);
                let Some((.., example)) = ATTR_EXAMPLES
                    .iter()
                    .find(|(b, n, _)| *b == component.builder && *n == prop.name)
                else {
                    wrong.push(format!("{name}: no entry in ATTR_EXAMPLES"));
                    continue;
                };
                for caps in [Caps::all(), Caps::NONE] {
                    let [without, with] = example(&Ui::from(caps)).map(Markup::into_string);
                    if without == with {
                        wrong.push(format!("{name} changes nothing (caps {:?})", caps.names()));
                    } else if !has_attribute(&with, prop.attr) {
                        wrong.push(format!(
                            "{name} sets no `{}` (caps {:?})",
                            prop.attr,
                            caps.names()
                        ));
                    }
                }
            }
        }
        for (builder, name, _) in ATTR_EXAMPLES {
            let listed = props().iter().any(|c| {
                c.builder == *builder
                    && c.props
                        .iter()
                        .any(|p| p.name == *name && !p.attr.is_empty())
            });
            if !listed {
                wrong.push(format!(
                    "{builder}::{name}: in ATTR_EXAMPLES, but PROPS gives it no attr"
                ));
            }
        }
        assert!(wrong.is_empty(), "{}", wrong.join("\n"));
    }
}
