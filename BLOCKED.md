# Blocked

Questions only the owner can answer. The name, hosting and Loco questions were answered on
2026-09-24, copy-paste mode and the first version on 2026-10-14 (below). What is still with
the owner, as of 2026-10-15:

1. The crates.io publish, as 0.2.0 (on hold until you say yes).
2. Posting the announcement (M26) and asking Loco for a link (M29), both after the publish.
3. Filing the Blitz and browser gaps that FINDINGS marks "the owner files it".
4. The Dependabot pull requests that fail CI.
5. The `lui!` builder mode for the `/wizard` and `/palette` snippets.

## Name, license and repository (M13, box 1), answered

Answered 2026-09-24: **license is MIT**. `license = "MIT"` is set in both manifests, and
`LICENSE` sits at the root with `luis-serrano-l` (the git user name) as the copyright holder.
Edit that line if you want your full name there.

Answered 2026-09-24: the name was `axum-nojs`. **Renamed 2026-09-25 to `loco-ui`**: "nojs"
promised no JavaScript while an optional script ships. The crates are `loco-ui`,
`loco-ui-caps`, `loco-ui-macros` and `loco-ui-test` (not published); classes, tokens, cookies,
attributes and routes use the `lui-` prefix (`.lui-dialog`, `--lui-accent`, `lui-ui`,
`data-lui`, `/lui/enhance.js`), and the macro is `lui!`. `loco-ui` was free on crates.io on
2026-09-25. Yours: rename the GitHub repository to `loco-ui` (GitHub redirects the old URL)
before the next push, since the manifests and README already point there.

Answered 2026-09-24: **the repository is https://github.com/luis-serrano-l/loco-ui**
(public), set as `repository` in both manifests.

## The publish itself (M13, box 4), on hold

On hold at the owner's request (2026-09-24). The first publish is **0.2.0** (answered
2026-10-14, below); nothing is on crates.io yet. The rename is done, `repository` is set, and
M37 made the three crates ready (`publish-dry-run` and `docsrs` jobs in CI). The manifests
still say 0.1.0: the `release` skill bumps them and stops before `cargo publish`. Publish
only on your yes; it is never run on your behalf.

## Rename to `nojs-ui`? (M26, box 1), answered

Answered 2026-09-24: keep `axum-nojs`. Reversed 2026-09-25: renamed to `loco-ui` (above).

## Launch: host the demo, post the announcement (M26, last box), partly answered

Hosting answered 2026-09-24: **a static snapshot on GitHub Pages.** Pages serves only static
files, so the snapshot shows every page as it renders, and `<dialog>`, `popover`, `<details>`
and tooltips still work. Forms, cookies and paging need the server, and a banner on every page
says so. The export (`scripts/snapshot.sh`) and the deploy (the `pages` job in `.github/workflows/rust.yml`,
run only after every other check passes) are done.
Yours: Settings → Pages → Source "GitHub Actions", then push `main`; the site lands at
https://luis-serrano-l.github.io/loco-ui/ (put that link in `docs/launch-post.md`).

Posting stays with you and waits for the publish: the draft is `docs/launch-post.md`, and the
suggested order is crate first, then r/rust and This Week in Rust, with the Pages link filled in.

Done locally: the crates.io keywords are `no-js`, `maud`, `ssr`, `components` and `axum`.
`progressive-enhancement` is longer than crates.io's 20-character limit, so the phrase is in
the description instead.

## Make Loco the primary target? (M27, box 1), answered

Answered 2026-09-24: **yes, as a `loco` feature on `loco-ui`**, not a separate crate. The rest
of M27 is unblocked. The Loco API names in ROADMAP M27 are from memory and get checked against
loco-rs's source before anything is built on them.

## Reply to Blitz issue #923

Posted on 2026-09-23 after the owner's approval:
https://github.com/DioxusLabs/blitz/issues/923#issuecomment-5800768284

## Ask Loco to link loco-ui (M29, last box), open

An outward action, and it waits for the publish (M13/M29): a crate that is only a git
dependency is a hard thing to link to. Suggested, once `loco-ui` is on crates.io: a post in
Loco's GitHub Discussions ("Show and tell"), not an issue, along these lines:

> loco-ui: server-rendered scaffolds for Loco 1.x. `cargo lui install` adds an initializer
> and scaffold templates, so `cargo loco generate scaffold` writes HTML controllers and Maud
> views (Post/Redirect/Get, validation that re-renders, paging from `PagerMeta`), and
> `cargo lui auth` writes sign-in, sign-up, reset and magic-link pages. Every page works with
> JavaScript off (tested in CI with a script-less renderer); an optional 11 KB script updates
> pages in place. Example app: examples/loco-app. Would a link from the generators docs,
> next to the note on the removed `--html` scaffolds, be welcome?

Yours: whether, when and where to post it.

## File the Blitz and browser gaps upstream (M30 and later), open

Filing an issue is an outward action, so these wait on you. Each FINDINGS.md entry below says
"no upstream issue yet; the owner files it" (or close to it). Blitz is 0.3.0-beta.2.

M30 · The Linear / Magic UI look:

1. `@starting-style` is parsed but never applied: Stylo resolves starting styles only in its
   Gecko build (`maybe_resolve_starting_style` is `cfg(feature = "gecko")`).
2. `get_client_bounding_rect` ignores `translate` and `scale`.
3. `@property` is unsupported: a registered property's `initial-value` is ignored and
   `@keyframes` cannot animate it.
4. `@supports (animation-timeline: view())` is true, but no scroll timeline runs.
5. `@media (prefers-reduced-motion: no-preference)` never matches.
6. The card `.glow()`, a `z-index: -1` `::after` inside an `isolation: isolate` element with
   a sized `radial-gradient`, is not painted; the cause is not isolated yet.

Later milestones:

7. M34: Stylo does not know `interpolate-size`.
8. M36: Blitz paints a `position: relative` element with a `z-index` at the page origin.
9. M39: Blitz always takes the light side of `light-dark()` (Stylo's servo device answers
   `is_dark_color_scheme` with `false`).
10. M41: a whole-row link needs `position: relative` on a `<tr>`; Safari is not checked.
11. M41: Blitz paints the sticky first column, the group rows and the tinted rows at the
    page origin (the M36 gap again).
12. M40d: Blitz has no `popover`, `::scroll-button()` or `::scroll-marker`.
13. M40b: Blitz places an absolutely positioned `<summary>` at its static position.

Suggested: one issue per item on github.com/DioxusLabs/blitz (items 1, 3, 4 and 5 can share
one "motion features" issue if the maintainers prefer; item 9 may belong on servo/stylo; item
10 is a Safari check, not a Blitz bug), each with a minimal HTML file, then replace the
phrase in FINDINGS.md with the links. Nothing in the library waits on it: every effect has
its at-rest fallback proved in Blitz today.

## The Timeline copies Flowbite (M40b), answered

CLAUDE.md convention 9 says each component copies one of Radix Themes, Origin UI, shadcn/ui,
Tremor or Dice UI. `loco-ui/src/timeline.rs` (M40b) copies the Flowbite Timeline, which is
not on that list, as the M40b box asked. Answered 2026-10-15: restyle it. It now copies the
Origin UI Timeline.

## Dependabot pull requests fail CI, open

The Dependabot pull requests were cut before the CI fixes on `main` (the latest is
3383650), so they fail for reasons that are not theirs. Rebasing them (`@dependabot rebase`) is an action
on GitHub, so it is yours. One may need code: `toml_edit` 0.22 to 0.25 (the `cli` feature,
`loco-ui/src/bin/cargo-lui/`) changes its API, so that bump should be tried locally before
it is merged.

## One snippet style: `/wizard` and `/palette` (M34, "One snippet style" box), open

`/counter` now shows `lui!` (its rules are three constants the post handler shares). The
wizard and the palette cannot follow without a choice: their handlers call methods on the
builder after building it (`wizard.at(step)`, `.is_last(..)`, `.link(..)`; `palette.exact()`),
and `lui!` only returns `Markup`. The options:

1. **A builder mode in `lui!`** (suggested): for example `lui!(build: Wizard("signup",
   "/wizard") { step "Account" (..); .. })`, expanding to the same chain but returning the
   builder instead of wrapping it in `html!`. The component functions keep one definition,
   the snippets read like every other page, and the macro's `component()` already produces
   that expression, so it is small. It is new macro syntax, which is why it is yours.
2. Show `lui!` on the page and keep a second builder chain for the handler, from shared data.
   No macro change, but each page defines its steps or links twice.
3. Keep the chain on these two pages, and let the snippet test allow it.

The demo test that stops snippets drifting back waits on this answer.

## Copy-paste mode (M29 / M43), answered and built

Answered 2026-10-14: **yes, build `cargo lui add <component>`**, option 1: the vendored
file is the component's own `loco-ui/src/<name>.rs` kept whole (header, `PROPS`, CSS,
doctests), registered in the app through an extension trait on `Ui`. M43 built it (`cargo lui add`);
22 modules can be copied, the others use crate-private helpers.

## First published version, answered

Answered 2026-10-14: the API under `[0.1.0]` in CHANGELOG no longer exists, so the first
publish is **0.2.0**. CHANGELOG has `[0.1.0]` as an unpublished prototype and every change
since under `[0.2.0] - unreleased`, with a list of the breaking changes. The publish itself
stays with the owner.

## Text setters: `Cow` or `&'a str` (M38), decided, yours to reverse

M38 asked for `impl Into<Cow<'a, str>>` text setters only if a spike showed `lui!` and
`PROPS` survive them. The spike on `Dialog::title` compiled and `lui!` was unaffected, but
`PROPS` then lists two spellings of one argument and every builder would have to change at
once, so the change was rejected: text setters stay `&'a str`. The reasoning is in
`docs/api.md`, "Text arguments borrow (`&'a str`, not `Cow`)". `Tokens` values are
`Cow<'static, str>` (M39); that is a separate case. Nothing to do unless you want `Cow`
after all.
