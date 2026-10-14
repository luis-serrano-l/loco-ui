//! A component page's reference sections, from the library itself, so they cannot drift from
//! it: "When to use" from the index entry, "Platform" from `spec::SPECS`, "Accessibility",
//! "Without script" and "Fallback" from the `//!` header of each module the page's code
//! calls (its source is included at compile time, as `code.rs` includes the routes), and
//! "Anatomy" from the `lui-<component>-<part>` classes in the module's `CSS`, each with the
//! tokens it reads. "Keyboard" lists the keys a module answers to once `props()` carries
//! them; until then it is left out.

use crate::site::Entry;
use loco_ui::props::Component;
use loco_ui::spec::{NeedsJs, SPECS};
use maud::{Markup, html};
use std::collections::HashMap;
use std::sync::LazyLock;

/// Every module's source, for its doc header and its CSS: each module `props()` or
/// `spec::SPECS` names (a test keeps the list complete).
pub(crate) const SOURCES: &[(&str, &str)] = &[
    ("accordion", include_str!("../../loco-ui/src/accordion.rs")),
    ("alert", include_str!("../../loco-ui/src/alert.rs")),
    ("avatar", include_str!("../../loco-ui/src/avatar.rs")),
    ("badge", include_str!("../../loco-ui/src/badge.rs")),
    (
        "blocks/app_shell",
        include_str!("../../loco-ui/src/blocks/app_shell.rs"),
    ),
    (
        "blocks/auth_page",
        include_str!("../../loco-ui/src/blocks/auth_page.rs"),
    ),
    (
        "blocks/dashboard_page",
        include_str!("../../loco-ui/src/blocks/dashboard_page.rs"),
    ),
    (
        "blocks/error_page",
        include_str!("../../loco-ui/src/blocks/error_page.rs"),
    ),
    (
        "blocks/record_page",
        include_str!("../../loco-ui/src/blocks/record_page.rs"),
    ),
    (
        "blocks/settings_page",
        include_str!("../../loco-ui/src/blocks/settings_page.rs"),
    ),
    (
        "breadcrumbs",
        include_str!("../../loco-ui/src/breadcrumbs.rs"),
    ),
    ("button", include_str!("../../loco-ui/src/button.rs")),
    ("calendar", include_str!("../../loco-ui/src/calendar.rs")),
    ("caps", include_str!("../../loco-ui-caps/src/lib.rs")),
    ("card", include_str!("../../loco-ui/src/card.rs")),
    ("chart", include_str!("../../loco-ui/src/chart.rs")),
    ("cluster", include_str!("../../loco-ui/src/cluster.rs")),
    ("color", include_str!("../../loco-ui/src/color.rs")),
    ("combobox", include_str!("../../loco-ui/src/combobox.rs")),
    (
        "context_menu",
        include_str!("../../loco-ui/src/context_menu.rs"),
    ),
    ("counter", include_str!("../../loco-ui/src/counter.rs")),
    (
        "date_picker",
        include_str!("../../loco-ui/src/date_picker.rs"),
    ),
    (
        "description_list",
        include_str!("../../loco-ui/src/description_list.rs"),
    ),
    ("dialog", include_str!("../../loco-ui/src/dialog.rs")),
    ("drawer", include_str!("../../loco-ui/src/drawer.rs")),
    (
        "empty_state",
        include_str!("../../loco-ui/src/empty_state.rs"),
    ),
    ("enhance", include_str!("../../loco-ui/src/enhance.rs")),
    (
        "error_summary",
        include_str!("../../loco-ui/src/error_summary.rs"),
    ),
    ("flash", include_str!("../../loco-ui/src/flash.rs")),
    ("form", include_str!("../../loco-ui/src/form.rs")),
    ("grid", include_str!("../../loco-ui/src/grid.rs")),
    ("icon", include_str!("../../loco-ui/src/icon.rs")),
    ("input", include_str!("../../loco-ui/src/input.rs")),
    ("input_otp", include_str!("../../loco-ui/src/input_otp.rs")),
    ("kanban", include_str!("../../loco-ui/src/kanban.rs")),
    ("layout", include_str!("../../loco-ui/src/layout.rs")),
    ("marquee", include_str!("../../loco-ui/src/marquee.rs")),
    ("meter", include_str!("../../loco-ui/src/meter.rs")),
    ("nav_menu", include_str!("../../loco-ui/src/nav_menu.rs")),
    (
        "paged_table",
        include_str!("../../loco-ui/src/paged_table.rs"),
    ),
    ("pager", include_str!("../../loco-ui/src/pager.rs")),
    ("palette", include_str!("../../loco-ui/src/palette.rs")),
    ("popover", include_str!("../../loco-ui/src/popover.rs")),
    ("progress", include_str!("../../loco-ui/src/progress.rs")),
    ("range", include_str!("../../loco-ui/src/range.rs")),
    ("select", include_str!("../../loco-ui/src/select.rs")),
    ("separator", include_str!("../../loco-ui/src/separator.rs")),
    ("sidebar", include_str!("../../loco-ui/src/sidebar.rs")),
    ("skeleton", include_str!("../../loco-ui/src/skeleton.rs")),
    ("sortable", include_str!("../../loco-ui/src/sortable.rs")),
    ("split", include_str!("../../loco-ui/src/split.rs")),
    ("stack", include_str!("../../loco-ui/src/stack.rs")),
    ("stat", include_str!("../../loco-ui/src/stat.rs")),
    ("state", include_str!("../../loco-ui/src/state.rs")),
    ("stream", include_str!("../../loco-ui/src/stream.rs")),
    ("table", include_str!("../../loco-ui/src/table.rs")),
    ("tabs", include_str!("../../loco-ui/src/tabs.rs")),
    ("theme", include_str!("../../loco-ui/src/theme.rs")),
    ("toast", include_str!("../../loco-ui/src/toast.rs")),
    (
        "toggle_group",
        include_str!("../../loco-ui/src/toggle_group.rs"),
    ),
    ("tooltip", include_str!("../../loco-ui/src/tooltip.rs")),
    ("upload", include_str!("../../loco-ui/src/upload.rs")),
    ("wizard", include_str!("../../loco-ui/src/wizard.rs")),
];

/// The layouts, which arrange a page's components without being its subject.
const LAYOUTS: [&str; 5] = ["layout", "stack", "cluster", "grid", "split"];

/// The primitives most components are built from: on a page about something else they are
/// parts, not the subject, so their sections stay on their own pages.
const PARTS: [&str; 4] = ["icon", "button", "badge", "input"];

/// The modules a page documents: those of the builders its code calls, in order, without the
/// layouts and primitives it only uses as parts (a page of primitives keeps those; the
/// layout page keeps its layouts).
pub(crate) fn modules(builders: &[&Component]) -> Vec<&'static str> {
    let mut all: Vec<&'static str> = Vec::new();
    for b in builders {
        if !all.contains(&b.module) {
            all.push(b.module);
        }
    }
    let subject = |m: &&str| !LAYOUTS.contains(m) && !PARTS.contains(m);
    if all.iter().any(subject) {
        all.retain(subject);
    } else if all.iter().any(|m| !LAYOUTS.contains(m)) {
        all.retain(|m| !LAYOUTS.contains(m));
    }
    all
}

/// A module's source, or `""` when [`SOURCES`] lacks it.
fn source(module: &str) -> &'static str {
    SOURCES.iter().find(|s| s.0 == module).map_or("", |s| s.1)
}

/// The paragraph after `**{label}:**` in a module's `//!` header, up to the next blank line
/// or the next bold label: `header_section(source, "Fallback")`.
pub(crate) fn header_section(source: &str, label: &str) -> Option<String> {
    let header = source
        .lines()
        .take_while(|l| l.starts_with("//!"))
        .map(|l| l.trim_start_matches("//!").trim());
    let open = format!("**{label}:**");
    let mut lines = header.skip_while(|l| !l.starts_with(&open));
    let first = lines.next()?.trim_start_matches(&open).trim().to_string();
    let rest = lines.take_while(|l| !l.is_empty() && !l.starts_with("**") && !l.starts_with("```"));
    let text = std::iter::once(first)
        .chain(rest.map(str::to_string))
        .collect::<Vec<_>>()
        .join(" ");
    // Capitalised: a header writes "**Fallback:** when …" as one sentence with its label.
    let mut chars = text.trim().chars();
    let first = chars.next()?;
    Some(first.to_uppercase().chain(chars).collect())
}

/// The keys a module answers to, as `(keys, what they do)`. Empty until each component
/// carries its `KEYS` const and `props()` lists it (ROADMAP M42, the last commit before
/// merge); the "Keyboard" section shows as soon as one has any.
pub(crate) fn keys(_module: &str) -> &'static [(&'static str, &'static str)] {
    &[]
}

/// A module's `pub const CSS`, as written in its source.
fn css(source: &str) -> &str {
    let open = "pub const CSS: &str = r#\"";
    let Some(start) = source.find(open).map(|i| i + open.len()) else {
        return "";
    };
    let rest = &source[start..];
    &rest[..rest.find("\"#;").unwrap_or(rest.len())]
}

/// Each class a stylesheet styles (the last `lui-*` class of each selector), in the order
/// they first appear, with the `--lui-*` tokens its rules read.
pub(crate) fn styled(css: &str) -> Vec<(String, Vec<String>)> {
    let mut found: Vec<(String, Vec<String>)> = Vec::new();
    let mut record = |selector: &str, declarations: &str| {
        let tokens: Vec<String> = declarations
            .match_indices("var(--lui-")
            .map(|(i, _)| {
                let name = &declarations[i + 4..];
                let end = name
                    .find(|c: char| !(c.is_ascii_alphanumeric() || c == '-'))
                    .unwrap_or(name.len());
                name[..end].to_string()
            })
            .collect();
        for one in selector.split(',') {
            let Some(at) = one.rfind(".lui-") else {
                continue;
            };
            let class = &one[at + 1..];
            let end = class
                .find(|c: char| !(c.is_ascii_alphanumeric() || c == '-'))
                .unwrap_or(class.len());
            let class = &class[..end];
            let entry = match found.iter_mut().position(|f| f.0 == class) {
                Some(i) => &mut found[i],
                None => {
                    found.push((class.to_string(), Vec::new()));
                    found.last_mut().expect("just pushed")
                }
            };
            for t in &tokens {
                if !entry.1.contains(t) {
                    entry.1.push(t.clone());
                }
            }
        }
    };
    // Comments out, then one pass over the braces: the text before a `{` (after the last `;`)
    // is a selector or an at-rule, the text before a `}` the declarations of the innermost
    // rule. Declarations before a nested rule belong to its parent.
    let mut text = String::new();
    let mut rest = css;
    while let Some(i) = rest.find("/*") {
        text.push_str(&rest[..i]);
        rest = rest[i..].find("*/").map_or("", |j| &rest[i + j + 2..]);
    }
    text.push_str(rest);
    let mut stack: Vec<String> = Vec::new();
    let mut buffer = String::new();
    for ch in text.chars() {
        match ch {
            '{' => {
                let (declarations, prelude) = buffer.rsplit_once(';').unwrap_or(("", &buffer));
                if let Some(parent) = stack.last().filter(|s| !s.starts_with('@')) {
                    record(parent, declarations);
                }
                let prelude = prelude.trim();
                let selector = match stack.last() {
                    Some(parent) if prelude.contains('&') => prelude.replace('&', parent),
                    _ => prelude.to_string(),
                };
                stack.push(selector);
                buffer.clear();
            }
            '}' => {
                if let Some(selector) = stack.pop().filter(|s| !s.starts_with('@')) {
                    record(&selector, &buffer);
                }
                buffer.clear();
            }
            ';' if stack.last().is_none_or(|s| s.starts_with('@')) => buffer.clear(),
            c => buffer.push(c),
        }
    }
    found
}

/// For each class, the module whose CSS styles it most often: its home, where its anatomy
/// lists it (a dialog's CSS restyling `.lui-button` does not make the button a dialog part).
static HOMES: LazyLock<HashMap<String, &'static str>> = LazyLock::new(|| {
    let mut counts: HashMap<String, (&str, usize)> = HashMap::new();
    for (module, src) in SOURCES {
        let mut mine: HashMap<String, usize> = HashMap::new();
        let css = css(src);
        for (class, _) in styled(css) {
            *mine.entry(class.clone()).or_default() += css.matches(&format!(".{class}")).count();
        }
        for (class, n) in mine {
            let best = counts.entry(class).or_insert((module, 0));
            if n > best.1 {
                *best = (module, n);
            }
        }
    }
    counts.into_iter().map(|(c, (m, _))| (c, m)).collect()
});

/// A module's parts: the classes its CSS styles that are its own, with the tokens each reads.
pub(crate) fn anatomy(module: &str) -> Vec<(String, Vec<String>)> {
    styled(css(source(module)))
        .into_iter()
        .filter(|(class, _)| HOMES.get(class).is_some_and(|m| *m == module))
        .collect()
}

/// What a module is called on the page: its spec name, else its builder's.
fn name(module: &str, builders: &[&Component]) -> String {
    SPECS
        .iter()
        .find(|s| s.module == module)
        .map(|s| s.name.to_string())
        .or_else(|| {
            builders
                .iter()
                .find(|b| b.module == module)
                .map(|b| b.builder.to_string())
        })
        .unwrap_or_else(|| module.to_string())
}

/// The sections under a component page's props: when to use it, then for each module its
/// code calls, the platform it needs, its accessibility, its keys, what it does without
/// script, its fallback and its parts.
pub(crate) fn sections(entry: &Entry, builders: &[&Component]) -> Markup {
    let modules = modules(builders);
    let several = modules.len() > 1;
    let per = |text: &dyn Fn(&str) -> Option<Markup>| -> Vec<(String, Markup)> {
        modules
            .iter()
            .filter_map(|m| text(m).map(|t| (name(m, builders), t)))
            .collect()
    };
    let paragraphs = |label: &'static str| {
        per(&move |m| header_section(source(m), label).map(|t| crate::docs::inline(&t)))
    };
    let accessibility = paragraphs("Accessibility");
    let without = per(&|m| {
        let src = source(m);
        let parts: Vec<String> = ["What it does not do without script", "Without script"]
            .iter()
            .filter_map(|l| header_section(src, l))
            .collect();
        (!parts.is_empty()).then(|| html! { @for p in &parts { (crate::docs::inline(p)) " " } })
    });
    let fallback = paragraphs("Fallback");
    let platform: Vec<_> = modules
        .iter()
        .filter_map(|m| SPECS.iter().find(|s| s.module == *m))
        .collect();
    let keyboard: Vec<_> = modules
        .iter()
        .map(|m| (name(m, builders), keys(m)))
        .filter(|(_, k)| !k.is_empty())
        .collect();
    let anatomy: Vec<_> = modules
        .iter()
        .map(|m| (name(m, builders), anatomy(m)))
        .filter(|(_, a)| !a.is_empty())
        .collect();
    let prose = |id: &str, title: &str, items: &[(String, Markup)]| {
        html! {
            @if !items.is_empty() {
                h2 id=(id) { (title) }
                @for (name, text) in items {
                    p { @if several { strong { (name) ": " } } (text) }
                }
            }
        }
    };
    html! {
        section class="lui-reference" {
            h2 id="when-to-use" { "When to use" }
            p { (crate::docs::inline(entry.when)) }
            @if !platform.is_empty() {
                h2 id="platform" { "Platform" }
                @for spec in &platform {
                    @if several { h3 { (spec.name) } }
                    p {
                        @match spec.needs_js {
                            NeedsJs::No => "Works without JavaScript.",
                            NeedsJs::Partial(what) => { "Works without JavaScript, except: " (what) "." },
                        }
                        " Its layout answers to " @match spec.responsive.as_str() { "-" => "nothing of its own", r => (r) } "."
                    }
                    @if !spec.features.is_empty() {
                        div class="lui-props-scroll" tabindex="0" role="region" aria-label={ (spec.name) " browser support" } {
                            table {
                                thead { tr { th { "Feature" } th { "Chrome" } th { "Firefox" } th { "Safari" } } }
                                tbody { @for f in spec.features { tr {
                                    td { code { (f.name) } }
                                    @for v in [f.baseline.chrome, f.baseline.firefox, f.baseline.safari] {
                                        td { @if v == "no" { span class="lui-no" { "no" } } @else { (v) } }
                                    }
                                } } }
                            }
                        }
                    }
                }
            }
            (prose("accessibility", "Accessibility", &accessibility))
            @if !keyboard.is_empty() {
                h2 id="keyboard" { "Keyboard" }
                @for (name, keys) in &keyboard {
                    @if several { h3 { (name) } }
                    table {
                        thead { tr { th { "Keys" } th { "What they do" } } }
                        tbody { @for (k, what) in keys.iter() { tr { td { kbd { (k) } } td { (what) } } } }
                    }
                }
            }
            (prose("without-script", "Without script", &without))
            (prose("fallback", "Fallback", &fallback))
            @if !anatomy.is_empty() {
                h2 id="anatomy" { "Anatomy" }
                p class="lui-note" { "The classes the component's CSS styles, and the " code { "--lui-*" } " tokens each one reads: what a theme or your own CSS can change." }
                @for (name, parts) in &anatomy {
                    @if several { h3 { (name) } }
                    div class="lui-props-scroll" tabindex="0" role="region" aria-label={ (name) " anatomy" } {
                        table {
                            thead { tr { th { "Class" } th { "Tokens it reads" } } }
                            tbody { @for (class, tokens) in parts { tr {
                                td { code { "." (class) } }
                                td { @for (i, t) in tokens.iter().enumerate() { @if i > 0 { " " } code { (t) } } }
                            } } }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each module `props()` or `spec::SPECS` names has its source here, so its page can show
    /// its header's sections and its anatomy.
    #[test]
    fn every_module_has_its_source() {
        let named =
            (loco_ui::props().iter().map(|c| c.module)).chain(SPECS.iter().map(|s| s.module));
        for module in named {
            assert!(
                SOURCES.iter().any(|s| s.0 == module),
                "{module}: add it to reference::SOURCES"
            );
        }
    }

    #[test]
    fn header_sections_and_anatomy_come_from_the_source() {
        let dialog = source("dialog");
        let fallback = header_section(dialog, "Fallback").unwrap();
        assert!(
            fallback.starts_with("When `Caps` lacks `Invokers`"),
            "{fallback}"
        );
        assert!(
            fallback.ends_with("Only one variant is ever in the markup."),
            "{fallback}"
        );
        assert!(
            header_section(dialog, "Accessibility")
                .unwrap()
                .contains("aria-labelledby")
        );
        assert_eq!(header_section(dialog, "Nothing like this"), None);
        let parts = anatomy("dialog");
        assert_eq!(parts[0].0, "lui-dialog", "the root first");
        assert!(
            parts.iter().all(|(c, _)| c.starts_with("lui-dialog")),
            "{parts:?}"
        );
        assert!(
            parts[0].1.iter().any(|t| t.starts_with("--lui-")),
            "{parts:?}"
        );
        let css = ".lui-a { color: var(--lui-fg); }\n@media (x) { .lui-b, .lui-a:hover { background: var(--lui-bg) } }\n.lui-c { gap: 1px; & .lui-c-d { color: var(--lui-muted); } }";
        assert_eq!(
            styled(css),
            [
                (
                    "lui-a".to_string(),
                    vec!["--lui-fg".to_string(), "--lui-bg".to_string()]
                ),
                ("lui-b".to_string(), vec!["--lui-bg".to_string()]),
                ("lui-c".to_string(), vec![]),
                ("lui-c-d".to_string(), vec!["--lui-muted".to_string()]),
            ]
        );
    }

    /// Every index entry says when to use it.
    #[test]
    fn every_component_says_when_to_use_it() {
        for c in crate::site::COMPONENTS {
            assert!(
                c.when.trim().len() > 20,
                "{}: write its `when` in site::COMPONENTS",
                c.href
            );
        }
    }
}
