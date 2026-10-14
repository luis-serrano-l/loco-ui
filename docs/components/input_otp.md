# One-time code

`loco_ui::input_otp` · demo: `/otp`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `InputOtp` | `ui.input_otp(name: &str, label: &str)` | beta |

## Props of `InputOtp`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `length` | number | `digits: u8` | `6` | `maxlength` | How many digits the code has. |
| `group` | number | `size: u8` |  |  | A stronger divider after every `size` cells (3 for a 123 456 code). |
| `id` | value | `id: &'a str` |  | `id` | The field's id, `f-<name>` by default. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `autocomplete="one-time-code"` | 84 | no | 12 |
| `inputmode="numeric"` | 66 | 95 | 12.1 |
| `pattern` | 4 | 4 | 5 |

- Fallback: a plain spaced-out field; without autocomplete the code is typed or pasted
- Needs script: no
- Responsive by: content
- Look after: shadcn Input OTP, Origin UI

## Documentation

The box for a code sent by text or email: one field, drawn as a row of cells, that the
phone's keyboard offers to fill from the message. One field (not six) so paste, autofill
and the back key all just work.

**Platform features:** `autocomplete="one-time-code"` (Safari 12, and Chrome on Android
through the WebOTP flow), `inputmode="numeric"` (Chrome 66, Firefox 95, Safari 12.1),
`pattern` and `maxlength` for the length; a monospace face with `letter-spacing` and a
repeating background draws the cells; container query units (`cqi`: Chrome 105, Firefox
110, Safari 16) size them.

**Accessibility:** a labelled text field (not a row of fields), so it is announced once and
typed or pasted in one go; the pattern's hint says how many digits. Checked by axe-core in
headless Firefox on every demo route, both capability variants, light and dark (no serious
or critical violation).

The look follows the Origin UI OTP input and shadcn InputOTP: joined cells rounded only at
the group's ends, `.group(n)` for a stronger divider after every `n` cells, a ring on the
cell the next digit goes in while the field has focus, and cells that shrink to fit a
narrow container.

**What it does not do without script:** move the caret to the next cell as it would with
separate boxes (there is one box), or send the form when the last digit is typed. The
ringed cell is the one after the value the server rendered; the enhancement script moves
it as digits are typed.

**Fallback:** without the cell background it is a plain spaced-out field.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.input_otp("code", "Code from the text message").render().into_string();
assert!(m.contains(r#"autocomplete="one-time-code""#) && m.contains(r#"inputmode="numeric""#));
assert!(m.contains(r#"pattern="[0-9]{6}""#) && m.contains(r#"maxlength="6""#));

let m = ui.input_otp("code", "Code").length(4).render().into_string();
assert!(m.contains(r#"pattern="[0-9]{4}""#) && m.contains("--lui-otp-cells: 4"));
// The same in `lui!`:
let same = lui! { InputOtp("code", "Code") length=4; };
assert_eq!(same.into_string(), m);
// Grouped three and three, like "123 456" in the message.
let m = ui.input_otp("code", "Code").group(3).render().into_string();
assert!(m.contains("lui-otp-grouped") && m.contains("--lui-otp-group: 3"));
assert_eq!(lui! { InputOtp("code", "Code") group=3; }.into_string(), m);
```
