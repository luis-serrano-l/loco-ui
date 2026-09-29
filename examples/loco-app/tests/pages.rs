//! Every page of the app as a browser without script sees it: the account pages (sign up,
//! verify, sign in, forgot and reset password, magic link), then notebooks, notes (tags, the
//! tag filter, pin, archive) and the task board, through the router Loco boots (its
//! middleware, `auth::JWT`, the loco-ui initializer, the app's look). Each GET page ships
//! only the enhancement script, and Blitz (no script engine) renders it into
//! `tests/shots/loco-*.png`.

use axum::{
    Router,
    body::Body,
    http::{Request, Response, StatusCode, header},
};
use loco_app::{
    app::App,
    models::{
        _entities::{notebooks, notes, users as user_rows},
        users,
    },
};
use loco_rs::testing::prelude::*;
use loco_ui_test::Page;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, sea_query::Expr};
use serial_test::serial;
use tower::ServiceExt;

const SHOTS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/shots");

async fn send(
    router: &Router,
    method: &str,
    path: &str,
    cookie: &str,
    form: &str,
) -> Response<Body> {
    let req = Request::builder()
        .method(method)
        .uri(path)
        .header(header::COOKIE, cookie)
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(Body::from(form.to_string()))
        .unwrap();
    router.clone().oneshot(req).await.unwrap()
}

async fn text(res: Response<Body>) -> String {
    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
        .await
        .unwrap();
    String::from_utf8(bytes.to_vec()).unwrap()
}

fn location(res: &Response<Body>) -> &str {
    res.headers()[header::LOCATION].to_str().unwrap()
}

/// The `auth=…` pair from a response's `Set-Cookie` headers.
fn auth_cookie(res: &Response<Body>) -> String {
    res.headers()
        .get_all(header::SET_COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .find(|v| v.starts_with("auth="))
        .and_then(|v| v.split(';').next())
        .expect("sign-in sets the auth cookie")
        .to_string()
}

/// Ada's row, for the tokens her mails would carry.
async fn ada(db: &sea_orm::DatabaseConnection) -> users::Model {
    users::Model::find_by_email(db, "ada@example.com")
        .await
        .unwrap()
}

/// GET `path` signed in with `auth`: it answers 200 in the app's look with only the
/// enhancement script (`tag`), and Blitz lays it out into `tests/shots/loco-<name>.png`.
async fn shoot(router: &Router, auth: &str, tag: &str, path: &str, name: &str) {
    let res = send(router, "GET", path, auth, "").await;
    assert_eq!(res.status(), StatusCode::OK, "{path}");
    let html = text(res).await;
    if path == "/notes/1/edit" {
        // The edit form starts from the saved note, every field filled in.
        for filled in [
            r#"value="First""#,
            ">Hello\n\nAgain</textarea>",
            r#"value="2026-10-01""#,
            r#"value="ideas, rust""#,
            r#"<option value="1" selected>"#,
            "checked",
        ] {
            assert!(html.contains(filled), "edit form: no {filled} in {html}");
        }
    }
    assert_eq!(html.matches("<script").count(), 1, "{path}: one script");
    assert!(
        html.contains("--lui-brand-9: #0f766e"),
        "{path}: in the app's look"
    );
    assert!(
        html.contains(tag),
        "{path}: and it is the enhancement script"
    );

    let mut page = Page::render(router.clone(), path, auth).await;
    assert!(page.is_visible("h1"), "{path}: Blitz lays the page out");
    page.screenshot(format!("{SHOTS}/loco-{name}.png")).unwrap();
}

#[tokio::test]
#[serial]
async fn every_page_works_without_script() {
    let boot = boot_test::<App>().await.unwrap();
    let db = boot.app_context.db.clone();
    let router = boot.router.unwrap();
    let tag = loco_ui::enhance::script_tag().into_string();

    // Signed out, a signed-in page (Loco's 401) goes to the sign-in form, not a JSON error.
    // `?next=` names the page, and the form posts back to the same URL.
    let res = send(&router, "GET", "/notes/1?tab=edit", "", "").await;
    assert_eq!(res.status(), StatusCode::SEE_OTHER);
    let signin = location(&res).to_string();
    assert_eq!(signin, "/signin?next=%2Fnotes%2F1%3Ftab%3Dedit");
    let html = text(send(&router, "GET", &signin, "", "").await).await;
    assert!(
        html.contains(r#"action="/signin?next=%2Fnotes%2F1%3Ftab%3Dedit""#),
        "{html}"
    );

    // A blank sign-up comes back with a message on each field, not a JSON error.
    let res = send(&router, "POST", "/signup", "", "name=&email=&password=").await;
    assert_eq!(res.status(), StatusCode::OK);
    assert!(text(res).await.contains("This field is required."));

    let res = send(
        &router,
        "POST",
        "/signup",
        "",
        "name=Ada&email=ada%40example.com&password=secret",
    )
    .await;
    assert_eq!(res.status(), StatusCode::SEE_OTHER);
    assert_eq!(location(&res), "/");
    let auth = auth_cookie(&res);

    // The welcome mail's link verifies the address once; a made-up one is refused.
    let token = ada(&db)
        .await
        .email_verification_token
        .expect("sign-up mails a link");
    let res = send(&router, "GET", &format!("/verify/{token}"), "", "").await;
    assert_eq!(location(&res), "/");
    assert!(ada(&db).await.email_verified_at.is_some());
    let res = send(&router, "GET", "/verify/made-up", "", "").await;
    assert_eq!(location(&res), "/signin");

    // A wrong password is a message on the form; the right one signs in.
    let res = send(
        &router,
        "POST",
        "/signin",
        "",
        "email=ada%40example.com&password=nope",
    )
    .await;
    let body = text(res).await;
    assert!(body.contains("<li>Wrong email or password.</li>"), "{body}");
    assert!(
        !body.contains(r#"aria-invalid="true""#),
        "no field is marked wrong"
    );
    let res = send(
        &router,
        "POST",
        "/signin",
        "",
        "email=ada%40example.com&password=secret",
    )
    .await;
    assert_eq!(location(&res), "/");
    // Sent to sign in from a page, signing in goes back to it; a `next` off this site is
    // ignored.
    let ada_signin = "email=ada%40example.com&password=secret";
    let res = send(
        &router,
        "POST",
        "/signin?next=%2Fnotes%3Ftab%3D1",
        "",
        ada_signin,
    )
    .await;
    assert_eq!(location(&res), "/notes?tab=1");
    for elsewhere in [
        "https%3A%2F%2Fevil.example",
        "%2F%2Fevil.example",
        "%2F%5Cevil.example",
    ] {
        let res = send(
            &router,
            "POST",
            &format!("/signin?next={elsewhere}"),
            "",
            ada_signin,
        )
        .await;
        assert_eq!(location(&res), "/", "{elsewhere}");
    }

    // Forgot password: the same answer for any email; the mailed link sets a new password
    // once, and the old one stops working.
    let res = send(&router, "POST", "/forgot", "", "email=nobody%40example.com").await;
    assert_eq!(location(&res), "/signin");
    let res = send(&router, "POST", "/forgot", "", "email=ada%40example.com").await;
    assert_eq!(location(&res), "/signin");
    let reset = format!(
        "/reset/{}",
        ada(&db).await.reset_token.expect("a reset link")
    );
    let res = send(&router, "POST", &reset, "", "password=").await;
    assert!(text(res).await.contains("This field is required."));
    let res = send(&router, "POST", &reset, "", "password=better").await;
    assert_eq!(location(&res), "/signin");
    let res = send(
        &router,
        "POST",
        "/signin",
        "",
        "email=ada%40example.com&password=secret",
    )
    .await;
    assert!(text(res).await.contains("Wrong email or password."));
    let res = send(
        &router,
        "POST",
        "/signin",
        "",
        "email=ada%40example.com&password=better",
    )
    .await;
    assert_eq!(location(&res), "/");
    assert!(
        text(send(&router, "GET", &reset, "", "").await)
            .await
            .contains("Link expired")
    );

    // Magic link: signs in once, then the link is spent.
    let res = send(
        &router,
        "POST",
        "/magic-link",
        "",
        "email=ada%40example.com",
    )
    .await;
    assert_eq!(location(&res), "/signin");
    let magic = format!(
        "/magic-link/{}",
        ada(&db).await.magic_link_token.expect("a sign-in link")
    );
    let res = send(&router, "GET", &magic, "", "").await;
    assert_eq!(location(&res), "/");
    auth_cookie(&res);
    assert!(
        text(send(&router, "GET", &magic, "", "").await)
            .await
            .contains("Link expired")
    );

    // A reset link for the pages below.
    send(&router, "POST", "/forgot", "", "email=ada%40example.com").await;
    let reset = format!("/reset/{}", ada(&db).await.reset_token.unwrap());

    // A notebook (generated scaffold, its show page a record page), then a note in it with
    // two tags. A missing title re-renders the form with what was typed.
    let res = send(&router, "POST", "/notebooks", &auth, "name=Ideas").await;
    assert_eq!(location(&res), "/notebooks/1");
    let res = send(&router, "POST", "/notes", &auth, "title=&body=kept").await;
    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let html = text(res).await;
    assert!(
        html.contains("This field is required.") && html.contains("kept"),
        "{html}"
    );
    let note = "title=First&body=Hello%0D%0A%0D%0AAgain&notebook_id=1&tags=Rust%2C+ideas%2C+rust\
                &pinned=on&due=2026-10-01";
    let res = send(&router, "POST", "/notes", &auth, note).await;
    assert_eq!(res.status(), StatusCode::SEE_OTHER);
    assert_eq!(location(&res), "/notes/1");
    let html = text(send(&router, "GET", "/notes/1", &auth, "").await).await;
    for shown in [
        "<p>Hello</p><p>Again</p>",
        r#"href="/notes?sel=ideas""#,
        r#"href="/notes?sel=rust""#,
        r#"<a href="/notebooks/1">Ideas</a>"#,
        "lui-description-list",
    ] {
        assert!(html.contains(shown), "reading view: no {shown} in {html}");
    }
    // The tag filter keeps the notes with every chosen tag; the sidebar counts them.
    let html = text(send(&router, "GET", "/notes?sel=rust", &auth, "").await).await;
    assert!(html.contains(">First</a>"), "{html}");
    let html = text(send(&router, "GET", "/notes?sel=rust&sel=web", &auth, "").await).await;
    assert!(!html.contains(">First</a>") && html.contains("No notes here"));
    let html = text(send(&router, "GET", "/notes?view=list", &auth, "").await).await;
    assert!(html.contains("<table") && html.contains(r#"action="/notes/1/pin""#));
    // Pin and archive toggle, each Post/Redirect/Get.
    let res = send(&router, "POST", "/notes/1/pin", &auth, "").await;
    assert_eq!(location(&res), "/notes/1");
    assert!(
        !text(send(&router, "GET", "/notes/pinned", &auth, "").await)
            .await
            .contains(">First</a>")
    );
    let res = send(&router, "POST", "/notes/1/archive", &auth, "").await;
    assert_eq!(location(&res), "/notes/1");
    assert!(
        text(send(&router, "GET", "/notes/archive", &auth, "").await)
            .await
            .contains(">First</a>")
    );
    send(&router, "POST", "/notes/1/archive", &auth, "").await;
    send(&router, "POST", "/notes/1/pin", &auth, "").await;

    // Every field kind the scaffold knows, on `tasks`: date-times are `datetime-local`
    // (posted without seconds), the decimal a pattern. The owner is the signed-in user, so
    // the form has no `user_id` field.
    let html = text(send(&router, "GET", "/tasks/new", &auth, "").await).await;
    assert!(
        !html.contains(r#"name="user_id""#),
        "no owner field: {html}"
    );
    assert!(html.contains(r#"type="datetime-local""#) && html.contains(r#"type="number""#));
    let res = send(
        &router,
        "POST",
        "/tasks",
        &auth,
        "title=&size=many&starts_at=soon",
    )
    .await;
    let html = text(res).await;
    for message in [
        "Title: This field is required.",
        "Size: Check this field.",
        "Starts at: Check this field.",
    ] {
        assert!(html.contains(message), "no {message} in {html}");
    }
    let task = "title=Ship&user_id=2&done=true&due_on=2026-10-01&starts_at=2026-10-01T09%3A30\
                &remind_at=2026-09-30T18%3A00&price=12.50&status=doing&size=3";
    let res = send(&router, "POST", "/tasks", &auth, task).await;
    assert_eq!(location(&res), "/tasks/1");
    let html = text(send(&router, "GET", "/tasks/1/edit", &auth, "").await).await;
    for filled in [
        r#"value="2026-10-01T09:30""#,
        r#"value="2026-09-30T18:00""#,
        r#"value="12.5""#, // SQLite keeps a decimal as a real
        r#"<option value="doing" selected>"#,
    ] {
        assert!(
            html.contains(filled),
            "task edit form: no {filled} in {html}"
        );
    }

    // A posted `user_id` is ignored: the task is Ada's. Another user cannot see, list, change
    // or delete it.
    let res = send(
        &router,
        "POST",
        "/signup",
        "",
        "name=Grace&email=grace%40example.com&password=secret",
    )
    .await;
    let grace = auth_cookie(&res);
    let list = text(send(&router, "GET", "/tasks", &grace, "").await).await;
    assert!(!list.contains("Ship"), "Grace lists Ada's task: {list}");
    for (method, path, form) in [
        ("GET", "/tasks/1", ""),
        ("GET", "/tasks/1/edit", ""),
        ("POST", "/tasks/1", task),
        ("POST", "/tasks/1/delete", ""),
    ] {
        let res = send(&router, method, path, &grace, form).await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND, "{method} {path}");
    }
    assert!(
        text(send(&router, "GET", "/tasks", &auth, "").await)
            .await
            .contains("Ship"),
        "still Ada's"
    );

    // Notes, notebooks and the board are Ada's too: Grace gets 404, and a note she writes
    // cannot be put in Ada's notebook.
    for (method, path, form) in [
        ("GET", "/notes/1", ""),
        ("GET", "/notes/1/edit", ""),
        ("POST", "/notes/1", "title=Mine"),
        ("POST", "/notes/1/pin", ""),
        ("POST", "/notes/1/archive", ""),
        ("POST", "/notes/1/delete", ""),
        ("GET", "/notebooks/1", ""),
        ("POST", "/tasks/move", "card=1&to=done"),
    ] {
        let res = send(&router, method, path, &grace, form).await;
        assert_eq!(
            res.status(),
            StatusCode::NOT_FOUND,
            "Grace: {method} {path}"
        );
    }
    assert!(
        !text(send(&router, "GET", "/notes", &grace, "").await)
            .await
            .contains(">First</a>")
    );
    let res = send(
        &router,
        "POST",
        "/notes",
        &grace,
        "title=Hers&notebook_id=1",
    )
    .await;
    let hers = location(&res).to_string();
    let html = text(send(&router, "GET", &hers, &grace, "").await).await;
    assert!(
        !html.contains("/notebooks/1"),
        "Grace's note is in no notebook: {html}"
    );

    // The board moves a card between columns: its status changes.
    let res = send(&router, "POST", "/tasks/move", &auth, "card=1&to=done").await;
    assert_eq!(location(&res), "/tasks");
    let html = text(send(&router, "GET", "/tasks/1/edit", &auth, "").await).await;
    assert!(html.contains(r#"<option value="done" selected>"#), "{html}");

    // A session whose user is gone (a database reset since the cookie was set) is signed out:
    // the landing page at `/`, the sign-in form elsewhere, never a server error.
    let res = send(
        &router,
        "POST",
        "/signup",
        "",
        "name=Gone&email=gone%40example.com&password=secret",
    )
    .await;
    let gone = auth_cookie(&res);
    user_rows::Entity::delete_many()
        .filter(user_rows::Column::Email.eq("gone@example.com"))
        .exec(&db)
        .await
        .unwrap();
    let res = send(&router, "GET", "/", &gone, "").await;
    assert_eq!(res.status(), StatusCode::OK);
    assert!(text(res).await.contains("Notes that live on your server."));
    let res = send(&router, "GET", "/notes", &gone, "").await;
    assert_eq!(location(&res), "/signin?next=%2Fnotes");

    // Signed out, the front page is the landing page.
    let html = text(send(&router, "GET", "/", "", "").await).await;
    assert!(html.contains("Notes that live on your server."), "{html}");
    let mut page = Page::render(router.clone(), "/", "").await;
    assert!(page.is_visible("h1"));
    page.screenshot(format!("{SHOTS}/loco-landing.png"))
        .unwrap();

    // The pages print when a note changed ("Edited …") and was created: fixed times, so a shot
    // changes only when its page does. A minute apart by id, so the notes keep their order.
    let day = chrono::DateTime::parse_from_rfc3339("2026-09-01T09:00:00Z").unwrap();
    for note in notes::Entity::find().all(&db).await.unwrap() {
        notes::Entity::update_many()
            .col_expr(
                notes::Column::UpdatedAt,
                Expr::value(day + chrono::Duration::minutes(note.id)),
            )
            .filter(notes::Column::Id.eq(note.id))
            .exec(&db)
            .await
            .unwrap();
    }
    // The overview before the creation day: its chart counts the notes of the eight weeks up
    // to today, so that shot still moves with the calendar.
    shoot(&router, &auth, &tag, "/", "index").await;
    notes::Entity::update_many()
        .col_expr(notes::Column::CreatedAt, Expr::value(day))
        .exec(&db)
        .await
        .unwrap();
    notebooks::Entity::update_many()
        .col_expr(notebooks::Column::CreatedAt, Expr::value(day))
        .exec(&db)
        .await
        .unwrap();

    let pages = [
        ("/signin", "signin"),
        ("/signup", "signup"),
        ("/forgot", "forgot"),
        (reset.as_str(), "reset"),
        ("/reset/made-up", "link-expired"),
        ("/magic-link", "magic-link"),
        ("/notes", "notes"),
        ("/notes?view=list", "notes-table"),
        ("/notes?sel=rust", "notes-tagged"),
        ("/notes/pinned", "notes-pinned"),
        ("/notes/archive", "notes-archive"),
        ("/notes/new", "notes-new"),
        ("/notes/1", "notes-show"),
        ("/notes/1/edit", "notes-edit"),
        ("/notebooks", "notebooks"),
        ("/notebooks/new", "notebooks-new"),
        ("/notebooks/1", "notebooks-show"),
        ("/tasks", "tasks"),
        ("/tasks/new", "tasks-new"),
        ("/tasks/1", "tasks-show"),
        ("/tasks/1/edit", "tasks-edit"),
    ];
    for (path, name) in pages {
        shoot(&router, &auth, &tag, path, name).await;
    }

    // Switching between cards and the table keeps the search and the chosen tags (the tag
    // box has its own hidden `sel`, so look inside the switch's form).
    let html = text(send(&router, "GET", "/notes?q=Hello&sel=rust", &auth, "").await).await;
    let switch = html
        .split(r#"<form class="notes-view""#)
        .nth(1)
        .expect("the view switch");
    let switch = &switch[..switch.find("</form>").unwrap()];
    for kept in [
        r#"<input type="hidden" name="q" value="Hello">"#,
        r#"<input type="hidden" name="sel" value="rust">"#,
    ] {
        assert!(switch.contains(kept), "view switch: no {kept} in {switch}");
    }

    // A tag link encodes its tag: `c#` filters by `c#`, not by `c`.
    let res = send(&router, "POST", "/notes", &auth, "title=Sharp&tags=c%23").await;
    let sharp = location(&res).to_string();
    let html = text(send(&router, "GET", "/notes", &auth, "").await).await;
    assert!(html.contains(r#"href="/notes?sel=c%23""#), "{html}");
    let res = send(&router, "POST", &format!("{sharp}/delete"), &auth, "").await;
    assert_eq!(location(&res), "/notes");

    // A path no route answers is loco-ui's 404 page, not Loco's plain one.
    let res = send(&router, "GET", "/no-such-page", "", "").await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
    let html = text(res).await;
    assert!(html.contains("Page not found"), "{html}");
    assert!(
        html.contains("--lui-brand-9: #0f766e"),
        "the 404 is in the app's look too"
    );

    // The script and the beacon the pages ask for are mounted.
    let res = send(&router, "GET", loco_ui::enhance::SCRIPT_PATH, "", "").await;
    assert_eq!(res.status(), StatusCode::OK);

    // Update and delete, each Post/Redirect/Get.
    let res = send(
        &router,
        "POST",
        "/notes/1",
        &auth,
        "title=Renamed&tags=rust",
    )
    .await;
    assert_eq!(location(&res), "/notes/1");
    assert!(
        text(send(&router, "GET", "/notes/1", &auth, "").await)
            .await
            .contains("Renamed")
    );
    let res = send(&router, "POST", "/notes/1/delete", &auth, "").await;
    assert_eq!(location(&res), "/notes");
    let res = send(&router, "GET", "/notes/1", &auth, "").await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}
