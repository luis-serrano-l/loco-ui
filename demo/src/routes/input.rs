//! Input: combobox, validated form, wizard, select, range and colour, and the M40c inputs:
//! checkbox group, number, search, rating and tag input.

use crate::site::page;
use axum::{
    Form, Router,
    extract::{Query, rejection::QueryRejection},
    routing::{get, post},
};
use loco_ui::prelude::*;
use loco_ui::wizard::Wizard;
use serde::{Deserialize, Serialize};

pub(crate) fn routes() -> Router {
    super::pages(PAGES)
        .route("/combobox/new", post(combobox_new))
        .route("/form", get(form_page).post(form_submit))
        .route("/wizard", get(wizard_page).post(wizard_submit))
        .route("/inputs", get(inputs_page).post(inputs_submit))
}

/// The pages that are their component and a note (`super::pages`).
pub(crate) const PAGES: &[super::Simple] = &[
    (
        "/combobox",
        combobox,
        "Pick several: each result adds a chip, each chip's \u{d7} removes it, and the chips ride along with the next search. Type a language that is not here to get a Create row.",
    ),
    ("/toggle-group", toggle_group, ""),
    ("/otp", otp, ""),
    (
        "/checkbox-group",
        checkbox_group,
        "HTML has no `required` for a group of checkboxes: the server checks that one is ticked.",
    ),
    (
        "/number",
        number,
        "Without the script a stepper sends the form with the stepped value first, and the page answers with it; with the script it steps in place. A stepper is a submit button, and Enter presses a form's first one, so the stepped quantity has a form of its own.",
    ),
    (
        "/search",
        search,
        "The query is in the URL, so the answer can be bookmarked. Inside a swap root the enhancement script searches as you type.",
    ),
    ("/rating", rating, ""),
    (
        "/tag-input",
        tag_input,
        "Without the script Add sends the form back with GET and the tag becomes a chip; each \u{d7} is a link to the page without it.",
    ),
];

/// The other pages' live components, which the index shows too (`site::preview`), empty
/// there.
pub(crate) const PREVIEWS: &[super::Preview] = &[
    ("/form", |ui| signup_form(ui, &[], &[])),
    ("/wizard", |ui| signup(ui, &Signup::default(), &[]).render()),
    ("/inputs", |ui| inputs(ui, &Inputs::default())),
];

fn combobox(ui: &Ui) -> Markup {
    lui! {
            // One swap root around the form and its results: the script searches as you type.
            div id="langs" data-lui="swap" {
                // code: /combobox
                Combobox("q", "/combobox") multiple create="/combobox/new"
                    label="Language" placeholder="Type a language" {
                    group "Systems" (["Rust", "Zig", "Swift"]);
                    group "Scripting" (["Ruby", "Python", "Racket"]);
                    options(["Prolog", "Scala"]);
                }
                // end code
            }
    }
}

#[derive(Deserialize)]
struct NewLang {
    name: String,
    #[serde(default)]
    sel: Vec<String>,
}

async fn combobox_new(ui: Ui, Form(form): Form<NewLang>) -> Redirect {
    let name = form.name.trim();
    let to: String = form
        .sel
        .iter()
        .map(String::as_str)
        .chain([name])
        .map(|s| format!("&sel={s}"))
        .collect();
    ui.redirect(&format!("/combobox?q={to}")).flash(&format!(
        "Added {name} (not really: the demo has no database)."
    ))
}

/// What the wizard has collected so far, as the steps posted it.
#[derive(Default, Deserialize, Serialize)]
struct Signup(Vec<(String, String)>);

impl Signup {
    fn get(&self, name: &str) -> &str {
        self.0
            .iter()
            .find(|(n, _)| n == name)
            .map_or("", |(_, v)| v.trim())
    }
}

fn signup<'a>(ui: &'a Ui, answers: &'a Signup, errors: &'a [(&'a str, &'a str)]) -> Wizard<'a> {
    // code: /wizard
    ui.wizard("signup", "/wizard")
        .step(
            "Account",
            ui.fields()
                .text("name", "Name")
                .required()
                .email("email", "Email")
                .required(),
        )
        .step(
            "Newsletter",
            ui.fields()
                .select("digest", "Digest", ["daily", "weekly", "never"])
                .text("topics", "Topics")
                .placeholder("rust, html"),
        )
        .optional()
        .review("Review")
        .values(&answers.0)
        .errors(errors)
        .finish("Create account")
    // end code
}

/// Server rules for a wizard step: `(field, message)` per problem.
fn signup_errors(step: usize, answers: &Signup) -> Vec<(&'static str, &'static str)> {
    let (name, email) = (answers.get("name"), answers.get("email"));
    match step {
        0 if name.is_empty() => vec![("name", "Enter your name.")],
        0 if !email.contains('@') => vec![("email", "Enter an email address with an @.")],
        0 if email.ends_with("@example.com") => {
            vec![("email", "example.com addresses are not accepted.")]
        }
        _ => Vec::new(),
    }
}

fn wizard_view(ui: &Ui, wizard: Wizard) -> Page {
    let body = html! {
        p { "Three steps, one form each. The server checks every step; the second can be skipped. Close the tab and come back to " a href="/wizard" { "/wizard" } ": you resume where you left off." }
        (wizard)
    };
    page(ui, "Wizard", body)
}

async fn wizard_page(ui: Ui, Saved(answers): Saved<Signup>) -> Page {
    wizard_view(&ui, signup(&ui, &answers, &[]))
}

/// Check the posted step: answer 422 with the same step and its messages, or keep the fields
/// (a year, so closing the browser loses nothing) and redirect to the next step.
async fn wizard_submit(
    ui: Ui,
    Saved(mut answers): Saved<Signup>,
    posted: Posted,
) -> Result<Redirect, Page> {
    let (step, skip) = (posted.step(), posted.skip());
    for (k, v) in posted
        .pairs()
        .iter()
        .filter(|(k, _)| !skip && k != "step" && k != "skip")
    {
        answers.0.retain(|(n, _)| n != k);
        answers.0.push((k.clone(), v.clone()));
    }
    let errors = if skip {
        Vec::new()
    } else {
        signup_errors(step, &answers)
    };
    let wizard = signup(&ui, &answers, &errors).at(step);
    if !errors.is_empty() {
        return Err(wizard_view(&ui, wizard).invalid());
    }
    if wizard.is_last(step) {
        let done = "Account created (well, the cookie was cleared).";
        return Ok(ui.redirect(&wizard.link(0)).flash(done).forget::<Signup>());
    }
    Ok(ui.redirect(&wizard.link(step + 1)).save(&answers))
}

/// The sign-up form, with the values and messages of a post the server refused.
fn signup_form(ui: &Ui, values: &[(String, String)], errors: &[(&str, &str)]) -> Markup {
    let inline = ui.param("layout") == Some("inline");
    // code: /form
    let form = lui! {
        Form("/form") submit="Sign up" action_secondary=("Cancel", "/") values=(values) errors=(errors) inline[inline] {
            group "Account" description="How you sign in and what you pay.";
            text "name" "Name" required;
            email "email" "Email" required;
            number "age" "Age" 13 120 required;
            pattern "handle" "Handle" "[a-z0-9_]{3,16}" "3–16 lowercase letters, digits or _" required;
            select "plan" "Plan" ([("free", "Free"), ("team", "Team"), ("enterprise", "Enterprise")]);
            group "Profile" description="What others see on your page.";
            textarea "bio" "Bio" 3 maxlength=160 help="Grows as you type where the browser supports it.";
            file "avatar" "Avatar" "image/png,image/jpeg" help="PNG or JPEG.";
            date "start" "Start date" "2026-01-01" "2027-12-31";
            time "call" "Best time to call" "09:00" "17:00" help="Office hours, 09:00 to 17:00.";
            datetime "demo" "Demo" "2026-01-01T09:00" "2027-12-31T17:00" help="A date and a time in one field.";
            group "Contact" description="How and about what we write to you.";
            radio "reach" "Contact me by" ([("email", "Email"), ("phone", "Phone"), ("never", "Never")]);
            checkboxes "news" "News about" ([("releases", "Releases"), ("events", "Events"), ("tips", "Tips")]);
            tel "mobile" "Mobile" optional autocomplete="tel" width=2;
            url "homepage" "Homepage" optional placeholder="https://";
            range "alerts" "Alert volume" 0 10;
            color "profile_color" "Profile colour";
        }

    };
    // end code
    form
}

fn form_view(ui: &Ui, values: &[(String, String)], errors: &[(&str, &str)]) -> Page {
    let inline = ui.param("layout") == Some("inline");
    let body = html! {
        p { "Labels " @if inline { "beside the fields. " a href="/form" { "Put them above" } } @else { "above the fields. " a href="/form?layout=inline" { "Put them beside" } } "." }
        p class="lui-note" { "The handle " code { "admin" } " and " code { "@example.com" } " addresses pass the browser's checks and fail the server's: the form comes back with an error summary on top that takes the focus and links to each field. " a href="/form?errors=1" { "See it" } "." }
        (signup_form(ui, values, errors))
    };
    page(ui, "Validated form", body)
}

/// `?errors=1` shows the answer to a post the server refused, without posting.
async fn form_page(ui: Ui) -> Page {
    if ui.param("errors").is_none() {
        return form_view(&ui, &[], &[]);
    }
    let values = [
        ("name", "Ada"),
        ("email", "ada@example.com"),
        ("handle", "admin"),
    ];
    let values: Vec<(String, String)> = values.map(|(n, v)| (n.into(), v.into())).into();
    let errors = [
        ("email", "example.com addresses are not accepted."),
        ("handle", "That handle is reserved."),
    ];
    form_view(&ui, &values, &errors)
}

/// A multipart post (the avatar is a file): server rules, then PRG with a flash or the form
/// again with 422.
async fn form_submit(ui: Ui, posted: Posted) -> Result<Redirect, Page> {
    let mut errors = Vec::new();
    if posted.get("handle") == "admin" {
        errors.push(("handle", "That handle is reserved."));
    }
    if posted.get("email").ends_with("@example.com") {
        errors.push(("email", "example.com addresses are not accepted."));
    }
    if !errors.is_empty() {
        return Err(form_view(&ui, posted.pairs(), &errors).invalid());
    }
    let files: String = posted
        .files()
        .iter()
        .map(|f| format!(" with {} ({} bytes)", f.file_name, f.bytes.len()))
        .collect();
    let handle = posted.get("handle");
    Ok(ui
        .redirect("/form")
        .flash(&format!("Signed up as {handle}{files}.")))
}

/// The inputs page's values: from the query while filtering (unsaved), else saved; a missing
/// one is its default.
#[derive(Deserialize, Serialize)]
#[serde(default)]
struct Inputs {
    size: String,
    volume: f64,
    accent: String,
    #[serde(rename = "accent-alpha")]
    alpha: u8,
    #[serde(rename = "accent-preset")]
    preset: Option<String>,
    price_min: f64,
    price_max: f64,
    country: String,
}

impl Default for Inputs {
    fn default() -> Self {
        Inputs {
            size: "m".into(),
            volume: 40.0,
            accent: ACCENTS[0].into(),
            alpha: 100,
            preset: None,
            price_min: 20.0,
            price_max: 80.0,
            country: "es".into(),
        }
    }
}

const SIZES: [(&str, &str, &str); 3] = [
    ("s", "Small", "🐭"),
    ("m", "Medium", "🐕"),
    ("l", "Large", "🐘"),
];
const ACCENTS: [&str; 5] = ["#1f6f5f", "#2f5bea", "#b3261e", "#8a5a00", "#6b3fa0"];
/// `(value, name, flag)`.
type Country = (&'static str, &'static str, &'static str);
const COUNTRIES: [(&str, [Country; 7]); 3] = [
    (
        "Europe",
        [
            ("es", "Spain", "🇪🇸"),
            ("fr", "France", "🇫🇷"),
            ("de", "Germany", "🇩🇪"),
            ("it", "Italy", "🇮🇹"),
            ("pt", "Portugal", "🇵🇹"),
            ("nl", "Netherlands", "🇳🇱"),
            ("se", "Sweden", "🇸🇪"),
        ],
    ),
    (
        "Americas",
        [
            ("us", "United States", "🇺🇸"),
            ("ca", "Canada", "🇨🇦"),
            ("mx", "Mexico", "🇲🇽"),
            ("br", "Brazil", "🇧🇷"),
            ("ar", "Argentina", "🇦🇷"),
            ("cl", "Chile", "🇨🇱"),
            ("co", "Colombia", "🇨🇴"),
        ],
    ),
    (
        "Asia",
        [
            ("jp", "Japan", "🇯🇵"),
            ("kr", "South Korea", "🇰🇷"),
            ("in", "India", "🇮🇳"),
            ("id", "Indonesia", "🇮🇩"),
            ("vn", "Vietnam", "🇻🇳"),
            ("th", "Thailand", "🇹🇭"),
            ("ph", "Philippines", "🇵🇭"),
        ],
    ),
];

/// Select, range and colour in one form, saved in `lui-inputs`. The country filter is a GET
/// through the same form, so while filtering the values come from the query.
/// A query that does not parse (`?volume=abc`) shows the saved values instead of a 400.
async fn inputs_page(
    ui: Ui,
    query: Result<Query<Inputs>, QueryRejection>,
    Saved(saved): Saved<Inputs>,
) -> Page {
    let values = match query {
        Ok(Query(query)) if ui.param("country-q").is_some() => query,
        _ => saved,
    };
    let body = lui! { Stack {
        (inputs(&ui, &values))
        p class="lui-note" { "Without the enhancement script the outputs and the swatch show the last saved values and update on submit, and the country filter needs its button." }
    } };
    page(&ui, "Select, range, colour", body)
}

/// The form of select, range and colour, showing `values`.
fn inputs(ui: &Ui, values: &Inputs) -> Markup {
    lui! {
        // code: /inputs
        Form("/inputs") submit="Save" {
            Select("size", "Size") value=(&values.size) options=(SIZES);
            Select("country", "Country") value=(&values.country) groups=(COUNTRIES) search="/inputs";
            Range("volume", "Volume") value=(values.volume) step=5 ticks;
            RangePair("price", "Price") values=(values.price_min, values.price_max) step=5 ticks;
            Color("accent", "Accent") value=(&values.accent) presets=(&ACCENTS) alpha=(values.alpha);
        }
        // end code
    }
}

/// Only known sizes, countries and `#rrggbb` colours are kept; numbers are clamped.
async fn inputs_submit(ui: Ui, Form(form): Form<Inputs>) -> Redirect {
    let defaults = Inputs::default();
    let hex = |c: &String| c.len() == 7 && c.starts_with('#');
    let size = SIZES.iter().any(|(v, ..)| *v == form.size);
    let country = COUNTRIES
        .iter()
        .flat_map(|(_, cs)| cs)
        .any(|(v, ..)| *v == form.country);
    let accent = form.preset.filter(hex).or(Some(form.accent).filter(hex));
    let (lo, hi) = loco_ui::range::order(
        form.price_min.clamp(0.0, 100.0),
        form.price_max.clamp(0.0, 100.0),
    );
    let clean = Inputs {
        size: if size { form.size } else { defaults.size },
        country: if country {
            form.country
        } else {
            defaults.country
        },
        accent: accent.unwrap_or(defaults.accent),
        alpha: form.alpha.min(100),
        volume: form.volume.clamp(0.0, 100.0),
        price_min: lo,
        price_max: hi,
        preset: None,
    };
    ui.redirect("/inputs").flash("Inputs saved.").save(&clean)
}

/// One pick (alignment) and several (style), sent with the form they sit in, which says what
/// it was sent with.
fn toggle_group(ui: &Ui) -> Markup {
    let picked = |name| ui.params(name).collect::<Vec<_>>().join(", ");
    lui! {
        Form("/toggle-group") get submit="Apply" {
            // code: /toggle-group
            ToggleGroup("align", "Alignment") { option "left" "Left"; option "center" "Center"; option "right" "Right"; }
            ToggleGroup("style", "Text style") multiple {
                option "bold" "Bold" icon=(Icon::Bold); option "italic" "Italic" icon=(Icon::Italic);
            }
            // end code
            @if ui.param("align").is_some() { p class="lui-note" { "Alignment: " (picked("align")) ". Style: " (picked("style")) "." } }
        }
    }
}

/// Topics to follow, ticked in a GET form that says what it was sent with.
fn checkbox_group(ui: &Ui) -> Markup {
    let picked = ui.params("topics[]").collect::<Vec<_>>().join(", ");
    lui! {
        Form("/checkbox-group") get submit="Follow" {
            // code: /checkbox-group
            CheckboxGroup("topics", "Topics") help="Pick any." {
                option "rust" "Rust" description="Releases and RFCs.";
                option "css" "CSS";
                option "html" "HTML";
                option "flash" "Flash" disabled;
            }
            CheckboxGroup("days", "Delivery days") cards {
                option "mon" "Monday"; option "wed" "Wednesday"; option "fri" "Friday";
            }
            // end code
            @if !picked.is_empty() { p class="lui-note" { "Following " (picked) "." } }
        }
    }
}

/// A cart line: a quantity with steppers, a price and a weight with their units.
fn number(ui: &Ui) -> Markup {
    lui! {
        // code: /number
        Form("/number") id="cart-price" get submit="Update" {
            Number("price", "Price") prefix="$" step=0.5 min=0 placeholder="0";
            Number("parcel", "Parcel weight") suffix="kg" step=0.1 min=0 help="Up to one decimal.";
        }
        Form("/number") id="cart-qty" get submit="Update" {
            Number("qty", "Quantity") min=1 max=10 steppers required;
        }
        // end code
    }
}

/// Notes searched through a GET form in a swap root, the results named by `results_id`.
fn search(ui: &Ui) -> Markup {
    const NOTES: [&str; 6] = [
        "Rust ownership, briefly",
        "CSS container queries",
        "Shipping without JavaScript",
        "Rust error handling",
        "Forms that post",
        "HTML dialogs",
    ];
    let q = ui.param("find").unwrap_or("").trim().to_lowercase();
    let found: Vec<&str> = NOTES
        .into_iter()
        .filter(|n| n.to_lowercase().contains(&q))
        .collect();
    lui! {
        div id="notes-search" data-lui="swap" class="lui-stack" {
            // code: /search
            Search("find", "Search notes") results_id="notes" placeholder="Search notes";
            // end code
            ul id="notes" aria-live="polite" { @for n in &found { li { (n) } } }
            @if found.is_empty() { p class="lui-note" { "No notes match." } }
        }
    }
}

/// A review form: a score to pick, and the average shown and posted unchanged.
fn rating(ui: &Ui) -> Markup {
    lui! {
        Form("/rating") get submit="Rate" {
            // code: /rating
            Rating("stars", "Your rating") required;
            Rating("average", "Average so far") value=4 readonly;
            // end code
            @if let Some(s) = ui.param("stars") { p class="lui-note" { "Rated " (s) " out of 5." } }
        }
    }
}

/// A note's tags: chips that travel in the URL while editing.
fn tag_input(ui: &Ui) -> Markup {
    let tags = ui
        .params("tags[]")
        .filter(|t| !t.trim().is_empty())
        .collect::<Vec<_>>()
        .join(", ");
    lui! {
        Form("/tag-input") get submit="Save" {
            // code: /tag-input
            TagInput("tags", "Tags") placeholder="Add a tag" help="Press Add, or Save with the tag typed.";
            // end code
            @if !tags.is_empty() { p class="lui-note" { "Tags: " (tags) "." } }
        }
    }
}

/// A one-time code: one field the phone offers to fill from the message.
fn otp(ui: &Ui) -> Markup {
    lui! {
        Form("/otp") get submit="Verify" {
            // code: /otp
            InputOtp("code", "Code from the text message") group=3;
            // end code
            @if let Some(code) = ui.param("code") { p class="lui-note" { "Sent " code { (code) } "." } }
        }
    }
}
