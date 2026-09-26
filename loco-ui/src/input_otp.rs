//! # One-time code
//!
//! The box for a code sent by text or email: one field, drawn as a row of cells, that the
//! phone's keyboard offers to fill from the message. One field (not six) so paste, autofill
//! and the back key all just work.
//!
//! **Platform features:** `autocomplete="one-time-code"` (Safari 12, and Chrome on Android
//! through the WebOTP flow), `inputmode="numeric"` (Chrome 66, Firefox 95, Safari 12.1),
//! `pattern` and `maxlength` for the length; a monospace face with `letter-spacing` and a
//! repeating background draws the cells; container query units (`cqi`: Chrome 105, Firefox
//! 110, Safari 16) size them.
//!
//! **Accessibility:** a labelled text field (not a row of fields), so it is announced once and
//! typed or pasted in one go; the pattern's hint says how many digits. Checked by axe-core in
//! headless Firefox on every demo route, both capability variants, light and dark (no serious
//! or critical violation).
//!
//! The look follows the Origin UI OTP input and shadcn InputOTP: joined cells rounded only at
//! the group's ends, `.group(n)` for a stronger divider after every `n` cells, a ring on the
//! cell the next digit goes in while the field has focus, and cells that shrink to fit a
//! narrow container.
//!
//! **What it does not do without script:** move the caret to the next cell as it would with
//! separate boxes (there is one box), or send the form when the last digit is typed. The
//! ringed cell is the one after the value the server rendered; the enhancement script moves
//! it as digits are typed.
//!
//! **Fallback:** without the cell background it is a plain spaced-out field.
//!
//! ```rust
//! use loco_ui::prelude::*;
//! let ui = Ui::default();
//! let m = ui.input_otp("code", "Code from the text message").render().into_string();
//! assert!(m.contains(r#"autocomplete="one-time-code""#) && m.contains(r#"inputmode="numeric""#));
//! assert!(m.contains(r#"pattern="[0-9]{6}""#) && m.contains(r#"maxlength="6""#));
//!
//! let m = ui.input_otp("code", "Code").length(4).render().into_string();
//! assert!(m.contains(r#"pattern="[0-9]{4}""#) && m.contains("--lui-otp-cells: 4"));
//! // The same in `lui!`:
//! let same = lui! { InputOtp("code", "Code") length=4; };
//! assert_eq!(same.into_string(), m);
//! // Grouped three and three, like "123 456" in the message.
//! let m = ui.input_otp("code", "Code").group(3).render().into_string();
//! assert!(m.contains("lui-otp-grouped") && m.contains("--lui-otp-group: 3"));
//! assert_eq!(lui! { InputOtp("code", "Code") group=3; }.into_string(), m);
//! ```

use maud::{Markup, Render, html};

use crate::Ui;
use crate::i18n::Text;
use crate::props::{Prop, PropKind};

/// A one-time code field, made by [`Ui::input_otp`].
///
/// **Setters.** Values and items: `.length(..)`, `.group(..)`.
#[derive(Clone, Debug)]
pub struct InputOtp<'a> {
    ui: &'a Ui,
    name: &'a str,
    label: &'a str,
    length: u8,
    group: Option<u8>,
}

impl InputOtp<'_> {
    /// Every setter with its kind, arguments, default and the HTML attribute it sets; listed by
    /// [`crate::props()`] and kept in step with the setters by a test.
    pub const PROPS: &'static [Prop] = &[
        Prop::new("length", PropKind::Number, "digits: u8")
            .default("6")
            .attr("maxlength")
            .doc("How many digits the code has."),
        Prop::new("group", PropKind::Number, "size: u8")
            .doc("A stronger divider after every `size` cells (3 for a 123 456 code)."),
    ];
}

impl Ui {
    /// A field posting `name`, labelled `label`, for a six-digit code.
    pub fn input_otp<'a>(&'a self, name: &'a str, label: &'a str) -> InputOtp<'a> {
        InputOtp {
            ui: self,
            name,
            label,
            length: 6,
            group: None,
        }
    }
}

impl InputOtp<'_> {
    /// How many digits the code has.
    pub fn length(mut self, digits: u8) -> Self {
        self.length = digits.max(1);
        self
    }

    /// A stronger divider after every `size` cells, the way the code is printed in the
    /// message (3 for 123 456).
    pub fn group(mut self, size: u8) -> Self {
        self.group = (size > 0).then_some(size);
        self
    }
}

impl Render for InputOtp<'_> {
    fn render(&self) -> Markup {
        let n = self.length;
        let pattern = format!("[0-9]{{{n}}}");
        let hint = self.ui.fill(Text::Digits, &[&n]);
        let value = self.ui.param(self.name).unwrap_or("");
        html! {
            div class={ "lui-otp" @if self.group.is_some() { " lui-otp-grouped" } }
                style={ "--lui-otp-cells: " (n) "; --lui-otp-at: " (value.chars().count().min(usize::from(n) - 1))
                    @if let Some(g) = self.group { "; --lui-otp-group: " (g) } } {
                (self.ui.input(self.name, self.label)
                    .value(value)
                    .pattern(&pattern, &hint)
                    .maxlength(usize::from(n))
                    .inputmode("numeric")
                    .autocomplete("one-time-code")
                    .class("lui-otp-input"))
            }
        }
    }
}

/// Styles for this component; included in [`crate::stylesheet`].
pub const CSS: &str = r#"
/* After the Origin UI OTP input and shadcn InputOTP: joined square cells (shared borders, the
   group rounded at its ends), drawn by one field's background, with letter spacing putting
   one digit in each. Every layer is a custom property, so the active cell and the group
   dividers stack without a rule per combination. The cells shrink to fit a narrow
   container (100cqi), down to a 320px phone. */
.lui-otp { container: lui-otp / inline-size; }
/* The cells already count the digits; the counter stays only as the field's description. */
.lui-otp .lui-field-count { position: absolute; width: 1px; height: 1px; overflow: hidden; clip-path: inset(50%); white-space: nowrap; }
.lui-otp .lui-otp-input {
  --lui-otp-cell: min(2.5rem, (100cqi - 2px) / var(--lui-otp-cells));
  --lui-otp-none: linear-gradient(transparent, transparent);
  --lui-otp-active: var(--lui-otp-none); --lui-otp-sep: var(--lui-otp-none);
  box-sizing: content-box; width: calc(var(--lui-otp-cell) * var(--lui-otp-cells)); height: var(--lui-otp-cell); padding: 0;
  border-radius: var(--lui-radius); caret-color: var(--lui-primary);
  font-family: var(--lui-font-mono); font-size: min(1.25rem, var(--lui-otp-cell) / 2); font-variant-numeric: tabular-nums;
  letter-spacing: calc(var(--lui-otp-cell) - 1ch); text-indent: calc((var(--lui-otp-cell) - 1ch) / 2);
  background: var(--lui-otp-active), var(--lui-otp-sep),
    linear-gradient(to right, transparent calc(var(--lui-otp-cell) - 1px), var(--lui-input) 0) 0 0 / var(--lui-otp-cell) 100% repeat-x, var(--lui-bg);
}
/* .group(n): a 2px --lui-gray-8 divider after every n cells. */
.lui-otp-grouped .lui-otp-input {
  --lui-otp-sep: linear-gradient(to right, transparent calc(100% - 2px), var(--lui-gray-8) 0) 1px 0 / calc(var(--lui-otp-cell) * var(--lui-otp-group)) 100% repeat-x;
}
/* Focused: the cell the next digit goes in (--lui-otp-at: the server's value length, kept
   by the enhancement script while typing) gets a 2px ring over a faint tint. */
.lui-otp .lui-otp-input:focus-visible {
  outline: none; border-color: var(--lui-ring);
  --lui-otp-active: linear-gradient(color-mix(in srgb, var(--lui-ring) 10%, var(--lui-bg)) 0 0) calc(var(--lui-otp-cell) * var(--lui-otp-at) + 2px) 2px / calc(var(--lui-otp-cell) - 4px) calc(100% - 4px) no-repeat,
    linear-gradient(var(--lui-ring) 0 0) calc(var(--lui-otp-cell) * var(--lui-otp-at)) 0 / var(--lui-otp-cell) 100% no-repeat;
}
"#;
