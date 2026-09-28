//! # Cookie
//!
//! The one place loco-ui writes a `Set-Cookie` value, and the check that refuses a form post
//! from another site. Every cookie is `Path=/` and `SameSite=Lax`; it is `Secure` when the
//! request came over HTTPS ([`crate::caps::is_https`]), so plain-HTTP development keeps its
//! cookies and production never sends them in the clear.
//!
//! [`SetCookie`] builds the value. [`crate::Ui`] learns `secure` from the request (the Axum
//! extractor reads the URI scheme, `X-Forwarded-Proto` and `Forwarded`; other servers call
//! [`crate::Ui::secure`]), and hands it to the state cookie, the flash, the theme, the
//! language and `Saved<T>`.
//!
//! [`same_origin`] is the cross-site check the Axum `Ui` extractor runs on every request: a
//! POST whose `Origin` names another host, or whose `Sec-Fetch-Site` says another site, is
//! refused with `403` before the handler runs. It backs up `SameSite=Lax` (which already keeps
//! the cookies off a cross-site POST) and also covers sibling subdomains, which `SameSite`
//! counts as the same site.
//!
//! **Platform features:** `Secure` and `SameSite=Lax` cookies (Chrome 51, Firefox 60,
//! Safari 12); the `Origin` request header on POST (every engine); `Sec-Fetch-Site`
//! (Chrome 76, Firefox 90, Safari 16.4).
//!
//! **Fallback:** a request with neither `Origin` nor `Sec-Fetch-Site` (curl, a test, a very
//! old browser) passes the check; `SameSite=Lax` still keeps a browser's cookies off a
//! cross-site POST.
//!
//! ```rust
//! use loco_ui::cookie::{SetCookie, same_origin};
//! assert_eq!(
//!     SetCookie::new("theme", "dark", 60).to_string(),
//!     "theme=dark; Path=/; Max-Age=60; SameSite=Lax"
//! );
//! assert_eq!(
//!     SetCookie::clear("lui-flash").http_only().secure(true).to_string(),
//!     "lui-flash=; Path=/; Max-Age=0; SameSite=Lax; HttpOnly; Secure"
//! );
//! // A post from this site passes, one from another site does not.
//! assert!(same_origin("POST", Some("shop.example"), Some("https://shop.example"), None));
//! assert!(!same_origin("POST", Some("shop.example"), Some("https://evil.example"), None));
//! ```

use std::fmt;

/// A `Set-Cookie` value: `name=value; Path=/; Max-Age=..; SameSite=Lax`, then `HttpOnly` and
/// `Secure` when asked. The value is written as given, so encode it first.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SetCookie<'a> {
    name: &'a str,
    value: &'a str,
    max_age: u64,
    http_only: bool,
    secure: bool,
}

impl<'a> SetCookie<'a> {
    /// Keep `value` under `name` for `max_age` seconds.
    pub fn new(name: &'a str, value: &'a str, max_age: u64) -> Self {
        SetCookie {
            name,
            value,
            max_age,
            http_only: false,
            secure: false,
        }
    }

    /// Remove the cookie `name` (an empty value that expires at once).
    pub fn clear(name: &'a str) -> Self {
        SetCookie::new(name, "", 0)
    }

    /// Hide the cookie from page script (`HttpOnly`).
    pub fn http_only(mut self) -> Self {
        self.http_only = true;
        self
    }

    /// Send the cookie over HTTPS only (`Secure`); pass whether this request was HTTPS.
    pub fn secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    }
}

impl fmt::Display for SetCookie<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}={}; Path=/; Max-Age={}; SameSite=Lax",
            self.name, self.value, self.max_age
        )?;
        if self.http_only {
            f.write_str("; HttpOnly")?;
        }
        if self.secure {
            f.write_str("; Secure")?;
        }
        Ok(())
    }
}

/// Whether a request may run: `false` for a request that changes something (any method but
/// GET, HEAD and OPTIONS) sent from another site.
///
/// - `origin` present: its host (and port) must equal `host`; `Origin: null` fails.
/// - no `origin`, `sec_fetch_site` present: `same-origin` and `none` pass, `same-site` and
///   `cross-site` fail.
/// - neither: pass (curl, tests, clients that send no browser headers).
///
/// `host` is the request's `Host` header (or `X-Forwarded-Host` behind a proxy that rewrites
/// it).
///
/// ```rust
/// use loco_ui::cookie::same_origin;
/// let host = Some("app.example:8080");
/// assert!(same_origin("GET", host, Some("https://evil.example"), None), "reads always pass");
/// assert!(same_origin("POST", host, Some("http://app.example:8080"), None));
/// assert!(!same_origin("POST", host, Some("http://app.example"), None), "port differs");
/// assert!(!same_origin("POST", host, Some("null"), None));
/// assert!(!same_origin("POST", host, None, Some("same-site")), "a sibling subdomain");
/// assert!(same_origin("POST", host, None, Some("same-origin")));
/// assert!(same_origin("POST", host, None, None), "curl");
/// ```
pub fn same_origin(
    method: &str,
    host: Option<&str>,
    origin: Option<&str>,
    sec_fetch_site: Option<&str>,
) -> bool {
    if ["GET", "HEAD", "OPTIONS"]
        .iter()
        .any(|m| m.eq_ignore_ascii_case(method))
    {
        return true;
    }
    if let Some(origin) = origin {
        let origin_host = origin.split_once("://").map(|(_, rest)| rest);
        return match (origin_host, host) {
            (Some(o), Some(h)) => o.trim_end_matches('/').eq_ignore_ascii_case(h.trim()),
            _ => false,
        };
    }
    match sec_fetch_site {
        Some(site) => matches!(site.trim(), "same-origin" | "none"),
        None => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_cookie_writes_attributes_in_one_order() {
        let c = SetCookie::new("lui-x", "a%20b", 31_536_000);
        assert_eq!(
            c.clone().http_only().to_string(),
            "lui-x=a%20b; Path=/; Max-Age=31536000; SameSite=Lax; HttpOnly"
        );
        assert_eq!(
            c.secure(true).to_string(),
            "lui-x=a%20b; Path=/; Max-Age=31536000; SameSite=Lax; Secure"
        );
    }

    #[test]
    fn same_origin_refuses_other_sites() {
        let host = Some("app.example");
        assert!(same_origin("post", host, Some("https://app.example"), None));
        assert!(same_origin(
            "POST",
            host,
            Some("https://APP.example/"),
            None
        ));
        assert!(!same_origin(
            "POST",
            host,
            Some("https://app.example.evil"),
            None
        ));
        assert!(!same_origin(
            "POST",
            host,
            Some("https://sub.app.example"),
            None
        ));
        assert!(!same_origin("DELETE", host, None, Some("cross-site")));
        assert!(
            same_origin("POST", host, None, Some("none")),
            "typed in the address bar"
        );
        assert!(
            !same_origin("POST", None, Some("https://app.example"), None),
            "no Host"
        );
        assert!(same_origin("HEAD", None, Some("null"), Some("cross-site")));
    }
}
