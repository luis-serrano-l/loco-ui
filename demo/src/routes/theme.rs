//! The theme builder: one brand colour and one gray, each grown into a 12-step scale
//! (`layout::Scale::derive`) for light and dark, a radius ramp, a density and a font, sent
//! as a GET form so the theme is in the URL; a preview of a few components, their shadows,
//! radii and the primary gradient under those values in both schemes; every token with a
//! swatch per scheme; and `theme.css`, the tokens as a file to paste after the stylesheet.
//! No script: the form posts back and the page renders the new values.

use crate::site::page;
use axum::{
    Router,
    http::header,
    response::{IntoResponse, Response},
    routing::get,
};
use loco_ui::layout::{Density, Depth, Fonts, Palette, Radius, Scale, Tokens};
use loco_ui::prelude::*;
use std::borrow::Cow;

pub(crate) fn routes() -> Router {
    Router::new()
        .route("/theme", get(builder).post(crate::site::theme_submit))
        .route("/theme.css", get(download))
        .route("/modes", get(modes_page))
}

/// The page's live component, which the index shows too (`site::preview`).
pub(crate) const PREVIEWS: &[super::Preview] = &[
    ("/theme", |ui| pickers(ui, &chosen(ui), html! {})),
    ("/modes", modes),
];

/// Radix Colors' step 9 of a few brand scales, offered as swatches.
const BRANDS: [&str; 7] = [
    "#3e63dd", "#0090ff", "#12a594", "#46a758", "#f76b15", "#e93d82", "#6e56cf",
];
/// Radix Colors' step 9 of its grays: slate, gray, mauve, sage, olive, sand.
const GRAYS: [&str; 6] = [
    "#8b8d98", "#8d8d8d", "#8e8c99", "#868e8b", "#898e87", "#8d8d86",
];
/// The radius factors offered: Radix Themes' none, small, medium and large.
const FACTORS: [(&str, &str); 4] = [
    ("0", "None (0)"),
    ("0.75", "Small (0.75)"),
    ("1", "Medium (1)"),
    ("1.5", "Large (1.5)"),
];
const DENSITIES: [(&str, &str); 2] = [("comfortable", "Comfortable"), ("compact", "Compact")];
/// The body fonts offered: the system stacks only, so nothing is downloaded.
const FONTS: [(&str, &str); 3] = [
    ("sans", "System sans"),
    ("serif", "System serif"),
    ("mono", "System mono"),
];

/// The chosen theme, read from the query: a clicked swatch (`brand-preset`) wins over the
/// picker, and anything not offered is the default.
struct Chosen {
    brand: String,
    gray: String,
    radius: u32,
    factor: &'static str,
    density: Density,
    font: &'static str,
    tokens: Tokens,
}

fn chosen(ui: &Ui) -> Chosen {
    let hex = |v: &str| {
        v.len() == 7 && v.starts_with('#') && v[1..].chars().all(|c| c.is_ascii_hexdigit())
    };
    // The asked colour counts only once it derives a scale; the default is a constant that does.
    let pick = |name: &str, default: &str, like: &Scale| {
        let asked = ui
            .param(&format!("{name}-preset"))
            .or_else(|| ui.param(name))
            .filter(|v| hex(v))
            .map(str::to_lowercase);
        asked
            .into_iter()
            .chain([default.to_string()])
            .find_map(|value| Some((Scale::derive(&value, like)?, value)))
            .expect("the default is a #rrggbb colour")
    };
    let offered = |list: &[(&'static str, &str)], key: &str, default: &'static str| {
        let asked = ui.param(key).unwrap_or(default);
        list.iter()
            .map(|(v, _)| *v)
            .find(|v| *v == asked)
            .unwrap_or(default)
    };
    let (brand_scale, brand) = pick("brand", BRANDS[0], &Scale::INDIGO);
    let (gray_scale, gray) = pick("gray", GRAYS[0], &Scale::SLATE);
    let radius = ui
        .param("radius")
        .and_then(|r| r.parse().ok())
        .unwrap_or(8)
        .min(24);
    let factor = offered(&FACTORS, "factor", "1");
    let density = Density::parse(offered(&DENSITIES, "density", "comfortable"));
    let font = offered(&FONTS, "font", "sans");
    // White or near-black text on the solid step; with near-black text the gradient ends on
    // step 10, which keeps it readable.
    let on: Cow<'static, str> = Cow::Borrowed(brand_scale.on_solid());
    let gradient = if on == "#ffffff" {
        Depth::DEFAULT.gradient_primary
    } else {
        Cow::Borrowed(
            "linear-gradient(in oklch to bottom, var(--lui-brand-9), var(--lui-brand-10))",
        )
    };
    let sans = match font {
        "serif" => Fonts::DEFAULT.serif,
        "mono" => Fonts::DEFAULT.mono,
        _ => Fonts::DEFAULT.sans,
    };
    let tokens = Tokens {
        gray: gray_scale.into(),
        brand: brand_scale.into(),
        light: Palette {
            on_primary: on.clone(),
            ..Tokens::DEFAULT.light
        },
        dark: Palette {
            on_primary: on,
            ..Tokens::DEFAULT.dark
        },
        font: Fonts {
            sans,
            ..Fonts::DEFAULT
        },
        radius: Radius {
            base: Cow::Owned(format!("{radius}px")),
            factor: Cow::Borrowed(factor),
        },
        shadow: Depth {
            gradient_primary: gradient,
            ..Depth::DEFAULT
        },
        density,
        ..Tokens::DEFAULT
    };
    Chosen {
        brand,
        gray,
        radius,
        factor,
        density,
        font,
        tokens,
    }
}

/// The tokens as a stylesheet: `Tokens::css`, to paste after the stylesheet.
fn css(chosen: &Chosen) -> String {
    format!(
        "/* loco-ui theme from /theme: paste after the stylesheet (Page::css), or build the same\n   layout::Tokens and pass it to Page::tokens. */\n{}",
        chosen.tokens.css()
    )
}

/// The chosen tokens declared on one element, in one scheme: a role such as
/// `--lui-bg: var(--lui-gray-1)` resolves where it is declared (on `:root`), so the preview
/// declares the tokens again; `color-scheme` picks the side of each `light-dark()`. Type,
/// motion, effects and stacking are left out: the builder does not change them.
fn scoped(chosen: &Chosen, scheme: &str) -> String {
    const KEPT: [&str; 10] = [
        "text-", "leading-", "z-", "duration", "ease", "shimmer", "glow", "beam", "marquee",
        "overlay",
    ];
    let declarations = chosen.tokens.declarations();
    let mut out: String = (declarations.split(';'))
        .map(str::trim)
        .filter(|d| {
            (d.strip_prefix("--lui-")).is_some_and(|d| !KEPT.iter().any(|k| d.starts_with(k)))
        })
        .map(|d| format!("{d}; "))
        .collect();
    out.push_str("color-scheme: ");
    out.push_str(scheme);
    out
}

/// A few components under one scheme's values: the preview.
fn preview(ui: &Ui, dark: bool) -> Markup {
    let scheme = if dark { "dark" } else { "light" };
    html! {
        div class="lui-theme-preview" style={ "color-scheme: " (scheme) } {
            (ui.card().title("Invite a teammate").description("They get an email with a link.").body(html! {
                (ui.input(&format!("preview-{scheme}-email"), "Email").email().placeholder("ada@example.com"))
                (ui.cluster().body(html! {
                    (ui.button("Send invite").primary())
                    (ui.button("Cancel"))
                    (ui.badge("Beta").secondary())
                }))
                (ui.alert("Two seats left").danger().description("Upgrade to add more."))
            }))
            div class="lui-theme-depth" aria-label="Shadows xs to lg, and the primary gradient" role="img" {
                @for size in ["xs", "sm", "md", "lg"] { span style={ "box-shadow: var(--lui-shadow-" (size) "), var(--lui-highlight)" } { (size) } }
                span class="lui-theme-gradient" {}
            }
            div class="lui-theme-depth" aria-label="The radius ramp: none, 1, 2, 3 and full" role="img" {
                @for step in ["none", "1", "2", "3", "full"] { span class="lui-theme-corner" style={ "border-radius: var(--lui-radius-" (step) ")" } { (step) } }
            }
        }
    }
}

/// Every colour role of the chosen theme with its value and a swatch in each scheme. Each
/// row sets `--lui-swatch` to its role; a cell's `color-scheme` picks the side of its
/// `light-dark()`. The tokens are declared on the previews around it.
fn table(chosen: &Chosen) -> Markup {
    let (light, dark) = (chosen.tokens.light.roles(), chosen.tokens.dark.roles());
    html! {
        details class="lui-theme-tokens" {
            summary { "Colour roles" }
            div tabindex="0" role="region" aria-label="Colour roles" { table {
                thead { tr { th { "Token" } th { "Light" } th { "Dark" } } }
                tbody { @for ((name, l), (_, d)) in light.iter().zip(&dark) { tr style={ "--lui-swatch: var(--lui-" (name) ")" } {
                    td { code { "--lui-" (name) } }
                    td class="lui-theme-light" { i {} (l) }
                    td class="lui-theme-dark" { i {} (d) }
                } } }
            } }
        }
    }
}

/// The form that picks the theme, the previews in both schemes, and `extra` under them
/// (the colour roles on the builder's own page).
fn pickers(ui: &Ui, chosen: &Chosen, extra: Markup) -> Markup {
    let download = loco_ui::href(
        "/theme.css",
        [
            ("brand", chosen.brand.as_str()),
            ("gray", chosen.gray.as_str()),
            ("radius", &chosen.radius.to_string()),
            ("factor", chosen.factor),
            ("density", chosen.density.as_str()),
            ("font", chosen.font),
        ],
    );
    lui! {
        // code: /theme
        form method="get" action="/theme" class="lui-theme-builder" {
            fieldset {
                legend { "Scales" }
                div {
                    Color("brand", "Brand") value=(&chosen.brand) presets=(&BRANDS);
                    Color("gray", "Gray") value=(&chosen.gray) presets=(&GRAYS);
                }
            }
            fieldset {
                legend { "Shape and type" }
                div {
                    Range("radius", "Radius (px)") value=(i64::from(chosen.radius)) min=0 max=24;
                    Select("factor", "Radius ramp") value=(chosen.factor) options=(FACTORS);
                    Select("density", "Density") value=(chosen.density.as_str()) options=(DENSITIES);
                    Select("font", "Font") value=(chosen.font) options=(FONTS);
                }
            }
            Cluster {
                Button("Preview") primary;
                LinkButton("Download theme.css", &download);
                a href="/theme" { "Reset" }
            }
        }
        div class="lui-theme-previews" style=(scoped(chosen, "light dark")) {
            (preview(ui, false))
            (preview(ui, true))
            (extra)
        }
        // end code
    }
}

async fn builder(ui: Ui) -> Page {
    let chosen = chosen(&ui);
    let body = lui! {
        (pickers(&ui, &chosen, table(&chosen)))
    };
    page(&ui, "Theme builder", body)
}

/// The chosen tokens as a file.
async fn download(ui: Ui) -> Response {
    (
        [
            (header::CONTENT_TYPE, "text/css; charset=utf-8"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"theme.css\"",
            ),
        ],
        css(&chosen(&ui)),
    )
        .into_response()
}

/// The controls drawn with `appearance: none`, plus badges: what the forced-colours rules
/// keep visible.
fn stage(ui: &Ui, key: &str) -> Markup {
    let (check, switch, range) = (
        format!("{key}-check"),
        format!("{key}-switch"),
        format!("{key}-range"),
    );
    lui! {
        Stack gap=3 {
            Checkbox(&check, "Checked") checked=(true);
            Switch(&switch, "Switch on") checked=(true);
            Range(&range, "Volume") value=60;
            Progress(40, 100) label="Upload";
            Cluster { Badge("Draft"); Badge("Live") ok; }
        }
    }
}

/// The modes page: the same stage as drawn and under an emulated mode, `?forced=1` (forced
/// colours) or `?contrast=1` (more contrast), applied by a class since a media query cannot
/// be forced from the server. Print hides toasts, drawers and sidebars: the browser's print
/// preview shows it.
fn modes(ui: &Ui) -> Markup {
    let class = match (ui.param("forced"), ui.param("contrast")) {
        (Some("1"), _) => "lui-forced",
        (_, Some("1")) => "lui-contrast",
        _ => "lui-forced",
    };
    lui! {
        // code: /modes
        Cluster {
            a href="/modes?forced=1" { "Forced colours" }
            a href="/modes?contrast=1" { "More contrast" }
        }
        div class="lui-modes" {
            section { h2 { "As drawn" } (stage(ui, "plain")) }
            section class=(class) { h2 { "Under ." (class) } (stage(ui, "mode")) }
        }
        // end code
    }
}

async fn modes_page(ui: Ui) -> Page {
    page(&ui, "Modes", modes(&ui))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_colours_reach_the_css() {
        let ui = Ui::from_request(
            "/theme",
            "brand=%2312a594&gray=red;x&radius=99&factor=9",
            "",
        );
        let out = css(&chosen(&ui));
        assert!(out.contains("--lui-brand-9: #12a594;"), "{out}");
        assert!(
            out.contains("--lui-gray-9: #8b8d98;") && !out.contains("red;x"),
            "{out}"
        );
        assert!(
            out.contains("calc(24px * var(--lui-radius-factor))"),
            "{out}"
        );
        assert!(
            out.contains("--lui-radius-factor: 1;"),
            "an unoffered factor is the default"
        );
    }

    #[test]
    fn a_swatch_wins_and_a_light_brand_gets_dark_text() {
        let ui = Ui::from_request("/theme", "brand=%2312a594&brand-preset=%23ffe629", "");
        let out = css(&chosen(&ui));
        assert!(out.contains("--lui-brand-9: #ffe629;"), "{out}");
        assert!(out.contains("--lui-on-primary: #111113;"), "{out}");
    }

    #[test]
    fn density_and_font_reach_the_tokens() {
        let ui = Ui::from_request("/theme", "density=compact&font=serif", "");
        let out = css(&chosen(&ui));
        assert!(out.contains("--lui-space: calc(8px * 0.85);"), "{out}");
        assert!(out.contains("--lui-font-sans: ui-serif"), "{out}");
    }
}
