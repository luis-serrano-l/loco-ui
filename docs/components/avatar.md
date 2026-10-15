# Avatar

`loco_ui::avatar` · demo: `/card`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Avatar` | `ui.avatar(name: &str)` | stable |
| `Avatars` | `ui.avatars(label: &str)` | stable |

## Props of `Avatar`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `src` | value | `src: &'a str` |  | `src` | The picture's URL. |
| `size` | value | `size: u8` | `2` |  | Radix's sizes 1–5: 1.5, 2, 2.5, 3 or 4rem across. |
| `small` | switch |  | `off` |  | `.size(1)`: 1.5rem across, for lists and table rows. |
| `large` | switch |  | `off` |  | `.size(4)`: 3rem across, for a profile header. |
| `square` | switch |  | `off` |  | Rounded corners instead of a circle (a team, a project). |
| `tone` | choice: neutral, brand, ok, warn, danger, info | `tone: Tone` | `brand` |  | The colour of the initials' tile. |
| `variant` | choice: solid, soft, surface, outline, ghost | `variant: Variant` | `soft` |  | Soft (a tint) or solid (filled); Radix has no other, so the rest draw as soft. |
| `radius` | choice: none, small, medium, large, full | `radius: Radius` | `full` |  | The corners, from square to the circle. |

## Props of `Avatars`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `avatar` | item | `name: &'a str` |  |  | A person in the row. |
| `src` | modifier | `src: &'a str` |  |  | The picture of the person added last. |
| `max` | number | `max: usize` |  |  | Show this many, then a `+n` count of the rest. |
| `size` | value | `size: u8` | `2` |  | Radix's sizes 1–5 for every avatar in the row. |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-avatar` | `--lui-brand-3`, `--lui-brand-11`, `--lui-text-1`, `--lui-primary`, `--lui-gray-11`, `--lui-danger`, `--lui-ok`, `--lui-warn`, `--lui-avatar-tone`, `--lui-bg`, `--lui-fg`, `--lui-gray-1` |
| `.lui-avatar-1` | `--lui-text-1` |
| `.lui-avatar-3` | `--lui-text-2` |
| `.lui-avatar-4` | `--lui-text-3` |
| `.lui-avatar-5` | `--lui-text-5` |
| `.lui-avatar-square` | `--lui-radius` |
| `.lui-avatars` |  |
| `.lui-avatar-more` | `--lui-gray-3`, `--lui-fg` |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `alt=""` | 1 | 1 | 1 |
| `loading="lazy"` | 77 | 75 | 15.4 |
| `role="img"` | 1 | 1 | 1 |

- Fallback: the initials are the fallback
- Needs script: no
- Responsive by: content
- Look after: Radix Themes Avatar

## Documentation

A round picture of a person, with their initials underneath for when there is no picture
or it fails to load. After Radix Themes Avatar: five sizes, the initials on a soft brand
tint, or any `Tone` soft or solid, and `.radius(..)` for the
corners.

**Platform features:** the initials are painted first and the `<img>` is stacked over
them; an image that fails to load has `alt=""`, so it renders nothing and the initials
show through, with no `onerror` handler. `loading="lazy"` defers images below the fold.
The root is `role="img"` named after the person, so a screen reader says the name once.

**Accessibility:** `role="img"` named by the person's name; the initials inside are
`aria-hidden`. Checked by axe-core in headless Firefox on every demo route, both capability
variants, light and dark (no serious or critical violation).

**What it does not do without script:** nothing is missing.

**Fallback:** the initials are the fallback.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.avatar("Ada Lovelace").render().into_string();
assert!(m.contains(r#"role="img" aria-label="Ada Lovelace""#) && m.contains(">AL</span>"));
// The same in `lui!`:
let same = lui! { Avatar("Ada Lovelace"); };
assert_eq!(same.into_string(), m);
let m = ui.avatar("Grace Hopper").src("/img/grace.jpg").large().render().into_string();
assert!(m.contains(r#"<img src="/img/grace.jpg" alt="" loading="lazy">"#) && m.contains("lui-avatar-4"));
// A row of overlapping avatars, three shown and a count of the rest.
let team = ui.avatars("Team").max(3).avatar("Ada Lovelace").avatar("Grace Hopper").avatar("Alan Turing").avatar("Ken Thompson");
let team = team.render().into_string();
assert!(team.contains(r#"role="group" aria-label="Team""#) && team.contains(r#"aria-label="+1""#));
let same = lui! { Avatars("Team") max=3 { avatar "Ada Lovelace"; avatar "Grace Hopper"; avatar "Alan Turing"; avatar "Ken Thompson"; } };
assert_eq!(same.into_string(), team);
// A solid gray tile with rounded corners.
let bot = ui.avatar("Build Bot").tone(Tone::Neutral).variant(Variant::Solid).radius(Radius::Medium);
let bot = bot.render().into_string();
assert!(bot.contains(r#"data-tone="neutral" data-variant="solid" data-radius="medium""#));
assert_eq!(lui! { Avatar("Build Bot") tone="neutral" variant="solid" radius="medium"; }.into_string(), bot);
```
