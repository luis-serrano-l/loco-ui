//! The app's look, applied to every page by one layer in `App::after_routes`: a teal brand,
//! rounder corners, a wider main column for the app shell, and a reading column for notes.
//! Only `--lui-*` tokens, like the library's own CSS.

use loco_ui::layout::{Look, Scale, Tokens};
use maud::{Markup, html};

/// The seed the brand scale is derived from (`Scale::derive(SEED, &Scale::INDIGO)`).
pub const SEED: &str = "#0f766e";

/// `Scale::derive(SEED, &Scale::INDIGO)` written out, so the look can be a `static`; a test
/// checks it still matches.
pub const BRAND: Scale = Scale {
    light: [
        "#fdfdfd", "#f7faf9", "#edf4f3", "#e0edeb", "#d0e4e1", "#bcd9d5", "#a4c8c3", "#84b3ad",
        "#0f766e", "#006d65", "#22746d", "#173734",
    ],
    dark: [
        "#0e1615", "#111a19", "#132c29", "#143936", "#1c4641", "#25514d", "#2e5f5a", "#356f69",
        "#0f766e", "#31827a", "#91c1bb", "#d4e6e3",
    ],
};

pub static LOOK: Look = Look {
    tokens: Tokens {
        brand: BRAND,
        radius: "0.625rem",
        ..Tokens::DEFAULT
    },
    css: &[CSS],
    header: Some(no_header),
};

/// The app shell names the app, so pages carry no site header above it.
fn no_header() -> Markup {
    html! {}
}

const CSS: &str = r#"
main { max-width: 76rem; padding-top: var(--lui-space-6); }
.notes-bar { display: flex; flex-wrap: wrap; align-items: center; justify-content: space-between; gap: var(--lui-space-3); margin-bottom: var(--lui-space-6); }
.notes-bar h1 { margin: 0; }
.notes-search { display: flex; gap: var(--lui-space-2); align-items: center; flex: 1 1 16rem; max-width: 28rem; }
.notes-search > * { flex: 1; }
.notes-top { display: flex; justify-content: space-between; align-items: center; gap: var(--lui-space-3); margin-bottom: var(--lui-space-6); }
.notes-filters { display: flex; flex-wrap: wrap; align-items: end; gap: var(--lui-space-4); margin-bottom: var(--lui-space-6); }
.notes-filters > .lui-combobox { flex: 1 1 18rem; }
.notes-view { display: flex; align-items: end; gap: var(--lui-space-2); margin: 0; }
/* A note card: the whole card is its link; the title is the link's text. */
.notes-card { position: relative; }
.notes-card-body { display: grid; gap: var(--lui-space-3); }
.notes-card > .lui-card { height: 100%; box-sizing: border-box; transition: box-shadow var(--lui-duration-fast), translate var(--lui-duration-fast); }
.notes-card:hover > .lui-card { box-shadow: var(--lui-shadow-md); translate: 0 -1px; }
.notes-card h3 { margin: 0; font-size: 1rem; }
.notes-card h3 a { color: var(--lui-fg); text-decoration: none; }
.notes-card h3 a::after { content: ""; position: absolute; inset: 0; border-radius: inherit; }
.notes-card h3 a:focus-visible { outline: none; }
.notes-card:has(h3 a:focus-visible) > .lui-card { outline: 2px solid var(--lui-ring); outline-offset: 2px; }
.notes-excerpt { margin: 0; color: var(--lui-muted); font-size: 0.875rem; display: -webkit-box; -webkit-line-clamp: 3; -webkit-box-orient: vertical; overflow: hidden; }
.notes-meta { display: flex; flex-wrap: wrap; align-items: center; gap: var(--lui-space-2); color: var(--lui-muted); font-size: 0.8125rem; }
.notes-card .notes-meta .lui-badge { position: relative; z-index: 1; }
/* The reading view: a serif column about 68 characters wide. */
.notes-article { min-width: 0; }
.notes-article h1 { margin: 0 0 var(--lui-space-2); font-size: 2.25rem; line-height: 1.15; letter-spacing: -0.02em; }
.notes-prose { max-width: 68ch; margin-top: var(--lui-space-8); font-family: "Iowan Old Style", "Palatino Linotype", Charter, Georgia, serif; font-size: 1.125rem; line-height: 1.7; }
.notes-prose p { max-width: none; margin: 0 0 1.1em; }
.notes-actions { display: flex; flex-wrap: wrap; gap: var(--lui-space-2); align-items: center; }
.notes-side { position: sticky; top: var(--lui-space-6); }
.notes-landing { max-width: 44rem; margin: 10vh auto 0; text-align: center; }
.notes-landing h1 { font-size: clamp(2.25rem, 6vw, 3.5rem); line-height: 1.05; letter-spacing: -0.03em; margin: 0 0 var(--lui-space-4); }
.notes-landing p { margin: 0 auto var(--lui-space-8); color: var(--lui-muted); font-size: 1.125rem; }
.notes-landing .lui-cluster { justify-content: center; }
.notes-features { margin-top: var(--lui-space-16); text-align: start; }
.notes-section { margin-top: var(--lui-space-10); }
.notes-section h2 { font-size: 1rem; margin: 0 0 var(--lui-space-3); }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_brand_is_the_seed_derived() {
        let derived = Scale::derive(SEED, &Scale::INDIGO).unwrap();
        assert_eq!(BRAND.light.map(String::from), derived.light);
        assert_eq!(BRAND.dark.map(String::from), derived.dark);
    }
}
