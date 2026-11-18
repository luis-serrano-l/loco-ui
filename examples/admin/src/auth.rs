//! Signing in and out. A password is an Argon2id hash; a session is a random token in the
//! `sessions` table and in an `HttpOnly`, `SameSite=Lax` cookie, which is also what keeps
//! another site from posting these forms. [`Admin`] is the extractor every signed-in
//! handler takes: without a session it answers with a redirect to `/signin`.

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use axum::{
    Router,
    extract::{FromRequestParts, State},
    http::{header, request::Parts},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use loco_ui::prelude::*;

use crate::{Db, Res, db};

const COOKIE: &str = "admin-session";
/// What the stand-in hash for unknown addresses is made from; nobody's password.
const SEED_UNUSED: &str = "no account has this password";

/// The Argon2id hash of `password`, with a fresh salt. Empty when hashing fails, which no
/// password verifies against.
pub fn hash(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .unwrap_or_default()
}

/// Whether `password` is the one `hash` was made from.
pub fn verify(hash: &str, password: &str) -> bool {
    PasswordHash::new(hash).is_ok_and(|h| {
        Argon2::default()
            .verify_password(password.as_bytes(), &h)
            .is_ok()
    })
}

/// The signed-in user.
#[derive(Clone, Debug)]
pub struct Admin {
    pub id: i64,
    pub name: String,
    /// The session's token, kept when a password change ends the other sessions.
    pub token: String,
}

fn token(parts: &Parts) -> Option<String> {
    let cookies = parts
        .headers
        .get_all(header::COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok());
    cookies.flat_map(|v| v.split(';')).find_map(|pair| {
        pair.trim()
            .strip_prefix(COOKIE)?
            .strip_prefix('=')
            .map(str::to_string)
    })
}

impl FromRequestParts<Db> for Admin {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, db: &Db) -> Result<Admin, Response> {
        let to_signin = || axum::response::Redirect::to("/signin").into_response();
        let token = token(parts).ok_or_else(to_signin)?;
        let found = db::session_user(db, &token)
            .await
            .map_err(|e| crate::Failure::from(e).into_response())?;
        let (id, name) = found.ok_or_else(to_signin)?;
        Ok(Admin { id, name, token })
    }
}

pub fn routes() -> Router<Db> {
    Router::new()
        .route("/signin", get(signin_page).post(signin))
        .route("/signout", post(signout))
}

fn page(ui: &Ui, values: &[(String, String)], errors: &[(&str, &str)]) -> Page {
    let form = ui
        .form("/signin")
        .email("email", "Email")
        .required()
        .autocomplete("username")
        .password("password", "Password")
        .required()
        .autocomplete("current-password")
        .values(values)
        .errors(errors)
        .submit("Sign in");
    let hint = html! { "The seeded account is " code { (db::SEED_EMAIL) } " with the password " code { (db::SEED_PASSWORD) } "." };
    ui.page(
        "Sign in",
        ui.auth_page("Sign in")
            .description("The admin panel.")
            .body(form.render())
            .footer(hint)
            .render(),
    )
}

async fn signin_page(ui: Ui) -> Page {
    page(&ui, &[], &[])
}

async fn signin(ui: Ui, State(db): State<Db>, posted: Posted) -> Res<Result<Redirect, Page>> {
    let email = posted.get("email").trim().to_lowercase();
    let found = db::credentials(&db, &email).await?;
    // The same message whether the address is unknown, the password wrong or the account
    // suspended, so the form does not say which addresses have accounts.
    // The password is checked even when no active account has the address (against a hash
    // nothing matches), so the time the answer takes does not say which addresses do.
    static NOBODY: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    let account = found.filter(|(_, _, active)| *active);
    let hash = account.as_ref().map_or_else(
        || NOBODY.get_or_init(|| hash(SEED_UNUSED)).as_str(),
        |(_, hash, _)| hash.as_str(),
    );
    let right = verify(hash, posted.get("password"));
    let Some((id, _, _)) = account.filter(|_| right) else {
        db::log(&db, &email, "failed sign-in", "").await?;
        let values = [("email".to_string(), email)];
        return Ok(Err(page(
            &ui,
            &values,
            &[(
                "password",
                "That email and password do not match an active account.",
            )],
        )
        .invalid()));
    };
    let token = db::start_session(&db, id).await?;
    db::log(&db, &email, "signed in", "").await?;
    let cookie = format!("{COOKIE}={token}; Path=/; HttpOnly; SameSite=Lax; Max-Age=604800");
    Ok(Ok(ui.redirect("/users").cookie(cookie)))
}

async fn signout(ui: Ui, admin: Admin, State(db): State<Db>) -> Res<Redirect> {
    db::end_session(&db, &admin.token).await?;
    db::log(&db, &admin.name, "signed out", "").await?;
    Ok(ui
        .redirect("/signin")
        .cookie(format!(
            "{COOKIE}=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0"
        ))
        .flash("Signed out."))
}
