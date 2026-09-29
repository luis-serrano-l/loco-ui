//! The account pages' bodies, written in `lui!`: each a centred card (`AuthPage`).
//! Written by `cargo lui auth`; edit freely.
use loco_ui::prelude::*;

pub fn signin(ui: &Ui, values: &[(String, String)], errors: &[(&str, &str)]) -> Markup {
    // The form posts to its own URL, so a `?next=` survives a wrong password.
    let action = ui
        .param("next")
        .map_or("/signin".into(), |next| ui.link_with("next", next));
    lui! {
        Flash;
        AuthPage("Sign in") description="Welcome back."
            footer={ "No account? " a href="/signup" { "Sign up" } } {
            Form(&action) values=(values) errors=(errors) submit="Sign in" {
                email "email" "Email" required;
                password "password" "Password" required;
            }
            p { a href="/forgot" { "Forgot your password?" } " · " a href="/magic-link" { "Email me a sign-in link" } }
        }
    }
}

pub fn signup(ui: &Ui, values: &[(String, String)], errors: &[(&str, &str)]) -> Markup {
    lui! {
        AuthPage("Sign up") description="We will email you a link to confirm the address."
            footer={ "Have an account? " a href="/signin" { "Sign in" } } {
            Form("/signup") values=(values) errors=(errors) submit="Sign up" {
                text "name" "Name" required;
                email "email" "Email" required;
                password "password" "Password" required;
            }
        }
    }
}

pub fn forgot(ui: &Ui, values: &[(String, String)], errors: &[(&str, &str)]) -> Markup {
    lui! {
        AuthPage("Forgot password") description="We will email you a link to choose a new password."
            footer={ a href="/signin" { "Back to sign in" } } {
            Form("/forgot") values=(values) errors=(errors) submit="Email me a link" {
                email "email" "Email" required;
            }
        }
    }
}

/// The new-password form; `action` is the link's own `/reset/<token>`.
pub fn reset(ui: &Ui, action: &str, errors: &[(&str, &str)]) -> Markup {
    lui! {
        AuthPage("Choose a new password") {
            Form(action) errors=(errors) submit="Change password" {
                password "password" "New password" required;
            }
        }
    }
}

pub fn magic(ui: &Ui, values: &[(String, String)], errors: &[(&str, &str)]) -> Markup {
    lui! {
        AuthPage("Email me a link")
            description="We will email you a link that signs you in once, no password needed."
            footer={ a href="/signin" { "Sign in with a password" } } {
            Form("/magic-link") values=(values) errors=(errors) submit="Email me a link" {
                email "email" "Email" required;
            }
        }
    }
}

/// A spent or unknown link; `again` is the page that sends a new one.
pub fn expired(ui: &Ui, again: &str) -> Markup {
    lui! {
        AuthPage("Link expired") description="That link was used already, is too old, or was mistyped." {
            LinkButton("Send a new link", again) primary;
        }
    }
}
