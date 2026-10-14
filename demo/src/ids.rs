//! One component drawn twice on a page needs two sets of ids: the index shows previews that
//! render the same field, the props playground draws the stage's component again, and the
//! "Both themes" stage draws the whole stage a second time. [`scope`] gives a copy its own
//! ids by prefixing them, along with every attribute in the copy that points at one, so
//! labels, descriptions and invokers still find their element.

use std::collections::HashSet;

/// Attributes whose value is an id, or a space-separated list of ids, in the same document.
const REFS: [&str; 14] = [
    "for",
    "list",
    "form",
    "commandfor",
    "popovertarget",
    "aria-labelledby",
    "aria-describedby",
    "aria-controls",
    "aria-owns",
    "aria-details",
    "aria-errormessage",
    "aria-activedescendant",
    "headers",
    "anchor",
];

/// Every `id="…"` in `html`, in order.
pub(crate) fn ids(html: &str) -> Vec<&str> {
    attributes(html)
        .filter(|(name, ..)| *name == "id")
        .map(|(_, value, _)| value)
        .collect()
}

/// `html` with each id that `rename` accepts prefixed by `prefix`, and every reference to one
/// (`for`, `aria-*`, `commandfor`, `href="#…"`, the script's `data-lui-target="#…"` and the
/// rest of [`REFS`]) prefixed to match.
/// Maud escapes `"` in text and attribute values, so every `name="` found is an attribute.
pub(crate) fn scope(html: &str, prefix: &str, rename: impl Fn(&str) -> bool) -> String {
    let own: HashSet<&str> = ids(html).into_iter().filter(|id| rename(id)).collect();
    if own.is_empty() {
        return html.to_string();
    }
    let mut out = String::with_capacity(html.len() + own.len() * prefix.len() * 2);
    let mut at = 0;
    for (name, value, start) in attributes(html) {
        let renamed = match name {
            "id" if own.contains(value) => format!("{prefix}{value}"),
            "href" | "data-lui-target" | "data-lui-indicator" => match value.strip_prefix('#') {
                Some(id) if own.contains(id) => format!("#{prefix}{id}"),
                _ => continue,
            },
            n if REFS.contains(&n) && value.split(' ').any(|v| own.contains(v)) => value
                .split(' ')
                .map(|v| {
                    if own.contains(v) {
                        format!("{prefix}{v}")
                    } else {
                        v.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join(" "),
            _ => continue,
        };
        out.push_str(&html[at..start]);
        out.push_str(&renamed);
        at = start + value.len();
    }
    out.push_str(&html[at..]);
    out
}

/// Each `name="value"` attribute in `html`: its name, its value and where the value starts.
fn attributes(html: &str) -> impl Iterator<Item = (&str, &str, usize)> {
    html.match_indices("=\"").filter_map(move |(i, _)| {
        let before = &html[..i];
        let name_start = before
            .rfind(|c: char| c.is_whitespace())
            .map_or(0, |s| s + 1);
        let name = &before[name_start..];
        if name.is_empty() || !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            return None;
        }
        let start = i + 2;
        let end = html[start..].find('"')? + start;
        Some((name, &html[start..end], start))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_copy_gets_its_own_ids_and_its_references_follow() {
        let html = r##"<label for="f-a">A</label><input id="f-a" aria-describedby="f-a-help other"><p id="f-a-help">h</p><a href="#f-a">x</a><a href="#top">t</a><button commandfor="d">o</button>"##;
        let scoped = scope(html, "two-", |_| true);
        assert_eq!(
            scoped,
            r##"<label for="two-f-a">A</label><input id="two-f-a" aria-describedby="two-f-a-help other"><p id="two-f-a-help">h</p><a href="#two-f-a">x</a><a href="#top">t</a><button commandfor="d">o</button>"##
        );
        let only_one = scope(html, "two-", |id| id == "f-a-help");
        assert!(only_one.contains(r#"<label for="f-a">"#));
        assert!(only_one.contains(r#"aria-describedby="two-f-a-help other""#));
        assert_eq!(ids(&only_one), ["f-a", "two-f-a-help"]);
    }
}
