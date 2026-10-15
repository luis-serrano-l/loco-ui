//! What every page shares: the component index, the shell around each page (the sidebar of
//! every component, toolbar, title, the stage and its code), the index page (a gallery of the
//! components themselves) and the theme switch.

use crate::code::CODE;
use axum::{
    Form, Router,
    http::HeaderMap,
    routing::{get, post},
};
use loco_ui::layout::{Look, Palette, Radius, Scale, Tokens};
use loco_ui::prelude::*;
use maud::PreEscaped;
use serde::Deserialize;
use std::borrow::Cow;

pub(crate) fn routes() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/lang", post(lang_submit))
}

/// One component page in the index.
pub(crate) struct Entry {
    /// Its path.
    pub href: &'static str,
    /// Its title, what its route passes to `page`.
    pub title: &'static str,
    /// The index group it sits in (see [`LAYERS`]).
    pub group: &'static str,
    /// The platform features it is built on, comma-separated.
    pub built_on: &'static str,
    /// What it is for, in one line.
    pub what: &'static str,
    /// When to reach for it and when another component fits better: the page's "When to
    /// use" section (a test fails when it is empty).
    pub when: &'static str,
}

/// Every component in the index, in its order.
pub(crate) const COMPONENTS: &[Entry] = &[
    Entry {
        href: "/feedback",
        title: "Alerts, progress and tooltips",
        group: "Feedback",
        built_on: "role=alert, <progress>, <meter>, :hover/:focus-within, <hr>",
        what: "Callouts, bars and meters, a tooltip on hover or focus, and separators.",
        when: "Use an alert for a message that belongs to the page and stays until the page changes, a progress bar for a known amount of work, a meter for a value within a range, and a tooltip for a short hint on something the reader can hover or focus. A message that follows an action and then goes away is a toast.",
    },
    Entry {
        href: "/app/signin",
        title: "Sign in",
        group: "Complete flows",
        built_on: "server validation, errors re-rendered, a session cookie, PRG",
        what: "A sign-in form whose mistakes come back from the server, next to the fields.",
        when: "Copy this flow when a form must be checked on the server and shown again with its mistakes: the handler returns the same page with a 422 and the messages, or redirects on success.",
    },
    Entry {
        href: "/app/notes",
        title: "Notes",
        group: "Complete flows",
        built_on: "create, edit in place, delete, filter, pages, flash, PRG",
        what: "A small app: add, rename and delete notes, with script off.",
        when: "Read it as the pattern for a small CRUD screen: every change is a form post answered with a redirect, the list filters and pages through the URL, and the script only makes it faster.",
    },
    Entry {
        href: "/pricing",
        title: "Pricing card",
        group: "Your own",
        built_on: "ui.card, ui.badge, ui.link_button, Icon, ui.grid, Page::css",
        what: "A component written in the demo crate, from the public primitives only.",
        when: "Write your own component like this one when the library has the parts but not the whole: a function on `Ui` that puts primitives together, with its CSS passed to `Page::css`.",
    },
    Entry {
        href: "/kanban",
        title: "Kanban",
        group: "Widgets",
        built_on: "form POST per move, PRG, view-transition-name, scroll-snap",
        what: "Cards in columns; each move is a form post the server keeps.",
        when: "Use it when items move between a few named states (to do, doing, done) and the order inside a state matters less than the state itself. For one ordered list, use the sortable list.",
    },
    Entry {
        href: "/sortable",
        title: "Sortable list",
        group: "Widgets",
        built_on: "form POST per move, PRG, view-transition-name, drag and drop with the script",
        what: "A list in the order the server keeps; arrows without script, a grip to drag with it.",
        when: "Use it when the visitor sets the order of one list and the server keeps it: a playlist, priorities, the steps of a checklist. Items that move between columns are a kanban board.",
    },
    Entry {
        href: "/upload",
        title: "Upload",
        group: "Widgets",
        built_on: "multipart POST, <input type=file>, drop on the input, <progress>, PRG",
        what: "Send files; the list under the form is what the server kept.",
        when: "Use it to send files with a form. The list under it shows what the server kept, so a reload or a second tab shows the same files.",
    },
    Entry {
        href: "/calendar",
        title: "Calendar",
        group: "Widgets",
        built_on: "<table>, links or radios, aria-current=date, :has(:checked), ?month=",
        what: "A month you can page through and pick a day from.",
        when: "Use the calendar to show a month and pick a day from it. For a date in a form, the date picker (an input with this calendar under it) is usually the better fit.",
    },
    Entry {
        href: "/marquee",
        title: "Marquee",
        group: "Widgets",
        built_on: "@keyframes, translate, :hover/:focus-within, aria-hidden + inert",
        what: "A row that loops sideways on its own and stops under the pointer or focus.",
        when: "Use it for a row of logos or quotes that is decoration, not content: it is hidden from assistive technology, so anything the reader must see belongs in the page as well.",
    },
    Entry {
        href: "/button",
        title: "Buttons and badges",
        group: "Primitives",
        built_on: "<button>, invoker commands, popovertarget, aria-busy, inline <svg>",
        what: "The button every other component is built from, with badges and icons.",
        when: "Use a button for an action (submit, open, delete) and a link button for navigation that should look like one. Badges label a state or a count; they are not clickable.",
    },
    Entry {
        href: "/field",
        title: "Fields",
        group: "Primitives",
        built_on: "<label>, aria-describedby, :user-invalid, role=switch, <fieldset>",
        what: "A labelled input, checkbox, switch and radio group, with help and errors.",
        when: "Use these for any single value in a form: a text input, a checkbox for one yes or no, a switch for a setting that applies at once on submit, a radio group for one choice among a few.",
    },
    Entry {
        href: "/card",
        title: "Cards and avatars",
        group: "Primitives",
        built_on: "grid, <img alt=\"\">, loading=lazy",
        what: "A box with a header, body and footer, and a picture that falls back to initials.",
        when: "Use a card to group one thing's content with its title and actions. An avatar stands for a person, with initials when there is no picture.",
    },
    Entry {
        href: "/layout",
        title: "Layout",
        group: "Primitives",
        built_on: "flex gap, flex-wrap, repeat(auto-fill), custom properties",
        what: "Stack, cluster, grid and split: even spacing with no margins, and columns that wrap on their own.",
        when: "Use the layouts instead of margins: a stack for things one under the other, a cluster for a wrapping row, a grid for equal columns that wrap, a split for a sidebar beside the main content.",
    },
    Entry {
        href: "/palette",
        title: "Command palette",
        group: "Navigation",
        built_on: "popover, <datalist>, <search>, accesskey, GET + 303",
        what: "Jump to any page by typing its name.",
        when: "Use it when an app has more destinations than its navigation can show: a box that jumps to a page by name and lists matches for anything else.",
    },
    Entry {
        href: "/nav",
        title: "Drawer and breadcrumbs",
        group: "Navigation",
        built_on: "<dialog>, invoker commands, closedby, @starting-style, <details>",
        what: "A sidebar that turns into a drawer on small screens, with a trail back up.",
        when: "Use the drawer for an app's navigation on small screens and breadcrumbs when pages sit in a hierarchy the reader may want to climb.",
    },
    Entry {
        href: "/toast",
        title: "Toasts",
        group: "Feedback",
        built_on: "position: fixed, role=alert, CSS fade, PRG",
        what: "Short messages in the corner after a form is sent.",
        when: "Use a toast to confirm an action that worked (saved, sent, deleted) where the page itself does not show the change. A message the reader must act on is an alert or a dialog.",
    },
    Entry {
        href: "/dashboard",
        title: "Stats and empty states",
        group: "Feedback",
        built_on: "auto-fit grid, form POST",
        what: "Numbers with how they changed, and what to show when there is nothing yet.",
        when: "Use stats for a few numbers that matter and how they changed. Use an empty state wherever a list can be empty, with the action that fills it.",
    },
    Entry {
        href: "/dialog",
        title: "Dialog",
        group: "Overlays",
        built_on: "<dialog>, closedby, invoker commands, form footer",
        what: "Ask before doing something that cannot be undone.",
        when: "Use a dialog to confirm something that cannot be undone or to ask for a small piece of input without leaving the page. Content the reader may want to link to belongs on its own page.",
    },
    Entry {
        href: "/popover",
        title: "Popover menu",
        group: "Overlays",
        built_on: "popover, anchor positioning, nested popover, form actions",
        what: "A menu of links and actions that opens over the page.",
        when: "Use a popover menu for a few actions or links that do not need to be visible all the time. For actions on one item in a list, use the context menu.",
    },
    Entry {
        href: "/tabs",
        title: "Tabs",
        group: "Disclosure",
        built_on: "links, hidden, view-transition-name, grid",
        what: "Several panels in one place, one open at a time.",
        when: "Use tabs for a few panels of the same thing that the reader switches between, one at a time. When the reader may want several open, or the panels are long, use an accordion.",
    },
    Entry {
        href: "/accordion",
        title: "Accordion",
        group: "Disclosure",
        built_on: "links, hidden, aria-expanded",
        what: "Questions that open to their answers.",
        when: "Use an accordion for sections the reader opens one by one, such as questions and answers. For a few short panels side by side, tabs fit better.",
    },
    Entry {
        href: "/combobox",
        title: "Combobox",
        group: "Input",
        built_on: "<datalist>, <optgroup>, <search>, aria-live",
        what: "Search a list and pick one item or several.",
        when: "Use a combobox to pick from a list too long for a select: the reader types to narrow it, and can pick several. For a handful of options, use a select or radios.",
    },
    Entry {
        href: "/form",
        title: "Validated form",
        group: "Input",
        built_on: ":user-invalid, <fieldset>, <output> counters, field-sizing, multipart, PRG, error summary with autofocus",
        what: "Fields the browser checks first and the server checks again.",
        when: "Use the form builder for any form with more than one field: it lays out the fields, writes the error summary, keeps what was typed and posts with Post/Redirect/Get.",
    },
    Entry {
        href: "/wizard",
        title: "Wizard",
        group: "Input",
        built_on: "one form per step, PRG, formnovalidate, <progress>, UiState",
        what: "A long form split into steps you can leave and come back to.",
        when: "Use a wizard when a form is too long for one page and its steps follow an order. Each step posts on its own, so the reader can leave and come back.",
    },
    Entry {
        href: "/inputs",
        title: "Select, range, colour",
        group: "Input",
        built_on: "<selectedcontent>, <optgroup>, formmethod, two-thumb range, color-mix()",
        what: "Pick a size, a country, a volume, a price range and a colour.",
        when: "Use a select for one choice from a list, a range for a value where the exact number matters less than its place in the range, and a colour input for a colour.",
    },
    Entry {
        href: "/counter",
        title: "Counter",
        group: "Server state",
        built_on: "form POST + cookie, type=number, disabled",
        what: "A number that goes up and down within limits.",
        when: "Use it for a small whole number the visitor sets with buttons, such as a quantity in a cart, where the server keeps the value and its limits.",
    },
    Entry {
        href: "/settings",
        title: "Settings",
        group: "Server state",
        built_on: "UiState, PRG + flash, role=alert, CSS auto-hide",
        what: "Tabs of settings that stay where you left them.",
        when: "Copy this page when settings are grouped and each group is saved on its own, and the page should reopen on the tab the visitor left.",
    },
    Entry {
        href: "/list",
        title: "Load-more list",
        group: "Server state",
        built_on: "links + view transitions",
        what: "A long list shown a page at a time.",
        when: "Use a load-more list for a feed the reader scrolls through. For data the reader sorts, filters or jumps around in, use a table with pages.",
    },
    Entry {
        href: "/table/minimal",
        title: "Table, minimal",
        group: "Server state",
        built_on: "sort links, <search> filter, ?page.<id>=n",
        what: "The smallest useful table: sorted, filtered and paged by the URL.",
        when: "Start from this table when a list of records needs sorting, a search box and pages, and nothing else. Add selection, column choice and row details from the full table when they are needed.",
    },
    Entry {
        href: "/table",
        title: "Table",
        group: "Server state",
        built_on: "sort links, <search> filter, form= checkboxes, ?cols.<id>=, <details> rows, sticky first column, ?page.<id>=n",
        what: "Sort, filter, page through and select rows of data.",
        when: "Use the table for records the reader compares, sorts, filters, selects and acts on. For one record's fields, use a description list or the record page.",
    },
    Entry {
        href: "/caps",
        title: "Capabilities",
        group: "Server state",
        built_on: "@supports beacons + cookie",
        what: "What the server knows this browser can do.",
        when: "Read this page to see what the server knows about the browser and which variant each component sends. Use `caps.has(..)` in your own components the same way.",
    },
    Entry {
        href: "/stream",
        title: "Streaming",
        group: "Server state",
        built_on: "declarative shadow DOM slots, skeleton placeholders, aria-busy",
        what: "A page that sends its fast parts first.",
        when: "Stream a page when one part is slow (a report, a remote call) and the rest can be shown at once: the fast parts arrive first and the slow ones fill in.",
    },
    Entry {
        href: "/swap",
        title: "Swap targets",
        group: "Server state",
        built_on: "data-lui-target, data-lui-swap, data-lui-oob, data-lui-indicator, data-lui-push, Lui-Enhance header",
        what: "Update one part of the page without reloading it.",
        when: "Use swap targets when an action should update one part of the page, or several, in place. Without the script the same form or link loads the whole page.",
    },
    Entry {
        href: "/blocks/shell",
        title: "App shell",
        group: "Blocks",
        built_on: "a sidebar that is a drawer on narrow screens, aria-current",
        what: "The frame of a signed-in app: navigation, who is signed in, the page.",
        when: "Start an app's pages from the shell: navigation, the signed-in person and the page's content, with a drawer on small screens.",
    },
    Entry {
        href: "/blocks/auth",
        title: "Auth page",
        group: "Blocks",
        built_on: "a card, a form that posts",
        what: "A sign-in or sign-up page, centred, with the links to the other account pages.",
        when: "Use it for sign-in, sign-up and the other account pages, so they share one look and link to each other.",
    },
    Entry {
        href: "/blocks/settings",
        title: "Settings page",
        group: "Blocks",
        built_on: "fragment links, a two-column grid",
        what: "Settings in sections, each with its own form, and a list that jumps to each.",
        when: "Use it when settings fall into sections that are each saved on their own, with a list on the side that jumps to each section.",
    },
    Entry {
        href: "/blocks/record",
        title: "Record page",
        group: "Blocks",
        built_on: "<dl>, a delete form, PRG",
        what: "One record's fields and the actions on it.",
        when: "Use it for the page of one record: its fields, its actions and a delete that asks first.",
    },
    Entry {
        href: "/blocks/dashboard",
        title: "Dashboard page",
        group: "Blocks",
        built_on: "auto-fit grid",
        what: "A row of numbers and what goes under them.",
        when: "Use it for an app's home or overview page: a row of numbers, then the lists or charts that explain them.",
    },
    Entry {
        href: "/blocks/error",
        title: "Error page",
        group: "Blocks",
        built_on: "Router::fallback, HTTP status",
        what: "The 404 and 500 pages in the site's look; the demo's fallback.",
        when: "Use it as the router's fallback and for server errors, so a missing page still has the site's navigation and look.",
    },
    Entry {
        href: "/chart",
        title: "Charts",
        group: "Feedback",
        built_on: "inline <svg>, focusable marks, :focus-visible chips, a hidden data table",
        what: "Bars, a line and a sparkline drawn on the server, no chart library.",
        when: "Use a chart when the shape of the numbers matters more than each value: bars to compare, a line for change over time, a sparkline beside a number. The values are also in a table for screen readers.",
    },
    Entry {
        href: "/sidebar",
        title: "Sidebar",
        group: "Navigation",
        built_on: "<nav>, aria-current",
        what: "An app's navigation as a column: groups, icons, counts, the current page marked.",
        when: "Use a sidebar for an app's main navigation when there is room for a column. On small screens put it in the drawer.",
    },
    Entry {
        href: "/nav-menu",
        title: "Navigation menu",
        group: "Navigation",
        built_on: "popover, anchor-name, <details> fallback",
        what: "Top navigation whose items open panels of links.",
        when: "Use a navigation menu for a site's top bar when its sections each hold several pages. For an app with many sections, use a sidebar.",
    },
    Entry {
        href: "/description-list",
        title: "Description list",
        group: "Feedback",
        built_on: "<dl>, @container, subgrid",
        what: "Terms and their details, side by side or stacked.",
        when: "Use a description list for one thing's fields and values, such as a record's details or an order summary. Several records side by side belong in a table.",
    },
    Entry {
        href: "/toggle-group",
        title: "Toggle group",
        group: "Input",
        built_on: "radios and checkboxes drawn as segments, :checked",
        what: "Pressable options, one or several, sent with their form.",
        when: "Use a toggle group for a few options shown at once and switched with one click, such as text alignment or a view mode. For a longer list of options, use radios or a select.",
    },
    Entry {
        href: "/otp",
        title: "One-time code",
        group: "Input",
        built_on: "autocomplete=one-time-code, inputmode=numeric, pattern",
        what: "The box for a code from a text message, which the phone offers to fill.",
        when: "Use it for a code sent by text message or email: phones offer to fill it from the message.",
    },
    Entry {
        href: "/checkbox-group",
        title: "Checkbox group",
        group: "Input",
        built_on: "<fieldset>, name[], :has(:checked)",
        what: "Several choices under one legend, as a list or as cards.",
        when: "Use a checkbox group when the reader may pick any number of options from a short list, each posted as `name[]`. For exactly one choice use a radio group; for a long list, a combobox with several picks.",
    },
    Entry {
        href: "/number",
        title: "Number",
        group: "Input",
        built_on: "type=number, min/max/step, formnovalidate steppers",
        what: "A quantity or a price with its unit and buttons that step it.",
        when: "Use it for a quantity or an amount with bounds and a step, where steppers help. For a value whose exact digits matter more than its size (a postcode, a card number) use a text input with `inputmode`.",
    },
    Entry {
        href: "/search",
        title: "Search",
        group: "Input",
        built_on: "<search>, a GET form, aria-controls",
        what: "A search box whose answer is a page you can bookmark.",
        when: "Use it for the search box of a page or a list: a GET form in a `<search>` landmark whose query stays in the URL. To jump to a page by name from anywhere, use the command palette.",
    },
    Entry {
        href: "/rating",
        title: "Rating",
        group: "Input",
        built_on: "radios drawn as stars, :has()",
        what: "Stars to pick a score with, or to show one.",
        when: "Use it to ask for or show a score out of a few stars. For a value on a long scale use a range slider; for a measured value, a meter.",
    },
    Entry {
        href: "/tag-input",
        title: "Tag input",
        group: "Input",
        built_on: "name[] chips, formmethod=get, remove links",
        what: "Labels collected as removable chips, posted with the form.",
        when: "Use it when the reader types free-form labels and sees each as a chip they can remove. When the labels come from a known list, a combobox with several picks fits better.",
    },
    Entry {
        href: "/context-menu",
        title: "Context menu",
        group: "Overlays",
        built_on: "popover, a menu on a secondary button",
        what: "Actions on one thing from a button in its corner.",
        when: "Use a context menu for the actions on one item in a list or a card (edit, duplicate, delete), from a button in its corner.",
    },
    Entry {
        href: "/theme",
        title: "Theme builder",
        group: "Primitives",
        built_on: "<input type=color>, --lui-* custom properties, a GET form, a CSS download",
        what: "Pick the colours and the radius, see a few components in them, download theme.css.",
        when: "Use the theme builder to choose the colours and radius of your app and download the CSS that sets them. Code that sets them per request uses `Tokens` instead.",
    },
    Entry {
        href: "/modes",
        title: "Modes",
        group: "Primitives",
        built_on: "@media (forced-colors: active), (prefers-contrast: more), print",
        what: "The controls drawn in CSS under forced colours and more contrast, emulated by a class.",
        when: "Check this page when you change a component's CSS: forced colours, more contrast and print each need the control to stay visible and usable.",
    },
    Entry {
        href: "/table/rows",
        title: "Table, rows",
        group: "Server state",
        built_on: "group rows, stretched row links, tones, <tfoot>",
        what: "Rows grouped by a column, linked as a whole, toned or disabled.",
        when: "Use these options when rows belong to groups, when a whole row should open a record, or when some rows need to stand out or be disabled.",
    },
    Entry {
        href: "/typography",
        title: "Typography",
        group: "Primitives",
        built_on: "<h1>–<h6>, text-wrap: balance, text-box, <kbd>, <code>, <blockquote>",
        what: "Heading, text, link, keycap, inline code and a quotation on one type scale.",
        when: "Use heading, text, link, kbd and code for the words of an app's own screens, so sizes and colours come from the tokens. For long-form content written elsewhere (Markdown, a CMS), wrap it in prose instead.",
    },
    Entry {
        href: "/code-block",
        title: "Code block",
        group: "Primitives",
        built_on: "<pre><code>, overflow-x: auto, navigator.clipboard (script only)",
        what: "Lines of code that scroll inside their box, with a file name and a copy button the script shows.",
        when: "Use a code block for a snippet the reader may read or copy, with an optional file name. For a few words of code inside a sentence, use inline code.",
    },
    Entry {
        href: "/prose",
        title: "Prose",
        group: "Primitives",
        built_on: ":where(), :is(), text-wrap: pretty",
        what: "Readable styles for plain HTML from Markdown or a CMS, scoped to one block.",
        when: "Use prose around HTML the app did not build itself, such as rendered Markdown, to give its bare elements the library's type. Around components it is not needed.",
    },
    Entry {
        href: "/container",
        title: "Page layout",
        group: "Primitives",
        built_on: "max-width, padding-block, aspect-ratio, place-items, flex-grow",
        what: "Container, section, inset, aspect ratio, center and spacer: the page's bones.",
        when: "Use container, section and inset to give a page its width and vertical rhythm, aspect ratio for media that must keep its shape, and center or spacer for single alignments. For rows and columns of parts, use the layout primitives.",
    },
    Entry {
        href: "/scroll-area",
        title: "Scroll area",
        group: "Primitives",
        built_on: "overflow: auto, scrollbar-gutter, scroll-state()",
        what: "A box that scrolls inside itself, with edge shadows where the browser can tell.",
        when: "Use it for a box of fixed height whose content scrolls inside it, such as a long list in a sidebar. Let the page scroll instead whenever the content is the page.",
    },
    Entry {
        href: "/spinner",
        title: "Spinner and indicator",
        group: "Primitives",
        built_on: "@keyframes, role=status, inset-inline-end",
        what: "A loading ring, and a count or dot on the corner of a button or avatar.",
        when: "Use a spinner while a part of the page waits for an answer of unknown length, and an indicator to mark a count or an unread dot on another element. For known progress, a progress bar says more.",
    },
    Entry {
        href: "/banner",
        title: "Banner",
        group: "Feedback",
        built_on: "<aside>, a cookie set by a POST form",
        what: "News for the whole site across the top, closed for good by its close button.",
        when: "Use a banner for one message about the whole app (maintenance, a trial ending) across the top of every page, dismissable for good. A message about one page is an alert; one after an action, a toast.",
    },
    Entry {
        href: "/popover-card",
        title: "Popover",
        group: "Overlays",
        built_on: "popover, anchor positioning, position-try-fallbacks",
        what: "A panel of anything, a short form or some help, that opens beside its button.",
        when: "Use a popover for a little more content than a tooltip, opened on click: a share box, a short form, details. For a list of actions use a menu; for content that needs the reader's full attention, a dialog.",
    },
    Entry {
        href: "/carousel",
        title: "Carousel",
        group: "Overlays",
        built_on: "scroll-snap, ::scroll-button(), ::scroll-marker",
        what: "Slides in a row that snaps, with buttons and dots.",
        when: "Use a carousel for a few items of the same kind the reader may browse sideways, such as images. Anything the reader must not miss belongs in the page, not on a later slide.",
    },
    Entry {
        href: "/pagination",
        title: "Pagination",
        group: "Navigation",
        built_on: "<nav> of links, aria-current, rel=prev/next, <link rel> in the head, @container",
        what: "Numbered page links with an ellipsis over the gaps, every page a URL.",
        when: "Use pagination under a long list the server cuts into pages, with the page in the URL. A table builds its own pager; use this one for other lists.",
    },
    Entry {
        href: "/steps",
        title: "Steps",
        group: "Navigation",
        built_on: "<ol>, aria-current=step, @container",
        what: "Where a visitor is in a process: done, current and still to come.",
        when: "Use steps to show where the reader is in a fixed sequence, such as a sign-up in parts. To run that sequence as forms with state between them, use the wizard, which draws its progress with steps.",
    },
    Entry {
        href: "/timeline",
        title: "Timeline",
        group: "Navigation",
        built_on: "<ol>, <time datetime>, ::before, :has()",
        what: "What happened, in order, on a rail with a ring or an icon per event.",
        when: "Use a timeline for dated events in order, such as an activity log or a release history. For steps the reader still has to go through, use steps.",
    },
    Entry {
        href: "/tree",
        title: "Tree",
        group: "Navigation",
        built_on: "nested <details>, aria-current, --lui-tree-depth",
        what: "Folders that open and close, the current page's folders open.",
        when: "Use a tree for nested navigation the reader opens folder by folder: files, a docs outline, categories. For a flat list of links, a sidebar is simpler.",
    },
    Entry {
        href: "/collapsible",
        title: "Collapsible",
        group: "Navigation",
        built_on: "<details name>, ::details-content",
        what: "One section that shows and hides under a button.",
        when: "Use a collapsible for one section the reader may open, such as advanced options. For several sections of the same kind, use an accordion.",
    },
    Entry {
        href: "/hover-card",
        title: "Hover card",
        group: "Navigation",
        built_on: "interestfor, popover=hint, popovertarget fallback",
        what: "A preview card on hover and focus, or on click where hover cannot open it.",
        when: "Use a hover card for a preview of what a link leads to (a profile, a page), shown on hover or focus. It must only repeat what the link's destination says, since a touch reader may never see it.",
    },
];

/// The index's layers, bottom up, each with the groups it holds (as in `docs/layers.svg`).
pub(crate) const LAYERS: [(&str, &str, &[&str]); 6] = [
    (
        "Primitives",
        "The parts every component is built from.",
        &["Primitives"],
    ),
    (
        "Components",
        "Built from the primitives: one change to the button restyles them all.",
        &[
            "Overlays",
            "Disclosure",
            "Navigation",
            "Input",
            "Feedback",
            "Server state",
        ],
    ),
    (
        "Widgets",
        "Larger pieces built from components and primitives.",
        &["Widgets"],
    ),
    (
        "Blocks",
        "Whole pages from the components: fill one in, or copy its file when yours differs.",
        &["Blocks"],
    ),
    (
        "Your own",
        "A component written in the demo crate, the way you would write one.",
        &["Your own"],
    ),
    (
        "Complete flows",
        "Whole tasks, end to end, with JavaScript off: the parts working together.",
        &["Complete flows"],
    ),
];

/// The second palette from `docs/theming.md`: warm paper, copper primary, amber in the dark.
const LINEN: Tokens = Tokens {
    gray: Scale::SLATE,
    brand: Scale::INDIGO,
    light: Palette {
        bg: Cow::Borrowed("#f4efe6"),
        fg: Cow::Borrowed("#1d1a17"),
        muted: Cow::Borrowed("#5d574f"),
        line: Cow::Borrowed("#d6cdbf"),
        surface: Cow::Borrowed("#fffdf9"),
        card: Cow::Borrowed("#fffdf9"),
        popover: Cow::Borrowed("#fffdf9"),
        secondary: Cow::Borrowed("#ebe3d6"),
        accent: Cow::Borrowed("#ebe3d6"),
        on_accent: Cow::Borrowed("#1d1a17"),
        primary: Cow::Borrowed("#8a3b12"),
        on_primary: Cow::Borrowed("#ffffff"),
        input: Cow::Borrowed("#d6cdbf"),
        ring: Cow::Borrowed("#b5764f"),
        link: Cow::Borrowed("#8a3b12"),
        danger: Cow::Borrowed("#a0261c"),
        on_danger: Cow::Borrowed("#ffffff"),
        ok: Cow::Borrowed("#2f6b3a"),
        warn: Cow::Borrowed("#7a5500"),
        info: Cow::Borrowed("#245f8f"),
        on_info: Cow::Borrowed("#ffffff"),
    },
    dark: Palette {
        bg: Cow::Borrowed("#161311"),
        fg: Cow::Borrowed("#ece6dc"),
        muted: Cow::Borrowed("#a59c90"),
        line: Cow::Borrowed("#3a332c"),
        surface: Cow::Borrowed("#1f1b18"),
        card: Cow::Borrowed("#1f1b18"),
        popover: Cow::Borrowed("#1f1b18"),
        secondary: Cow::Borrowed("#2b2521"),
        accent: Cow::Borrowed("#2b2521"),
        on_accent: Cow::Borrowed("#ece6dc"),
        primary: Cow::Borrowed("#e8965a"),
        on_primary: Cow::Borrowed("#1a0f06"),
        input: Cow::Borrowed("#4a4038"),
        ring: Cow::Borrowed("#a8683a"),
        link: Cow::Borrowed("#e8965a"),
        danger: Cow::Borrowed("#ff8f85"),
        on_danger: Cow::Borrowed("#1a0f06"),
        ok: Cow::Borrowed("#8fd39a"),
        warn: Cow::Borrowed("#f0c060"),
        info: Cow::Borrowed("#8cc4f0"),
        on_info: Cow::Borrowed("#1a0f06"),
    },
    radius: Radius::new("3px"),
    ..Tokens::DEFAULT
};

/// A component page's live component, from the `PAGES` or `PREVIEWS` beside its route: the
/// function its page calls between the `// code:` markers, so the index shows the same call
/// the page does.
pub(crate) fn preview(href: &str) -> Option<fn(&Ui) -> Markup> {
    use crate::routes::*;
    let pages = [
        primitives::PAGES,
        overlays::PAGES,
        disclosure::PAGES,
        navigation::PAGES,
        input::PAGES,
        feedback::PAGES,
        server_state::PAGES,
        widgets::PAGES,
        blocks::PAGES,
    ];
    let previews = [
        primitives::PREVIEWS,
        theme::PREVIEWS,
        disclosure::PREVIEWS,
        navigation::PREVIEWS,
        input::PREVIEWS,
        feedback::PREVIEWS,
        table::PREVIEWS,
        server_state::PREVIEWS,
        widgets::PREVIEWS,
        flows::PREVIEWS,
        own::PREVIEWS,
    ];
    let simple = pages.into_iter().flatten().map(|p| (p.0, p.1));
    simple
        .chain(previews.into_iter().flatten().copied())
        .find(|p| p.0 == href)
        .map(|p| p.1)
}

/// A component page's title, as `COMPONENTS` has it.
pub(crate) fn title(href: &'static str) -> &'static str {
    COMPONENTS
        .iter()
        .find(|c| c.href == href)
        .map_or(href, |c| c.title)
}

/// A note's text as markup: `` `x` `` is code, `[text](href)` a link.
pub(crate) fn note(text: &str) -> Markup {
    let link = |part: &str| {
        let mut rest = part;
        let mut out = Vec::new();
        while let Some((before, after)) = rest.split_once('[')
            && let Some((text, after)) = after.split_once("](")
            && let Some((href, after)) = after.split_once(')')
        {
            out.push(html! { (before) a href=(href) { (text) } });
            rest = after;
        }
        html! { @for m in out { (m) } (rest) }
    };
    html! { @for (i, part) in text.split('`').enumerate() { @if i % 2 == 1 { code { (part) } } @else { (link(part)) } } }
}

/// The docs pages under their groups, then every component under its group, in the index's
/// order, the current page marked. From 60rem it is a sticky column beside the page;
/// narrower, the `<details>` before it hides it until opened (CSS below, no script).
fn sidebar(ui: &Ui) -> Markup {
    let mut docs = ui.sidebar("Docs");
    let mut group = "";
    for doc in crate::docs::DOCS {
        if doc.group != group {
            group = doc.group;
            docs = docs.group(group);
        }
        docs = docs.link(doc.label, doc.href);
    }
    let mut nav = ui
        .sidebar("Components")
        .group("Components")
        .link("Overview", "/");
    for group in LAYERS.iter().flat_map(|l| l.2) {
        nav = nav.group(group);
        for c in COMPONENTS.iter().filter(|c| c.group == *group) {
            nav = nav.link(c.title, c.href);
        }
    }
    html! {
        div class="lui-site-nav" {
            details class="lui-site-menu" { summary class="lui-button" { "Browse the docs and components" (Icon::ChevronDown) } }
            (docs)
            (nav)
        }
    }
}

/// The top of every page: the way back to the index (not on the index), then the title with
/// one toolbar beside it, the language of the components' own words and the theme switch
/// (under the title when the page is narrow).
fn title_bar(ui: &Ui, back: bool, title: Markup) -> Markup {
    let languages = html! {
        form method="post" action="/lang" class="lui-lang" {
            @for (tag, name) in [("en", "English"), ("es", "Español"), ("ar", "العربية")] {
                (ui.button(name).small().ghost().name("lang").value(tag).pressed(ui.lang() == tag))
            }
        }
    };
    html! {
        @if back { a class="lui-back" href="/" { "All components" } }
        div class="lui-title-bar" {
            h1 { (title) }
            div class="lui-toolbar" { (languages) (ui.theme_toggle("/theme")) }
        }
    }
}

/// What every page shows around its body: the toolbar and the title, and on a component page
/// what it is for and built on, then the body on a stage with the code that drew it underneath.
pub(crate) fn shell(ui: &Ui, title: &str, body: Markup) -> Markup {
    let component = COMPONENTS.iter().find(|c| c.title == title);
    let page = match component {
        Some(c) => component_page(ui, c, body),
        None => html! { (title_bar(ui, false, html! { (title) })) (body) },
    };
    html! {
        div class="lui-site" {
            (sidebar(ui))
            div class={ "lui-site-main" @if component.is_none() { " lui-site-wide" } } { (page) }
        }
    }
}

/// A component page: what it is for and built on, the stage's toggles, then the body on a
/// stage with the code that drew it underneath, the props of the builders that code calls,
/// and the reference sections (`reference.rs`).
fn component_page(ui: &Ui, component: &Entry, body: Markup) -> Markup {
    let code = CODE.iter().find(|h| h.0 == component.href);
    let builders: &[&loco_ui::props::Component] = code.map_or(&[], |h| &h.3);
    let width = (ui.param("stage"))
        .filter(|w| matches!(*w, "narrow" | "medium"))
        .unwrap_or("wide");
    let both = ui.param("themes") == Some("both");
    let stage = |theme: Option<&str>, body: Markup| {
        html! { div class="lui-stage" data-theme=[theme] data-width=(width) { div class="lui-stage-frame" { (body) } } }
    };
    html! {
        (title_bar(ui, true, html! { (component.title) @if beta(component.href) { " " (ui.badge("beta").warn()) } }))
        p class="lui-lede" { (component.what) }
        p class="lui-built" { "Built on " @for feature in component.built_on.split(", ") { code { (feature) } " " } }
        (stage_tools(ui, width, both))
        div class="lui-plate" {
            @if both {
                // The second stage is a copy: its ids get a prefix so the page keeps one of each.
                div class="lui-stages" {
                    (stage(Some("light"), html! { (body) }))
                    (stage(Some("dark"), PreEscaped(crate::ids::scope(&body.0, "dark-", |_| true))))
                }
            } @else {
                (stage(None, body))
            }
            figure class="lui-snippet" {
                @if let Some((_, path, code, _)) = code {
                    figcaption { span { (path) } span { "The code behind the component above" } }
                    pre id="lui-snippet-code" tabindex="0" aria-label=(path) { code { (PreEscaped(code)) } }
                    (crate::code::copy_button(ui, "lui-snippet-code"))
                }
            }
        }
        @if !builders.is_empty() { (props(ui, builders)) }
        (crate::reference::sections(component, builders))
    }
}

/// The toggles above a component's stage, each a link that keeps the rest of the query: the
/// stage in both themes side by side (`?themes=both`), the stage's width (`?stage=narrow`,
/// `medium`; the components' container queries answer to it), and the page without the
/// enhancement script (`?script=off`).
fn stage_tools(ui: &Ui, width: &str, both: bool) -> Markup {
    let toggle = |label: &str, href: String, on: bool| {
        html! { a href=(href) aria-current=[on.then_some("true")] { (label) } }
    };
    let script_off = ui.param("script") == Some("off");
    html! {
        nav class="lui-stage-tools" aria-label="Stage" {
            span { "Theme" }
            (toggle("This page's", ui.link_without("themes"), !both))
            (toggle("Both", ui.link_with("themes", "both"), both))
            span { "Width" }
            (toggle("Wide", ui.link_without("stage"), width == "wide"))
            (toggle("Medium", ui.link_with("stage", "medium"), width == "medium"))
            (toggle("Narrow", ui.link_with("stage", "narrow"), width == "narrow"))
            span { "Script" }
            (toggle("On", ui.link_without("script"), !script_off))
            (toggle("Off", ui.link_with("script", "off"), script_off))
        }
    }
}

/// Whether a component page shows a builder whose API is still beta.
fn beta(href: &str) -> bool {
    CODE.iter()
        .find(|h| h.0 == href)
        .is_some_and(|h| h.3.iter().any(|b| b.status == loco_ui::props::Status::Beta))
}

/// `text` with each `` `span` `` as `<code>`, as rustdoc shows it.
fn inline_code(text: &str) -> Markup {
    html! { @for (i, part) in text.split('`').enumerate() { @if i % 2 == 1 { code { (part) } } @else { (part) } } }
}

/// What each builder on the page accepts, from `loco_ui::props()`: one `<details>` per
/// builder (the first open, or the one being tried), its constructors in the summary and a
/// table of its setters inside. For a builder the playground knows, the table is a form with
/// a "Try" column, followed by the component as chosen and its `lui!` line.
fn props(ui: &Ui, builders: &[&loco_ui::props::Component]) -> Markup {
    let tried = |b: &str| crate::playground::tried(ui, b);
    let any_tried = builders.iter().any(|b| tried(b.builder));
    html! {
        section class="lui-props" {
            h2 id="props" { "Props" }
            @for (i, b) in builders.iter().enumerate() {
                @let has_try = crate::playground::entry(b.builder).is_some();
                // `#props-<builder>` is where search sends a prop.
                details id={ "props-" (b.builder.to_lowercase()) } open[if any_tried { tried(b.builder) } else { i == 0 }] {
                    summary {
                        code { (b.lui()) }
                        @for call in b.calls { " " code { (call) } }
                        " " (ui.badge(b.status.as_str()).outline())
                        span { (b.props.len()) @if b.props.len() == 1 { " prop" } @else { " props" } }
                    }
                    @if !b.props.is_empty() {
                        (crate::playground::playground(ui, ui.state.path(), b, |cell| html! {
                            div class="lui-props-scroll" tabindex="0" role="region" aria-label={ (b.builder) " props" } { table {
                                thead { tr { th { "Prop" } th { "Kind" } th { "Arguments" } th { "Default" } th { "HTML" } th { "What it does" } @if has_try { th { "Try" } } } }
                                tbody { @for p in b.props { tr {
                                    td { code { (p.name) } }
                                    td { (p.kind.as_str()) @if let loco_ui::props::PropKind::Choice { of } = p.kind { br; small { (of.join(", ")) } } @if !p.applies_to.is_empty() { br; small { "of " (p.applies_to.join(", ")) } } }
                                    td { @if !p.args.is_empty() { code { (p.args) } } }
                                    td { @if !p.default.is_empty() { code { (p.default) } } }
                                    td { @if !p.attr.is_empty() { code { (p.attr) } } }
                                    td { (inline_code(p.doc)) }
                                    @if has_try { td { (cell(p)) } }
                                } } }
                            } }
                        }))
                    }
                }
            }
        }
    }
}

pub(crate) fn page(ui: &Ui, title: &str, body: Markup) -> Page {
    // A pending flash on the stage, over the component that sent it, unless the body shows
    // it itself (as toasts, or with setters); `ui.page` would put it above the whole shell.
    let own = ["class=\"lui-flash\"", "class=\"lui-toasts\""];
    let body = if own.iter().any(|c| body.0.contains(c)) {
        body
    } else {
        html! { (ui.flash()) (body) }
    };
    let page = head(
        ui.page(title, shell(ui, title, body))
            .header(header_with_search(ui)),
        title,
    );
    // `?script=off`: the same page without the enhancement script, served under
    // `script-src 'none'`, so the no-script path can be tried in any browser.
    let page = if ui.param("script") == Some("off") {
        page.without_script()
    } else {
        page
    };
    // `?css=linked`: the stylesheet as `/lui.css` instead of inline, served without
    // `'unsafe-inline'` for `<style>` (the demo's own CSS goes in by hash).
    if ui.param("css") == Some("linked") {
        page.linked_stylesheet()
    } else {
        page
    }
}

/// The mark, three layers on the brand colour, as the tab's icon: an SVG `data:` URI, which
/// the CSP's `img-src 'self' data:` allows, so the site needs no favicon file.
const FAVICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 32 32'%3E%3Crect width='32' height='32' rx='7' fill='%233e63dd'/%3E%3Cpath d='M8 10.5h16M8 16h16M8 21.5h10' stroke='%23fff' stroke-width='3' stroke-linecap='round'/%3E%3C/svg%3E";

/// What a link preview or a search result shows of a page: its description (what a component
/// is for, from `COMPONENTS`), the Open Graph title and description, and the favicon.
fn head(page: Page, title: &str) -> Page {
    let what = COMPONENTS.iter().find(|c| c.title == title).map_or(
        "HTML-first components for Axum and Maud that work with JavaScript turned off.",
        |c| c.what,
    );
    let title = format!("{title} · loco-ui");
    (page.description(what).icon(FAVICON))
        .og("type", "website")
        .og("site_name", "loco-ui")
        .og("title", &title)
        .og("description", what)
}

/// Every component's preview for the index, in the index's order. Two previews can draw the
/// same field or form (`f-email`, the sign-in form); each id an earlier preview already used
/// gets the later preview's prefix, so the page keeps one element per id and the first of
/// each keeps its own (its swap root still matches its page's).
fn previews(ui: &Ui) -> Vec<(&'static str, Markup)> {
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    for group in LAYERS.iter().flat_map(|l| l.2) {
        for c in COMPONENTS.iter().filter(|c| c.group == *group) {
            let Some(draw) = preview(c.href) else {
                continue;
            };
            let html = draw(ui).into_string();
            let prefix = format!("{}-", loco_ui::slug(c.href));
            let html = crate::ids::scope(&html, &prefix, |id| seen.contains(id));
            seen.extend(crate::ids::ids(&html).into_iter().map(str::to_string));
            out.push((c.href, PreEscaped(html)));
        }
    }
    out
}

async fn index(ui: Ui) -> Page {
    let linen = ui.param("palette") == Some("linen");
    let previews = previews(&ui);
    let page = page(
        &ui,
        "Components",
        html! {
            @let count = |groups: &[&str]| COMPONENTS.iter().filter(|c| groups.contains(&c.group)).count();
            p class="lui-lede" { (count(LAYERS[0].2)) " primitives, " (count(LAYERS[1].2)) " components, " (count(LAYERS[2].2)) " widgets and one of your own, for Axum and Maud, all working with JavaScript turned off. The HTML platform and plain form posts do the work. Each page loads one optional script, " code { "/lui/enhance.js" } ", which updates the same markup in place instead of reloading. Block it and every page still works. Every component below is live: the same call its page shows, and its name leads to that page." }
            p { a href="/start" { "Get started" } ": install it on Axum, Loco or any server. Or search the components, their props and the guides from the box at the top of every page." }
            @if !ui.has(Cap::Probed) { p class="lui-note" { "First visit: this page is the fallback variant. Reload and the server will know your browser." } }
            p class="lui-note" { "Theme: " @if linen { a href="/" { "neutral" } " · linen and copper" } @else { "neutral · " a href="/?palette=linen" { "linen and copper" } } ", see " code { "docs/theming.md" } }
            div class="lui-index" { @for (layer, blurb, groups) in LAYERS {
                h2 { (layer) }
                p class="lui-index-layer" { (blurb) }
                @for group in groups.iter() {
                    @if groups.len() > 1 { h3 { (group) } }
                    ul { @for c in COMPONENTS.iter().filter(|c| c.group == *group) {
                        li class=[wide(c.href).then_some("lui-index-wide")] {
                            p { a href=(c.href) { (c.title) } @if beta(c.href) { " " (ui.badge("beta").warn()) } }
                            p class="lui-note" { (c.what) }
                            div class="lui-index-stage" { @if let Some((_, preview)) = previews.iter().find(|p| p.0 == c.href) { (preview) } }
                        }
                    } }
                }
            } }
            // Idle-time fetch of every component page, so the click is served from cache.
            @for c in COMPONENTS { link rel="prefetch" href=(c.href); }
        },
    );
    let page = page.css(crate::pricing::PRICING_CSS);
    if linen { page.tokens(&LINEN) } else { page }
}

/// Components shown across the whole row of the index: the blocks and flows (whole pages),
/// the pricing cards, and the few components as wide as the page they sit on.
fn wide(href: &str) -> bool {
    let groups = ["Blocks", "Your own", "Complete flows"];
    COMPONENTS
        .iter()
        .any(|c| c.href == href && groups.contains(&c.group))
        || [
            "/kanban",
            "/table",
            "/table/minimal",
            "/table/rows",
            "/theme",
            "/nav",
            "/inputs",
            "/chart",
        ]
        .contains(&href)
}

#[derive(Deserialize)]
pub(crate) struct ThemeForm {
    theme: String,
}

/// Keep the picked theme and go back to the page the toggle was on.
pub(crate) async fn theme_submit(
    ui: Ui,
    headers: HeaderMap,
    Form(form): Form<ThemeForm>,
) -> Redirect {
    ui.redirect(&back_to(&headers))
        .theme(Theme::parse(&form.theme))
}

#[derive(Deserialize)]
struct LangForm {
    lang: String,
}

/// Keep the picked language and go back to the page the switch was on.
async fn lang_submit(ui: Ui, headers: HeaderMap, Form(form): Form<LangForm>) -> Redirect {
    ui.redirect(&back_to(&headers)).lang(&form.lang)
}

/// The path of the page a form was posted from (same-origin `Referer`), or `/`.
fn back_to(headers: &HeaderMap) -> String {
    headers
        .get("referer")
        .and_then(|v| v.to_str().ok())
        .and_then(|url| url.splitn(4, '/').nth(3))
        .map(|path| format!("/{path}"))
        .filter(|path| !path.starts_with("//"))
        .unwrap_or_else(|| "/".to_string())
}

/// The demo's look: its header and its own CSS (the shell, the index, the snippets, the
/// props tables, the theme builder), on every page through one layer in `router()`. The
/// CSS is unlayered, so it wins over the library's layers.
pub static LOOK: Look = Look {
    tokens: Tokens::DEFAULT,
    css: &[CSS],
    header: Some(header),
};

/// The site header shown above every page: the name, what it is, and the way to the docs.
/// A page that knows the request ([`page`]) adds the search box ([`header_with_search`]).
fn header() -> Markup {
    header_parts(html! {})
}

/// The header with the search box: the command palette over the docs and components
/// (`search.rs`).
fn header_with_search(ui: &Ui) -> Markup {
    header_parts(crate::search::palette(ui))
}

fn header_parts(search: Markup) -> Markup {
    html! {
        header class="lui-site-header" {
            a href="/" { strong { "loco-ui" } }
            span { "Interactive HTML for Rust servers, works without JavaScript" }
            nav class="lui-site-links" aria-label="Site" {
                a href="/start" { "Get started" }
                a href="/" { "Components" }
                a href="/docs/theming" { "Theming" }
                a href="/changelog" { "Changelog" }
            }
            (search)
        }
    }
}

/// The demo's own styles, outside the library: the shell, the index, the snippets, the
/// props tables, the theme builder and the modes page.
/// [`CSS`] as pages carry it, for the markup budget in the tests.
#[cfg(test)]
pub(crate) static CSS_MINIFIED: std::sync::LazyLock<String> =
    std::sync::LazyLock::new(|| loco_ui::minify_css(CSS));

pub const CSS: &str = r#"
/* The site header above every page: the name and what it is. */
.lui-site-header {
  display: flex; flex-wrap: wrap; align-items: baseline; gap: 0.25rem 1rem;
  max-width: 52rem; margin: 0 auto; padding: calc(var(--lui-space) * 2);
  color: var(--lui-muted);
}
.lui-site-header > a { color: var(--lui-fg); text-decoration: none; font-size: 1rem; letter-spacing: -0.01em; }
.lui-site-header > span { font-size: 0.875rem; }
.lui-site-header strong { font-weight: 600; }
/* The header's links and its search box (the command palette), pushed to the end of the row. */
.lui-site-links { display: flex; flex-wrap: wrap; gap: 0.25rem 1rem; margin-inline-start: auto; font-size: 0.875rem; }
.lui-site-links a { color: var(--lui-muted); text-decoration: none; }
.lui-site-links a:hover { color: var(--lui-fg); }
.lui-site-header > .lui-palette { align-self: center; }
/* Docs pages: the body beside its table of contents once there is room, the contents
   above it before that. */
.lui-doc { container: lui-doc / inline-size; }
.lui-doc-grid { display: grid; gap: var(--lui-space-6); }
.lui-doc-body { min-width: 0; max-width: 46rem; }
.lui-doc-body > :first-child { margin-top: 0; }
.lui-doc-body h2 { margin-top: 2.5rem; }
.lui-doc-body h3 { margin-top: 1.75rem; }
.lui-doc-body :not(pre) > code { overflow-wrap: anywhere; }
.lui-doc-toc { font-size: 0.875rem; }
.lui-doc-toc > p { margin: 0 0 var(--lui-space-2); color: var(--lui-muted); font-weight: 500; }
.lui-doc-toc ul { list-style: none; margin: 0; padding: 0; display: grid; gap: var(--lui-space-1); }
.lui-doc-toc a { color: var(--lui-muted); text-decoration: none; overflow-wrap: anywhere; }
.lui-doc-toc a:hover { color: var(--lui-fg); }
.lui-doc-toc .lui-doc-toc-sub { padding-inline-start: var(--lui-space-3); }
@container lui-doc (width >= 48rem) {
  .lui-doc-grid { grid-template-columns: minmax(0, 1fr) 13rem; }
  .lui-doc-toc { grid-column: 2; grid-row: 1; position: sticky; top: var(--lui-space-4); max-height: calc(100vh - var(--lui-space-8)); overflow-y: auto; }
  .lui-doc-body { grid-column: 1; grid-row: 1; }
}
.lui-doc-table { overflow-x: auto; margin-block: var(--lui-space-4); }
.lui-doc-table table { border-collapse: collapse; font-size: 0.875rem; }
.lui-doc-table th, .lui-doc-table td { text-align: left; vertical-align: top; padding: 0.375rem 0.75rem; border-bottom: 1px solid var(--lui-line); }
.lui-doc-table th { color: var(--lui-muted); font-weight: 500; }
.lui-doc-source { margin-top: var(--lui-space-8); }
/* A code block (docs and snippets): the copy button the script shows sits in its corner. */
.lui-doc-code { position: relative; margin: var(--lui-space-4) 0; max-width: none; border: 1px solid var(--lui-line); border-radius: var(--lui-radius); background: var(--lui-surface); }
.lui-doc-code > div { overflow-x: auto; scrollbar-color: var(--lui-line) transparent; }
.lui-doc-code pre { margin: 0; padding: var(--lui-space-3) var(--lui-space-4); line-height: 1.6; tab-size: 4; }
.lui-doc-code pre code { background: none; border: 0; padding: 0; font-size: 0.8125rem; color: var(--lui-fg); }
.lui-snippet { position: relative; }
.lui-snippet:has(.lui-code-block-copy:not([hidden])) figcaption { padding-inline-end: 3rem; }
:is(.lui-snippet, .lui-doc-code) .lui-code-block-copy { background: var(--lui-surface); border-radius: var(--lui-radius-sm); }
/* The search page's box and its results. */
.lui-find > form { display: flex; flex-wrap: wrap; gap: var(--lui-space-2); margin-bottom: var(--lui-space-4); }
.lui-find > form > :first-child { flex: 1 1 16rem; }
.lui-find-results ul { list-style: none; margin: 0; padding: 0; display: grid; gap: var(--lui-space-3); }
.lui-find-results li > p { margin: 0; }
/* The stage's toggles above it, and the stage in both themes or at a width. */
.lui-stage-tools { display: flex; flex-wrap: wrap; align-items: baseline; gap: var(--lui-space-1) var(--lui-space-3); margin: 0 0 var(--lui-space-3); font-size: 0.8125rem; }
.lui-stage-tools span { color: var(--lui-muted); font-weight: 500; }
.lui-stage-tools span:not(:first-child) { margin-inline-start: var(--lui-space-3); }
.lui-stage-tools a { color: var(--lui-muted); text-decoration: none; }
.lui-stage-tools a[aria-current] { color: var(--lui-fg); text-decoration: underline; text-underline-offset: 0.2em; }
.lui-stage[data-theme="light"] { color-scheme: light; color: var(--lui-fg); }
.lui-stage[data-theme="dark"] { color-scheme: dark; color: var(--lui-fg); }
.lui-stages { display: grid; }
.lui-stages > .lui-stage + .lui-stage { border-radius: 0; }
.lui-stage-frame { margin-inline: auto; }
.lui-stage[data-width="medium"] > .lui-stage-frame { max-inline-size: 40rem; }
.lui-stage[data-width="narrow"] > .lui-stage-frame { max-inline-size: 20rem; }
/* The reference sections under the props: when to use, platform, accessibility, keyboard,
   without script, fallback, anatomy. */
.lui-reference { max-width: none; }
.lui-reference h2 { margin-top: 2rem; }
.lui-reference h3 { margin: 1rem 0 0.5rem; font-size: 0.9375rem; }
.lui-reference table { border-collapse: collapse; font-size: 0.8125rem; }
.lui-reference th, .lui-reference td { text-align: left; vertical-align: top; padding: 0.375rem 0.75rem; border-bottom: 1px solid var(--lui-line); }
.lui-reference th { color: var(--lui-muted); font-weight: 500; }
.lui-reference .lui-props-scroll { border-top: 0; margin-bottom: var(--lui-space-3); }
.lui-reference td:first-child { white-space: nowrap; }
/* The dashed boxes on the layout page. */
.lui-layout-tile { padding: calc(var(--lui-space) * 1.5); border: 1px dashed var(--lui-input); border-radius: var(--lui-radius-sm); background: var(--lui-surface); font-size: 0.875rem; }
.lui-yes { color: var(--lui-ok); font-weight: 600; }
.lui-no { color: var(--lui-danger); font-weight: 600; }
/* Demo shell, after the shadcn docs: the sidebar of every component beside the page, a
   toolbar with the way back and the theme switch, the lede under a title, "built on" as
   outline badges, the plate (a --lui-gray-2 stage over a muted code block, so a card on the stage
   is the only frame), and the index as a
   gallery of the components themselves, per group. */
/* The frame: from 60rem the component list is a sticky column beside a readable page (the
   index spreads wider); narrower, it follows a closed <details> that hides it until opened,
   so it is a plain <nav> of links either way and needs no script. */
/* Wider than <main> (52rem): the frame spreads to 84rem, centred, by negative margins (not
   :has(), which Blitz lacks: FINDINGS, M34); the header follows where :has() works. */
.lui-site {
  display: grid; gap: calc(var(--lui-space) * 3); width: min(84rem, calc(100vw - 2rem));
  margin-inline: calc((100% - min(84rem, calc(100vw - 2rem))) / 2);
}
body:has(.lui-site) > .lui-site-header { max-width: 84rem; }
.lui-site-main { min-width: 0; }
.lui-site-menu > summary { display: flex; width: 100%; justify-content: space-between; list-style: none; }
.lui-site-menu > summary::-webkit-details-marker { display: none; }
.lui-site-menu > summary svg { transition: rotate var(--lui-duration-fast) ease; }
.lui-site-menu[open] > summary svg { rotate: 180deg; }
.lui-site-menu[open] > summary { margin-bottom: var(--lui-space); }
@media (max-width: 59.99rem) {
  .lui-site-menu:not([open]) ~ .lui-sidebar { display: none; }
}
@media (min-width: 60rem) {
  .lui-site { grid-template-columns: 14rem minmax(0, 1fr); gap: calc(var(--lui-space) * 6); align-items: start; }
  .lui-site-menu { display: none; }
  .lui-site-nav {
    position: sticky; top: calc(var(--lui-space) * 2); max-height: calc(100vh - var(--lui-space) * 4); overflow-y: auto;
    scrollbar-width: thin; scrollbar-color: var(--lui-line) transparent;
  }
  .lui-site-main:not(.lui-site-wide) { max-width: 56rem; }
}
/* The title with the toolbar (languages, theme) on its row; the toolbar goes under the title
   when they do not fit side by side. */
.lui-title-bar { display: flex; flex-wrap: wrap; justify-content: space-between; align-items: center; gap: var(--lui-space) calc(var(--lui-space) * 2); margin: 0 0 0.75rem; }
.lui-title-bar h1 { margin: 0; }
.lui-toolbar { display: flex; flex-wrap: wrap; align-items: center; gap: var(--lui-space); }
.lui-popover-row { display: flex; justify-content: space-between; gap: var(--lui-space); margin-bottom: calc(var(--lui-space) * 2); }
.lui-back { display: inline-block; margin-bottom: calc(var(--lui-space) * 2); color: var(--lui-muted); text-decoration: none; font-size: 0.875rem; font-weight: 500; }
.lui-back::before { content: "\2190"; margin-right: 0.35em; }
.lui-back:hover { color: var(--lui-fg); }
.lui-lede { font-size: 1.125rem; line-height: 1.75rem; color: var(--lui-muted); margin-bottom: 1rem; }
.lui-built { color: var(--lui-muted); font-size: 0.875rem; margin: 0 0 1.5rem; }
.lui-built code {
  display: inline-block; max-width: 100%; box-sizing: border-box; margin: 0 0.25rem 0.25rem 0; padding: 0.125rem 0.5rem; overflow-wrap: anywhere;
  font-size: 0.75rem; line-height: 1rem; font-weight: 500; color: var(--lui-fg);
  background: transparent; border: 1px solid var(--lui-line); border-radius: var(--lui-radius-sm);
}
/* A component page's plate: the live component on a stage, the code that drew it joined
   underneath. One per page; no transform, overflow or contain on the stage, so dialogs,
   drawers and toasts still escape it. */
.lui-plate { margin: 0 0 2rem; }
.lui-stage {
  padding: calc(var(--lui-space) * 5) calc(var(--lui-space) * 4); background: var(--lui-gray-2);
  border-radius: var(--lui-radius-lg) var(--lui-radius-lg) 0 0;
}
.lui-stage > :last-child { margin-bottom: 0; }
.lui-stage h2:first-child { margin-top: 0; }
@media (max-width: 40rem) { .lui-stage { padding: calc(var(--lui-space) * 3) calc(var(--lui-space) * 2); } }
.lui-snippet {
  margin: 0; max-width: none; overflow: hidden;
  border: 1px solid var(--lui-line); border-radius: 0 0 var(--lui-radius-lg) var(--lui-radius-lg);
  background: var(--lui-surface);
}
.lui-snippet figcaption {
  display: flex; flex-wrap: wrap; justify-content: space-between; gap: 0.25rem 1rem;
  padding: 0.5rem calc(var(--lui-space) * 2); border-bottom: 1px solid var(--lui-line);
  color: var(--lui-muted); font-size: 0.75rem;
}
.lui-snippet figcaption span:first-child { color: var(--lui-fg); font-weight: 500; font-family: var(--lui-font-mono); }
.lui-snippet pre { margin: 0; padding: calc(var(--lui-space) * 2); overflow-x: auto; scrollbar-color: var(--lui-line) transparent; line-height: 1.7; tab-size: 4; }
.lui-snippet pre code { background: none; border: 0; padding: 0; font-size: 0.8125rem; color: var(--lui-fg); }
/* Highlighted Rust in the status colours, as a GitHub-like theme: keywords in danger, strings
   in ok, numbers and types in warn, comments muted, macros bold. */
.lui-hl-k { color: var(--lui-danger); }
.lui-hl-s { color: var(--lui-ok); }
.lui-hl-n, .lui-hl-t { color: color-mix(in srgb, var(--lui-warn) 70%, var(--lui-fg)); }
.lui-hl-c { color: var(--lui-muted); font-style: italic; }
.lui-hl-m { color: var(--lui-fg); font-weight: 600; }
.lui-hl-f { color: var(--lui-fg); }
/* The demo's props tables under the snippet: one <details> per builder. */
.lui-theme-builder { display: grid; gap: calc(var(--lui-space) * 2); max-width: none; }
.lui-theme-builder fieldset > div { display: grid; grid-template-columns: repeat(auto-fill, minmax(min(18rem, 100%), 1fr)); gap: var(--lui-space); }
.lui-theme-builder fieldset { margin: 0; padding: calc(var(--lui-space) * 2); border: 1px solid var(--lui-line); border-radius: var(--lui-radius); }
.lui-theme-previews { display: grid; grid-template-columns: repeat(auto-fit, minmax(min(18rem, 100%), 1fr)); gap: calc(var(--lui-space) * 2); margin-block: calc(var(--lui-space) * 2); }
.lui-theme-preview { padding: calc(var(--lui-space) * 2); background: var(--lui-bg); color: var(--lui-fg); border: 1px solid var(--lui-line); border-radius: var(--lui-radius); }
/* The builder's depth row: a tile per shadow step, then the primary gradient. */
.lui-theme-depth { display: flex; flex-wrap: wrap; gap: calc(var(--lui-space) * 2); margin-top: calc(var(--lui-space) * 3); }
.lui-theme-depth span { display: grid; place-items: center; width: 3.5rem; height: 3.5rem; border-radius: var(--lui-radius); background: var(--lui-card); color: var(--lui-muted); font-size: 0.75rem; }
.lui-theme-depth .lui-theme-gradient { background: var(--lui-primary); background-image: var(--lui-gradient-primary); box-shadow: var(--lui-shadow-sm), var(--lui-highlight); }
.lui-props { max-width: none; margin: 0 0 2rem; }
.lui-props details { border: 1px solid var(--lui-line); border-radius: var(--lui-radius); margin: 0 0 0.5rem; }
.lui-props summary { display: flex; flex-wrap: wrap; align-items: baseline; gap: 0.25rem 0.5rem; padding: 0.5rem 0.75rem; cursor: pointer; }
.lui-props summary span { margin-left: auto; color: var(--lui-muted); font-size: 0.8125rem; }
.lui-props-scroll { overflow-x: auto; border-top: 1px solid var(--lui-line); }
.lui-caps-scroll { overflow-x: auto; }
.lui-playground > form > p { margin: 0; padding: 0.5rem 0.75rem; border-top: 1px solid var(--lui-line); }
.lui-playground-preview { padding: 1rem 0.75rem; border-top: 1px solid var(--lui-line); }
.lui-playground > pre { margin: 0; padding: 0.5rem 0.75rem; border-top: 1px solid var(--lui-line); overflow-x: auto; font-size: 0.8125rem; }
.lui-props table { width: 100%; border-collapse: collapse; font-size: 0.8125rem; }
.lui-props th, .lui-props td { text-align: left; vertical-align: top; padding: 0.375rem 0.75rem; border-bottom: 1px solid var(--lui-line); }
.lui-props th { color: var(--lui-muted); font-weight: 500; white-space: nowrap; }
.lui-props tbody tr:last-child td { border-bottom: 0; }
.lui-props td:nth-child(-n+2), .lui-props td:nth-child(4), .lui-props td:nth-child(5) { white-space: nowrap; }
.lui-props td:nth-child(3) { min-width: 10rem; }
.lui-props td:nth-child(6) { min-width: 16rem; }
.lui-props td:nth-child(7) { white-space: nowrap; }
.lui-index { max-width: none; }
.lui-index h2 { margin: 3rem 0 0.25rem; font-size: 1.5rem; line-height: 2rem; }
.lui-index-layer { margin: 0 0 1rem; color: var(--lui-muted); font-size: 0.875rem; }
.lui-index h3 { margin: 1.5rem 0 0.75rem; font-size: 0.875rem; font-weight: 500; color: var(--lui-muted); text-transform: uppercase; letter-spacing: 0.05em; }
.lui-index > ul { list-style: none; margin: 0; padding: 0; display: grid; grid-template-columns: repeat(auto-fill, minmax(18rem, 1fr)); gap: 1rem; }
/* Each component is a card: its name (the link to its page), what it is for, and the
   component itself, live, on a small stage. Widgets, blocks and flows take the whole row. */
.lui-index > ul > li {
  display: grid; grid-template-rows: auto auto 1fr; min-width: 0; max-width: none; padding: 1rem;
  background: var(--lui-card); border: 1px solid var(--lui-line); border-radius: var(--lui-radius-lg);
  box-shadow: var(--lui-shadow-xs);
}
.lui-index > ul > li.lui-index-wide { grid-column: 1 / -1; }
.lui-index > ul > li > p { margin: 0 0 0.25rem; }
.lui-index > ul > li > p:first-child > a { font-size: 1rem; font-weight: 600; line-height: 1.5rem; color: var(--lui-fg); text-decoration: none; }
.lui-index > ul > li > p:first-child > a:hover { text-decoration: underline; }
.lui-index-stage {
  min-width: 0; margin-top: 0.75rem; padding: calc(var(--lui-space) * 3) calc(var(--lui-space) * 2);
  background: var(--lui-bg); border: 1px solid var(--lui-line); border-radius: var(--lui-radius);
}
.lui-index-stage > :last-child { margin-bottom: 0; }
/* The theme builder's radius tiles and its table of every token, a swatch per scheme. */
.lui-theme-depth .lui-theme-corner { background: var(--lui-secondary); border: 1px solid var(--lui-line); }
.lui-theme-tokens { grid-column: 1 / -1; }
.lui-theme-tokens > div { max-height: 32rem; overflow: auto; margin-top: var(--lui-space-2); }
.lui-theme-tokens td { vertical-align: middle; font-family: var(--lui-font-mono); font-size: 0.75rem; }
.lui-theme-light, .lui-theme-dark { background: var(--lui-bg); color: var(--lui-fg); }
.lui-theme-light { color-scheme: light; }
.lui-theme-dark { color-scheme: dark; }
.lui-theme-tokens i { display: inline-block; vertical-align: middle; margin-inline-end: var(--lui-space-2); background: var(--lui-swatch); width: 2rem; height: 1.25rem; border: 1px solid var(--lui-line); border-radius: var(--lui-radius-sm); }
/* The modes page: one stage per mode, side by side when they fit. */
.lui-modes { display: grid; grid-template-columns: repeat(auto-fit, minmax(min(18rem, 100%), 1fr)); gap: var(--lui-space-4); }
.lui-modes > section { padding: var(--lui-space-4); border: 1px solid var(--lui-line); border-radius: var(--lui-radius); }
.lui-modes h2 { margin-top: 0; font-size: 1rem; }
"#;
