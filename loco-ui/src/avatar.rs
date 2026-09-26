//! # Avatar
//!
//! A round picture of a person, with their initials underneath for when there is no picture
//! or it fails to load.
//!
//! **Platform features:** the initials are painted first and the `<img>` is stacked over
//! them; an image that fails to load has `alt=""`, so it renders nothing and the initials
//! show through, with no `onerror` handler. `loading="lazy"` defers images below the fold.
//! The root is `role="img"` named after the person, so a screen reader says the name once.
//!
//! **Accessibility:** `role="img"` named by the person's name; the initials inside are
//! `aria-hidden`. Checked by axe-core in headless Firefox on every demo route, both capability
//! variants, light and dark (no serious or critical violation).
//!
//! **What it does not do without script:** nothing is missing.
//!
//! **Fallback:** the initials are the fallback.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let m = ui.avatar("Ada Lovelace").render().into_string();
//! assert!(m.contains(r#"role="img" aria-label="Ada Lovelace""#) && m.contains(">AL</span>"));
//! // The same in `lui!`:
//! let same = lui! { Avatar("Ada Lovelace"); };
//! assert_eq!(same.into_string(), m);
//! let m = ui.avatar("Grace Hopper").src("/img/grace.jpg").large().render().into_string();
//! assert!(m.contains(r#"<img src="/img/grace.jpg" alt="" loading="lazy">"#) && m.contains("lui-avatar-4"));
//! // A row of overlapping avatars, three shown and a count of the rest.
//! let team = ui.avatars("Team").max(3).avatar("Ada Lovelace").avatar("Grace Hopper").avatar("Alan Turing").avatar("Ken Thompson");
//! let team = team.render().into_string();
//! assert!(team.contains(r#"role="group" aria-label="Team""#) && team.contains(r#"aria-label="+1""#));
//! let same = lui! { Avatars("Team") max=3 { avatar "Ada Lovelace"; avatar "Grace Hopper"; avatar "Alan Turing"; avatar "Ken Thompson"; } };
//! assert_eq!(same.into_string(), team);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::props::{Prop, PropKind};

/// An avatar, made by [`Ui::avatar`].
///
/// **Setters.** Values and items: `.src(..)`, `.size(..)`; switches: `.small()`, `.large()`,
/// `.square()`.
#[derive(Clone, Debug)]
pub struct Avatar<'a> {
    name: &'a str,
    src: Option<&'a str>,
    size: u8,
    square: bool,
}

impl Avatar<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("src", PropKind::Value, "src: &'a str")
            .attr("src")
            .doc("The picture's URL."),
        Prop::new("size", PropKind::Value, "size: u8")
            .default("2")
            .doc("Radix's sizes 1–5: 1.5, 2, 2.5, 3 or 4rem across."),
        Prop::new("small", PropKind::Switch, "")
            .doc("`.size(1)`: 1.5rem across, for lists and table rows."),
        Prop::new("large", PropKind::Switch, "")
            .doc("`.size(4)`: 3rem across, for a profile header."),
        Prop::new("square", PropKind::Switch, "")
            .doc("Rounded corners instead of a circle (a team, a project)."),
    ];
}

impl Ui {
    /// An avatar for the person called `name`: their initials, until `.src()` gives a
    /// picture.
    pub fn avatar<'a>(&self, name: &'a str) -> Avatar<'a> {
        Avatar {
            name,
            src: None,
            size: 2,
            square: false,
        }
    }

    /// A row of overlapping avatars named `label` for assistive tech (the team, the people on
    /// a document); add people with [`Avatars::avatar`].
    pub fn avatars<'a>(&self, label: &'a str) -> Avatars<'a> {
        Avatars {
            label,
            avatars: Vec::new(),
            max: None,
            size: 2,
        }
    }
}

impl<'a> Avatar<'a> {
    /// The picture's URL.
    pub fn src(mut self, src: &'a str) -> Self {
        self.src = Some(src);
        self
    }

    /// Radix's sizes 1–5 (clamped): 1.5, 2 (the default), 2.5, 3 or 4rem across.
    pub fn size(mut self, size: u8) -> Self {
        self.size = size.clamp(1, 5);
        self
    }

    /// `.size(1)`: 1.5rem across, for lists and table rows.
    pub fn small(self) -> Self {
        self.size(1)
    }

    /// `.size(4)`: 3rem across, for a profile header.
    pub fn large(self) -> Self {
        self.size(4)
    }

    /// Rounded corners instead of a circle: a team, a project, a company.
    pub fn square(mut self) -> Self {
        self.square = true;
        self
    }
}

/// A row of overlapping avatars, made by [`Ui::avatars`].
///
/// **Setters.** Values and items: `.avatar(..)`, `.src(..)`, `.max(..)`, `.size(..)`.
#[derive(Clone, Debug)]
pub struct Avatars<'a> {
    label: &'a str,
    avatars: Vec<Avatar<'a>>,
    max: Option<usize>,
    size: u8,
}

impl Avatars<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("avatar", PropKind::Item, "name: &'a str").doc("A person in the row."),
        Prop::new("src", PropKind::Modifier, "src: &'a str")
            .doc("The picture of the person added last."),
        Prop::new("max", PropKind::Number, "max: usize")
            .doc("Show this many, then a `+n` count of the rest."),
        Prop::new("size", PropKind::Value, "size: u8")
            .default("2")
            .doc("Radix's sizes 1–5 for every avatar in the row."),
    ];
}

impl<'a> Avatars<'a> {
    /// A person in the row, their initials until `.src()` gives a picture.
    pub fn avatar(mut self, name: &'a str) -> Self {
        self.avatars.push(Avatar {
            name,
            src: None,
            size: 2,
            square: false,
        });
        self
    }

    /// The picture of the person added last.
    pub fn src(mut self, src: &'a str) -> Self {
        if let Some(a) = self.avatars.last_mut() {
            a.src = Some(src);
        }
        self
    }

    /// Show this many, then a `+n` count of the rest.
    pub fn max(mut self, max: usize) -> Self {
        self.max = Some(max);
        self
    }

    /// Radix's sizes 1–5 (clamped) for every avatar in the row.
    pub fn size(mut self, size: u8) -> Self {
        self.size = size.clamp(1, 5);
        self
    }
}

impl Render for Avatars<'_> {
    fn render(&self) -> Markup {
        let shown = self
            .max
            .unwrap_or(self.avatars.len())
            .min(self.avatars.len());
        let rest = self.avatars.len() - shown;
        html! {
            span class="lui-avatars" role="group" aria-label=(self.label) {
                @for a in &self.avatars[..shown] { (a.clone().size(self.size)) }
                @if rest > 0 {
                    span class={ "lui-avatar lui-avatar-" (self.size) " lui-avatar-more" } role="img" aria-label={ "+" (rest) } {
                        span aria-hidden="true" { "+" (rest) }
                    }
                }
            }
        }
    }
}

/// The first letter of the first and last words, upper-cased: `"Ada King Lovelace"` → `AL`.
fn initials(name: &str) -> String {
    let mut words = name.split_whitespace().filter_map(|w| w.chars().next());
    let first = words.next();
    let last = words.next_back();
    first
        .into_iter()
        .chain(last)
        .flat_map(char::to_uppercase)
        .collect()
}

impl Render for Avatar<'_> {
    fn render(&self) -> Markup {
        html! {
            span class={ "lui-avatar lui-avatar-" (self.size) @if self.square { " lui-avatar-square" } } role="img" aria-label=(self.name) {
                span class="lui-avatar-initials" aria-hidden="true" { (initials(self.name)) }
                @if let Some(src) = self.src { img src=(src) alt="" loading="lazy"; }
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After Radix Themes Avatar: sizes 1–5 (24 to 64px), round or square, the initials on a soft
   brand tint; a group overlaps its avatars, each ringed in the page colour. */
.lui-avatar {
  position: relative; display: inline-grid; place-items: center; flex: none; overflow: hidden;
  width: 2rem; height: 2rem; border-radius: 9999px; background: var(--lui-brand-3); color: var(--lui-brand-11);
  font-size: 0.75rem; font-weight: 500; line-height: 1; vertical-align: middle; user-select: none;
}
/* 2px past the edge on every side: the frame Firefox draws round a broken image falls
   outside the circle and is clipped away. */
.lui-avatar img { position: absolute; inset: -2px; width: calc(100% + 4px); height: calc(100% + 4px); max-width: none; object-fit: cover; color: transparent; }
.lui-avatar-1 { width: 1.5rem; height: 1.5rem; font-size: 0.625rem; }
.lui-avatar-3 { width: 2.5rem; height: 2.5rem; font-size: 0.875rem; }
.lui-avatar-4 { width: 3rem; height: 3rem; font-size: 1rem; }
.lui-avatar-5 { width: 4rem; height: 4rem; font-size: 1.25rem; }
.lui-avatar-square { border-radius: var(--lui-radius); }
.lui-avatars { display: inline-flex; align-items: center; }
.lui-avatars > .lui-avatar { box-shadow: 0 0 0 2px var(--lui-bg); }
.lui-avatars > .lui-avatar + .lui-avatar { margin-left: -0.5rem; }
.lui-avatar-more { background: var(--lui-gray-3); color: var(--lui-fg); }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initials_take_the_first_and_last_words() {
        assert_eq!(initials("Ada King Lovelace"), "AL");
        assert_eq!(initials("prince"), "P");
        assert_eq!(initials("  "), "");
        assert_eq!(initials("éric satie"), "ÉS");
    }
}
