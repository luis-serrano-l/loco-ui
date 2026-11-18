# OnboardingPage

`loco_ui::blocks::onboarding_page` · demo: `/blocks/onboarding`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `OnboardingPage` | `ui.onboarding_page(title: &str)` | beta |

## Props of `OnboardingPage`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `description` | value | `text: &'a str` |  |  | A line under the title. |
| `at` | number | `n: usize` | `0` |  | The step shown, counting from 0; the ones before it are done. One past the last shows every step done. |
| `step` | item | `title: &'a str` |  |  | A step, after the ones added before it. |
| `href` | modifier | `href: &'a str` |  | `href` | The last step's page: its title is a link once the step is done. |
| `skip` | value | `href: &'a str` |  | `href` | A Skip link under the card, to the app. |
| `body` | value | `body: Markup` |  |  | The current step: its form, in a card. |

## Keyboard of `OnboardingPage`

| Keys | What they do | Needs the script |
|---|---|---|
| `Enter` | Follows a finished step's link or the Skip link. | no |

## Anatomy

| Class | Tokens it reads |
|---|---|
| `.lui-onboarding` | `--lui-space-6` |
| `.lui-onboarding-head` |  |
| `.lui-onboarding-progress` | `--lui-space-1`, `--lui-muted`, `--lui-text-2` |
| `.lui-onboarding-description` | `--lui-space-1`, `--lui-muted` |
| `.lui-onboarding-skip` | `--lui-muted`, `--lui-text-2`, `--lui-fg`, `--lui-hit` |

## Documentation

The first-run page of an app: where the person is ("Step 2 of 3"), a title and a line, the
steps as a row, the current step's form in a card, and a way to skip. Each step is a page
of its own, so Back and a reload keep the place. After the Origin UI Stepper in an
onboarding card, built from `crate::steps` and `crate::card`.

**Platform features:** those of `crate::steps` (an ordered list, `aria-current="step"`,
a container query that turns the row into a column under 30rem); the page is a column
capped at a reading width.

**Accessibility:** the title is the page's `<h1>`, with the position in words before it;
finished steps with an `href` are links back; Skip is a link. Checked by axe-core with the
demo routes.

**What it does not do without script:** nothing is missing; each step is a round trip.

**Fallback:** none needed.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.onboarding_page("Set up your workspace")
    .step("Profile").step("Team").step("Invite")
    .body(html! { p { "form" } }).render().into_string();
assert!(m.contains(r#"<p class="lui-onboarding-progress">Step 1 of 3</p><h1>Set up your workspace</h1>"#));
// The second step, with a way back and a way out:
let m = ui.onboarding_page("Set up your workspace").description("About two minutes.")
    .at(1).step("Profile").href("/welcome/1").step("Team").step("Invite")
    .skip("/app").body(html! { p { "form" } }).render().into_string();
assert!(m.contains("Step 2 of 3") && m.contains(r#"<a href="/welcome/1">Profile</a>"#));
assert!(m.contains(r#"<a class="lui-onboarding-skip" href="/app">Skip</a>"#));
// The same in `lui!`:
let same = lui! { OnboardingPage("Set up your workspace") description="About two minutes." at=1 skip="/app" {
    step "Profile" href="/welcome/1"; step "Team"; step "Invite";
    body { p { "form" } }
} };
assert_eq!(same.into_string(), m);
```
