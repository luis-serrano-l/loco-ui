//! Every page answers, ships no script but loco-ui's optional one, and carries the CSP.

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode, header};
use tower::ServiceExt;

async fn get(path: &str) -> (StatusCode, String, Option<String>) {
    let request = Request::get(path).body(Body::empty()).unwrap();
    let response = {{crate}}::router().oneshot(request).await.unwrap();
    let status = response.status();
    let csp = response
        .headers()
        .get(header::CONTENT_SECURITY_POLICY)
        .map(|v| v.to_str().unwrap().to_string());
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    (status, String::from_utf8(body.to_vec()).unwrap(), csp)
}

#[tokio::test]
async fn every_page_answers_without_script() {
    for (path, status) in [
        ("/", StatusCode::OK),
        ("/settings", StatusCode::OK),
        ("/nowhere", StatusCode::NOT_FOUND),
    ] {
        let (got, html, csp) = get(path).await;
        assert_eq!(got, status, "{path}");
        assert!(
            csp.is_some_and(|c| c.contains("script-src")),
            "{path}: no CSP"
        );
        assert_eq!(
            html.matches("<script").count(),
            html.matches("/lui/enhance.js").count(),
            "{path}: a script other than loco-ui's"
        );
    }
}

#[tokio::test]
async fn the_theme_is_kept_in_a_cookie() {
    let request = Request::post("/theme")
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(Body::from("theme=dark"))
        .unwrap();
    let response = {{crate}}::router().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    let cookie = response.headers().get(header::SET_COOKIE).unwrap();
    assert!(cookie.to_str().unwrap().starts_with("theme=dark;"));
}
