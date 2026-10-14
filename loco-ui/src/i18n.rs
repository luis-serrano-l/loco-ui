//! # Languages
//!
//! Every word a component writes by itself ("Next", "Close", "Load more", "Page 2 of 9",
//! month names) comes from one table, [`Strings`], chosen per request: `ui.lang()` is the
//! visitor's language and `<html lang>` says it. English is built in; an app adds its own
//! tables and registers them once with [`languages`]. The language is the `lui-lang` cookie
//! when it names a registered one, else the first `Accept-Language` entry that does (by
//! exact tag, then by primary subtag: `de-AT` finds `de`), else the first registered table.
//!
//! ```rust
//! use loco_ui::i18n::{self, Strings, Text};
//! use loco_ui::prelude::*;
//!
//! static SPANISH: Strings = Strings::new("es")
//!     .with(Text::Next, "Siguiente")
//!     .with(Text::Previous, "Anterior")
//!     .with(Text::LoadMore, "Cargar más");
//! static LANGUAGES: [&Strings; 2] = [&Strings::ENGLISH, &SPANISH];
//! i18n::languages(&LANGUAGES); // once, at start-up
//!
//! let ui = Ui::from_request("/", "", "").accept_language("es-MX,es;q=0.9,en;q=0.5");
//! assert_eq!(ui.lang(), "es");
//! assert_eq!(ui.text(Text::Next), "Siguiente");
//! assert_eq!(ui.text(Text::Close), "Close"); // not translated: English
//! assert!(ui.page("Hola", html! {}).into_string().contains(r#"<html lang="es""#));
//! // The cookie wins over the header.
//! let ui = Ui::from_request("/", "", "lui-lang=en").accept_language("es");
//! assert_eq!(ui.lang(), "en");
//! ```
//!
//! A table can also be filled from any lookup, such as Loco's `fluent-templates` loader
//! (`LOCALES.try_lookup(&lang, key)`): [`Strings::from_lookup`] asks for each [`Text::key`]
//! (`lui-next`, `lui-load-more`, …) and keeps English for what the lookup does not have.
//!
//! A table is more than words. Its tag brings CLDR data for en, es, fr, de, ar, he, ru and
//! pl (English's for any other tag), each part overridden by a setter:
//!
//! - direction, [`Strings::dir`], written as `<html dir>`; `?dir=rtl` or `?dir=ltr` on any
//!   page overrides it ([`crate::Ui::dir`]);
//! - the plural rule, [`Strings::count`]: a counted text holds a form per CLDR category,
//!   `one:{} match|other:{} matches` ([`Strings::with_plural`] for another language);
//! - number separators, [`Strings::number`]: `1,234.5`, `1.234,5`, `1 234,5`
//!   ([`Strings::with_number`]);
//! - the first day of the week, [`Strings::first_day`], where a calendar's rows start
//!   ([`Strings::with_first_day`]).
//!
//! An app's own components keep their words in the same table: [`Strings::with_custom`]
//! takes `(key, text)` pairs and `ui.text_key("notes.pinned")` reads them.
//!
//! ```rust
//! use loco_ui::i18n::{Dir, Strings, Text};
//! static GERMAN: Strings = Strings::new("de").with(Text::Matches, "one:{} Treffer|other:{} Treffer");
//! assert_eq!(GERMAN.number(1234.5), "1.234,5");
//! assert_eq!(GERMAN.count(Text::Matches, 2), "2 Treffer");
//! assert_eq!(Strings::new("he").dir(), Dir::Rtl);
//! assert_eq!(Strings::new("en-US").first_day(), 6);
//! ```

use std::fmt::Display;
use std::sync::{Mutex, OnceLock};

/// Every string a component writes by itself. `{}` in a text is filled in order by
/// [`Strings::fill`]; `{0}`, `{1}` by position, for a language that orders them otherwise.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Text {
    /// "Next": wizard, pager.
    Next,
    /// "Previous": pager.
    Previous,
    /// "Back": wizard.
    Back,
    /// "Skip": wizard.
    Skip,
    /// "Finish": wizard.
    Finish,
    /// "Close": dialog, drawer.
    Close,
    /// "Cancel": dialog, table edit.
    Cancel,
    /// "Dismiss": flash, toast.
    Dismiss,
    /// "Dismiss: {}": the dismiss link's name, with the message.
    DismissMessage,
    /// "Submit": a form's button when none is named.
    Submit,
    /// "Search": combobox, palette.
    Search,
    /// "Type to search…": combobox.
    TypeToSearch,
    /// "No matches.": combobox.
    NoMatches,
    /// "Create": combobox.
    Create,
    /// "Create “{}”": combobox, with what was typed.
    CreateValue,
    /// "Remove {}": combobox chip, upload file.
    RemoveValue,
    /// "Selected": combobox chips.
    Selected,
    /// "Results": combobox list.
    Results,
    /// "Load more": pager.
    LoadMore,
    /// "Showing {} of {}": pager.
    ShowingOf,
    /// "Loading": skeleton, spinner.
    Loading,
    /// "Pages": the table pager's `<nav>`.
    Pages,
    /// "Page": before the page number box.
    Page,
    /// "of {}": after the page number box.
    OfTotal,
    /// "{}–{} of {}": the rows shown.
    RangeOf,
    /// "Go": the page jump and tab select buttons.
    Go,
    /// "Rows per page": table.
    RowsPerPage,
    /// "Show": the rows-per-page button.
    Show,
    /// "No rows match.": table.
    NoRows,
    /// "Columns": table column chooser.
    Columns,
    /// "Edit": table row, wizard summary.
    Edit,
    /// "Save": table row edit.
    Save,
    /// "Expand all": accordion.
    ExpandAll,
    /// "Collapse all": accordion.
    CollapseAll,
    /// "Breadcrumb": the breadcrumbs' `<nav>`.
    Breadcrumb,
    /// "Show {} more": collapsed breadcrumbs.
    ShowMore,
    /// "Previous month": calendar.
    PreviousMonth,
    /// "Next month": calendar.
    NextMonth,
    /// "{0}, {1} {2} {3}": a day in full (weekday, day, month, year).
    LongDate,
    /// "Pick a date": date picker.
    PickDate,
    /// "Opacity {}": colour picker.
    Opacity,
    /// "Presets": colour picker.
    Presets,
    /// "Use {}": a colour preset.
    UseValue,
    /// "Reset": counter.
    Reset,
    /// "Value": counter.
    Value,
    /// "Set": counter.
    Set,
    /// "{} to {}": counter bounds.
    FromTo,
    /// "at least {}": counter bound.
    AtLeast,
    /// "at most {}": counter bound.
    AtMost,
    /// ", in steps of {}": counter step.
    InStepsOf,
    /// "No cards": kanban column.
    NoCards,
    /// ", over the limit": kanban column count.
    OverLimit,
    /// "Move {} to {}": kanban card.
    MoveTo,
    /// "Move {} up": sortable list item.
    MoveUp,
    /// "Move {} down": sortable list item.
    MoveDown,
    /// "Nothing by that name. Try one word, or pick from the list.": palette.
    NothingByThatName,
    /// "Type a command or a page": palette.
    TypeCommand,
    /// "Minimum": range.
    Minimum,
    /// "Maximum": range.
    Maximum,
    /// "Filter options": select.
    FilterOptions,
    /// "Filter": select.
    Filter,
    /// "Tab": tabs select.
    Tab,
    /// "Choose files or drop them on the button": upload.
    ChooseFiles,
    /// "Choose a file or drop it on the button": upload.
    ChooseFile,
    /// "Upload": upload.
    Upload,
    /// "Uploaded files": upload.
    UploadedFiles,
    /// "Picked up where you left off, at step {}.": wizard.
    Resumed,
    /// "Start over": wizard.
    StartOver,
    /// " (has errors)": wizard step.
    HasErrors,
    /// "Progress": wizard.
    Progress,
    /// "{} of {} steps done": wizard.
    StepsDone,
    /// "Step {}": wizard.
    Step,
    /// "(optional)": wizard step.
    Optional,
    /// "There is a problem": error summary.
    Problem,
    /// "More actions": context menu: its button.
    MoreActions,
    /// "{} digits": one-time code: the pattern's hint.
    Digits,
    /// "Delete": record page.
    Delete,
    /// "Sign out": app shell.
    SignOut,
    /// "Page not found": 404 page.
    NotFound,
    /// "The page you asked for is not here. It may have moved, or the link was mistyped.": 404 page.
    NotFoundMessage,
    /// "Something went wrong": 500 page.
    ServerError,
    /// "The server could not answer this time. Try again in a moment.": 500 page.
    ServerErrorMessage,
    /// "Go home": error pages.
    GoHome,
    /// "Sign in to see that page.": `loco::SignIn`, the flash on the sign-in form.
    SignInToSee,
    /// "(skipped)": wizard review.
    Skipped,
    /// "Edit {}": wizard review: the edit link's name.
    EditValue,
    /// "Step {} of {}": wizard.
    StepOf,
    /// "{0} {1} {2}": a day on a button (day, month, year).
    DayMonthYear,
    /// "{0} {1}": the calendar's title (month, year).
    MonthYear,
    /// "First": table pager.
    First,
    /// "Last": table pager.
    Last,
    /// "Filter rows": table filter label.
    FilterRows,
    /// "Filter rows…": table filter placeholder.
    FilterRowsHint,
    /// "Clear": table filter.
    Clear,
    /// "Download CSV": table.
    DownloadCsv,
    /// "Select": table: the select-all box.
    Select,
    /// "Select {}": table: a row's box.
    SelectRow,
    /// "Actions": table: the actions column.
    Actions,
    /// "Row actions": table: a row's menu.
    RowActions,
    /// "With the selected rows:": table bulk bar.
    WithSelected,
    /// "one:{} match|other:{} matches": combobox, palette, read with [`Strings::count`].
    Matches,
    /// "{} for “{}”": palette: the count and the query.
    ForQuery,
    /// " selected": combobox: after a picked result.
    IsSelected,
    /// "decrement": counter.
    Decrement,
    /// "increment": counter.
    Increment,
    /// "January": calendar.
    January,
    /// "February": calendar.
    February,
    /// "March": calendar.
    March,
    /// "April": calendar.
    April,
    /// "May": calendar.
    May,
    /// "June": calendar.
    June,
    /// "July": calendar.
    July,
    /// "August": calendar.
    August,
    /// "September": calendar.
    September,
    /// "October": calendar.
    October,
    /// "November": calendar.
    November,
    /// "December": calendar.
    December,
    /// "Monday": calendar.
    Monday,
    /// "Tuesday": calendar.
    Tuesday,
    /// "Wednesday": calendar.
    Wednesday,
    /// "Thursday": calendar.
    Thursday,
    /// "Friday": calendar.
    Friday,
    /// "Saturday": calendar.
    Saturday,
    /// "Sunday": calendar.
    Sunday,
    /// "Mo": calendar column head.
    MondayShort,
    /// "Tu": calendar column head.
    TuesdayShort,
    /// "We": calendar column head.
    WednesdayShort,
    /// "Th": calendar column head.
    ThursdayShort,
    /// "Fr": calendar column head.
    FridayShort,
    /// "Sa": calendar column head.
    SaturdayShort,
    /// "Su": calendar column head.
    SundayShort,
    /// "* required": a form with a required field, under its fields.
    RequiredHint,
    /// "This field is required.": a validator message.
    FieldRequired,
    /// "Enter a valid email address.": a validator message.
    FieldEmail,
    /// "Enter a valid URL.": a validator message.
    FieldUrl,
    /// "Check the length.": a validator message.
    FieldLength,
    /// "Enter a value in range.": a validator message.
    FieldRange,
    /// "The values do not match.": a validator message.
    FieldMustMatch,
    /// "Check the format.": a validator message.
    FieldFormat,
    /// "Check this field.": a validator message, and a value that does not parse.
    FieldCheck,
    /// "Check the form.": a posted form that could not be read at all.
    FormCheck,
    /// "New": the scaffold's new-record page and button ("New note").
    New,
    /// "Created.": the scaffold's flash after a create.
    Created,
    /// "Saved.": the scaffold's flash after an update.
    Saved,
    /// "Deleted.": the scaffold's flash after a delete.
    Deleted,
    /// "Delete {}?": record page: the delete dialog's title.
    DeleteConfirm,
    /// "This cannot be undone.": record page: the delete dialog's text.
    CannotUndo,
    /// "Skip to content": the skip link every page starts with.
    SkipToContent,
    /// "Then by {}": table: the link that adds a column as the next sort key.
    SortThenBy,
    /// "Filters": table: the panel of column filters.
    Filters,
    /// "Select all {} matching rows": table: the select-all box.
    SelectAllMatching,
    /// "Download JSON": table.
    DownloadJson,
    /// "Open": table: the row links column.
    Open,
    /// "Open {}": table: a row's link.
    OpenRow,
    /// "(opens in a new tab)": link: an external link's hint for screen readers.
    OpensInNewTab,
    /// "Code": code block: the region's name when it has no title.
    Code,
    /// "Copy": code block: the copy button.
    Copy,
    /// "Copied": code block: the copy button once it has copied.
    Copied,
    /// "one:{} new|other:{} new": indicator: the count in words, read with [`Strings::count`].
    IndicatorNew,
    /// "Sign in to continue": 401 page.
    Unauthorized,
    /// "This page is for signed-in visitors. Sign in, then come back.": 401 page.
    UnauthorizedMessage,
    /// "You cannot open this page": 403 page.
    Forbidden,
    /// "Your account does not have access to it. Ask an owner if you need it.": 403 page.
    ForbiddenMessage,
    /// "That could not be saved": 422 page.
    Unprocessable,
    /// "The form had something the server could not accept. Go back and check it.": 422 page.
    UnprocessableMessage,
    /// "Back in a moment": 503 page.
    Unavailable,
    /// "The site is down for maintenance or under heavy load. Try again in a few minutes.": 503 page.
    UnavailableMessage,
    /// "Apply": menu: a checkbox or radio item's button without script.
    Apply,
    /// "carousel": carousel: its role description.
    Carousel,
    /// "Previous slide": carousel.
    PreviousSlide,
    /// "Next slide": carousel.
    NextSlide,
    /// "Slide {} of {}": carousel: a dot's name.
    SlideOf,
    /// "slide": carousel: a slide's role description.
    Slide,
    /// "completed": steps: read after a done step's title.
    Completed,
    /// "More about {}": hover card: the button that opens the card by click.
    MoreAbout,
    /// "Show password": a password field's reveal button.
    ShowPassword,
    /// "one:{} star|other:{} stars": rating, one choice's name.
    Stars,
    /// "{} out of {}": rating, the value of a read-only rating.
    RatingOf,
    /// "Add": tag input, the add button.
    Add,
    /// "Wk": calendar, the week-number column's head.
    WeekShort,
    /// "Week {}": calendar, a week number as a screen reader hears it.
    WeekNumber,
    /// "Start date": date picker range, the first date's field.
    StartDate,
    /// "End date": date picker range, the last date's field.
    EndDate,
}

/// How many texts there are.
const N: usize = Text::ALL.len();

impl Text {
    /// Every text, in table order.
    pub const ALL: [Text; 183] = [
        Text::Next,
        Text::Previous,
        Text::Back,
        Text::Skip,
        Text::Finish,
        Text::Close,
        Text::Cancel,
        Text::Dismiss,
        Text::DismissMessage,
        Text::Submit,
        Text::Search,
        Text::TypeToSearch,
        Text::NoMatches,
        Text::Create,
        Text::CreateValue,
        Text::RemoveValue,
        Text::Selected,
        Text::Results,
        Text::LoadMore,
        Text::ShowingOf,
        Text::Loading,
        Text::Pages,
        Text::Page,
        Text::OfTotal,
        Text::RangeOf,
        Text::Go,
        Text::RowsPerPage,
        Text::Show,
        Text::NoRows,
        Text::Columns,
        Text::Edit,
        Text::Save,
        Text::ExpandAll,
        Text::CollapseAll,
        Text::Breadcrumb,
        Text::ShowMore,
        Text::PreviousMonth,
        Text::NextMonth,
        Text::LongDate,
        Text::PickDate,
        Text::Opacity,
        Text::Presets,
        Text::UseValue,
        Text::Reset,
        Text::Value,
        Text::Set,
        Text::FromTo,
        Text::AtLeast,
        Text::AtMost,
        Text::InStepsOf,
        Text::NoCards,
        Text::OverLimit,
        Text::MoveTo,
        Text::MoveUp,
        Text::MoveDown,
        Text::NothingByThatName,
        Text::TypeCommand,
        Text::Minimum,
        Text::Maximum,
        Text::FilterOptions,
        Text::Filter,
        Text::Tab,
        Text::ChooseFiles,
        Text::ChooseFile,
        Text::Upload,
        Text::UploadedFiles,
        Text::Resumed,
        Text::StartOver,
        Text::HasErrors,
        Text::Progress,
        Text::StepsDone,
        Text::Step,
        Text::Optional,
        Text::Problem,
        Text::MoreActions,
        Text::Digits,
        Text::Delete,
        Text::SignOut,
        Text::NotFound,
        Text::NotFoundMessage,
        Text::ServerError,
        Text::ServerErrorMessage,
        Text::GoHome,
        Text::SignInToSee,
        Text::Skipped,
        Text::EditValue,
        Text::StepOf,
        Text::DayMonthYear,
        Text::MonthYear,
        Text::First,
        Text::Last,
        Text::FilterRows,
        Text::FilterRowsHint,
        Text::Clear,
        Text::DownloadCsv,
        Text::Select,
        Text::SelectRow,
        Text::Actions,
        Text::RowActions,
        Text::WithSelected,
        Text::Matches,
        Text::ForQuery,
        Text::IsSelected,
        Text::Decrement,
        Text::Increment,
        Text::January,
        Text::February,
        Text::March,
        Text::April,
        Text::May,
        Text::June,
        Text::July,
        Text::August,
        Text::September,
        Text::October,
        Text::November,
        Text::December,
        Text::Monday,
        Text::Tuesday,
        Text::Wednesday,
        Text::Thursday,
        Text::Friday,
        Text::Saturday,
        Text::Sunday,
        Text::MondayShort,
        Text::TuesdayShort,
        Text::WednesdayShort,
        Text::ThursdayShort,
        Text::FridayShort,
        Text::SaturdayShort,
        Text::SundayShort,
        Text::RequiredHint,
        Text::FieldRequired,
        Text::FieldEmail,
        Text::FieldUrl,
        Text::FieldLength,
        Text::FieldRange,
        Text::FieldMustMatch,
        Text::FieldFormat,
        Text::FieldCheck,
        Text::FormCheck,
        Text::New,
        Text::Created,
        Text::Saved,
        Text::Deleted,
        Text::DeleteConfirm,
        Text::CannotUndo,
        Text::SkipToContent,
        Text::SortThenBy,
        Text::Filters,
        Text::SelectAllMatching,
        Text::DownloadJson,
        Text::Open,
        Text::OpenRow,
        Text::OpensInNewTab,
        Text::Code,
        Text::Copy,
        Text::Copied,
        Text::IndicatorNew,
        Text::Unauthorized,
        Text::UnauthorizedMessage,
        Text::Forbidden,
        Text::ForbiddenMessage,
        Text::Unprocessable,
        Text::UnprocessableMessage,
        Text::Unavailable,
        Text::UnavailableMessage,
        Text::Apply,
        Text::Carousel,
        Text::PreviousSlide,
        Text::NextSlide,
        Text::SlideOf,
        Text::Slide,
        Text::Completed,
        Text::MoreAbout,
        Text::ShowPassword,
        Text::Stars,
        Text::RatingOf,
        Text::Add,
        Text::WeekShort,
        Text::WeekNumber,
        Text::StartDate,
        Text::EndDate,
    ];

    /// The month `number`, 1 to 12 (0 is January, 13 January again).
    pub const fn month(number: u32) -> Text {
        Text::ALL[Text::January as usize + number.saturating_sub(1) as usize % 12]
    }

    /// The weekday `number`, 0 for Monday to 6 for Sunday.
    pub const fn weekday(number: u32) -> Text {
        Text::ALL[Text::Monday as usize + number as usize % 7]
    }

    /// The weekday `number` abbreviated for a calendar's column head ("Mo"), 0 for Monday.
    pub const fn weekday_short(number: u32) -> Text {
        Text::ALL[Text::MondayShort as usize + number as usize % 7]
    }

    /// The key a lookup is asked for: `lui-` and the name in kebab case (`lui-load-more`).
    pub fn key(self) -> String {
        let name = format!("{self:?}");
        let mut key = String::from("lui");
        for c in name.chars() {
            if c.is_ascii_uppercase() {
                key.push('-');
            }
            key.push(c.to_ascii_lowercase());
        }
        key
    }
}

/// The English table, in [`Text::ALL`] order.
const ENGLISH: [&str; N] = [
    "Next",
    "Previous",
    "Back",
    "Skip",
    "Finish",
    "Close",
    "Cancel",
    "Dismiss",
    "Dismiss: {}",
    "Submit",
    "Search",
    "Type to search\u{2026}",
    "No matches.",
    "Create",
    "Create \u{201c}{}\u{201d}",
    "Remove {}",
    "Selected",
    "Results",
    "Load more",
    "Showing {} of {}",
    "Loading",
    "Pages",
    "Page",
    "of {}",
    "{}\u{2013}{} of {}",
    "Go",
    "Rows per page",
    "Show",
    "No rows match.",
    "Columns",
    "Edit",
    "Save",
    "Expand all",
    "Collapse all",
    "Breadcrumb",
    "Show {} more",
    "Previous month",
    "Next month",
    "{0}, {1} {2} {3}",
    "Pick a date",
    "Opacity {}",
    "Presets",
    "Use {}",
    "Reset",
    "Value",
    "Set",
    "{} to {}",
    "at least {}",
    "at most {}",
    ", in steps of {}",
    "No cards",
    ", over the limit",
    "Move {} to {}",
    "Move {} up",
    "Move {} down",
    "Nothing by that name. Try one word, or pick from the list.",
    "Type a command or a page",
    "Minimum",
    "Maximum",
    "Filter options",
    "Filter",
    "Tab",
    "Choose files or drop them on the button",
    "Choose a file or drop it on the button",
    "Upload",
    "Uploaded files",
    "Picked up where you left off, at step {}.",
    "Start over",
    " (has errors)",
    "Progress",
    "{} of {} steps done",
    "Step {}",
    "(optional)",
    "There is a problem",
    "More actions",
    "{} digits",
    "Delete",
    "Sign out",
    "Page not found",
    "The page you asked for is not here. It may have moved, or the link was mistyped.",
    "Something went wrong",
    "The server could not answer this time. Try again in a moment.",
    "Go home",
    "Sign in to see that page.",
    "(skipped)",
    "Edit {}",
    "Step {} of {}",
    "{0} {1} {2}",
    "{0} {1}",
    "First",
    "Last",
    "Filter rows",
    "Filter rows\u{2026}",
    "Clear",
    "Download CSV",
    "Select",
    "Select {}",
    "Actions",
    "Row actions",
    "With the selected rows:",
    "one:{} match|other:{} matches",
    "{} for \u{201c}{}\u{201d}",
    " selected",
    "decrement",
    "increment",
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
    "Mo",
    "Tu",
    "We",
    "Th",
    "Fr",
    "Sa",
    "Su",
    "* required",
    "This field is required.",
    "Enter a valid email address.",
    "Enter a valid URL.",
    "Check the length.",
    "Enter a value in range.",
    "The values do not match.",
    "Check the format.",
    "Check this field.",
    "Check the form.",
    "New",
    "Created.",
    "Saved.",
    "Deleted.",
    "Delete {}?",
    "This cannot be undone.",
    "Skip to content",
    "Then by {}",
    "Filters",
    "Select all {} matching rows",
    "Download JSON",
    "Open",
    "Open {}",
    "(opens in a new tab)",
    "Code",
    "Copy",
    "Copied",
    "one:{} new|other:{} new",
    "Sign in to continue",
    "This page is for signed-in visitors. Sign in, then come back.",
    "You cannot open this page",
    "Your account does not have access to it. Ask an owner if you need it.",
    "That could not be saved",
    "The form had something the server could not accept. Go back and check it.",
    "Back in a moment",
    "The site is down for maintenance or under heavy load. Try again in a few minutes.",
    "Apply",
    "carousel",
    "Previous slide",
    "Next slide",
    "Slide {} of {}",
    "slide",
    "completed",
    "More about {}",
    "Show password",
    "one:{} star|other:{} stars",
    "{} out of {}",
    "Add",
    "Wk",
    "Week {}",
    "Start date",
    "End date",
];

/// Which way a language's text runs: `<html dir>`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Dir {
    /// Left to right.
    #[default]
    Ltr,
    /// Right to left: Arabic, Hebrew.
    Rtl,
}

impl Dir {
    /// `"ltr"` or `"rtl"`, as the `dir` attribute writes it.
    pub const fn as_str(self) -> &'static str {
        match self {
            Dir::Ltr => "ltr",
            Dir::Rtl => "rtl",
        }
    }

    /// `ltr` or `rtl` (any case); `None` for anything else.
    pub fn parse(text: &str) -> Option<Dir> {
        let text = text.trim();
        if text.eq_ignore_ascii_case("ltr") {
            Some(Dir::Ltr)
        } else if text.eq_ignore_ascii_case("rtl") {
            Some(Dir::Rtl)
        } else {
            None
        }
    }
}

/// A CLDR plural category: which form of a counted text a number takes in a language
/// (English has `One` and `Other`, Russian `One`, `Few` and `Many`, Arabic all six).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Plural {
    /// Arabic 0.
    Zero,
    /// 1 in most languages; 21, 31 in Russian.
    One,
    /// Arabic and Hebrew 2.
    Two,
    /// Russian and Polish 2 to 4, Arabic 3 to 10.
    Few,
    /// Russian and Polish 5 to 20, Arabic 11 to 99, Spanish and French a million.
    Many,
    /// Everything else.
    Other,
}

impl Plural {
    /// The category's CLDR name, the tag that marks its form in a text (`one:`, `few:`).
    pub const fn as_str(self) -> &'static str {
        match self {
            Plural::Zero => "zero",
            Plural::One => "one",
            Plural::Two => "two",
            Plural::Few => "few",
            Plural::Many => "many",
            Plural::Other => "other",
        }
    }

    const ALL: [Plural; 6] = [
        Plural::Zero,
        Plural::One,
        Plural::Two,
        Plural::Few,
        Plural::Many,
        Plural::Other,
    ];
}

/// A language's plural rule: the category a whole number takes.
pub type PluralRule = fn(u64) -> Plural;

fn one_other(n: u64) -> Plural {
    if n == 1 { Plural::One } else { Plural::Other }
}

fn spanish(n: u64) -> Plural {
    match n {
        1 => Plural::One,
        n if n != 0 && n % 1_000_000 == 0 => Plural::Many,
        _ => Plural::Other,
    }
}

fn french(n: u64) -> Plural {
    match n {
        0 | 1 => Plural::One,
        n if n % 1_000_000 == 0 => Plural::Many,
        _ => Plural::Other,
    }
}

fn arabic(n: u64) -> Plural {
    match (n, n % 100) {
        (0, _) => Plural::Zero,
        (1, _) => Plural::One,
        (2, _) => Plural::Two,
        (_, 3..=10) => Plural::Few,
        (_, 11..=99) => Plural::Many,
        _ => Plural::Other,
    }
}

fn hebrew(n: u64) -> Plural {
    match n {
        1 => Plural::One,
        2 => Plural::Two,
        _ => Plural::Other,
    }
}

fn russian(n: u64) -> Plural {
    match (n % 10, n % 100) {
        (1, r) if r != 11 => Plural::One,
        (2..=4, r) if !(12..=14).contains(&r) => Plural::Few,
        _ => Plural::Many,
    }
}

fn polish(n: u64) -> Plural {
    match (n, n % 10, n % 100) {
        (1, _, _) => Plural::One,
        (_, 2..=4, r) if !(12..=14).contains(&r) => Plural::Few,
        _ => Plural::Many,
    }
}

/// What a language tag brings with it besides words: CLDR data for the languages the
/// components are known to meet. A tag not listed takes English's.
#[derive(Clone, Copy)]
struct Locale {
    tag: &'static str,
    dir: Dir,
    plural: PluralRule,
    group: &'static str,
    decimal: &'static str,
    /// CLDR `minimumGroupingDigits`: Spanish and Polish write 1234 but 12 345.
    min_grouping: u8,
    /// 0 for Monday to 6 for Sunday.
    first_day: u8,
}

const fn locale(
    tag: &'static str,
    dir: Dir,
    plural: PluralRule,
    (group, decimal, min_grouping): (&'static str, &'static str, u8),
    first_day: u8,
) -> Locale {
    Locale {
        tag,
        dir,
        plural,
        group,
        decimal,
        min_grouping,
        first_day,
    }
}

const NBSP: &str = "\u{a0}";
const LOCALES: [Locale; 9] = [
    locale("en", Dir::Ltr, one_other, (",", ".", 1), 0),
    locale("en-US", Dir::Ltr, one_other, (",", ".", 1), 6),
    locale("es", Dir::Ltr, spanish, (".", ",", 2), 0),
    locale("fr", Dir::Ltr, french, ("\u{202f}", ",", 1), 0),
    locale("de", Dir::Ltr, one_other, (".", ",", 1), 0),
    locale("ar", Dir::Rtl, arabic, (",", ".", 1), 5),
    locale("he", Dir::Rtl, hebrew, (",", ".", 1), 6),
    locale("ru", Dir::Ltr, russian, (NBSP, ",", 1), 0),
    locale("pl", Dir::Ltr, polish, (NBSP, ",", 2), 0),
];

const fn same_tag(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        let (x, y) = (a[i].to_ascii_lowercase(), b[i].to_ascii_lowercase());
        if x != y && !(matches!(x, b'-' | b'_') && matches!(y, b'-' | b'_')) {
            return false;
        }
        i += 1;
    }
    true
}

const fn primary_subtag(tag: &[u8]) -> &[u8] {
    let mut i = 0;
    while i < tag.len() && !matches!(tag[i], b'-' | b'_') {
        i += 1;
    }
    tag.split_at(i).0
}

/// The data for `tag`: exact tag, then primary subtag, else English.
const fn locale_for(tag: &str) -> Locale {
    let tag = tag.as_bytes();
    let mut i = 0;
    while i < LOCALES.len() {
        if same_tag(LOCALES[i].tag.as_bytes(), tag) {
            return LOCALES[i];
        }
        i += 1;
    }
    let primary = primary_subtag(tag);
    let mut i = 0;
    while i < LOCALES.len() {
        if same_tag(LOCALES[i].tag.as_bytes(), primary) {
            return LOCALES[i];
        }
        i += 1;
    }
    LOCALES[0]
}

/// One language's texts, and how it writes numbers, counts, dates and direction. Start
/// from [`Strings::new`] (English words, and the CLDR data of the tag when it is one of en,
/// es, fr, de, ar, he, ru, pl) and set what you translate with [`Strings::with`], in a
/// `static`; or fill one at run time with [`Strings::from_lookup`].
///
/// ```rust
/// use loco_ui::i18n::{Dir, Plural, Strings, Text};
/// static RUSSIAN: Strings = Strings::new("ru")
///     .with(Text::Matches, "one:{} совпадение|few:{} совпадения|many:{} совпадений");
/// assert_eq!(RUSSIAN.count(Text::Matches, 21), "21 совпадение");
/// assert_eq!(RUSSIAN.count(Text::Matches, 3), "3 совпадения");
/// assert_eq!(RUSSIAN.count(Text::Matches, 12), "12 совпадений");
/// assert_eq!(RUSSIAN.number(1234567.5), "1\u{a0}234\u{a0}567,5");
/// assert_eq!(Strings::new("ar").dir(), Dir::Rtl);
/// assert_eq!(Strings::ENGLISH.plural(1), Plural::One);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Strings {
    lang: &'static str,
    texts: [&'static str; N],
    dir: Dir,
    plural: PluralRule,
    group: &'static str,
    decimal: &'static str,
    min_grouping: u8,
    first_day: u8,
    custom: &'static [(&'static str, &'static str)],
}

/// Two tables are the same when they say the same; the plural rule, a function, is left
/// out (two copies of one `fn` need not compare equal).
impl PartialEq for Strings {
    fn eq(&self, other: &Strings) -> bool {
        (self.lang, self.texts, self.dir, self.group, self.decimal)
            == (
                other.lang,
                other.texts,
                other.dir,
                other.group,
                other.decimal,
            )
            && (self.min_grouping, self.first_day, self.custom)
                == (other.min_grouping, other.first_day, other.custom)
    }
}

impl Eq for Strings {}

impl Default for &'static Strings {
    fn default() -> Self {
        &Strings::ENGLISH
    }
}

/// Where a text sits in the table: `ALL` is in declaration order.
const fn slot(text: Text) -> usize {
    text as usize
}

/// A text's form for `category`: a text written `one:… |other:…` holds one form per CLDR
/// category; `other` stands in for a category it lacks, and a text without tags is its
/// only form.
fn form(template: &'static str, category: Plural) -> &'static str {
    let tagged = |part: &'static str| {
        Plural::ALL.into_iter().find_map(|p| {
            let form = part.strip_prefix(p.as_str())?.strip_prefix(':')?;
            Some((p, form))
        })
    };
    if tagged(template).is_none() {
        return template;
    }
    let forms = template.split('|').filter_map(tagged);
    let mut fallback = template;
    for (p, text) in forms {
        if p == category {
            return text;
        }
        if p == Plural::Other || fallback == template {
            fallback = text;
        }
    }
    fallback
}

impl Strings {
    /// The built-in table, `lang="en"`.
    pub const ENGLISH: Strings = Strings::new("en");

    /// English texts under the language tag `lang` (`"es"`, `"pt-BR"`), to translate with
    /// [`Strings::with`]. Direction, plural rule, number separators and the first day of
    /// the week come from the tag (CLDR, for en, es, fr, de, ar, he, ru and pl; English's for
    /// any other), each overridden by its own setter.
    pub const fn new(lang: &'static str) -> Strings {
        let l = locale_for(lang);
        Strings {
            lang,
            texts: ENGLISH,
            dir: l.dir,
            plural: l.plural,
            group: l.group,
            decimal: l.decimal,
            min_grouping: l.min_grouping,
            first_day: l.first_day,
            custom: &[],
        }
    }

    /// The same table with `text` translated.
    pub const fn with(mut self, text: Text, translated: &'static str) -> Strings {
        self.texts[slot(text)] = translated;
        self
    }

    /// The same table written in direction `dir`.
    pub const fn with_dir(mut self, dir: Dir) -> Strings {
        self.dir = dir;
        self
    }

    /// The same table counting with `rule`, for a language the built-in data lacks.
    pub const fn with_plural(mut self, rule: PluralRule) -> Strings {
        self.plural = rule;
        self
    }

    /// The same table writing numbers with `group` between thousands and `decimal` before
    /// the fraction (`"."` and `","` for German).
    pub const fn with_number(mut self, group: &'static str, decimal: &'static str) -> Strings {
        self.group = group;
        self.decimal = decimal;
        self
    }

    /// The same table with weeks starting on `day`, 0 for Monday to 6 for Sunday.
    pub const fn with_first_day(mut self, day: u8) -> Strings {
        self.first_day = day % 7;
        self
    }

    /// The same table with the app's own texts, `(key, text)` pairs read by
    /// [`Strings::custom`] and `ui.text_key(key)`.
    ///
    /// ```rust
    /// use loco_ui::i18n::Strings;
    /// static SPANISH: Strings = Strings::new("es").with_custom(&[("notes.pinned", "Fijada")]);
    /// assert_eq!(SPANISH.custom("notes.pinned"), Some("Fijada"));
    /// assert_eq!(SPANISH.custom("notes.archived"), None);
    /// ```
    pub const fn with_custom(mut self, table: &'static [(&'static str, &'static str)]) -> Strings {
        self.custom = table;
        self
    }

    /// The language tag, for `<html lang>`.
    pub const fn lang(&self) -> &'static str {
        self.lang
    }

    /// Which way the language runs, for `<html dir>`.
    pub const fn dir(&self) -> Dir {
        self.dir
    }

    /// The first day of the week, 0 for Monday to 6 for Sunday: where a calendar's rows
    /// start.
    pub const fn first_day(&self) -> u8 {
        self.first_day
    }

    /// The text as written in the table, placeholders (and plural forms) and all.
    pub const fn get(&self, text: Text) -> &'static str {
        self.texts[slot(text)]
    }

    /// The app's own text `key`, from [`Strings::with_custom`].
    pub fn custom(&self, key: &str) -> Option<&'static str> {
        self.custom.iter().find(|(k, _)| *k == key).map(|(_, v)| *v)
    }

    /// The plural category of `n` in this language.
    pub fn plural(&self, n: u64) -> Plural {
        (self.plural)(n)
    }

    /// The text with its placeholders filled: `{}` by the next argument, `{0}`, `{1}` by
    /// position. A text with plural forms uses its `other` form; [`Strings::count`] picks.
    ///
    /// ```rust
    /// use loco_ui::i18n::{Strings, Text};
    /// assert_eq!(Strings::ENGLISH.fill(Text::RangeOf, &[&11, &20, &42]), "11–20 of 42");
    /// assert_eq!(Strings::ENGLISH.fill(Text::LongDate, &[&"Thursday", &24, &"September", &2026]),
    ///            "Thursday, 24 September 2026");
    /// ```
    pub fn fill(&self, text: Text, args: &[&dyn Display]) -> String {
        fill_in(form(self.get(text), Plural::Other), args)
    }

    /// The form of `text` for the count `n`, with `n` (as [`Strings::number`] writes it) in
    /// its `{}`. A counted text holds one form per plural category, `one:{} match|other:{}
    /// matches`; one without tags is used for every count.
    ///
    /// ```rust
    /// use loco_ui::i18n::{Strings, Text};
    /// assert_eq!(Strings::ENGLISH.count(Text::Matches, 1), "1 match");
    /// assert_eq!(Strings::ENGLISH.count(Text::Matches, 1200), "1,200 matches");
    /// ```
    pub fn count(&self, text: Text, n: u64) -> String {
        fill_in(form(self.get(text), self.plural(n)), &[&self.digits(n)])
    }

    /// `n` with this language's separators: `1,234.5` in English, `1.234,5` in German,
    /// `1234,5` but `12.345` in Spanish. As many decimals as the shortest exact form needs.
    pub fn number(&self, n: f64) -> String {
        self.digits(n).to_string()
    }

    /// Any number written as Rust writes it (`i64`, `usize`, `f64`), regrouped: no `String`
    /// per number when written straight into a page.
    pub(crate) fn digits<T: Display>(&self, n: T) -> Digits<'_, T> {
        Digits(self, n)
    }

    /// `"24 September 2026"`: a day on a button or in a sentence.
    pub fn date(&self, date: crate::calendar::Date) -> String {
        let month = self.get(Text::month(u32::from(date.month)));
        self.fill(Text::DayMonthYear, &[&date.day, &month, &date.year])
    }

    /// `"Thursday, 24 September 2026"`: the day in full, as a screen reader hears it.
    pub fn date_long(&self, date: crate::calendar::Date) -> String {
        let weekday = self.get(Text::weekday(u32::from(date.weekday())));
        let month = self.get(Text::month(u32::from(date.month)));
        self.fill(Text::LongDate, &[&weekday, &date.day, &month, &date.year])
    }

    /// A table for `lang` filled from `lookup`, asked for each [`Text::key`]; English where
    /// it answers `None`. Built once per `lang` and kept for the life of the process, so the
    /// same `lang` always gives the first table built. Up to 32 languages are kept; past
    /// that the English table is returned. A lookup table has no [`Strings::custom`] texts:
    /// the app reads its own keys from the same lookup.
    ///
    /// With Loco's `fluent-templates` (keys like `lui-load-more = Cargar más` in
    /// `assets/i18n/es/main.ftl`):
    /// `Strings::from_lookup("es", |key| LOCALES.try_lookup(&langid!("es"), key))`.
    ///
    /// ```rust
    /// use loco_ui::i18n::{Strings, Text};
    /// let ftl = [("lui-load-more", "Mehr laden"), ("lui-next", "Weiter")];
    /// let german = Strings::from_lookup("de", |key| {
    ///     ftl.iter().find(|(k, _)| *k == key).map(|(_, v)| v.to_string())
    /// });
    /// assert_eq!(german.get(Text::LoadMore), "Mehr laden");
    /// assert_eq!(german.get(Text::Close), "Close");
    /// assert_eq!(german.number(1234.5), "1.234,5");
    /// ```
    pub fn from_lookup(lang: &str, lookup: impl Fn(&str) -> Option<String>) -> &'static Strings {
        for entry in &BUILT {
            let ours = match entry.lang.get() {
                Some(tag) => **tag == *lang,
                None => {
                    entry.lang.set(lang.into()).is_ok()
                        || entry.lang.get().is_some_and(|tag| **tag == *lang)
                }
            };
            let Some(tag) = entry.lang.get().filter(|_| ours) else {
                continue;
            };
            return entry.strings.get_or_init(|| {
                let texts = entry.texts.get_or_init(|| {
                    let found = Text::ALL.iter().map(|t| lookup(&t.key()));
                    found.map(|f| f.map(String::into_boxed_str)).collect()
                });
                let mut strings = Strings::new(tag);
                for (text, found) in Text::ALL.iter().zip(texts) {
                    if let Some(found) = found {
                        strings.texts[slot(*text)] = &**found;
                    }
                }
                strings
            });
        }
        &Strings::ENGLISH
    }
}

/// One language built by [`Strings::from_lookup`]: its tag, the texts the lookup gave and
/// the table pointing into them, each set once, so the table can be handed out as
/// `&'static` without leaking a box per call.
struct Built {
    lang: OnceLock<Box<str>>,
    texts: OnceLock<Vec<Option<Box<str>>>>,
    strings: OnceLock<Strings>,
}

static BUILT: [Built; 32] = [const {
    Built {
        lang: OnceLock::new(),
        texts: OnceLock::new(),
        strings: OnceLock::new(),
    }
}; 32];

/// `template` with `{}` filled by the next argument and `{0}`, `{1}` by position.
fn fill_in(template: &str, args: &[&dyn Display]) -> String {
    let mut out = String::with_capacity(template.len() + 16);
    let mut next = 0;
    let mut rest = template;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let Some(close) = rest[open..].find('}') else {
            out.push_str(&rest[open..]);
            return out;
        };
        let inside = &rest[open + 1..open + close];
        let index = if inside.is_empty() {
            next += 1;
            Some(next - 1)
        } else {
            inside.parse().ok()
        };
        match index.and_then(|i: usize| args.get(i)) {
            Some(arg) => out.push_str(&arg.to_string()),
            None => out.push_str(&rest[open..=open + close]),
        }
        rest = &rest[open + close + 1..];
    }
    out.push_str(rest);
    out
}

/// A number regrouped with a language's separators, made by [`Strings::digits`].
pub(crate) struct Digits<'a, T>(&'a Strings, T);

impl<T: Display> Display for Digits<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = self.1.to_string();
        let (sign, rest) = match text.strip_prefix('-') {
            Some(rest) => ("-", rest),
            None => ("", text.as_str()),
        };
        let (int, fraction) = rest.split_once('.').unwrap_or((rest, ""));
        if int.is_empty() || !int.bytes().all(|b| b.is_ascii_digit()) {
            // NaN and infinity, or a type that writes itself otherwise.
            return f.write_str(&text);
        }
        f.write_str(sign)?;
        let strings = self.0;
        let grouped = int.len() > 3 && int.len() >= 3 + usize::from(strings.min_grouping);
        for (i, digit) in int.char_indices() {
            if grouped && i > 0 && (int.len() - i) % 3 == 0 {
                f.write_str(strings.group)?;
            }
            std::fmt::Write::write_char(f, digit)?;
        }
        if !fraction.is_empty() {
            f.write_str(strings.decimal)?;
            f.write_str(fraction)?;
        }
        Ok(())
    }
}

impl crate::Redirect {
    /// Remember the language `tag` for this visitor (the `lui-lang` cookie, a year), for a
    /// language switch. A tag no table is registered for is ignored when read.
    pub fn lang(self, tag: &str) -> Self {
        let tag: String = tag
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
            .collect();
        let cookie = self
            .set_cookie(crate::cookie::SetCookie::new(LANG_COOKIE, &tag, 31_536_000))
            .to_string();
        self.cookie(cookie)
    }
}

static LANGUAGES: OnceLock<Mutex<&'static [&'static Strings]>> = OnceLock::new();

/// The tables an app offers, the first being the default (and the one whose
/// [`Strings::custom`] texts stand in for a key another table lacks). English only until
/// this is called; call it at start-up (a later call replaces the list).
pub fn languages(tables: &'static [&'static Strings]) {
    let list = LANGUAGES.get_or_init(|| Mutex::new(&[&Strings::ENGLISH]));
    *list.lock().unwrap_or_else(|e| e.into_inner()) = tables;
}

fn registered() -> &'static [&'static Strings] {
    LANGUAGES.get().map_or(&[&Strings::ENGLISH], |l| {
        *l.lock().unwrap_or_else(|e| e.into_inner())
    })
}

/// The app's default table: the first one given to [`languages`], English before that.
pub(crate) fn default_strings() -> &'static Strings {
    registered().first().copied().unwrap_or(&Strings::ENGLISH)
}

/// The cookie that picks a language over `Accept-Language`.
pub const LANG_COOKIE: &str = "lui-lang";

/// The table in `list` for `tag`: exact match, then by primary subtag.
fn find(list: &[&'static Strings], tag: &str) -> Option<&'static Strings> {
    let tag = tag.trim();
    let primary = |t: &str| t.split('-').next().unwrap_or("").to_ascii_lowercase();
    list.iter()
        .find(|s| s.lang.eq_ignore_ascii_case(tag))
        .or_else(|| list.iter().find(|s| primary(s.lang) == primary(tag)))
        .copied()
}

/// The table for a request: the `lui-lang` cookie, then `Accept-Language` (by `q`), then
/// the first registered table.
pub(crate) fn choose(cookie: Option<&str>, accept_language: &str) -> &'static Strings {
    choose_in(registered(), cookie, accept_language)
}

fn choose_in(
    list: &[&'static Strings],
    cookie: Option<&str>,
    accept_language: &str,
) -> &'static Strings {
    if let Some(s) = cookie.and_then(|c| find(list, c)) {
        return s;
    }
    let mut wanted: Vec<(&str, f32)> = accept_language
        .split(',')
        .filter_map(|part| {
            let mut bits = part.split(';');
            let tag = bits.next()?.trim();
            let q = bits
                .find_map(|b| b.trim().strip_prefix("q="))
                .and_then(|q| q.parse().ok())
                .unwrap_or(1.0);
            (!tag.is_empty() && tag != "*").then_some((tag, q))
        })
        .collect();
    wanted.sort_by(|a, b| b.1.total_cmp(&a.1));
    wanted
        .iter()
        .find_map(|(tag, _)| find(list, tag))
        .or_else(|| list.first().copied())
        .unwrap_or(&Strings::ENGLISH)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_are_kebab_case_and_the_table_is_complete() {
        for (i, t) in Text::ALL.iter().enumerate() {
            assert_eq!(*t as usize, i, "{t:?} is out of place in Text::ALL");
        }
        assert_eq!(Text::LoadMore.key(), "lui-load-more");
        assert_eq!(Text::Next.key(), "lui-next");
        assert_eq!(Strings::ENGLISH.get(Text::Sunday), "Sunday");
        assert_eq!(Strings::ENGLISH.get(Text::month(9)), "September");
        assert_eq!(Strings::ENGLISH.get(Text::weekday(3)), "Thursday");
        assert_eq!(Strings::ENGLISH.get(Text::Problem), "There is a problem");
    }

    #[test]
    fn accept_language_by_quality_then_primary_subtag() {
        static DE: Strings = Strings::new("de").with(Text::Next, "Weiter");
        static PT: Strings = Strings::new("pt-BR");
        let list: &[&Strings] = &[&Strings::ENGLISH, &DE, &PT];
        let lang = |cookie, header| choose_in(list, cookie, header).lang();
        assert_eq!(lang(None, "fr, de-AT;q=0.8, en;q=0.5"), "de");
        assert_eq!(lang(None, "en;q=0.4, pt-br"), "pt-BR");
        assert_eq!(lang(None, "fr"), "en");
        assert_eq!(lang(Some("de"), "pt-BR"), "de");
        assert_eq!(lang(Some("xx"), ""), "en");
    }

    #[test]
    fn plural_categories_follow_cldr() {
        use Plural::*;
        let cases: [(&str, &[(u64, Plural)]); 7] = [
            ("en", &[(0, Other), (1, One), (2, Other), (21, Other)]),
            ("es", &[(0, Other), (1, One), (2, Other), (1_000_000, Many)]),
            ("fr", &[(0, One), (1, One), (2, Other), (2_000_000, Many)]),
            ("de", &[(0, Other), (1, One), (5, Other)]),
            (
                "ar",
                &[
                    (0, Zero),
                    (1, One),
                    (2, Two),
                    (3, Few),
                    (110, Few),
                    (11, Many),
                    (199, Many),
                    (100, Other),
                    (102, Other),
                ],
            ),
            (
                "ru",
                &[
                    (1, One),
                    (21, One),
                    (11, Many),
                    (2, Few),
                    (24, Few),
                    (12, Many),
                    (5, Many),
                    (0, Many),
                    (111, Many),
                ],
            ),
            (
                "pl",
                &[
                    (1, One),
                    (21, Many),
                    (2, Few),
                    (22, Few),
                    (12, Many),
                    (5, Many),
                    (0, Many),
                ],
            ),
        ];
        for (lang, expected) in cases {
            let strings = Strings::new(lang);
            for (n, category) in expected {
                assert_eq!(strings.plural(*n), *category, "{lang} {n}");
            }
        }
    }

    #[test]
    fn counted_texts_pick_their_form() {
        static RU: Strings = Strings::new("ru").with(
            Text::Matches,
            "one:{} совпадение|few:{} совпадения|other:{} совпадений",
        );
        assert_eq!(RU.count(Text::Matches, 1), "1 совпадение");
        assert_eq!(RU.count(Text::Matches, 3), "3 совпадения");
        // No `many` form: `other` stands in.
        assert_eq!(RU.count(Text::Matches, 5), "5 совпадений");
        assert_eq!(Strings::ENGLISH.count(Text::Matches, 0), "0 matches");
        assert_eq!(Strings::ENGLISH.fill(Text::Matches, &[&7]), "7 matches");
        // A text without tags is every form, colons and all.
        assert_eq!(
            Strings::ENGLISH.count(Text::DismissMessage, 1),
            "Dismiss: 1"
        );
    }

    #[test]
    fn numbers_take_the_languages_separators() {
        let en = Strings::ENGLISH;
        assert_eq!(en.number(0.0), "0");
        assert_eq!(en.number(999.0), "999");
        assert_eq!(en.number(1000.0), "1,000");
        assert_eq!(en.number(-1234567.25), "-1,234,567.25");
        assert_eq!(en.number(0.5), "0.5");
        assert_eq!(en.number(f64::NAN), "NaN");
        assert_eq!(en.digits(i64::MAX).to_string(), "9,223,372,036,854,775,807");
        let es = Strings::new("es-MX");
        assert_eq!(es.number(1234.0), "1234");
        assert_eq!(es.number(12345.5), "12.345,5");
        assert_eq!(Strings::new("fr").number(1234.5), "1\u{202f}234,5");
        assert_eq!(Strings::new("pl").number(1234.0), "1234");
        let custom = Strings::new("xx").with_number("'", ".");
        assert_eq!(custom.number(1234567.0), "1'234'567");
    }

    #[test]
    fn tags_bring_direction_and_first_day() {
        assert_eq!(Strings::new("ar-EG").dir(), Dir::Rtl);
        assert_eq!(Strings::new("es").dir(), Dir::Ltr);
        assert_eq!(Strings::new("en").first_day(), 0);
        assert_eq!(Strings::new("en_us").first_day(), 6);
        assert_eq!(Strings::new("ar").first_day(), 5);
        assert_eq!(Strings::new("zz").first_day(), 0);
        assert_eq!(Strings::new("es").with_first_day(13).first_day(), 6);
        assert_eq!(Dir::parse(" RTL "), Some(Dir::Rtl));
        assert_eq!(Dir::parse("desc"), None);
        assert_eq!(Text::month(0), Text::January);
        assert_eq!(Text::weekday_short(6), Text::SundayShort);
    }

    #[test]
    fn lookup_tables_are_built_once_per_language() {
        let first =
            Strings::from_lookup("xx-lookup", |key| (key == "lui-next").then(|| "Nxt".into()));
        let again = Strings::from_lookup("xx-lookup", |_| Some("other".into()));
        assert!(std::ptr::eq(first, again));
        assert_eq!(again.get(Text::Next), "Nxt");
        assert_eq!(again.get(Text::Close), "Close");
        assert_eq!(again.lang(), "xx-lookup");
    }

    #[test]
    fn fill_leaves_unknown_placeholders() {
        assert_eq!(
            Strings::ENGLISH.fill(Text::MoveTo, &[&"Card"]),
            "Move Card to {}"
        );
    }
}
