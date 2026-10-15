//! # Layout
//!
//! The HTML shell every page uses: doctype, head, the stylesheet (inline, or linked to
//! `/lui.css`), a skip link, `<main id="main">`, the capability beacons that teach the server
//! what this browser supports, and the optional [`crate::enhance`] script tag (the page works
//! the same without it).
//!
//! **Platform features:** cascade layers, `@layer` (Chrome 99, Firefox 97, Safari 15.4): the
//! stylesheet is `lui.reset`, `lui.tokens`, `lui.components`, `lui.blocks` and `lui.modes`,
//! and an app's own CSS ([`Look::css`], [`Page::css`](crate::Page::css)) comes unlayered after
//! it, so it wins without `!important`. `light-dark()` (Chrome 123, Firefox 120, Safari 17.5)
//! holds both schemes in one declaration per token; `color-scheme` on `:root` picks one, from
//! `prefers-color-scheme` or `data-theme`. `@view-transition { navigation: auto }` (Chrome
//! 126+, Safari 18.2+, not Firefox) lets elements with a `view-transition-name` morph across
//! full-page navigations; the root itself swaps instantly. `<link rel="expect"
//! blocking="render">` (Chrome 124+) holds the transition until `<main>` is parsed; it is
//! emitted only with `Cap::ViewTransitions`. `@media (forced-colors: active)`,
//! `(prefers-contrast: more)` and `print` adjust the components in [`MODES`].
//!
//! **Fallback:** browsers without view transitions navigate normally. A browser without
//! `light-dark()` (Chrome before 123) drops every colour token and shows the page in its
//! default black on white: readable, unthemed (FINDINGS, M39).
//!
//! **Theming:** every colour, size, radius, shadow, duration and layer the components use is
//! one of the `--lui-*` custom properties. [`Tokens`] holds them; `ui.page(..).tokens(&tokens)` emits them
//! once per page after the stylesheet, so a different theme is a value, not a CSS file.
//! `docs/theming.md` lists each token and what it affects.
//!
//! **Responsive (M34):** components respond to the box they sit in, not the viewport. Every
//! component root that changes layout sets `container: lui-<component> / inline-size`, and
//! its CSS writes the narrow layout as the base, then adds wider layouts with
//! `@container lui-<component> (width >= …)`. Three breakpoints, used everywhere and never
//! others: *narrow* below `30rem`, *medium* from `30rem`, *wide* from `48rem`. Custom
//! properties cannot appear in an `@container` condition, so the numbers are written
//! literally. Viewport `@media` is kept for `prefers-*`, `forced-colors`, `print` and
//! `pointer` only. Blitz does not support `@container` (FINDINGS, M34), so its shots show the
//! narrow base.
//!
//! ```rust
//! use std::borrow::Cow;
//! use loco_ui::{prelude::*, layout::{Palette, Tokens}};
//! let ui = Ui::from(Caps::all());
//! let page = ui.page("Title", html! { p { "body" } });
//! let tokens = Tokens { light: Palette { primary: Cow::Borrowed("#7a3b1e"), ..Tokens::DEFAULT.light }, ..Tokens::DEFAULT };
//! assert!(page.tokens(&tokens).into_string().contains("--lui-primary: light-dark(#7a3b1e, var(--lui-brand-9))"));
//! ```

use std::borrow::Cow;
use std::fmt::Write;

use maud::{DOCTYPE, Markup, PreEscaped, Render, html};

use crate::i18n::{Strings, Text};
use crate::{Caps, Theme, caps, enhance, stylesheet};

/// A token value: borrowed in a `const` theme, owned in one built at run time (a tenant's
/// palette read from a database).
pub type Value = Cow<'static, str>;

/// One colour scheme's worth of roles, as CSS colour values. The roles follow shadcn/ui:
/// `primary` is the brand colour, `accent` is the quiet surface under a hovered item. By
/// default each role is an alias onto a step of [`Tokens::gray`] or [`Tokens::brand`]
/// (`var(--lui-gray-1)`), so changing a scale changes every role built on it; any CSS colour
/// works in its place. [`Tokens`] holds one for each scheme and emits a role once when both
/// agree, else as `light-dark(<light>, <dark>)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Palette {
    /// Page background (`--lui-bg`).
    pub bg: Value,
    /// Text on the page background (`--lui-fg`).
    pub fg: Value,
    /// Secondary text: notes, labels, table headers (`--lui-muted`).
    pub muted: Value,
    /// Borders and rules (`--lui-line`).
    pub line: Value,
    /// Raised surfaces: `<code>`, the demo stage, open panels (`--lui-surface`).
    pub surface: Value,
    /// Cards and stat tiles (`--lui-card`).
    pub card: Value,
    /// Dialogs, drawers, popovers, menus and toasts (`--lui-popover`).
    pub popover: Value,
    /// Secondary buttons, the tab list, chips and badges (`--lui-secondary`).
    pub secondary: Value,
    /// Hover and highlighted surface: menu items, ghost buttons, rows (`--lui-accent`).
    pub accent: Value,
    /// Text on the accent surface (`--lui-on-accent`).
    pub on_accent: Value,
    /// Primary buttons, checked controls, the current page and step (`--lui-primary`).
    pub primary: Value,
    /// Text on the primary colour (`--lui-on-primary`).
    pub on_primary: Value,
    /// Borders of inputs, selects and textareas (`--lui-input`).
    pub input: Value,
    /// The focus ring, drawn at 50% opacity (`--lui-ring`).
    pub ring: Value,
    /// Link text (`--lui-link`): the brand's text step, which clears 4.5:1 on the background
    /// where the solid `primary` may not (in the dark scheme it does not).
    pub link: Value,
    /// Errors and "no" (`--lui-danger`).
    pub danger: Value,
    /// Text on a danger fill: danger buttons and badges (`--lui-on-danger`).
    pub on_danger: Value,
    /// Success and "yes" (`--lui-ok`).
    pub ok: Value,
    /// Warnings: worked, but look (`--lui-warn`).
    pub warn: Value,
    /// Neutral news: an info alert, badge or toast (`--lui-info`).
    pub info: Value,
    /// Text on an info fill (`--lui-on-info`).
    pub on_info: Value,
}

impl Palette {
    /// Every role as `(name, value)`, in the order [`Tokens::css`] emits them; the name is
    /// the custom property without `--lui-`.
    pub fn roles(&self) -> [(&'static str, &str); 21] {
        [
            ("bg", &self.bg),
            ("fg", &self.fg),
            ("muted", &self.muted),
            ("line", &self.line),
            ("surface", &self.surface),
            ("card", &self.card),
            ("popover", &self.popover),
            ("secondary", &self.secondary),
            ("accent", &self.accent),
            ("on-accent", &self.on_accent),
            ("primary", &self.primary),
            ("on-primary", &self.on_primary),
            ("input", &self.input),
            ("ring", &self.ring),
            ("link", &self.link),
            ("danger", &self.danger),
            ("on-danger", &self.on_danger),
            ("ok", &self.ok),
            ("warn", &self.warn),
            ("info", &self.info),
            ("on-info", &self.on_info),
        ]
    }
}

/// A 12-step colour scale after Radix Colors, one array per scheme, each step written as a
/// CSS colour (`oklch(L C H)` or `#rrggbb`). Each step has one job:
///
/// | Steps | Job |
/// |-------|-----|
/// | 1–2 | App and subtle backgrounds (page, cards, code) |
/// | 3–5 | Component surfaces: normal, hover, pressed |
/// | 6–8 | Borders: rules, inputs, the focus ring |
/// | 9–10 | Solid fills and their hover |
/// | 11–12 | Muted and full-contrast text |
///
/// Emitted as `--lui-<name>-1` … `--lui-<name>-12`, each `light-dark(<light>, <dark>)`. oklch
/// steps are converted to hex on the way out; anything else is emitted as written.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scale {
    /// Steps 1–12 for the light scheme.
    pub light: [Value; 12],
    /// Steps 1–12 for the dark scheme.
    pub dark: [Value; 12],
}

const fn borrowed(steps: [&'static str; 12]) -> [Value; 12] {
    let [a, b, c, d, e, f, g, h, i, j, k, l] = steps;
    [
        Cow::Borrowed(a),
        Cow::Borrowed(b),
        Cow::Borrowed(c),
        Cow::Borrowed(d),
        Cow::Borrowed(e),
        Cow::Borrowed(f),
        Cow::Borrowed(g),
        Cow::Borrowed(h),
        Cow::Borrowed(i),
        Cow::Borrowed(j),
        Cow::Borrowed(k),
        Cow::Borrowed(l),
    ]
}

impl Scale {
    /// A scale from string literals, usable in a `const` or `static`.
    pub const fn new(light: [&'static str; 12], dark: [&'static str; 12]) -> Scale {
        Scale {
            light: borrowed(light),
            dark: borrowed(dark),
        }
    }

    /// Radix slate: a gray with a little blue in it, as Linear's.
    pub const SLATE: Scale = Scale {
        light: [
            Cow::Borrowed("oklch(0.991 0.001 286.4)"),
            Cow::Borrowed("oklch(0.983 0.003 286.4)"),
            Cow::Borrowed("oklch(0.956 0.004 286.3)"),
            Cow::Borrowed("oklch(0.932 0.005 286.3)"),
            Cow::Borrowed("oklch(0.910 0.007 277.2)"),
            Cow::Borrowed("oklch(0.887 0.010 286.2)"),
            Cow::Borrowed("oklch(0.853 0.011 280.4)"),
            Cow::Borrowed("oklch(0.794 0.016 277.8)"),
            Cow::Borrowed("oklch(0.645 0.016 277.7)"),
            Cow::Borrowed("oklch(0.611 0.015 272.6)"),
            Cow::Borrowed("oklch(0.502 0.014 264.4)"),
            Cow::Borrowed("oklch(0.241 0.010 248.2)"),
        ],
        dark: [
            Cow::Borrowed("oklch(0.179 0.004 286.0)"),
            Cow::Borrowed("oklch(0.213 0.004 264.5)"),
            Cow::Borrowed("oklch(0.252 0.006 271.2)"),
            Cow::Borrowed("oklch(0.283 0.007 248.1)"),
            Cow::Borrowed("oklch(0.312 0.008 255.6)"),
            Cow::Borrowed("oklch(0.347 0.010 254.0)"),
            Cow::Borrowed("oklch(0.399 0.012 252.9)"),
            Cow::Borrowed("oklch(0.489 0.016 251.7)"),
            Cow::Borrowed("oklch(0.537 0.015 262.3)"),
            Cow::Borrowed("oklch(0.583 0.015 266.6)"),
            Cow::Borrowed("oklch(0.769 0.010 258.3)"),
            Cow::Borrowed("oklch(0.949 0.003 264.5)"),
        ],
    };

    /// Radix indigo: the brand scale by default, close to Linear's blue-violet.
    pub const INDIGO: Scale = Scale {
        light: [
            Cow::Borrowed("oklch(0.994 0.001 286.4)"),
            Cow::Borrowed("oklch(0.982 0.008 271.3)"),
            Cow::Borrowed("oklch(0.961 0.017 267.8)"),
            Cow::Borrowed("oklch(0.935 0.031 269.8)"),
            Cow::Borrowed("oklch(0.902 0.047 269.6)"),
            Cow::Borrowed("oklch(0.862 0.068 271.1)"),
            Cow::Borrowed("oklch(0.806 0.088 271.4)"),
            Cow::Borrowed("oklch(0.731 0.112 270.4)"),
            Cow::Borrowed("oklch(0.544 0.191 267.0)"),
            Cow::Borrowed("oklch(0.511 0.195 266.6)"),
            Cow::Borrowed("oklch(0.509 0.172 267.2)"),
            Cow::Borrowed("oklch(0.313 0.086 268.6)"),
        ],
        dark: [
            Cow::Borrowed("oklch(0.191 0.025 276.5)"),
            Cow::Borrowed("oklch(0.209 0.030 274.8)"),
            Cow::Borrowed("oklch(0.272 0.071 268.0)"),
            Cow::Borrowed("oklch(0.318 0.095 267.2)"),
            Cow::Borrowed("oklch(0.362 0.104 267.0)"),
            Cow::Borrowed("oklch(0.403 0.111 268.8)"),
            Cow::Borrowed("oklch(0.449 0.120 268.9)"),
            Cow::Borrowed("oklch(0.502 0.137 268.3)"),
            Cow::Borrowed("oklch(0.544 0.191 267.0)"),
            Cow::Borrowed("oklch(0.589 0.176 269.3)"),
            Cow::Borrowed("oklch(0.776 0.114 273.0)"),
            Cow::Borrowed("oklch(0.911 0.043 269.6)"),
        ],
    };

    /// A whole scale from one colour, shaped like `like` (`Scale::INDIGO` for a brand,
    /// `Scale::SLATE` for a gray): `seed` becomes step 9 in both schemes, step 10 keeps the
    /// profile's distance from step 9, and every other step takes the profile's lightness
    /// with the seed's hue and its chroma scaled by the seed's. So steps 11 and 12 stay as
    /// readable as the profile's whatever the seed. `seed` is `#rrggbb` or `oklch(..)`;
    /// `None` for anything else. The theme builder uses it.
    ///
    /// ```rust
    /// use loco_ui::layout::{Scale, Tokens};
    /// let teal = Scale::derive("#12a594", &Scale::INDIGO).unwrap();
    /// assert_eq!(teal.light[8], "#12a594");
    /// let tokens = Tokens { brand: teal.into(), ..Tokens::DEFAULT };
    /// assert!(tokens.css().contains("--lui-brand-9: #12a594;"));
    /// ```
    pub fn derive(seed: &str, like: &Scale) -> Option<DerivedScale> {
        let (l0, c0, h0) = crate::oklch::parse(seed)?;
        let seed_hex = crate::oklch::to_hex((l0, c0, h0));
        let steps = |profile: &[Value; 12]| -> Option<[String; 12]> {
            let profile_points: Vec<_> = profile
                .iter()
                .map(|v| crate::oklch::parse(v))
                .collect::<Option<_>>()?;
            let (l9, c9, _) = profile_points[8];
            let ratio = if c9 > 0.001 { c0 / c9 } else { 1.0 };
            Some(std::array::from_fn(|i| match i {
                8 => seed_hex.clone(),
                9 => crate::oklch::to_hex((
                    (l0 + profile_points[9].0 - l9).clamp(0.0, 1.0),
                    profile_points[9].1 * ratio,
                    h0,
                )),
                _ => crate::oklch::to_hex((profile_points[i].0, profile_points[i].1 * ratio, h0)),
            }))
        };
        Some(DerivedScale {
            light: steps(&like.light)?,
            dark: steps(&like.dark)?,
        })
    }
}

/// A scale made at run time by [`Scale::derive`], as `#rrggbb` steps. `.into()` makes it a
/// [`Scale`] for [`Tokens`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DerivedScale {
    /// Steps 1–12 for the light scheme.
    pub light: [String; 12],
    /// Steps 1–12 for the dark scheme.
    pub dark: [String; 12],
}

impl From<DerivedScale> for Scale {
    fn from(scale: DerivedScale) -> Scale {
        Scale {
            light: scale.light.map(Cow::Owned),
            dark: scale.dark.map(Cow::Owned),
        }
    }
}

impl DerivedScale {
    /// The text colour for the solid step 9: white, or near-black when white would fall
    /// under 4.5:1 (a light seed such as yellow or lime).
    pub fn on_solid(&self) -> &'static str {
        if crate::oklch::contrast("#ffffff", &self.light[8]) >= 4.5 {
            "#ffffff"
        } else {
            "#111113"
        }
    }

    /// The scale as a `layout::Scale` const to paste into Rust.
    pub fn rust(&self, name: &str) -> String {
        let list = |s: &[String; 12]| {
            s.iter()
                .map(|v| format!("\"{v}\""))
                .collect::<Vec<_>>()
                .join(", ")
        };
        format!(
            "const {name}: Scale = Scale::new(\n    [{}],\n    [{}],\n);\n",
            list(&self.light),
            list(&self.dark)
        )
    }
}

/// The three font stacks (`--lui-font-sans`, `--lui-font-mono`, `--lui-font-serif`). The
/// defaults are system stacks only: no web font is loaded.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fonts {
    /// Body text, controls and headings.
    pub sans: Value,
    /// `<code>`, `<kbd>` and snippets.
    pub mono: Value,
    /// Long-form reading text, for an app that wants it; no component uses it by default.
    pub serif: Value,
}

impl Fonts {
    /// `ui-sans-serif` / `ui-monospace` / `ui-serif` and their usual fallbacks.
    pub const DEFAULT: Fonts = Fonts {
        sans: Cow::Borrowed(
            "ui-sans-serif, system-ui, -apple-system, \"Segoe UI\", Roboto, \"Helvetica Neue\", Arial, sans-serif",
        ),
        mono: Cow::Borrowed("ui-monospace, SFMono-Regular, Menlo, Consolas, monospace"),
        serif: Cow::Borrowed("ui-serif, Georgia, Cambria, \"Times New Roman\", Times, serif"),
    };
}

/// The type ramp, Radix Themes' nine steps: `--lui-text-1` … `-9` (font sizes) and
/// `--lui-leading-1` … `-9` (their line heights). Step 2 is the controls' and tables' size,
/// step 3 body text, 4 to 8 headings.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeScale {
    /// Font sizes, steps 1–9.
    pub size: [Value; 9],
    /// Line heights, steps 1–9.
    pub leading: [Value; 9],
}

impl TypeScale {
    /// Radix: 12, 14, 16, 18, 20, 24, 28, 35 and 60px, on 16, 20, 24, 26, 28, 30, 36, 40 and
    /// 60px lines.
    pub const DEFAULT: TypeScale = TypeScale {
        size: [
            Cow::Borrowed("0.75rem"),
            Cow::Borrowed("0.875rem"),
            Cow::Borrowed("1rem"),
            Cow::Borrowed("1.125rem"),
            Cow::Borrowed("1.25rem"),
            Cow::Borrowed("1.5rem"),
            Cow::Borrowed("1.75rem"),
            Cow::Borrowed("2.1875rem"),
            Cow::Borrowed("3.75rem"),
        ],
        leading: [
            Cow::Borrowed("1rem"),
            Cow::Borrowed("1.25rem"),
            Cow::Borrowed("1.5rem"),
            Cow::Borrowed("1.625rem"),
            Cow::Borrowed("1.75rem"),
            Cow::Borrowed("1.875rem"),
            Cow::Borrowed("2.25rem"),
            Cow::Borrowed("2.5rem"),
            Cow::Borrowed("3.75rem"),
        ],
    };
}

/// Control heights, small to large (`--lui-size-1` … `-3`): buttons, inputs, selects. On a
/// coarse pointer all three become 2.75rem (44px), the WCAG 2.5.5 target size.
/// `--lui-control-h-sm`, `--lui-control-h` and `--lui-control-h-lg` alias them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sizes {
    /// Heights of sizes 1, 2 (the default) and 3.
    pub height: [Value; 3],
}

impl Sizes {
    /// 2rem, 2.25rem and 2.5rem (32, 36 and 40px), as Radix sizes 2 to 4.
    pub const DEFAULT: Sizes = Sizes {
        height: [
            Cow::Borrowed("2rem"),
            Cow::Borrowed("2.25rem"),
            Cow::Borrowed("2.5rem"),
        ],
    };
}

/// The corner radius ramp: `--lui-radius-none` (0), `-1` (controls: base − 2px), `-2`
/// (dialogs, popovers: the base), `-3` (cards: base + 4px) and `-full` (pills), each of the
/// middle three multiplied by `factor`. `--lui-radius-sm`, `--lui-radius` and
/// `--lui-radius-lg` alias steps 1 to 3.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Radius {
    /// Step 2, with a unit (`"0.5rem"`, `"0px"`): steps 1 and 3 add to it.
    pub base: Value,
    /// A number every step is multiplied by: `"0"` squares every corner, `"1.5"` rounds more.
    pub factor: Value,
}

impl Radius {
    /// A radius ramp around `base` with factor 1.
    pub const fn new(base: &'static str) -> Radius {
        Radius {
            base: Cow::Borrowed(base),
            factor: Cow::Borrowed("1"),
        }
    }

    /// `0.5rem`, factor 1.
    pub const DEFAULT: Radius = Radius {
        base: Cow::Borrowed("0.5rem"),
        factor: Cow::Borrowed("1"),
    };
}

/// Depth and gradients, each value for both schemes at once (its colours in `light-dark()`).
/// Every shadow stacks a tight contact shadow over softer ambient ones, as Linear and Radix
/// Themes do; in the dark scheme they are deeper, since a soft shadow barely shows on a
/// near-black page.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Depth {
    /// `--lui-shadow-xs`: under controls (buttons, inputs, selects, switches).
    pub xs: Value,
    /// `--lui-shadow-sm`: under cards, stat tiles and kanban cards.
    pub sm: Value,
    /// `--lui-shadow-md`: under popovers, menus and pickers.
    pub md: Value,
    /// `--lui-shadow-lg`: under dialogs, drawers, the command palette and toasts.
    pub lg: Value,
    /// `--lui-highlight`: an inset top edge for raised surfaces (added to their `box-shadow`).
    pub highlight: Value,
    /// `--lui-gradient-primary`: fills primary buttons, in `background-image` over a
    /// `background-color: var(--lui-primary)`, so a browser without `in oklch` gradients
    /// (Chrome before 111) keeps the flat fill.
    pub gradient_primary: Value,
    /// `--lui-gradient-ring`: focus rings and gradient borders.
    pub gradient_ring: Value,
}

impl Depth {
    /// The M30 shadows, one geometry for both schemes and darker shadows in the dark one; the
    /// primary gradient runs from brand step 9 to 11 in the light scheme and to the darker
    /// step 8 in the dark one, so white text holds 4.5:1 over all of it.
    pub const DEFAULT: Depth = Depth {
        xs: Cow::Borrowed(
            "0 1px 1px light-dark(rgb(0 0 0 / 0.04), rgb(0 0 0 / 0.3)), 0 1px 2px light-dark(rgb(0 0 0 / 0.06), rgb(0 0 0 / 0.4))",
        ),
        sm: Cow::Borrowed(
            "0 1px 2px light-dark(rgb(0 0 0 / 0.06), rgb(0 0 0 / 0.4)), 0 2px 6px -1px light-dark(rgb(0 0 0 / 0.06), rgb(0 0 0 / 0.4))",
        ),
        md: Cow::Borrowed(
            "0 1px 2px light-dark(rgb(0 0 0 / 0.05), rgb(0 0 0 / 0.4)), 0 4px 8px -2px light-dark(rgb(0 0 0 / 0.08), rgb(0 0 0 / 0.5)), 0 12px 20px -6px light-dark(rgb(0 0 0 / 0.08), rgb(0 0 0 / 0.5))",
        ),
        lg: Cow::Borrowed(
            "0 1px 3px light-dark(rgb(0 0 0 / 0.06), rgb(0 0 0 / 0.4)), 0 8px 16px -4px light-dark(rgb(0 0 0 / 0.1), rgb(0 0 0 / 0.6)), 0 24px 48px -12px light-dark(rgb(0 0 0 / 0.18), rgb(0 0 0 / 0.7))",
        ),
        highlight: Cow::Borrowed(
            "inset 0 1px 0 light-dark(rgb(255 255 255 / 0.6), rgb(255 255 255 / 0.07))",
        ),
        gradient_primary: Cow::Borrowed(
            "linear-gradient(in oklch to bottom, var(--lui-brand-9), light-dark(var(--lui-brand-11), var(--lui-brand-8)))",
        ),
        gradient_ring: Cow::Borrowed(
            "linear-gradient(in oklch 135deg, var(--lui-brand-11), var(--lui-brand-8))",
        ),
    };
}

/// Durations and the easing curve (`--lui-duration-fast`, `--lui-duration`,
/// `--lui-duration-slow`, `--lui-ease-out`). `prefers-reduced-motion: reduce` zeroes every
/// duration. `--lui-ease-spring`, a damped spring sampled into `linear()`, is not a field:
/// a browser without `linear()` (Chrome before 113, Safari before 17.2) keeps `ease-out`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Motion {
    /// Hovers, ticks, small fades.
    pub fast: Value,
    /// Most transitions.
    pub base: Value,
    /// Panels and progress.
    pub slow: Value,
    /// The ease-out curve.
    pub ease: Value,
}

impl Motion {
    /// 150, 200 and 250ms on `cubic-bezier(0.16, 1, 0.3, 1)`.
    pub const DEFAULT: Motion = Motion {
        fast: Cow::Borrowed("150ms"),
        base: Cow::Borrowed("200ms"),
        slow: Cow::Borrowed("250ms"),
        ease: Cow::Borrowed("cubic-bezier(0.16, 1, 0.3, 1)"),
    };
}

/// The stacking order of the layers that float over the page (`--lui-z-<field>`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZIndex {
    /// Sticky table headers and columns.
    pub sticky: Value,
    /// The drawer panel.
    pub drawer: Value,
    /// A dialog without the top layer (the `:target` fallback).
    pub dialog: Value,
    /// Menus and dropdowns without the top layer (the `<details>` fallback).
    pub popover: Value,
    /// The toast stack.
    pub toast: Value,
    /// Tooltips, above everything else.
    pub tooltip: Value,
}

impl ZIndex {
    /// 2, 10, 10, 10, 20 and 30: the values the components used before they were tokens.
    pub const DEFAULT: ZIndex = ZIndex {
        sticky: Cow::Borrowed("2"),
        drawer: Cow::Borrowed("10"),
        dialog: Cow::Borrowed("10"),
        popover: Cow::Borrowed("10"),
        toast: Cow::Borrowed("20"),
        tooltip: Cow::Borrowed("30"),
    };
}

/// How tightly the page is packed: `Compact` multiplies the spacing unit and the control
/// heights by 0.85. A coarse pointer keeps its 44px targets either way.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Density {
    /// The default spacing and control heights.
    #[default]
    Comfortable,
    /// 0.85 of each, for dense admin screens.
    Compact,
}

impl Density {
    /// The multiplier: `"1"` or `"0.85"`.
    pub fn factor(self) -> &'static str {
        match self {
            Density::Comfortable => "1",
            Density::Compact => "0.85",
        }
    }

    /// `"comfortable"` or `"compact"`.
    pub fn as_str(self) -> &'static str {
        match self {
            Density::Comfortable => "comfortable",
            Density::Compact => "compact",
        }
    }

    /// `Compact` for `"compact"`, else `Comfortable`.
    pub fn parse(value: &str) -> Density {
        if value == "compact" {
            Density::Compact
        } else {
            Density::Comfortable
        }
    }
}

/// The showpiece effects' tokens, emitted after the depth tokens; every effect is opt-in by
/// its setter. `--lui-shimmer` is the sweep of `.shimmer()` over a filled button or badge,
/// `--lui-shimmer-surface` the sweep over a plain one; `--lui-glow` is the spotlight of a
/// card's `.glow()`, `--lui-beam` the lit arc of `.beam()` (the brand's lighter text step in
/// the dark scheme).
pub const EFFECTS: &str = "--lui-shimmer: light-dark(rgb(255 255 255 / 0.45), rgb(255 255 255 / 0.3)); --lui-shimmer-surface: light-dark(color-mix(in srgb, var(--lui-brand-9) 18%, transparent), rgb(255 255 255 / 0.1));
  --lui-glow: light-dark(color-mix(in srgb, var(--lui-brand-9) 22%, transparent), color-mix(in srgb, var(--lui-brand-9) 40%, transparent)); --lui-beam: light-dark(var(--lui-brand-9), var(--lui-brand-11));
  --lui-shimmer-duration: 2.5s; --lui-beam-duration: 6s; --lui-marquee-duration: 40s;";

/// An app's own look: [`Tokens`], extra CSS and its own header, that every [`Ui::page`](crate::Ui::page)
/// applies, so pages an app does not write (the account pages `cargo lui auth` writes, the
/// 404 fallback) look like the rest. Give it to a request with [`Ui::look`](crate::Ui::look);
/// the Axum extractor reads it from the request extensions, so one layer covers every route.
/// A theme that changes per request (a tenant's palette) goes through
/// [`Ui::tokens`](crate::Ui::tokens) instead, since a `Look` is a `static`.
///
/// ```rust
/// use loco_ui::layout::{Look, Radius, Tokens};
/// use loco_ui::prelude::*;
/// fn header() -> Markup { html! { header class="notes-top" { a href="/" { "Notes" } } } }
/// static LOOK: Look = Look {
///     tokens: Tokens { radius: Radius::new("0.75rem"), ..Tokens::DEFAULT },
///     css: &["main{max-width:70rem}"],
///     header: Some(header),
/// };
/// let html = Ui::default().look(&LOOK).page("Notes", html! { h1 { "Notes" } }).into_string();
/// assert!(html.contains("calc(0.75rem * var(--lui-radius-factor))") && html.contains("main{max-width:70rem}"));
/// assert!(html.contains(r#"<header class="notes-top">"#));
/// ```
#[derive(Clone, Debug)]
pub struct Look {
    /// The palette, scales and shape every page renders under ([`Page::tokens`](crate::Page::tokens)).
    pub tokens: Tokens,
    /// Stylesheets added after the library's ([`Page::css`](crate::Page::css)), unlayered so
    /// they win over it; `--lui-*` tokens only, like a component's.
    pub css: &'static [&'static str],
    /// What every page shows above `<main>` ([`Page::header`](crate::Page::header)); `None`
    /// shows nothing there.
    pub header: Option<fn() -> Markup>,
}

/// Every `--lui-*` token. `DEFAULT` is a Linear-like theme: Radix slate for the grays and
/// indigo for the brand, with status colours from Radix Colors step 11 (the step made for
/// text; the light green and amber a shade darker so they clear 4.5:1 on the subtle
/// background too). A test checks every text role on every surface in both schemes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tokens {
    /// The neutral scale (`--lui-gray-1` … `-12`): backgrounds, surfaces, lines and text.
    pub gray: Scale,
    /// The brand scale (`--lui-brand-1` … `-12`): primary fills, links and the focus ring.
    pub brand: Scale,
    /// Roles in the light scheme (`prefers-color-scheme: light`, `data-theme="light"`).
    pub light: Palette,
    /// Roles in the dark scheme (`prefers-color-scheme: dark`, `data-theme="dark"`).
    pub dark: Palette,
    /// The font stacks (`--lui-font-*`).
    pub font: Fonts,
    /// The type ramp (`--lui-text-1..9`, `--lui-leading-1..9`).
    pub type_scale: TypeScale,
    /// Control heights (`--lui-size-1..3`).
    pub size: Sizes,
    /// The corner radius ramp (`--lui-radius-{none,1,2,3,full}`).
    pub radius: Radius,
    /// Shadows and gradients (`--lui-shadow-*`, `--lui-highlight`, `--lui-gradient-*`).
    pub shadow: Depth,
    /// Durations and easing (`--lui-duration-*`, `--lui-ease-out`).
    pub motion: Motion,
    /// Stacking order (`--lui-z-*`).
    pub z: ZIndex,
    /// Spacing and control heights at 1 or 0.85.
    pub density: Density,
    /// The spacing unit every gap and padding is a multiple of (`--lui-space`). The scale
    /// `--lui-space-{1,2,3,4,6,8,10,12,16}` is derived from it: step n is n/2 units (4px each by
    /// default, as Tailwind's `gap-n`), and the layout primitives' `.gap(n)` uses it.
    pub space: Value,
}

impl Default for Tokens {
    fn default() -> Self {
        Tokens::DEFAULT
    }
}

/// `--lui-<name>: <value>;`, or `light-dark(<light>, <dark>)` when the schemes differ.
fn both(out: &mut String, name: &str, light: &str, dark: &str) {
    if light == dark {
        let _ = write!(out, "--lui-{name}: {light}; ");
    } else {
        let _ = write!(out, "--lui-{name}: light-dark({light}, {dark}); ");
    }
}

impl Tokens {
    /// [`Tokens::default`] as a constant, to build a `static` or `const` theme from with
    /// `Tokens { brand: .., ..Tokens::DEFAULT }`.
    pub const DEFAULT: Tokens = Tokens {
        gray: Scale::SLATE,
        brand: Scale::INDIGO,
        light: Palette {
            bg: Cow::Borrowed("var(--lui-gray-1)"),
            fg: Cow::Borrowed("var(--lui-gray-12)"),
            muted: Cow::Borrowed("var(--lui-gray-11)"),
            line: Cow::Borrowed("var(--lui-gray-6)"),
            surface: Cow::Borrowed("var(--lui-gray-2)"),
            card: Cow::Borrowed("var(--lui-gray-1)"),
            popover: Cow::Borrowed("var(--lui-gray-1)"),
            secondary: Cow::Borrowed("var(--lui-gray-3)"),
            accent: Cow::Borrowed("var(--lui-gray-3)"),
            on_accent: Cow::Borrowed("var(--lui-gray-12)"),
            primary: Cow::Borrowed("var(--lui-brand-9)"),
            on_primary: Cow::Borrowed("#ffffff"),
            input: Cow::Borrowed("var(--lui-gray-7)"),
            ring: Cow::Borrowed("var(--lui-brand-8)"),
            link: Cow::Borrowed("var(--lui-brand-11)"),
            danger: Cow::Borrowed("#ce2c31"),
            on_danger: Cow::Borrowed("var(--lui-gray-1)"),
            ok: Cow::Borrowed("#1f7d53"),
            warn: Cow::Borrowed("#9c5b00"),
            info: Cow::Borrowed("#0d74ce"),
            on_info: Cow::Borrowed("var(--lui-gray-1)"),
        },
        dark: Palette {
            bg: Cow::Borrowed("var(--lui-gray-1)"),
            fg: Cow::Borrowed("var(--lui-gray-12)"),
            muted: Cow::Borrowed("var(--lui-gray-11)"),
            line: Cow::Borrowed("var(--lui-gray-6)"),
            surface: Cow::Borrowed("var(--lui-gray-2)"),
            card: Cow::Borrowed("var(--lui-gray-2)"),
            popover: Cow::Borrowed("var(--lui-gray-2)"),
            secondary: Cow::Borrowed("var(--lui-gray-3)"),
            accent: Cow::Borrowed("var(--lui-gray-4)"),
            on_accent: Cow::Borrowed("var(--lui-gray-12)"),
            primary: Cow::Borrowed("var(--lui-brand-9)"),
            on_primary: Cow::Borrowed("#ffffff"),
            input: Cow::Borrowed("var(--lui-gray-7)"),
            ring: Cow::Borrowed("var(--lui-brand-8)"),
            link: Cow::Borrowed("var(--lui-brand-11)"),
            danger: Cow::Borrowed("#ff9592"),
            on_danger: Cow::Borrowed("var(--lui-gray-1)"),
            ok: Cow::Borrowed("#3dd68c"),
            warn: Cow::Borrowed("#ffca16"),
            info: Cow::Borrowed("#70b8ff"),
            on_info: Cow::Borrowed("var(--lui-gray-1)"),
        },
        font: Fonts::DEFAULT,
        type_scale: TypeScale::DEFAULT,
        size: Sizes::DEFAULT,
        radius: Radius::DEFAULT,
        shadow: Depth::DEFAULT,
        motion: Motion::DEFAULT,
        z: ZIndex::DEFAULT,
        density: Density::Comfortable,
        space: Cow::Borrowed("8px"),
    };

    /// Every custom property these tokens set, one `--lui-x: …;` after another: what
    /// [`Tokens::css`] puts on `:root`. Declared on any element (the theme builder's
    /// previews), they apply there; `color-scheme` on that element picks the scheme.
    pub fn declarations(&self) -> String {
        let mut out = String::with_capacity(8 * 1024);
        for (name, scale) in [("gray", &self.gray), ("brand", &self.brand)] {
            for (i, (light, dark)) in scale.light.iter().zip(&scale.dark).enumerate() {
                let hex = |s: &Value| crate::oklch::hex(s).unwrap_or_else(|| s.to_string());
                both(
                    &mut out,
                    &format!("{name}-{}", i + 1),
                    &hex(light),
                    &hex(dark),
                );
            }
            out.push('\n');
        }
        for ((name, light), (_, dark)) in self.light.roles().into_iter().zip(self.dark.roles()) {
            both(&mut out, name, light, dark);
        }
        let f = &self.font;
        let _ = write!(
            out,
            "\n--lui-font-sans: {}; --lui-font-mono: {}; --lui-font-serif: {};\n",
            f.sans, f.mono, f.serif
        );
        let t = &self.type_scale;
        for (i, (size, leading)) in t.size.iter().zip(&t.leading).enumerate() {
            let _ = write!(
                out,
                "--lui-text-{n}: {size}; --lui-leading-{n}: {leading}; ",
                n = i + 1
            );
        }
        let density = self.density.factor();
        let scaled = |value: &str| {
            if density == "1" {
                value.to_string()
            } else {
                format!("calc({value} * {density})")
            }
        };
        for (i, height) in self.size.height.iter().enumerate() {
            let _ = write!(out, "--lui-size-{}: {}; ", i + 1, scaled(height));
        }
        let _ = write!(
            out,
            "\n--lui-control-h-sm: var(--lui-size-1); --lui-control-h: var(--lui-size-2); --lui-control-h-lg: var(--lui-size-3); --lui-hit: 2.25rem;\n"
        );
        let (b, factor) = (&self.radius.base, &self.radius.factor);
        let _ = write!(
            out,
            "--lui-radius-factor: {factor}; --lui-radius-none: 0px; --lui-radius-1: calc(max(0px, {b} - 2px) * var(--lui-radius-factor)); --lui-radius-2: calc({b} * var(--lui-radius-factor)); --lui-radius-3: calc(({b} + 4px) * var(--lui-radius-factor)); --lui-radius-full: 9999px;\n--lui-radius-sm: var(--lui-radius-1); --lui-radius: var(--lui-radius-2); --lui-radius-lg: var(--lui-radius-3);\n"
        );
        let _ = write!(out, "--lui-space: {}; ", scaled(&self.space));
        for n in [1, 2, 3, 4, 6, 8, 10, 12, 16] {
            let _ = write!(
                out,
                "--lui-space-{n}: calc(var(--lui-space) * {}); ",
                f64::from(n) / 2.0
            );
        }
        let d = &self.shadow;
        let _ = write!(
            out,
            "\n--lui-shadow-xs: {}; --lui-shadow-sm: {}; --lui-shadow-md: {}; --lui-shadow-lg: {};\n--lui-highlight: {}; --lui-gradient-primary: {}; --lui-gradient-ring: {};\n{EFFECTS}\n",
            d.xs, d.sm, d.md, d.lg, d.highlight, d.gradient_primary, d.gradient_ring
        );
        let m = &self.motion;
        let _ = writeln!(
            out,
            "--lui-duration-fast: {}; --lui-duration: {}; --lui-duration-slow: {}; --lui-ease-out: {}; --lui-ease-spring: ease-out;",
            m.fast, m.base, m.slow, m.ease
        );
        let z = &self.z;
        let _ = write!(
            out,
            "--lui-z-sticky: {}; --lui-z-drawer: {}; --lui-z-dialog: {}; --lui-z-popover: {}; --lui-z-toast: {}; --lui-z-tooltip: {};\n--lui-overlay: rgb(0 0 0 / 0.5);\n",
            z.sticky, z.drawer, z.dialog, z.popover, z.toast, z.tooltip
        );
        out
    }

    /// The CSS that sets these tokens: [`Tokens::declarations`] on `:root` with
    /// `color-scheme: light dark`, the 44px control heights under `pointer: coarse`, the
    /// spring curve where `linear()` works, and `data-theme="light|dark"` setting
    /// `color-scheme` alone. [`crate::stylesheet`] holds `Tokens::DEFAULT.css()` in its
    /// `lui.tokens` layer.
    pub fn css(&self) -> String {
        format!(
            ":root {{ color-scheme: light dark;\n{}}}\n\
             @media (pointer: coarse) {{ :root {{ --lui-size-1: 2.75rem; --lui-size-2: 2.75rem; --lui-size-3: 2.75rem; --lui-hit: 2.75rem; }} }}\n\
             @supports (transition-timing-function: linear(0, 1)) {{ :root {{ --lui-ease-spring: linear(0, 0.033 3%, 0.116 6%, 0.268 10%, 0.435 14%, 0.594 18%, 0.761 23%, 0.886 28%, 0.981 34%, 1.029 40%, 1.046 48%, 1.037 56%, 1.019 66%, 1.004 78%, 1); }} }}\n\
             :root[data-theme=\"light\"] {{ color-scheme: light; }}\n\
             :root[data-theme=\"dark\"] {{ color-scheme: dark; }}\n",
            self.declarations()
        )
    }

    /// `value` as `#rrggbb` in the light or dark scheme: a hex or `oklch()` colour, or a
    /// `var(--lui-gray-n)` / `var(--lui-brand-n)` alias resolved through the scales. `None`
    /// for anything else. The theme builder fills its colour inputs with it.
    pub fn color(&self, dark: bool, value: &str) -> Option<String> {
        let alias = |name: &str| {
            let rest = value.trim().strip_prefix("var(--lui-")?.strip_suffix(')')?;
            let step: usize = rest.strip_prefix(name)?.strip_prefix('-')?.parse().ok()?;
            let scale = if name == "gray" {
                &self.gray
            } else {
                &self.brand
            };
            let steps = if dark { &scale.dark } else { &scale.light };
            steps.get(step.checked_sub(1)?).map(|s| s.as_ref())
        };
        let value = alias("gray").or_else(|| alias("brand")).unwrap_or(value);
        crate::oklch::hex(value)
    }
}

/// How a page carries the library's stylesheet.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum Sheet {
    /// Inline, the whole [`stylesheet`].
    #[default]
    Inline,
    /// A `<link>` to [`STYLESHEET_PATH`], cached forever.
    Linked,
    /// Inline, only part of it ([`crate::stylesheet_for`]).
    Subset(&'static str),
}

/// Where the axum feature's router serves [`stylesheet`]; [`stylesheet_url`] adds its hash.
pub const STYLESHEET_PATH: &str = "/lui.css";

/// `/lui.css?v=<hash>`: changes whenever the stylesheet does, so it may cache forever.
pub fn stylesheet_url() -> &'static str {
    static URL: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    URL.get_or_init(|| format!("{STYLESHEET_PATH}?v={}", stylesheet_etag()))
}

/// FNV-1a hash of [`stylesheet`], as 16 hex digits: its `ETag` and the `?v=` of its URL.
pub fn stylesheet_etag() -> &'static str {
    static TAG: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    TAG.get_or_init(|| {
        let hash = stylesheet()
            .bytes()
            .fold(0xcbf2_9ce4_8422_2325_u64, |h, b| {
                (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3)
            });
        format!("{hash:016x}")
    })
}

/// Everything the document shell needs besides the body.
pub(crate) struct Doc<'a> {
    pub(crate) caps: &'a Caps,
    pub(crate) strings: &'static Strings,
    /// Which way the text runs: `<html dir>`.
    pub(crate) dir: crate::i18n::Dir,
    pub(crate) title: &'a str,
    pub(crate) theme: Theme,
    pub(crate) tokens: Option<&'a Tokens>,
    pub(crate) css: &'a [&'static str],
    pub(crate) header: Option<&'a Markup>,
    /// What the page's head setters add ([`crate::Page::description`] and the rest).
    pub(crate) head: Option<&'a crate::ui::Head>,
    pub(crate) script: bool,
    pub(crate) sheet: Sheet,
}

impl<'a> Doc<'a> {
    pub(crate) fn new(caps: &'a Caps, title: &'a str, theme: Theme) -> Doc<'a> {
        Doc {
            caps,
            strings: &Strings::ENGLISH,
            dir: crate::i18n::Dir::Ltr,
            title,
            theme,
            tokens: None,
            css: &[],
            header: None,
            head: None,
            script: true,
            sheet: Sheet::Inline,
        }
    }

    /// The text of each `<style>` in `<head>`, in order, with its class: the stylesheet
    /// (unless linked), the token overrides in the `lui.tokens` layer, and the app's CSS,
    /// unlayered. [`crate::enhance::csp`] hashes them for a linked page.
    pub(crate) fn styles(&self) -> Vec<(Option<&'static str>, Cow<'static, str>)> {
        let mut out = Vec::with_capacity(3);
        match self.sheet {
            Sheet::Inline => out.push((None, Cow::Borrowed(stylesheet()))),
            Sheet::Subset(css) => out.push((None, Cow::Borrowed(css))),
            Sheet::Linked => {}
        }
        if let Some(t) = self.tokens {
            out.push((
                Some("lui-tokens"),
                Cow::Owned(format!("@layer lui.tokens {{\n{}}}", t.css())),
            ));
        }
        if !self.css.is_empty() {
            let user: String = self.css.iter().map(|c| minified(c)).collect();
            out.push((Some("lui-user"), Cow::Owned(user)));
        }
        out
    }

    /// `<head>`: charset, viewport, title, the page's meta, the view-transition hold, the
    /// styles, then the page's links and extra markup.
    pub(crate) fn head(&self) -> Markup {
        html! {
            head {
                (meta(self.title))
                @if let Some(head) = self.head { (head.meta()) }
                // Hold a cross-document view transition until `#main` is parsed. Streamed
                // pages must not: their parse ends only when the last slot has filled.
                @if self.caps.has(crate::Cap::ViewTransitions) { link rel="expect" href="#main" blocking="render"; }
                @if self.sheet == Sheet::Linked { link rel="stylesheet" href=(stylesheet_url()); }
                @for (class, text) in self.styles() { style class=[class] { (PreEscaped(text)) } }
                @if let Some(head) = self.head { (head.links()) }
            }
        }
    }

    /// The skip link and the header, what comes before `<main>`.
    pub(crate) fn top(&self) -> Markup {
        html! {
            a class="lui-skip" href="#main" { (self.strings.get(Text::SkipToContent)) }
            @if let Some(top) = self.header { (top) }
        }
    }

    /// The whole document around `body`.
    pub(crate) fn render(&self, body: Markup) -> Markup {
        // The stylesheet is most of the page: size the buffer once instead of doubling into it.
        let size = stylesheet().len() + body.0.len() + 2048;
        html! {
            (Reserve(size))
            (DOCTYPE)
            html lang=(self.strings.lang()) dir=(self.dir.as_str()) data-theme=(self.theme.as_str()) {
                (self.head())
                body {
                    (self.top())
                    main id="main" tabindex="-1" { (body) }
                    (caps::beacons(self.caps))
                    @if self.script { (enhance::script_tag()) }
                }
            }
        }
    }
}

/// `css` minified, once per string for the life of the process: an app's [`Look::css`] is
/// a `static`, so it is minified on the first page, not on every one.
fn minified(css: &'static str) -> &'static str {
    use std::collections::HashMap;
    use std::sync::{Mutex, OnceLock};
    static DONE: OnceLock<Mutex<HashMap<(usize, usize), &'static str>>> = OnceLock::new();
    let key = (css.as_ptr() as usize, css.len());
    let mut done = DONE
        .get_or_init(Mutex::default)
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    done.entry(key)
        .or_insert_with(|| Box::leak(crate::minify_css(css).into_boxed_str()))
}

/// `<head>`: charset, viewport, title and the inline stylesheet.
pub fn head(title: &str) -> Markup {
    let caps = Caps::default();
    html! {
        (Reserve(stylesheet().len() + 256))
        (Doc::new(&caps, title, Theme::Auto).head())
    }
}

fn meta(title: &str) -> Markup {
    html! {
        meta charset="utf-8";
        meta name="viewport" content="width=device-width, initial-scale=1";
        title { (title) }
    }
}

/// Grows the buffer `html!` is writing into by `.0` bytes and writes nothing: a size hint
/// for a template whose splices dwarf its literals.
pub(crate) struct Reserve(pub(crate) usize);

impl Render for Reserve {
    fn render_to(&self, buffer: &mut String) {
        buffer.reserve(self.0);
    }
}

/// The `@view-transition` rule, emitted before the layers (an at-rule of its own, not a
/// style rule a layer orders).
pub(crate) const VIEW_TRANSITION: &str = "@view-transition { navigation: auto; }";

/// The `lui.reset` layer: a minimal normalisation and the styles of bare elements (`main`,
/// headings, `p`, `a`, `code`, `table`). Every component and every app rule wins over it.
pub const RESET: &str = r#"
* { box-sizing: border-box; }
/* Type: the system stack only, no web font. Body text is step 3 of the ramp (1rem/1.5);
   controls, tables and menus step 2; weights are 500 for labels and buttons, 600 for
   headings. Numbers in tables and stats are tabular. */
html {
  font-family: var(--lui-font-sans); line-height: 1.5;
  -webkit-text-size-adjust: 100%; -webkit-font-smoothing: antialiased; -moz-osx-font-smoothing: grayscale;
}
body { margin: 0; background: var(--lui-bg); color: var(--lui-fg); }
main { max-width: 52rem; margin: 0 auto; padding: calc(var(--lui-space) * 4) calc(var(--lui-space) * 2) calc(var(--lui-space) * 8); }
/* main and a swapped root take the focus from a script (tabindex=-1) without a ring. */
[tabindex="-1"]:focus { outline: none; }
p, li { max-width: 44rem; }
/* A fieldset's UA min-inline-size is min-content: one wide field would push the page sideways. */
fieldset { min-inline-size: 0; }
h1 { font-size: var(--lui-text-8); line-height: var(--lui-leading-8); letter-spacing: -0.025em; font-weight: 600; margin: 0 0 0.75rem; }
h2 { font-size: var(--lui-text-6); line-height: var(--lui-leading-6); letter-spacing: -0.0125em; font-weight: 600; margin: 2rem 0 0.5rem; }
h3 { font-size: var(--lui-text-4); line-height: var(--lui-leading-4); font-weight: 600; }
p { margin: 0 0 1rem; }
a { color: var(--lui-link); text-underline-offset: 0.15em; text-decoration-thickness: 1px; }
code {
  font-family: var(--lui-font-mono); font-size: 0.875em;
  background: var(--lui-surface); border: 1px solid var(--lui-line); border-radius: var(--lui-radius-sm); padding: 0.05em 0.35em;
}
details > summary { cursor: pointer; font-weight: 500; }
table { border-collapse: collapse; width: 100%; font-size: var(--lui-text-2); line-height: var(--lui-leading-2); font-variant-numeric: tabular-nums; }
/* shadcn Table: h-10 heads in the muted colour, p-2 cells, a rule under each row, muted/50 hover. */
th, td { text-align: start; padding: 0.5rem; border-bottom: 1px solid var(--lui-line); vertical-align: middle; }
th { height: 2.5rem; color: var(--lui-muted); font-weight: 500; white-space: nowrap; }
tbody tr { transition: background-color 0.15s; }
tbody tr:hover { background: color-mix(in srgb, var(--lui-accent) 50%, transparent); }
"#;

/// The layout's own rules, first in the `lui.components` layer; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* The root does not cross-fade: the swap is instant and only named parts morph, so a
   navigation never feels slower than the plain reload it replaces. */
::view-transition-old(root), ::view-transition-new(root) { animation: none; }
::view-transition-group(*) { animation-duration: 120ms; animation-timing-function: ease-out; }
::view-transition-old(*), ::view-transition-new(*) { animation-duration: 120ms; }
/* The transition overlay must not eat clicks: a counter tapped twice quickly would lose the
   second tap while the first one is still morphing. */
::view-transition { pointer-events: none; }
/* Controls are styled in button.rs and input.rs. Focus everywhere is a 3px ring at 50%;
   aria-invalid turns borders and the ring to --lui-danger. */
:focus-visible { outline: 3px solid color-mix(in srgb, var(--lui-ring) 50%, transparent); outline-offset: 0; }
[aria-invalid=true] { border-color: var(--lui-danger); }
[aria-invalid=true]:focus-visible { outline-color: color-mix(in srgb, var(--lui-danger) 20%, transparent); }
/* Gap steps for the layout primitives (stack, cluster, grid, split); their default gaps sit
   in :where() so one of these always wins. */
.lui-gap-0 { gap: 0; }
.lui-gap-1 { gap: var(--lui-space-1); }
.lui-gap-2 { gap: var(--lui-space-2); }
.lui-gap-3 { gap: var(--lui-space-3); }
.lui-gap-4 { gap: var(--lui-space-4); }
.lui-gap-6 { gap: var(--lui-space-6); }
.lui-gap-8 { gap: var(--lui-space-8); }
.lui-sr, .lui-skip:not(:focus) { position: absolute; width: 1px; height: 1px; margin: -1px; padding: 0; overflow: hidden; clip-path: inset(50%); text-wrap: nowrap; border: 0; }
/* The skip link, the first thing a keyboard reaches: hidden until it has the focus. */
.lui-skip {
  position: absolute; inset-block-start: var(--lui-space-2); inset-inline-start: var(--lui-space-2); z-index: var(--lui-z-tooltip);
  padding: var(--lui-space-2) var(--lui-space-3); border-radius: var(--lui-radius-sm);
  background: var(--lui-popover); color: var(--lui-fg); box-shadow: var(--lui-shadow-md); font-weight: 500;
}
/* A swap root or form with a request in flight (set by the enhancement script only). The
   fade waits so a fast answer never flickers; --lui-busy: 1 turns it off. */
[data-lui-busy] { opacity: var(--lui-busy, 0.6); transition: opacity 0.15s 0.2s; cursor: progress; }
.lui-note { color: var(--lui-muted); font-size: var(--lui-text-2); }

@media (prefers-reduced-motion: reduce) {
  *, ::before, ::after, ::backdrop { animation-duration: 0s !important; transition-duration: 0s !important; }
  :root { --lui-duration-fast: 0s; --lui-duration: 0s; --lui-duration-slow: 0s; }
  ::view-transition-group(*), ::view-transition-old(*), ::view-transition-new(*) { animation: none !important; }
}
"#;

/// The `lui.modes` layer, last: forced colours, more contrast and print. Under
/// `forced-colors: active` the browser replaces every colour with a system one, so the
/// controls drawn with `appearance: none` (checkbox, radio, switch, range, progress, meter)
/// keep their state through `forced-color-adjust: none` and system colours, and badges get a
/// border to stay visible. `.lui-forced` and `.lui-contrast` apply the same rules to a subtree,
/// for a page that shows them (`/modes` in the demo): a media query cannot be forced from the
/// server.
pub const MODES: &str = r#"
@media (forced-colors: active) {
  input:is([type=checkbox], [type=radio]) { border-color: ButtonText; }
  input:is([type=checkbox], [type=radio]):checked { forced-color-adjust: none; background: Highlight; border-color: Highlight; }
  input[type=checkbox]:not(.lui-switch)::before { forced-color-adjust: none; border-color: HighlightText; }
  input[type=radio]::before { forced-color-adjust: none; background: HighlightText; }
  input.lui-switch { forced-color-adjust: none; background: Canvas; box-shadow: inset 0 0 0 1px ButtonText; }
  input.lui-switch::before { background: ButtonText; }
  input.lui-switch:checked { background: Highlight; }
  input.lui-switch:checked::before { background: HighlightText; }
  .lui-range-input { forced-color-adjust: none; }
  .lui-range-input::-webkit-slider-runnable-track { background: linear-gradient(to right, Highlight var(--lui-range-fill), ButtonText var(--lui-range-fill)); }
  .lui-range-input::-moz-range-track { background: ButtonText; }
  .lui-range-input::-moz-range-progress { background: Highlight; }
  .lui-range-input::-webkit-slider-thumb { background: ButtonFace; box-shadow: 0 0 0 1px ButtonText; }
  .lui-range-input::-moz-range-thumb { background: ButtonFace; box-shadow: 0 0 0 1px ButtonText; }
  :is(.lui-progress, .lui-meter, .lui-stat-progress) { forced-color-adjust: none; background: Canvas; box-shadow: inset 0 0 0 1px CanvasText; }
  :is(.lui-progress, .lui-stat-progress)::-webkit-progress-value { background: Highlight; }
  :is(.lui-progress, .lui-stat-progress)::-moz-progress-bar { background: Highlight; }
  .lui-meter::-webkit-meter-optimum-value { background: Highlight; }
  .lui-meter::-moz-meter-bar { background: Highlight; }
  .lui-badge { border-color: CanvasText; }
}
.lui-forced :is(input[type=checkbox], input[type=radio]) { border-color: CanvasText; }
.lui-forced :is(input[type=checkbox], input[type=radio]):checked { background: Highlight; border-color: Highlight; }
.lui-forced input.lui-switch { background: Canvas; box-shadow: inset 0 0 0 1px CanvasText; }
.lui-forced input.lui-switch::before { background: CanvasText; }
.lui-forced input.lui-switch:checked { background: Highlight; }
.lui-forced input.lui-switch:checked::before { background: HighlightText; }
.lui-forced :is(.lui-progress, .lui-meter) { background: Canvas; box-shadow: inset 0 0 0 1px CanvasText; }
.lui-forced .lui-progress::-webkit-progress-value { background: Highlight; }
.lui-forced .lui-progress::-moz-progress-bar { background: Highlight; }
.lui-forced .lui-badge { background: Canvas; color: CanvasText; border-color: CanvasText; }
.lui-forced { background: Canvas; color: CanvasText; }
/* More contrast: lines and input borders a step darker, a full-strength focus ring. */
@media (prefers-contrast: more) {
  :root { --lui-line: var(--lui-gray-8); --lui-input: var(--lui-gray-9); --lui-muted: var(--lui-gray-12); }
  :focus-visible { outline-color: var(--lui-ring); outline-width: 3px; }
}
.lui-contrast { --lui-line: var(--lui-gray-8); --lui-input: var(--lui-gray-9); --lui-muted: var(--lui-gray-12); }
.lui-contrast :focus-visible { outline-color: var(--lui-ring); }
/* Print: what only makes sense on a screen goes; a sticky header is printed in place. */
@media print {
  :is(.lui-skip, .lui-drawer, .lui-toasts, .lui-sidebar, .lui-palette, .lui-caps) { display: none !important; }
  :is(thead, th, td) { position: static !important; }
  body { background: none; }
  :root { color-scheme: light; }
}
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::oklch::contrast;

    /// `light-dark(a, b)` resolved to one scheme, for values written for both.
    fn scheme(value: &str, dark: bool) -> String {
        let Some(start) = value.find("light-dark(") else {
            return value.to_string();
        };
        let inner = &value[start + "light-dark(".len()..];
        let (mut depth, mut comma, mut end) = (0, 0, 0);
        for (i, c) in inner.char_indices() {
            match c {
                '(' => depth += 1,
                ')' if depth == 0 => {
                    end = i;
                    break;
                }
                ')' => depth -= 1,
                ',' if depth == 0 => comma = i,
                _ => {}
            }
        }
        let pick = if dark {
            inner[comma + 1..end].trim()
        } else {
            inner[..comma].trim()
        };
        scheme(
            &format!("{}{pick}{}", &value[..start], &inner[end + 1..]),
            dark,
        )
    }

    /// Every text colour clears WCAG AA (4.5:1) on each surface it is drawn on, in both
    /// schemes, for the default tokens: the roles resolve through the scales first.
    #[test]
    fn default_text_clears_4_5_to_1() {
        let (t, mut failed) = (Tokens::default(), Vec::new());
        for (dark, p) in [(false, &t.light), (true, &t.dark)] {
            let c = |v: &str| {
                t.color(dark, v)
                    .unwrap_or_else(|| panic!("{v} is not a colour"))
            };
            let mut pairs = vec![
                ("on_accent", &p.on_accent, "accent", &p.accent),
                ("on_primary", &p.on_primary, "primary", &p.primary),
                ("on_danger", &p.on_danger, "danger", &p.danger),
                ("on_info", &p.on_info, "info", &p.info),
            ];
            // Status colours are text on the page's surfaces; chips and badges tint their own.
            for (bn, bv) in [
                ("bg", &p.bg),
                ("surface", &p.surface),
                ("card", &p.card),
                ("popover", &p.popover),
                ("secondary", &p.secondary),
            ] {
                for (fname, fv) in [
                    ("fg", &p.fg),
                    ("muted", &p.muted),
                    ("link", &p.link),
                    ("danger", &p.danger),
                    ("ok", &p.ok),
                    ("warn", &p.warn),
                    ("info", &p.info),
                ] {
                    if bn != "secondary" || matches!(fname, "fg" | "muted" | "link") {
                        pairs.push((fname, fv, bn, bv));
                    }
                }
            }
            for (fname, fv, bn, bv) in pairs {
                let r = contrast(&c(fv), &c(bv));
                if r < 4.5 {
                    failed.push(format!("{fname} on {bn} is {r:.2}:1 (dark: {dark})"));
                }
            }
        }
        assert!(failed.is_empty(), "{failed:#?}");
    }

    /// White text stays readable over every stop of the primary gradient, in both schemes.
    #[test]
    fn on_primary_clears_4_5_to_1_over_the_gradient() {
        let t = Tokens::default();
        for (dark, p) in [(false, &t.light), (true, &t.dark)] {
            let line = scheme(&t.shadow.gradient_primary, dark);
            let stops: Vec<&str> = line
                .split("var(")
                .skip(1)
                .map(|v| v.split(')').next().unwrap())
                .collect();
            assert_eq!(stops.len(), 2, "{line}");
            for stop in stops {
                let bg = t.color(dark, &format!("var({stop})")).unwrap();
                let r = contrast(&t.color(dark, &p.on_primary).unwrap(), &bg);
                assert!(
                    r >= 4.5,
                    "on_primary over {stop} is {r:.2}:1 (dark: {dark})"
                );
            }
        }
    }

    /// A derived scale keeps the profile's text steps readable whatever the seed: brand 11
    /// (links) and gray 11 and 12 on the backgrounds, and `on_solid` on step 9.
    #[test]
    fn derived_scales_keep_text_readable() {
        let indigo = Scale::derive("#3e63dd", &Scale::INDIGO).unwrap();
        assert_eq!(indigo.light[8], "#3e63dd");
        assert_eq!(indigo.light[10], "#3a5bc7");
        let mut failed = Vec::new();
        for brand in [
            "#3e63dd", "#12a594", "#e5484d", "#ffe629", "#8e4ec6", "#f76b15", "#000000",
        ] {
            for gray in [
                "#8b8d98", "#8d8d8d", "#8e8c99", "#868e8b", "#8d8d86", "#ffffff",
            ] {
                let (b, g) = (
                    Scale::derive(brand, &Scale::INDIGO).unwrap(),
                    Scale::derive(gray, &Scale::SLATE).unwrap(),
                );
                if contrast(b.on_solid(), &b.light[8]) < 4.5 {
                    failed.push(format!("{brand}: on_solid on step 9"));
                }
                for (dark, bs, gs) in [(false, &b.light, &g.light), (true, &b.dark, &g.dark)] {
                    for bg in 0..3 {
                        for (name, fg) in [
                            ("brand 11", &bs[10]),
                            ("gray 11", &gs[10]),
                            ("gray 12", &gs[11]),
                        ] {
                            let r = contrast(fg, &gs[bg]);
                            if r < 4.5 {
                                failed.push(format!(
                                    "{brand}/{gray}: {name} on gray {} is {r:.2} (dark: {dark})",
                                    bg + 1
                                ));
                            }
                        }
                    }
                }
            }
        }
        assert!(failed.is_empty(), "{failed:#?}");
    }

    #[test]
    fn scales_are_emitted_as_hex_and_roles_alias_them() {
        let css = Tokens::default().css();
        assert!(
            css.contains("--lui-gray-12: light-dark(#1c2024, #edeef0);"),
            "{css}"
        );
        assert!(css.contains("--lui-brand-9: #3e63dd;"));
        assert!(css.contains("--lui-bg: var(--lui-gray-1);"));
        assert!(css.contains("--lui-card: light-dark(var(--lui-gray-1), var(--lui-gray-2));"));
        assert!(!css.contains("oklch("));
        assert_eq!(
            Tokens::default()
                .color(true, "var(--lui-gray-1)")
                .as_deref(),
            Some("#111113")
        );
    }

    /// One declaration per token: no dark block repeats the palette, and `data-theme` only
    /// sets `color-scheme`.
    #[test]
    fn one_block_holds_both_schemes() {
        let css = Tokens::default().css();
        assert_eq!(css.matches("--lui-bg:").count(), 1, "{css}");
        assert!(!css.contains("prefers-color-scheme"));
        assert!(css.contains(":root[data-theme=\"dark\"] { color-scheme: dark; }"));
    }

    #[test]
    fn density_scales_space_and_controls() {
        let compact = Tokens {
            density: Density::Compact,
            ..Tokens::DEFAULT
        }
        .css();
        assert!(
            compact.contains("--lui-space: calc(8px * 0.85);"),
            "{compact}"
        );
        assert!(compact.contains("--lui-size-2: calc(2.25rem * 0.85);"));
        assert!(
            compact.contains("--lui-size-2: 2.75rem;"),
            "coarse pointers keep 44px"
        );
        assert_eq!(Density::parse("compact"), Density::Compact);
    }

    /// A tenant's palette read at run time is a `Tokens` value too: owned strings, no leak.
    #[test]
    fn owned_values_build_tokens() {
        let primary = String::from("#123456");
        let tokens = Tokens {
            light: Palette {
                primary: primary.into(),
                ..Tokens::DEFAULT.light
            },
            ..Tokens::DEFAULT
        };
        assert!(
            tokens
                .css()
                .contains("--lui-primary: light-dark(#123456, var(--lui-brand-9));")
        );
    }
}
