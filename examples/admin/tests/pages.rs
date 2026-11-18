//! Every page of the panel as a browser without script sees it, through the same router the
//! binary serves, over an in-memory database with the seed: signing in and out, the users
//! list and its filters, creating, editing and deleting a user, roles, the audit log and
//! settings. Each GET page ships only the enhancement script, and Blitz (no script engine)
//! renders it into `tests/shots/admin-*.png`.

use axum::{
    Router,
    body::Body,
    http::{Request, Response, StatusCode, header},
};
use loco_ui_test::Page;
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

/// Every `name=value` pair a response sets, joined as a `Cookie` header (the session, and
/// the flash a redirect carries).
fn cookies(res: &Response<Body>) -> String {
    let pairs = res.headers().get_all(header::SET_COOKIE).iter();
    pairs
        .filter_map(|v| v.to_str().ok()?.split(';').next())
        .collect::<Vec<_>>()
        .join("; ")
}

/// A fresh panel and Ada's session cookie.
async fn signed_in() -> (Router, String) {
    let router = admin::router(admin::db::open("sqlite::memory:").await.unwrap());
    let form = format!(
        "email={}&password={}",
        admin::db::SEED_EMAIL,
        admin::db::SEED_PASSWORD
    );
    let res = send(&router, "POST", "/signin", "", &form).await;
    assert_eq!(
        (res.status(), location(&res)),
        (StatusCode::SEE_OTHER, "/users")
    );
    let session = cookies(&res);
    assert!(session.starts_with("admin-session="), "{session}");
    (router, session)
}

/// POST `form` to `path`, follow the redirect, and give back where it went and that page.
async fn post(router: &Router, session: &str, path: &str, form: &str) -> (String, String) {
    let res = send(router, "POST", path, session, form).await;
    assert_eq!(
        res.status(),
        StatusCode::SEE_OTHER,
        "{path}: {}",
        text(res).await
    );
    let to = location(&res).to_string();
    let with_flash = format!("{session}; {}", cookies(&res));
    (
        to.clone(),
        text(send(router, "GET", &to, &with_flash, "").await).await,
    )
}

#[tokio::test]
async fn signed_out_visitors_are_sent_to_sign_in() {
    let router = admin::router(admin::db::open("sqlite::memory:").await.unwrap());
    for path in ["/", "/users", "/users/1", "/roles", "/audit", "/settings"] {
        let mut res = send(&router, "GET", path, "", "").await;
        if path == "/" {
            assert_eq!(location(&res), "/users");
            res = send(&router, "GET", "/users", "", "").await;
        }
        assert_eq!(
            (res.status(), location(&res)),
            (StatusCode::SEE_OTHER, "/signin"),
            "{path}"
        );
    }
    let res = send(&router, "POST", "/users/2/delete", "", "").await;
    assert_eq!(
        location(&res),
        "/signin",
        "a post without a session changes nothing"
    );

    let res = send(
        &router,
        "POST",
        "/signin",
        "",
        "email=ada@example.com&password=wrong",
    )
    .await;
    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert!(cookies(&res).is_empty(), "no session for a wrong password");
    let html = text(res).await;
    assert!(
        html.contains("do not match an active account")
            && html.contains(r#"value="ada@example.com""#),
        "{html}"
    );

    // A suspended account gets the same answer as a wrong password.
    let form = format!(
        "email=tim@example.com&password={}",
        admin::db::SEED_PASSWORD
    );
    let res = send(&router, "POST", "/signin", "", &form).await;
    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let mut page = Page::render(router.clone(), "/signin", "").await;
    assert!(page.is_visible("h1") && page.is_visible("input[name=password]"));
    page.screenshot(format!("{SHOTS}/admin-signin.png"))
        .unwrap();
}

#[tokio::test]
async fn every_page_ships_one_script_and_renders_in_blitz() {
    let (router, session) = signed_in().await;
    let tag = loco_ui::enhance::script_tag().into_string();
    let pages = [
        ("/users", "users"),
        ("/users?status=suspended", "users-suspended"),
        ("/users/2", "user"),
        ("/users/2/edit", "user-edit"),
        ("/users/new", "user-new"),
        ("/roles", "roles"),
        // The seed's line, whose date is fixed; the full log starts with this run's sign-in.
        ("/audit?q=seeded", "audit"),
        ("/settings", "settings"),
    ];
    for (path, name) in pages {
        let res = send(&router, "GET", path, &session, "").await;
        assert_eq!(res.status(), StatusCode::OK, "{path}");
        let html = text(res).await;
        assert_eq!(html.matches("<script").count(), 1, "{path}: one script");
        assert!(
            html.contains(&tag),
            "{path}: and it is the enhancement script"
        );
        assert!(
            html.contains("Ada Lovelace") && html.contains(r#"action="/signout""#),
            "{path}: in the shell"
        );
        let mut page = Page::render(router.clone(), path, &session).await;
        assert!(page.is_visible("h1"), "{path}: Blitz lays the page out");
        page.screenshot(format!("{SHOTS}/admin-{name}.png"))
            .unwrap();
    }
    let res = send(&router, "GET", "/users/99", &session, "").await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
    assert!(text(res).await.contains("No such user"));
    let res = send(&router, "GET", "/nowhere", &session, "").await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn the_users_list_searches_and_filters() {
    let (router, session) = signed_in().await;
    let list = |path: &'static str| {
        let (router, session) = (router.clone(), session.clone());
        async move { text(send(&router, "GET", path, &session, "").await).await }
    };
    let all = list("/users").await;
    assert!(
        all.contains("10 of 10 users")
            && all.contains("Grace Hopper")
            && all.contains("Tim Berners-Lee")
    );
    let suspended = list("/users?status=suspended").await;
    assert!(
        suspended.contains("2 of 10 users") && suspended.contains("Edsger Dijkstra"),
        "{suspended}"
    );
    assert!(!suspended.contains("Grace Hopper"));
    assert!(
        suspended.contains(r#"href="/users?status=suspended" aria-current="page""#),
        "the filter is marked"
    );
    assert!(
        suspended.contains(r#"<input type="hidden" name="status" value="suspended">"#),
        "a search keeps the filter"
    );
    let found = list("/users?q=hopper").await;
    assert!(found.contains("1 of 10 users") && found.contains("grace@example.com"));
    let viewers = list("/users?role=4").await;
    assert!(viewers.contains("2 of 10 users") && viewers.contains("Radia Perlman"));
    let none = list("/users?q=zzz").await;
    assert!(none.contains("No user matches") && !none.contains("<table"));
    // A quote in the search is a value, not SQL.
    assert!(
        list("/users?q=%27%20OR%201%3D1%20--")
            .await
            .contains("0 of 10 users")
    );
}

#[tokio::test]
async fn a_user_is_created_edited_and_deleted() {
    let (router, session) = signed_in().await;

    // Mistakes send the form back with what was typed.
    let res = send(
        &router,
        "POST",
        "/users",
        &session,
        "name=&email=grace@example.com&password=short&role=3",
    )
    .await;
    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let html = text(res).await;
    for message in [
        "Enter a name.",
        "Another user has this address.",
        "Use at least 12 characters.",
    ] {
        assert!(html.contains(message), "{message}");
    }
    assert!(
        html.contains(r#"value="grace@example.com""#) && !html.contains(r#"value="short""#),
        "the password is not echoed"
    );

    let form =
        "name=Hedy+Lamarr&email=Hedy@Example.com&password=frequency-hopping&role=4&active=true";
    let (to, html) = post(&router, &session, "/users", form).await;
    assert_eq!(to, "/users/11");
    assert!(
        html.contains("User created.")
            && html.contains("hedy@example.com")
            && html.contains(">Viewer<"),
        "{html}"
    );
    assert!(
        html.contains(r#"action="/users/11/delete""#),
        "another user's page has Delete"
    );

    // The new user can sign in.
    let res = send(
        &router,
        "POST",
        "/signin",
        "",
        "email=hedy@example.com&password=frequency-hopping",
    )
    .await;
    assert_eq!(res.status(), StatusCode::SEE_OTHER);
    let hedy = cookies(&res);

    // Suspending her ends that session.
    let (to, html) = post(
        &router,
        &session,
        "/users/11",
        "name=Hedy+Lamarr&email=hedy@example.com&role=3",
    )
    .await;
    assert_eq!(to, "/users/11");
    assert!(
        html.contains("Saved.") && html.contains("Suspended") && html.contains(">Member<"),
        "{html}"
    );
    let res = send(&router, "GET", "/users", &hedy, "").await;
    assert_eq!(location(&res), "/signin");

    let (to, html) = post(&router, &session, "/users/11/delete", "").await;
    assert_eq!(to, "/users");
    assert!(
        html.contains("Hedy Lamarr was deleted.") && html.contains("10 of 10 users"),
        "{html}"
    );

    // Nobody deletes or suspends the account they are signed in with.
    let own = text(send(&router, "GET", "/users/1", &session, "").await).await;
    assert!(!own.contains(r#"action="/users/1/delete""#));
    let (to, html) = post(&router, &session, "/users/1/delete", "").await;
    assert!(
        to == "/users/1" && html.contains("cannot delete the account"),
        "{html}"
    );
    let res = send(
        &router,
        "POST",
        "/users/1",
        &session,
        "name=Ada&email=ada@example.com&role=1",
    )
    .await;
    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let log = text(send(&router, "GET", "/audit", &session, "").await).await;
    for line in ["created user", "updated user", "deleted user", "signed in"] {
        assert!(log.contains(line), "the audit log has: {line}");
    }
    let deleted =
        text(send(&router, "GET", "/audit?action=deleted+user", &session, "").await).await;
    assert!(
        deleted.contains("1 entries") && deleted.contains("hedy@example.com"),
        "{deleted}"
    );
}

#[tokio::test]
async fn roles_are_added_and_only_unused_ones_deleted() {
    let (router, session) = signed_in().await;
    let html = text(send(&router, "GET", "/roles", &session, "").await).await;
    assert!(
        html.contains("4 roles") && html.contains("In use") && !html.contains("/delete"),
        "every seeded role is in use"
    );

    let (_, html) = post(
        &router,
        &session,
        "/roles",
        "name=Auditor&description=Reads+the+audit+log.",
    )
    .await;
    assert!(
        html.contains("Role added.") && html.contains("5 roles"),
        "{html}"
    );
    assert!(
        html.contains(r#"action="/roles/5/delete""#) && html.contains("Delete Auditor?"),
        "an unused role has its dialog"
    );
    let (_, html) = post(
        &router,
        &session,
        "/roles",
        "name=Auditor&description=Again",
    )
    .await;
    assert!(html.contains("already a role named Auditor"));
    let (_, html) = post(&router, &session, "/roles/1/delete", "").await;
    assert!(html.contains("in use") && html.contains("5 roles"));
    let (_, html) = post(&router, &session, "/roles/5/delete", "").await;
    assert!(html.contains("Role deleted.") && html.contains("4 roles"));
}

#[tokio::test]
async fn settings_change_the_name_and_the_password() {
    let (router, session) = signed_in().await;
    let (to, html) = post(&router, &session, "/settings/profile", "name=Ada+King").await;
    assert_eq!(to, "/settings");
    assert!(
        html.contains("Name saved.") && html.contains(r#"value="Ada King""#),
        "{html}"
    );

    let res = send(
        &router,
        "POST",
        "/settings/password",
        &session,
        "current=wrong&new=short",
    )
    .await;
    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let html = text(res).await;
    assert!(
        html.contains("That is not your current password.")
            && html.contains("Use at least 12 characters.")
    );

    // A second session, which the change ends; the one that made it stays.
    let form = format!(
        "email={}&password={}",
        admin::db::SEED_EMAIL,
        admin::db::SEED_PASSWORD
    );
    let other = cookies(&send(&router, "POST", "/signin", "", &form).await);
    let change = format!("current={}&new=difference-engine", admin::db::SEED_PASSWORD);
    let (_, html) = post(&router, &session, "/settings/password", &change).await;
    assert!(html.contains("Password changed."));
    assert_eq!(
        location(&send(&router, "GET", "/users", &other, "").await),
        "/signin"
    );
    assert_eq!(
        send(&router, "GET", "/users", &session, "").await.status(),
        StatusCode::OK
    );

    let res = send(&router, "POST", "/signin", "", &form).await;
    assert_eq!(
        res.status(),
        StatusCode::UNPROCESSABLE_ENTITY,
        "the old password is gone"
    );
    let res = send(
        &router,
        "POST",
        "/signin",
        "",
        "email=ada@example.com&password=difference-engine",
    )
    .await;
    assert_eq!(res.status(), StatusCode::SEE_OTHER);

    let res = send(&router, "POST", "/signout", &session, "").await;
    assert_eq!(location(&res), "/signin");
    assert_eq!(
        location(&send(&router, "GET", "/users", &session, "").await),
        "/signin",
        "the session is over"
    );
}
