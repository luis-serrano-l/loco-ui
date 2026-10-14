//! # Variants
//!
//! The four choices most components share, after Radix Themes' props of the same names: a
//! [`Size`] (1 to 3), a [`Tone`] (the colour), a [`Variant`] (how the tone is painted) and a
//! [`Radius`] (the corners). Four more serve type and layout: a [`Weight`] (headings, text),
//! an [`Align`] and a [`Justify`] (stacks, clusters) and an [`Underline`] (links). A builder takes them with `.size(..)`, `.tone(..)`,
//! `.variant(..)` and `.radius(..)`, and writes each one that is not its default on its root
//! as `data-size`, `data-tone`, `data-variant` and `data-radius`, beside its classes, so an
//! app's CSS can select on them too.
//!
//! Each enum prints its attribute value with `Display`, parses it back with `FromStr`, and
//! lists every value in `NAMES` (the props table and the spec show them). `Size` also comes
//! from a number, saturating: `0` and `1` are size 1, `3` and up size 3.
//!
//! In `lui!` a choice is written as a number (`size=2`), a string (`tone="danger"`) or an
//! expression (`tone=(Tone::Danger)`). The string is looked up at compile time, so a misspelt
//! one is a compile error rather than a default at run time.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! assert_eq!(Size::from(1), Size::S1);
//! assert_eq!(Size::from(9), Size::S3);
//! assert_eq!("danger".parse::<Tone>(), Ok(Tone::Danger));
//! assert!("loud".parse::<Variant>().is_err());
//! assert_eq!(Radius::Full.to_string(), "full");
//! let ui = Ui::default();
//! let delete = ui.button("Delete").tone(Tone::Danger).variant(Variant::Ghost).render().into_string();
//! assert!(delete.contains(r#"data-tone="danger" data-variant="ghost""#));
//! assert_eq!(lui! { Button("Delete") tone="danger" variant="ghost"; }.into_string(), delete);
//! assert_eq!(lui! { Button("Delete") danger ghost; }.into_string(), delete);
//! ```

use std::fmt;
use std::str::FromStr;

/// A `&str` comparison that works in a `const fn`.
const fn same(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

/// A value that is not one of a choice's names, from [`FromStr`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnknownChoice {
    /// What was asked for (`"tone"`).
    pub choice: &'static str,
    /// The names it takes.
    pub names: &'static [&'static str],
}

impl fmt::Display for UnknownChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} takes one of {}", self.choice, self.names.join(", "))
    }
}

impl std::error::Error for UnknownChoice {}

/// Radix's sizes 1 to 3: the height, padding and text of a control. Size 2 is every
/// component's default.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Size {
    /// Size 1: small (a 2rem button).
    S1,
    /// Size 2: the default (2.25rem).
    #[default]
    S2,
    /// Size 3: large (2.5rem).
    S3,
}

impl Size {
    /// Size 1.
    pub const ONE: Size = Size::S1;
    /// Size 2.
    pub const TWO: Size = Size::S2;
    /// Size 3.
    pub const THREE: Size = Size::S3;
    /// Every value, in the order of [`Self::NAMES`].
    pub const ALL: &'static [Size] = &[Size::S1, Size::S2, Size::S3];
    /// Every value as written in `data-size` and in `lui!`.
    pub const NAMES: &'static [&'static str] = &["1", "2", "3"];

    /// The value as written in `data-size` and in `lui!`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Size::S1 => "1",
            Size::S2 => "2",
            Size::S3 => "3",
        }
    }

    /// The value named `name`, or `None`; a `const fn`, so `lui!` checks a string literal at
    /// compile time.
    pub const fn parse(name: &str) -> Option<Self> {
        let mut i = 0;
        while i < Self::ALL.len() {
            if same(name, Self::ALL[i].as_str()) {
                return Some(Self::ALL[i]);
            }
            i += 1;
        }
        None
    }
}

impl From<u8> for Size {
    /// `0` and `1` are size 1, `2` size 2, `3` and up size 3.
    fn from(size: u8) -> Self {
        match size {
            0 | 1 => Size::S1,
            2 => Size::S2,
            _ => Size::S3,
        }
    }
}

/// The colour a component is painted in. Brand is `--lui-brand-*`, neutral the gray scale,
/// ok, warn and danger their role colours; info is the brand scale until the palette has an
/// info scale of its own.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Tone {
    /// The gray scale.
    Neutral,
    /// The brand scale (`--lui-primary`).
    Brand,
    /// Success: `--lui-ok`.
    Ok,
    /// A warning: `--lui-warn`.
    Warn,
    /// An error or a destructive action: `--lui-danger`.
    Danger,
    /// Information: the brand scale for now.
    Info,
}

impl Tone {
    /// Every value, in the order of [`Self::NAMES`].
    pub const ALL: &'static [Tone] = &[
        Tone::Neutral,
        Tone::Brand,
        Tone::Ok,
        Tone::Warn,
        Tone::Danger,
        Tone::Info,
    ];
    /// Every value as written in `data-tone` and in `lui!`.
    pub const NAMES: &'static [&'static str] =
        &["neutral", "brand", "ok", "warn", "danger", "info"];

    /// The value as written in `data-tone` and in `lui!`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Tone::Neutral => "neutral",
            Tone::Brand => "brand",
            Tone::Ok => "ok",
            Tone::Warn => "warn",
            Tone::Danger => "danger",
            Tone::Info => "info",
        }
    }

    /// The value named `name`, or `None`; a `const fn`, so `lui!` checks a string literal at
    /// compile time.
    pub const fn parse(name: &str) -> Option<Self> {
        let mut i = 0;
        while i < Self::ALL.len() {
            if same(name, Self::ALL[i].as_str()) {
                return Some(Self::ALL[i]);
            }
            i += 1;
        }
        None
    }
}

/// How the tone is painted, Radix's variants: a fill with light text (solid), a tint with
/// darker text (soft), a pale tint in a border (surface), a border only (outline), or nothing
/// until hovered (ghost).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Variant {
    /// Filled with the tone.
    Solid,
    /// A tint of the tone, its text a darker step.
    Soft,
    /// A paler tint inside a border of the tone.
    Surface,
    /// A border of the tone and no fill.
    Outline,
    /// No border or fill until hovered.
    Ghost,
}

impl Variant {
    /// Every value, in the order of [`Self::NAMES`].
    pub const ALL: &'static [Variant] = &[
        Variant::Solid,
        Variant::Soft,
        Variant::Surface,
        Variant::Outline,
        Variant::Ghost,
    ];
    /// Every value as written in `data-variant` and in `lui!`.
    pub const NAMES: &'static [&'static str] = &["solid", "soft", "surface", "outline", "ghost"];

    /// The value as written in `data-variant` and in `lui!`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Variant::Solid => "solid",
            Variant::Soft => "soft",
            Variant::Surface => "surface",
            Variant::Outline => "outline",
            Variant::Ghost => "ghost",
        }
    }

    /// The value named `name`, or `None`; a `const fn`, so `lui!` checks a string literal at
    /// compile time.
    pub const fn parse(name: &str) -> Option<Self> {
        let mut i = 0;
        while i < Self::ALL.len() {
            if same(name, Self::ALL[i].as_str()) {
                return Some(Self::ALL[i]);
            }
            i += 1;
        }
        None
    }
}

/// The corners: square, the small, medium or large radius tokens, or fully round.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Radius {
    /// Square corners.
    None,
    /// `--lui-radius-sm`.
    Small,
    /// `--lui-radius`.
    Medium,
    /// `--lui-radius-lg`.
    Large,
    /// Fully round (a pill, a circle).
    Full,
}

impl Radius {
    /// Every value, in the order of [`Self::NAMES`].
    pub const ALL: &'static [Radius] = &[
        Radius::None,
        Radius::Small,
        Radius::Medium,
        Radius::Large,
        Radius::Full,
    ];
    /// Every value as written in `data-radius` and in `lui!`.
    pub const NAMES: &'static [&'static str] = &["none", "small", "medium", "large", "full"];

    /// The value as written in `data-radius` and in `lui!`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Radius::None => "none",
            Radius::Small => "small",
            Radius::Medium => "medium",
            Radius::Large => "large",
            Radius::Full => "full",
        }
    }

    /// The value named `name`, or `None`; a `const fn`, so `lui!` checks a string literal at
    /// compile time.
    pub const fn parse(name: &str) -> Option<Self> {
        let mut i = 0;
        while i < Self::ALL.len() {
            if same(name, Self::ALL[i].as_str()) {
                return Some(Self::ALL[i]);
            }
            i += 1;
        }
        None
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Display for Tone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Display for Radius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Size {
    type Err = UnknownChoice;
    fn from_str(name: &str) -> Result<Self, UnknownChoice> {
        Self::parse(name).ok_or(UnknownChoice {
            choice: "size",
            names: Self::NAMES,
        })
    }
}

impl FromStr for Tone {
    type Err = UnknownChoice;
    fn from_str(name: &str) -> Result<Self, UnknownChoice> {
        Self::parse(name).ok_or(UnknownChoice {
            choice: "tone",
            names: Self::NAMES,
        })
    }
}

impl FromStr for Variant {
    type Err = UnknownChoice;
    fn from_str(name: &str) -> Result<Self, UnknownChoice> {
        Self::parse(name).ok_or(UnknownChoice {
            choice: "variant",
            names: Self::NAMES,
        })
    }
}

impl FromStr for Radius {
    type Err = UnknownChoice;
    fn from_str(name: &str) -> Result<Self, UnknownChoice> {
        Self::parse(name).ok_or(UnknownChoice {
            choice: "radius",
            names: Self::NAMES,
        })
    }
}

// M40a: the typography and layout choices, in the same shape as the four above.

/// How heavy a heading's or a text's letters are, Radix's four weights.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Weight {
    /// 300.
    Light,
    /// 400, body text.
    Regular,
    /// 500, labels.
    Medium,
    /// 700, Radix's heading default.
    Bold,
}

impl Weight {
    /// Every value, in the order of [`Self::NAMES`].
    pub const ALL: &'static [Weight] = &[Weight::Light, Weight::Regular, Weight::Medium, Weight::Bold];
    /// Every value as written in `data-weight` and in `lui!`.
    pub const NAMES: &'static [&'static str] = &["light", "regular", "medium", "bold"];

    /// The value as written in `data-weight` and in `lui!`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Weight::Light => "light",
            Weight::Regular => "regular",
            Weight::Medium => "medium",
            Weight::Bold => "bold",
        }
    }

    /// The value named `name`, or `None`; a `const fn`, so `lui!` checks a string literal at
    /// compile time.
    pub const fn parse(name: &str) -> Option<Self> {
        let mut i = 0;
        while i < Self::ALL.len() {
            if same(name, Self::ALL[i].as_str()) {
                return Some(Self::ALL[i]);
            }
            i += 1;
        }
        None
    }
}

impl fmt::Display for Weight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Weight {
    type Err = UnknownChoice;
    fn from_str(name: &str) -> Result<Self, UnknownChoice> {
        Self::parse(name).ok_or(UnknownChoice {
            choice: "weight",
            names: Self::NAMES,
        })
    }
}

/// Where a stack's or a cluster's children sit across its main axis (`align-items`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Align {
    /// At the start (the top of a row, the left of a column in a left-to-right page).
    Start,
    /// Centred.
    Center,
    /// At the end.
    End,
    /// On their first text baseline (a row of mixed sizes).
    Baseline,
    /// Stretched to fill the cross axis.
    Stretch,
}

impl Align {
    /// Every value, in the order of [`Self::NAMES`].
    pub const ALL: &'static [Align] = &[Align::Start, Align::Center, Align::End, Align::Baseline, Align::Stretch];
    /// Every value as written in `data-align` and in `lui!`.
    pub const NAMES: &'static [&'static str] = &["start", "center", "end", "baseline", "stretch"];

    /// The value as written in `data-align` and in `lui!`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Align::Start => "start",
            Align::Center => "center",
            Align::End => "end",
            Align::Baseline => "baseline",
            Align::Stretch => "stretch",
        }
    }

    /// The value named `name`, or `None`; a `const fn`, so `lui!` checks a string literal at
    /// compile time.
    pub const fn parse(name: &str) -> Option<Self> {
        let mut i = 0;
        while i < Self::ALL.len() {
            if same(name, Self::ALL[i].as_str()) {
                return Some(Self::ALL[i]);
            }
            i += 1;
        }
        None
    }
}

impl fmt::Display for Align {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Align {
    type Err = UnknownChoice;
    fn from_str(name: &str) -> Result<Self, UnknownChoice> {
        Self::parse(name).ok_or(UnknownChoice {
            choice: "align",
            names: Self::NAMES,
        })
    }
}

/// How a stack's or a cluster's children share its main axis (`justify-content`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Justify {
    /// Packed at the start.
    Start,
    /// Packed in the middle.
    Center,
    /// Packed at the end.
    End,
    /// The first at the start, the last at the end, the rest spread between.
    Between,
}

impl Justify {
    /// Every value, in the order of [`Self::NAMES`].
    pub const ALL: &'static [Justify] = &[Justify::Start, Justify::Center, Justify::End, Justify::Between];
    /// Every value as written in `data-justify` and in `lui!`.
    pub const NAMES: &'static [&'static str] = &["start", "center", "end", "between"];

    /// The value as written in `data-justify` and in `lui!`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Justify::Start => "start",
            Justify::Center => "center",
            Justify::End => "end",
            Justify::Between => "between",
        }
    }

    /// The value named `name`, or `None`; a `const fn`, so `lui!` checks a string literal at
    /// compile time.
    pub const fn parse(name: &str) -> Option<Self> {
        let mut i = 0;
        while i < Self::ALL.len() {
            if same(name, Self::ALL[i].as_str()) {
                return Some(Self::ALL[i]);
            }
            i += 1;
        }
        None
    }
}

impl fmt::Display for Justify {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Justify {
    type Err = UnknownChoice;
    fn from_str(name: &str) -> Result<Self, UnknownChoice> {
        Self::parse(name).ok_or(UnknownChoice {
            choice: "justify",
            names: Self::NAMES,
        })
    }
}

/// When a link is underlined, Radix Link's `underline` prop.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Underline {
    /// Always: links in running text, so they do not rely on colour alone.
    Always,
    /// Only under the pointer or focus: links in navigation and lists.
    Hover,
    /// Never: a link whose place already says it is one.
    None,
}

impl Underline {
    /// Every value, in the order of [`Self::NAMES`].
    pub const ALL: &'static [Underline] = &[Underline::Always, Underline::Hover, Underline::None];
    /// Every value as written in `data-underline` and in `lui!`.
    pub const NAMES: &'static [&'static str] = &["always", "hover", "none"];

    /// The value as written in `data-underline` and in `lui!`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Underline::Always => "always",
            Underline::Hover => "hover",
            Underline::None => "none",
        }
    }

    /// The value named `name`, or `None`; a `const fn`, so `lui!` checks a string literal at
    /// compile time.
    pub const fn parse(name: &str) -> Option<Self> {
        let mut i = 0;
        while i < Self::ALL.len() {
            if same(name, Self::ALL[i].as_str()) {
                return Some(Self::ALL[i]);
            }
            i += 1;
        }
        None
    }
}

impl fmt::Display for Underline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Underline {
    type Err = UnknownChoice;
    fn from_str(name: &str) -> Result<Self, UnknownChoice> {
        Self::parse(name).ok_or(UnknownChoice {
            choice: "underline",
            names: Self::NAMES,
        })
    }
}

/// The `data-radius` rules every component with `.radius(..)` shares; included in
/// [`crate::stylesheet`]. A class and an attribute of weight, so they win over a component's
/// own `border-radius` on its root class.
pub const CSS: &str = r#"
:is(.lui-button, .lui-badge, .lui-avatar)[data-radius=none] { border-radius: 0; }
:is(.lui-button, .lui-badge, .lui-avatar)[data-radius=small] { border-radius: var(--lui-radius-sm); }
:is(.lui-button, .lui-badge, .lui-avatar)[data-radius=medium] { border-radius: var(--lui-radius); }
:is(.lui-button, .lui-badge, .lui-avatar)[data-radius=large] { border-radius: var(--lui-radius-lg); }
:is(.lui-button, .lui-badge, .lui-avatar)[data-radius=full] { border-radius: 9999px; }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_parses_back() {
        for name in Size::NAMES {
            assert_eq!(name.parse::<Size>().unwrap().to_string(), *name);
        }
        for name in Tone::NAMES {
            assert_eq!(name.parse::<Tone>().unwrap().to_string(), *name);
        }
        for name in Variant::NAMES {
            assert_eq!(name.parse::<Variant>().unwrap().to_string(), *name);
        }
        for name in Radius::NAMES {
            assert_eq!(name.parse::<Radius>().unwrap().to_string(), *name);
        }
        for name in Weight::NAMES {
            assert_eq!(name.parse::<Weight>().unwrap().to_string(), *name);
        }
        for name in Align::NAMES {
            assert_eq!(name.parse::<Align>().unwrap().to_string(), *name);
        }
        for name in Justify::NAMES {
            assert_eq!(name.parse::<Justify>().unwrap().to_string(), *name);
        }
        for name in Underline::NAMES {
            assert_eq!(name.parse::<Underline>().unwrap().to_string(), *name);
        }
        assert_eq!(
            "huge".parse::<Size>().unwrap_err().to_string(),
            "size takes one of 1, 2, 3"
        );
        assert_eq!(Size::from(0), Size::ONE);
        assert_eq!(Size::from(2), Size::TWO);
        assert_eq!(Size::from(255), Size::THREE);
    }
}
