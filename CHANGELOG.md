# Changelog

All notable changes to `loco-ui` and `loco-ui-caps`. Both crates share a version. The format
follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/); the project uses
[Semantic Versioning](https://semver.org/).

## [Unreleased]

Everything below also ships in 0.2.0, the first publish.

#### M37

- Ready for crates.io and docs.rs, nothing published: `rust-version = "1.88"` (the `loco`
  feature needs 1.94, as loco-rs 1.2 does), `homepage` and `documentation` in the three
  manifests, a `LICENSE` in each crate, `loco-ui-macros` with a README of its own. The
  docs.rs build uses `doc_cfg` (nightly merged `doc_auto_cfg` into it), and the three
  crates `forbid(unsafe_code)` and deny broken intra-doc links at every feature level.
- CI: `docsrs`, `msrv`, `publish-dry-run` (`cargo package`), `deny` (`deny.toml`) and
  `semver` (reports only, until 0.2.0 is tagged). `.github/` has a contributing guide, a
  security policy, issue and pull request templates and dependabot.

#### M44

- Repo tooling: skills `new-component`, `look-check`, `roadmap-tick` and `release`, the
  `component-reviewer` agent, a component template (`loco-ui/src/_template.rs.txt`, not
  packaged), hooks for `cargo fmt` and the `<script>` check, and `scripts/check-script.sh`
  shared by `verify.sh`, CI and the hook.

#### M46

- Property tests (`proptest`, `loco-ui/tests/prop.rs`): any path, query and cookie build a
  `Ui`, a `UiState` and a `TableQuery` without a panic; `href`, the `lui-ui` cookie, `Caps`
  (query and cookies) and `Date` read back what they write; `slug` stays id-safe and stable;
  `minify_css` keeps every brace and is idempotent. Random token streams through the `lui!`
  walker give markup or an error, never a panic.
- The demo parses every route with `html5ever` under all and no capabilities, light and
  dark, and fails on parse errors, duplicate ids, references to ids that do not exist,
  interactive content inside a link, button or `<summary>`, nested forms and `<img>`
  without `alt`. The bugs it found are listed in `CONFORMANCE_BUGS` with the milestone
  that fixes each.
- `insta` snapshots of every builder's header example under all and no capabilities in
  `loco-ui/tests/snapshots/`; review changes with `cargo insta review`.
- A test renders each setter whose `PROPS` entry names an HTML attribute and checks the
  attribute appears.
- `scripts/verify.sh` skips the Blitz crate unless given `--full`.

#### M45

- `Strings` carries more than words. `Strings::new(tag)` takes CLDR data for en, es, fr, de,
  ar, he, ru and pl from the tag (English's for any other): `dir()` (written as
  `<html dir>`), a plural rule, number separators and the first day of the week, each
  overridden by `with_dir`, `with_plural`, `with_number` and `with_first_day`. `?dir=rtl` or
  `?dir=ltr` on any page overrides the direction (`ui.dir()`).
- `Strings::count(text, n)` and `ui.count(..)` pick a counted text's form: a text written
  `one:{} match|other:{} matches` holds one form per CLDR category (`Plural`), `other`
  standing in for a missing one. `Text::Matches` is written that way; `Text::OneMatch` is
  deprecated and no longer read.
- `Strings::number(f64)` and `ui.number(..)` write `1,234.5`, `1.234,5`, `1234` / `12.345`
  (Spanish) or `1 234,5`. The paged table's range and page numbers, the pager, counter,
  meter and stat use them.
- `Strings::date` and `Strings::date_long`. The calendar's column heads come from new texts
  (`Text::MondayShort` to `SundayShort`) instead of the first two letters of the weekday,
  and its rows start on the language's first day; `.sunday_first()` still wins.
- App texts: `Strings::with_custom(&[(key, text)])`, read by `Strings::custom(key)` and
  `ui.text_key(key)` (visitor's language, then the default one, then the key).
- `Strings::from_lookup` keeps its tables in a fixed `OnceLock` registry (32 languages)
  instead of leaking a box per table.
- The star after a required label is `<span class="lui-field-required" aria-hidden="true">`;
  a form with a required field shows "* required" (`Text::RequiredHint`, also hidden from
  screen readers, which hear the `required` attribute).
- Validator messages (`FieldErrors`, `Submitted`, `Valid`) come from new `Text::Field*` and
  `Text::FormCheck` texts: `Valid` writes the visitor's language, the others the app's
  default; `Submitted::strings(ui.strings)` picks the visitor's.
- `Text::month(0)` no longer underflows. `i18n::languages(&[])` no longer panics on the next
  request.
- **Breaking:** `layout::layout` and `layout::layout_with` are deleted (both wrote
  `lang="en"`); use `ui.page(..)`. `Strings` gained fields, so a struct literal of it no
  longer compiles (its fields were private already); its `PartialEq` leaves out the plural
  rule.

#### M38

- New `loco_ui::variant` with four enums in the prelude: `Size` (`S1` to `S3`, also
  `Size::ONE`.., and `From<u8>` saturating), `Tone` (neutral, brand, ok, warn, danger, info),
  `Variant` (solid, soft, surface, outline, ghost) and `Radius` (none, small, medium, large,
  full). Each has `NAMES`, `as_str`, a `const fn parse`, `Display` and `FromStr`.
- `PropKind::Choice { of }` lists a setter's names; the spec JSON writes them as `"choices"`,
  the demo's props tables show them and the playground offers them in a `<select>`.
  **Breaking** for code that matches on `PropKind` exhaustively.
- `lui!` looks up a string given to `size`, `tone`, `variant` or `radius` at compile time:
  `tone="danger"` is `.tone(Tone::Danger)`, and `tone="dangr"` is a compile error at the
  literal. `size=2` and `tone=(Tone::Danger)` work as before.
- New setters: `.size(impl Into<Size>)` on Button, Badge, Alert, Card, Tabs, Input, Select,
  ToggleGroup and Progress; `.tone(Tone)` on Button, Badge, Alert, Avatar and Progress;
  `.variant(Variant)` on Button, Badge, Alert, Avatar, Card, Input, Select, Progress and
  Flash; `.radius(Radius)` on Button, Badge, Avatar, ToggleGroup and Progress. Each value
  that is not the component's default is also written on the root as `data-size`,
  `data-tone`, `data-variant` or `data-radius`. **Breaking**: `Button::size` and `Badge`'s
  tone and look now take these types (a `u8` still works through `From<u8>`).
- Button's tone and variant compose: `.danger().ghost()` is a red ghost button and
  `.primary()` is `.tone(Tone::Brand).variant(Variant::Solid)`. The switch setters stay, and
  the five looks that had a class keep it (`lui-button-primary` and so on), now beside
  `data-tone` and `data-variant`. **Breaking** in the HTML: a primary, danger, ghost, soft or
  surface button, a small or large one, and a toned badge carry the new attributes.

#### M43

- **Breaking (installer):** the `cargo-lui` binary needs the new `cli` feature
  (`cargo install --git .. loco-ui --bin cargo-lui --features cli`). The feature brings
  `toml_edit` and `serde_json` for the binary only; `cargo tree -p loco-ui` without it lists
  no new crate.
- `cargo lui add <name>..` copies a component's file from the app's loco-ui dependency into
  `src/ui/app_<name>.rs`, whole (header, doctests, `PROPS`, `CSS`), as `ui.app_<name>(..)`
  with an `App<Name>Ext` trait, registered in `src/ui/mod.rs`, `src/lib.rs` and the `css`
  list of the app's `Look`. `--prefix`, `--dry-run`, `--diff`, `--view`, `--force`; `cargo lui
  list`, `cargo lui info <name>` (the setters), `cargo lui eject` (all of loco-ui into
  `vendor/`). 26 components can be copied; the rest use crate-private helpers.
- `cargo lui new <app>` writes an Axum app (app shell, settings, theme toggle, 404 page, CSP,
  `Look`, a page test, `scripts/check.sh`); `--loco` builds on `loco new` (or `--from DIR`)
  with `install`, `auth` and a demo account seeded in development.
- `cargo lui install` adds `src/views/look.rs` and the `before_routes` (404 page) and
  `after_routes` (look and strict CSP) hooks, pins the git dependency to the installer's
  commit (`--rev` overrides), edits `Cargo.toml` with `toml_edit` (CRLF and dotted tables
  kept), and takes `--dry-run`, `--help` and `--version`.
- The scaffold's list searches (`LIKE` over text columns) and sorts (every column
  `sortable`) in SQL through `ui.table_query`, has a CSV download (`/<plural>/export.csv`),
  and ends with a generated test of its query; `cargo lui scaffold .. --parent <model>` adds
  `/<parents>/{id}/<plural>` routes. Its titles and flashes use `Text::New`, `Edit`,
  `Created`, `Saved`, `Deleted`. The scaffold's `views::<plural>::list` takes a `base` path
  (**breaking** for a template copy that calls it, not for generated apps).
- `RecordPage::delete` opens a dialog that asks first ("Delete Invoice 42?",
  `Text::DeleteConfirm`, `Text::CannotUndo`) instead of posting at once.

## [0.2.0] - unreleased

The first publish. The API of the 0.1.0 prototype is gone: components are builders reached
from `ui` (M18) instead of functions taking options structs, so this section lists changes
by milestone, newest first, rather than against 0.1.0.

### `loco-ui`

#### After M36: input hardening, `lui!` templates, sign-in redirects
#### M39 · CSS that an app can own

- **Breaking.** The stylesheet is five cascade layers, `lui.reset, lui.tokens,
  lui.components, lui.blocks, lui.modes`; `Look::css` and `Page::css` come unlayered after
  it, so an app's rule wins without `!important`. `LAYERS` and `layout::RESET` / `MODES`
  are public.
- **Breaking.** Every colour token is one `light-dark(light, dark)` declaration on `:root`
  with `color-scheme: light dark`; `data-theme` sets only `color-scheme`. Chrome before 123
  shows the page unthemed. `layout::DEPTH_LIGHT`, `DEPTH_DARK`, `EFFECTS_LIGHT`,
  `EFFECTS_DARK`, `Palette::declarations` and `DerivedScale::css` are gone (`EFFECTS`,
  `Tokens::declarations` replace them). The stylesheet went from 117.2 to 111.4 KB
  (20.0 to 19.5 KB gzipped).
- **Breaking.** `Tokens` values are `Cow<'static, str>` (a palette from a database is a
  value) and it gains `font: Fonts`, `type_scale: TypeScale` (`--lui-text-1..9`,
  `--lui-leading-1..9`), `size: Sizes` (`--lui-size-1..3`), `radius: Radius` (a ramp
  `--lui-radius-{none,1,2,3,full}` with `--lui-radius-factor`; build one with
  `Radius::new("0.5rem")`), `shadow: Depth`, `motion: Motion`, `z: ZIndex` (`--lui-z-*`)
  and `density: Density` (`Compact` scales spacing and control heights by 0.85). `Palette`
  gains `info` and `on_info`. `Scale::new(light, dark)` builds a scale in a `const`;
  `DerivedScale` converts into a `Scale`.
- **Breaking.** No default header: `layout::header()` is gone and `Look::header: None`
  shows nothing above `<main>`. The demo's header and every demo-only rule (`.lui-site*`,
  `.lui-props*`, `.lui-index*`, `.lui-snippet`, `.lui-hl-*`, the theme builder) moved to
  the demo's own `Look` in `demo/src/site.rs`.
- `Ui::tokens(Tokens)` themes every page of one request (a tenant's palette).
- `Page::linked_stylesheet()` links `/lui.css` (served by `enhance::router()` with an ETag,
  `immutable` and 304s); `enhance::csp` then drops `'unsafe-inline'` for `<style>` and
  allows the page's other inline styles by hash. `enhance::csp_hash`, `Page::style_hashes`.
- `stylesheet_for(&[Component])` and `Page::subset(&[Component])`: only the listed
  components' CSS plus `Component::BASE` (32.3 KB raw for the base).
- Forced colours, more contrast and print rules (`layout::MODES`), with `.lui-forced` and
  `.lui-contrast` emulation classes; the demo's `/modes` page shows them.
- A skip link (`Text::SkipToContent`) first in `<body>` and `<main id="main" tabindex="-1">`.
  The enhancement script moves a lost focus into the swapped root and announces the new
  title through a `role=status` region.
- Two lint tests: `no_literal_font_size_or_z_outside_tokens` and
  `no_physical_property_in_component_css`, with the files still to sweep listed in them.

#### Earlier


- Fixed two panics a request could trigger. A query with a non-ASCII character right after `%`
  (`?x=%aé`) no longer panics while it is decoded. A redirect whose target holds a control
  character (`/signin?next=/%0A`) now goes to `/` instead of panicking in
  `Redirect::into_http`, and `loco::landing` refuses such a `next`. Set-Cookie values that are
  not valid header values are skipped instead of unwrapped.
- More odd input that no longer panics or overflows. `Date::new` and `Date::parse` take years
  1 to 9999 only, as `<input type="date">` does, so `?due=999999999-01-01` renders the default
  month. `ui.pager` holds `?page=` to the list's length. A streamed page with an unterminated
  `<!--lui-slot:` keeps the rest as text. `lui!` reports `rows=move` with nothing after it as
  an error instead of panicking. `cargo lui` stops with the path when it cannot read a file it
  edits, instead of writing it over as empty.
- `cargo lui auth`'s controller tells a missing account or spent link apart from a database
  error: an outage is an error page, not "Wrong email or password." or "Link expired".
  examples/loco-app does the same on its home page, and its `start -d` / `stop` report a bad
  port, a failed spawn or a failed `kill` instead of panicking or claiming success.
- The generator templates write their markup in `lui!`, as the demo does: `cargo lui auth`'s
  account pages (`AuthPage` holding a `Form`) and the scaffold's list, show and form views
  (`Table` with a `column` per field, `RecordPage` with a `field` per value, `Form` with an
  adder per field). `examples/loco-app` follows, views included; controllers keep the builder
  only where they call a method on it (`table.page()`). The HTML is unchanged. `docs/loco.md`
  and the `loco` module's examples are in `lui!` too. The installer's ignored test now
  generates three scaffolds (a field of every kind, enums required and optional, a required
  and an optional reference) and builds them; it needs `sea-orm-cli`, which CI installs.
- `Combobox::keep(name)`, as `Table::keep`: a query parameter of this page (`view`, a search
  of its own) that the search form, every result and every chip's remove link carry, so
  picking a value no longer drops it. examples/loco-app's tag box keeps the search and the
  cards/table choice.
- `loco_ui::href(path, pairs)`: another page's URL with its query parameters, each
  percent-encoded as the components' own links are (`href("/notes", [("sel", "c#")])` is
  `/notes?sel=c%23`). `ui.link_with` and `ui.link_without` build theirs with it, and
  examples/loco-app's tag links use it instead of the `form_urlencoded` crate.
- Fix: the scaffold's select for an optional enum starts with an empty "None" choice, as an
  optional reference's does; without it the browser picked the first variant and saving the
  form wrote it.
- examples/loco-app: a tag link encodes its tag (`c#`, `r&d` filtered by the wrong tag), and
  the cards/table switch keeps the search. The screenshot test fixes when each note changed
  and was created, so only the overview's shot (its chart counts back from today) still
  moves with the date.
- `loco::SignIn("/signin")`, a Loco initializer (and `loco::sign_in` for a hand-built
  router): a 401 becomes a redirect to the sign-in form with a flash
  (`Text::SignInToSee`), naming the page asked for in `?next=` on a GET; a JSON-only
  request keeps its 401. `loco::landing(ui, home)` reads `next`, same-site paths only.
  `cargo lui auth` adds the initializer, and its sign-in form and handler carry `next`, so a
  signed-out visitor lands on the page they wanted after signing in.
- Fix: the enhancement script no longer copies a urlencoded POST's fields into the URL
  (a sign-up put the password in the request line and the server log).

#### M36 · A description list that lines up, and one look per app

- The description list puts every term in one shared column (subgrid) from 30rem, both
  tops on one line, rows split by hairlines; narrower, each term sits above its detail.
  An empty detail shows a dash. **Visual change.**
- `ui.record_page` renders a description list; its `.lui-record-field` class is gone.
- `layout::Look { tokens, css, header }`, `Ui::look` and `Look::layer()`: one layer themes
  every page of an app. `Tokens::DEFAULT` is the default tokens as a constant.
- `Page::header(markup)` replaces loco-ui's site header on a page.
- `ui.app_shell` takes `.group`, `.icon` and `.badge` like `ui.sidebar`; its links are
  now a sidebar's markup.
- Templates: the scaffold's show page is `ui.record_page`; the account pages are
  `ui.auth_page` cards.

#### M35 · Cookies and cross-site posts

- `cookie::SetCookie` writes every cookie (`Path=/`, `SameSite=Lax`, optional `HttpOnly`),
  adding `Secure` when the request was HTTPS. The extractors decide that with
  `caps::is_https`; other servers call `Ui::secure(bool)`, and `Ui::is_secure()` reads it.
- The `Ui` extractor refuses a cross-site non-GET request with `403`
  (`cookie::same_origin`: `Origin`, then `Sec-Fetch-Site`). **Breaking:** its `Rejection`
  is `Response`, no longer `Infallible`.
- `lui-ui` is capped at `state::MAX_COOKIE` (3 KB). Remembered keys this request did not
  set are dropped first.
- `Saved<T>` names its cookie after serde's container name, so `#[serde(rename = "..")]`
  separates two types with one short name; unrenamed types keep their cookie. **Breaking:**
  `Redirect::save` and `forget` need `T: DeserializeOwned`.
- Scaffold templates: with auth and a `user_id` field, the owner comes from the session and
  every query is scoped to it (another user's row is a 404). Without one, the generated
  header says every signed-in user sees every row. The auth template's `auth` cookie is
  `Secure` over HTTPS.

#### M33 · Follow-ups from M32

- A table's query keys carry its id, `<key>.<id>` like `per.<id>` and `edit.<id>`:
  `q.files`, `sort.files`, `dir.files`, `page.files`, `cols.files`, so two tables on one page
  sort, filter and page on their own. `table::Keys::new(id)` spells them. The bare `q`, `sort`,
  `dir`, `page` and `cols` are deprecated and still read, for one release, when the table's own
  key is absent. The load-more `Pager` keeps its bare `page`.

#### M32 · An easy, simple and consistent API

- Less code per page: `Form` holds any control (`.body(markup)` among its fields,
  `.switch(name, label)`) and `.get()` makes it a filter; `Page::invalid()` answers 422, so a
  POST handler returns `Result<Redirect, Page>`; `Posted` (feature `axum`) reads an urlencoded
  or multipart post by name (`get`, `all`, `pairs`, `files`, and a wizard's `step`/`skip`;
  `wizard::Posted` is deprecated); `ui.page` shows a pending flash unless the body already
  does; a card's footer packs at its end. `lui!` takes a markup block as an attribute value
  (`footer={ .. }`) and `body { .. }` among items, and names the fix when an item is given a
  component. **Breaking:** `ui.stack()`, `ui.cluster()` and `ui.grid(min)` take their content
  as `.body(..)` (`Stack gap=6 { .. }` in `lui!`).
- One word per concept (`docs/api.md` has the table): `multiple` (was `multi`),
  `description` (`summary`, `note`, `message`), `help` (`hint`), `body` (`content`, EmptyState
  `text`), `action` (EmptyState `post`), `group` (Menu `heading`), `option` (ToggleGroup
  `item`), `link`/`links` (Palette `command`/`commands`), `accesskey` (Palette `key`, now a
  `&str`), `icon_only` and `aria_label` (Button `icon()` and `label`), `hide_progress()`
  (Wizard `progress(bool)`), `disabled_dates` (Calendar and DatePicker `disabled`). The old
  names are `#[deprecated]` for one release, and so is `layout_with` (use `Page::tokens`).
  `.icon(..)` takes an `Icon` or a glyph everywhere; `.badge(..)` any `Display`.
- **Breaking:** `ui.select(name, label)`, `ui.color(name, label)`, `ui.range(name, label)` and
  `ui.range_pair(name, label)` read their value from the query, with `.value(..)` (`.values(..)`
  for a pair) for a saved one; their `.label(..)` is gone. Select's id is `f-<name>` like a
  form field's, so the error summary's link reaches it, and it takes `.error(..)`.
- ContextMenu takes Menu's adders and modifiers (`.items(..)` is deprecated); `.id(..)` on
  Meter, Progress, Sidebar and NavMenu; `ErrorSummary::field(name, label)` writes "Label:
  message" as a form's summary does. `props_follow_the_conventions` keeps the API to the rules.

#### M14 to M16 · Enhancement script, component depth, latency

Written when components took options structs; M18 replaced those with builders, so the
names below are the old ones.

- Latency: `stylesheet()` built once and minified (`minify_css`); `enhance::slim` middleware
  answers an enhanced request without the inline stylesheet and sets `Vary: Lui-Enhance, Cookie`
  (`enhance::slim_html` without a framework); the script sends `Accept: text/html`, fetches
  user actions with `priority: "high"`, prefetches links under `data-lui-prefetch` on hover or
  focus and reuses the answer for a click within five seconds; `enhance::served()` is the
  script without comment lines and indentation, and the 10 KB budget applies to it.
- Enhancement script: `data-lui-target`, `data-lui-swap`, `data-lui-oob`, the `Lui-Enhance: 1`
  request header, busy state (`data-lui-busy`, `aria-busy`, disabled submit buttons,
  `data-lui-indicator`, `--lui-busy`), failed requests fall back to a navigation,
  `data-lui-push="false"`, `data-lui-replace`, Back/Forward restore from history state,
  `lui:swap` event. Size limit raised to 10 KB.
- `dialog`: `title` with a close control, `size` (`DialogSize::Sm|Md|Lg`), `danger`,
  `confirm(label, action)` footer as a real `<form method="post">`, `returns_to`,
  `cancel_label`, `closedby`. `button.lui-danger` in the base styles.
- `popover_menu` takes `&[MenuItem]` and `PopoverOptions`: links, post-form actions, headings,
  separators, nested submenus, icons, shortcut labels, disabled and danger items,
  `Placement::BottomStart|BottomEnd|Right`. The enhancement script walks an open menu with the
  arrow keys. **Breaking:** the `(text, href)` tuple form is gone.
- `tabs` takes `&[Tab]` and `TabsOptions` (`state`, `vertical`, `select_below`): badges,
  lazy tabs (`Tab::lazy`), a vertical strip, a `<select>` under 40rem, the open tab's underline carries
  `view-transition-name` (only the bar morphs, the title never moves). `UiState::path()`. **Breaking:** the `(title, body)` tuple form and
  the `Option<&UiState>` argument are gone.
- `accordion` takes `&[AccordionItem]` (`icon`, `summary` line) and `AccordionOptions`
  (`state`, `multi`, `controls`): several sections open at once through `?open.<group>=0,2`,
  "Expand all" / "Collapse all" links, nested accordions with their own key.
  `UiState::opens()` returns the list. **Breaking:** the `(title, body)` tuple form and the
  `Option<&UiState>` argument are gone.
- `combobox` takes `ComboboxOptions` (`query`, `suggestions` as `OptionGroup`s with
  `<optgroup>`, `results`, `selected`, `multi`, `create`, `label`, `placeholder`): the
  selection shows as removable chips and rides along as `sel` fields, results are links that
  select, a "Create" post row appears when nothing matches, results carry `aria-live`, and
  the enhancement script walks input and results with the arrow keys. **Breaking:** the
  `(action, name, options, value)` positional form is gone; `name` now comes before `action`.
- `UiState::link(key, "")` keeps `key=` in the URL so an explicit empty beats the cookie.
- `table` takes `&[Row]` (`key`, `detail`, `menu`) and more `TableOptions`: `cols` with
  `cols_from_query` and a "Columns" chooser, `bulk` (checkboxes owned by a post form through
  the `form` attribute), `csv`, `empty`, `loading` (skeleton rows, `aria-busy`);
  `Column::numeric` and `Column::width`. `PagedTableOptions::table` carries them through the
  pager. **Breaking:** rows were `Vec<Markup>`; `Column` has two more fields.
- `counter` takes `CounterOptions` (`min`, `max`, `step`, `typed`): buttons disabled at the
  bounds, a number field posting `op=set`, and `CounterOptions::apply` for the handler.
- `range::range_pair` (two thumbs on one track, `<name>_min`/`<name>_max`) and `range::order`.
- `color` takes `ColorOptions` (`presets` posting `<name>-preset`, `alpha` slider posting
  `<name>-alpha`); `color::hex_alpha`. The swatch paints through `--lui-color-value`.
- `select` takes `&[select::Group]` of `SelectOption` (`icon`, `content`) and `SelectOptions`
  (`search(action, query)`, `search_over`): optgroups, icons, and a GET filter box over 15
  options. The root is now a `span.lui-select` around the `<select id=name>`.
- Enhancement script: honours a submitter's `formmethod` and `formaction`; a search box
  followed by a submit button filters through it.
- **Breaking:** `counter`, `color` and `select` signatures changed.
- `flash` takes `FlashOptions` (`dismiss(href)`, `auto_hide`): levels `info|ok|warn|danger`
  from a `level:` prefix per line (`flash::stack`, `flash::parse`, `flash::Level`), several
  messages stacked, `role="alert"` for danger, a CSS fade for info and ok that reduced motion
  turns off. `--lui-warn` token (`Palette::warn`). **Breaking:** `flash` gained an argument;
  `Palette` gained a field.
- New components: `toasts` (`ToastOptions`; the flash cookie in a fixed corner stack that
  fades, pauses on hover, danger stays), `breadcrumbs` (folds the middle of a long trail
  into `<details>`), `skeleton` (`SkeletonOptions`; shimmer bars with `aria-busy`, used as the
  streamed slot placeholder in the demo), `empty_state` (`EmptyOptions`: icon, text, link,
  post), `stat` (`StatOptions`, `Trend`; `.lui-stat-grid`), `drawer` (`DrawerOptions`;
  sidebar above 60rem, modal drawer below, `:target` fallback) and `command_palette`
  (`palette::Command`, `PaletteOptions`, `palette::exact`, `palette::matches`; popover +
  datalist + GET, `accesskey`).
- Enhancement script: carries the toast list across a swap like the flash.
- `form` takes `&[FieldGroup]` (a `<fieldset>` and `<legend>` each, or `FieldGroup::plain`)
  and `FormOptions` (`submit`, `layout: FormLayout::Stacked|Inline`). `Field::new` with
  `value`, `error`, `required`, `help` (tied by `aria-describedby`) and `max_len` (a
  `maxlength` with an `<output>` counter). New `FieldKind`s: `Textarea` (grows with
  `field-sizing: content`), `File` (`accept`, `multiple`; the form becomes multipart), `Date`,
  `Time`. **Breaking:** the signature and `Field` changed.
- Enhancement script: an `<output for>` of a field with `maxlength` counts its characters;
  multipart forms are sent as `FormData`, so files survive an in-place submit.
- `wizard`: `Step::new` with `optional` (a `formnovalidate` Skip button posting `skip=1`) and
  `error` (marked in the step list and on the fieldset), a `<progress>` bar
  (`WizardOptions::progress`, on by default), a "Picked up where you left off" notice with
  Start over when the step came from the cookie (`UiState::remembered`), and
  `wizard::summary` for a review whose values link back to their steps. **Breaking:** `Step`
  has two more fields; build it with `Step::new`.
- `paged_table`: First and Last links, an ellipsis past seven pages, a jump-to-page form,
  counts with thousands separators (`paged_table::thousands`), and `PagedTableOptions::state`
  to remember the page size per table as `per.<id>` (`UiState::per_page`). The whole block is
  now the swap root, so the page links follow an in-place sort.

### `loco-ui-caps`

#### M35

- `is_https(scheme, x_forwarded_proto, forwarded)`; the beacon cookie is `Secure` over
  HTTPS. **Breaking:** `beacon_cookie(query, secure)` and `cookie_for(cap, secure)` take the
  flag. The header now says the first view gets `Caps::ASSUMED`, not the bare fallback.

## [0.1.0] - 2026-09-23 (unpublished prototype)

Never published to crates.io. 0.1.0 named the first working prototype: `loco-ui-caps`
(capability beacons, one cookie per feature), out-of-order streaming through declarative
shadow DOM, query-and-cookie UI state, the optional enhancement script with the test that
allows no other `<script>`, seventeen components as functions taking options structs, the
machine-readable spec, and the Blitz test harness. The guarantees carried over to 0.2.0;
the API did not.
