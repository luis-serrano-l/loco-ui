//! The account pages' bodies, built from `ui.*`: each a centred card (`ui.auth_page`).
//! Written by `cargo lui auth`; edit freely.
use loco_ui::prelude::*;

pub fn signin(ui: &Ui, values: &[(String, String)], errors: &[(&str, &str)]) -> Markup {
    // The form posts to its own URL, so a `?next=` survives a wrong password.
    let action = ui
        .param("next")
        .map_or("/signin".into(), |next| ui.link_with("next", next));
    let form = ui
        .form(&action)
        .email("email", "Email")
        .required()
        .password("password", "Password")
        .required()
        .values(values)
        .errors(errors)
        .submit("Sign in");
    html! {
        (ui.flash())
        (ui.auth_page("Sign in")
            .description("Welcome back.")
            .body(html! {
                (form)
                p { a href="/forgot" { "Forgot your password?" } " · " a href="/magic-link" { "Email me a sign-in link" } }
            })
            .footer(html! { "No account? " a href="/signup" { "Sign up" } }))
    }
}

pub fn signup(ui: &Ui, values: &[(String, String)], errors: &[(&str, &str)]) -> Markup {
    let form = ui
        .form("/signup")
        .text("name", "Name")
        .required()
        .email("email", "Email")
        .required()
        .password("password", "Password")
        .required()
        .values(values)
        .errors(errors)
        .submit("Sign up");
    html! {
        (ui.auth_page("Sign up")
            .description("We will email you a link to confirm the address.")
            .body(form.render())
            .footer(html! { "Have an account? " a href="/signin" { "Sign in" } }))
    }
}

pub fn forgot(ui: &Ui, values: &[(String, String)], errors: &[(&str, &str)]) -> Markup {
    let form = ui
        .form("/forgot")
        .email("email", "Email")
        .required()
        .values(values)
        .errors(errors)
        .submit("Email me a link");
    html! {
        (ui.auth_page("Forgot password")
            .description("We will email you a link to choose a new password.")
            .body(form.render())
            .footer(html! { a href="/signin" { "Back to sign in" } }))
    }
}

/// The new-password form; `action` is the link's own `/reset/<token>`.
pub fn reset(ui: &Ui, action: &str, errors: &[(&str, &str)]) -> Markup {
    let form = ui
        .form(action)
        .password("password", "New password")
        .required()
        .errors(errors)
        .submit("Change password");
    html! {
        (ui.auth_page("Choose a new password").body(form.render()))
    }
}

pub fn magic(ui: &Ui, values: &[(String, String)], errors: &[(&str, &str)]) -> Markup {
    let form = ui
        .form("/magic-link")
        .email("email", "Email")
        .required()
        .values(values)
        .errors(errors)
        .submit("Email me a link");
    html! {
        (ui.auth_page("Email me a link")
            .description("We will email you a link that signs you in once, no password needed.")
            .body(form.render())
            .footer(html! { a href="/signin" { "Sign in with a password" } }))
    }
}

/// A spent or unknown link; `again` is the page that sends a new one.
pub fn expired(ui: &Ui, again: &str) -> Markup {
    html! {
        (ui.auth_page("Link expired")
            .description("That link was used already, is too old, or was mistyped.")
            .body(ui.link_button("Send a new link", again).primary().render()))
    }
}
