//! # loco-ui-caps
//!
//! Server-side feature detection with no script. The page carries a few empty beacon elements;
//! `@supports` rules give each one a background image only when the browser understands the
//! feature. Loading that image hits `/lui/caps?flag=<name>`, which sets a cookie. From the next
//! request on, the server knows what the browser can do and every component emits only the
//! markup that browser needs.
//!
//! **Platform features:** `@supports` (Chrome 28, Firefox 22, Safari 9), `@supports
//! selector()` (Chrome 83, Firefox 69, Safari 14.1), CSS background images, cookies.
//!
//! **Fallback:** a browser with no cookie yet gets [`Caps::ASSUMED`], the features at baseline
//! in every engine for over two years, and the fallback markup for everything else. The
//! beacons fire while that first view loads, and the second view is tailored. Browsers that
//! never load CSS images (`curl`, readers) stay on `ASSUMED`.
//!
//! **Finding:** CSS cannot test HTML attributes, so `invokers` and `streaming_dsd` are proxies
//! for CSS features that shipped in the same release. See `docs/caps.md` in the loco-ui repo.
//!
//! One cookie per flag (`lui-cap-<name>=1`) rather than one cookie holding a list: the beacons
//! fire in parallel, and parallel `Set-Cookie` headers on one name would overwrite each other.
//!
//! This crate is the detection half of `loco-ui` and stands on its own: any server that can
//! read a `Cookie:` header and answer one tiny route can use it.
//!
//! **Any server.** Three plain functions are the whole protocol, and none needs Axum:
//! [`Caps::from_cookie_header`] reads the flags out of a `Cookie:` header,
//! [`Caps::from_query`] lets `?caps=popover,anchor` force a set (tests, `curl`, clients
//! without cookies), and [`beacon_cookie`] turns the beacon route's query string into the
//! `Set-Cookie` value to answer with (status 204, `Cache-Control: no-store`). The `axum`
//! feature only wraps them: a `Caps` extractor and [`router`] for the beacon route.
//!
//! ```rust
//! use loco_ui_caps::{self as caps, Cap, Caps};
//! let caps = Caps::from_cookie_header("lui-cap-probed=1; lui-cap-popover=1; theme=dark");
//! assert!(caps.has(Cap::Popover));
//! assert!(!caps.has(Cap::Invokers));
//! assert_eq!(Caps::all().names().len(), Cap::ALL.len());
//! // A query string overrides the cookies, so any URL can be viewed as any browser.
//! assert!(Caps::from_query("caps=invokers,anchor").unwrap().has(Cap::Invokers));
//! // The beacon route, by hand: `GET /lui/caps?flag=popover` answers 204 with this cookie.
//! assert!(caps::beacon_cookie("flag=popover", false).unwrap().starts_with("lui-cap-popover=1"));
//! assert_eq!(caps::beacon_cookie("flag=nope", false), None);
//! // Over HTTPS (here, behind a proxy that says so) the cookie is `Secure`.
//! let https = caps::is_https(None, Some("https"), None);
//! assert!(caps::beacon_cookie("flag=popover", https).unwrap().ends_with("; Secure"));
//! ```

// docs.rs builds with nightly and `--cfg docsrs`: feature-gated items get a "requires feature" badge.
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

use maud::{Markup, html};

/// Path of the beacon route. `beacon_css` builds URLs from it; `router` serves it.
pub const BEACON_PATH: &str = "/lui/caps";

/// Cookie name prefix: the flag `popover` lives in the cookie `lui-cap-popover`.
pub const COOKIE_PREFIX: &str = "lui-cap-";

/// Cookie lifetime in seconds (30 days), so browser upgrades get re-detected eventually.
pub const COOKIE_MAX_AGE: u32 = 30 * 24 * 60 * 60;

/// One browser capability the server can learn about.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Cap {
    /// The browser loaded the beacons at all. Absent on a first visit or a non-visual client.
    Probed,
    /// `<button command commandfor>` invoker commands (Chrome 135, Firefox 144, Safari 26.2).
    Invokers,
    /// CSS anchor positioning (Chrome 125, Firefox 147, Safari 26).
    Anchor,
    /// `::details-content` pseudo-element (Chrome 131, Firefox 143, Safari 18.4).
    DetailsContent,
    /// View transitions: `view-transition-name` and `-class` (Chrome 125, Firefox 144, Safari 18.2).
    ViewTransitions,
    /// `popover` attribute and `:popover-open` (Chrome 114, Firefox 125, Safari 17).
    Popover,
    /// `light-dark()` colour function (Chrome 123, Firefox 120, Safari 17.5).
    LightDark,
    /// Declarative shadow DOM, `<template shadowrootmode>` (Chrome 111, Firefox 123, Safari 16.4).
    StreamingDsd,
    /// Customisable `<select>`: `appearance: base-select`, `<selectedcontent>` (Chrome 135, Safari 27).
    BaseSelect,
}

impl Cap {
    /// Every capability, in display order.
    pub const ALL: [Cap; 9] = [
        Cap::Probed,
        Cap::Invokers,
        Cap::Anchor,
        Cap::DetailsContent,
        Cap::ViewTransitions,
        Cap::Popover,
        Cap::LightDark,
        Cap::StreamingDsd,
        Cap::BaseSelect,
    ];

    /// Flag name used in cookies, beacon URLs and CSS class names.
    pub fn name(self) -> &'static str {
        match self {
            Cap::Probed => "probed",
            Cap::Invokers => "invokers",
            Cap::Anchor => "anchor",
            Cap::DetailsContent => "details_content",
            Cap::ViewTransitions => "view_transitions",
            Cap::Popover => "popover",
            Cap::LightDark => "light_dark",
            Cap::StreamingDsd => "streaming_dsd",
            Cap::BaseSelect => "base_select",
        }
    }

    /// Parse a flag name. Unknown names give `None`.
    pub fn parse(name: &str) -> Option<Cap> {
        Cap::ALL.into_iter().find(|c| c.name() == name)
    }

    /// The `@supports` condition that detects this capability, or `None` when the beacon fires
    /// unconditionally. Proxies are noted in the doc header and in `FINDINGS.md`.
    pub fn supports(self) -> Option<&'static str> {
        match self {
            Cap::Probed => None,
            // No CSS test exists for HTML attributes. `::picker(select)` shipped with invokers
            // in Chrome 135 (Safari: 27); Firefox 144 shipped `view-transition-class` with them.
            Cap::Invokers => Some(
                "selector(::picker(select)) or ((-moz-appearance: none) and (view-transition-class: x))",
            ),
            Cap::Anchor => Some("(anchor-name: --x)"),
            Cap::DetailsContent => Some("selector(::details-content)"),
            Cap::ViewTransitions => Some("(view-transition-class: x)"),
            Cap::Popover => Some("selector(:popover-open)"),
            Cap::LightDark => Some("(color: light-dark(#000, #fff))"),
            // Proxy: `:popover-open` shipped after declarative shadow DOM in every engine.
            Cap::StreamingDsd => Some("selector(:popover-open)"),
            Cap::BaseSelect => Some("selector(::picker(select))"),
        }
    }

    /// One line for humans and the `/caps` page.
    pub fn description(self) -> &'static str {
        match self {
            Cap::Probed => {
                "beacons loaded at all (set on every visual browser after the first view)"
            }
            Cap::Invokers => {
                "dialog opens with <button command=show-modal> instead of a :target link"
            }
            Cap::Anchor => "popover menus sit under their button via anchor positioning",
            Cap::DetailsContent => {
                "tabs lay out as a strip via ::details-content; otherwise an accordion"
            }
            Cap::ViewTransitions => {
                "counter and list get view-transition-name for morphing navigations"
            }
            Cap::Popover => "menus use the popover attribute; otherwise a <details> dropdown",
            Cap::LightDark => {
                "light-dark() is understood (informational; theming uses media queries)"
            }
            Cap::StreamingDsd => "out-of-order streaming via declarative shadow DOM slots",
            Cap::BaseSelect => "<select> shows rich option content via <selectedcontent>",
        }
    }

    fn bit(self) -> u16 {
        1 << (self as u16)
    }
}

/// The set of capabilities the server believes the current browser has.
///
/// `Caps::default()` is empty: every component renders its fallback. `Caps::all()` is the
/// modern-browser view used in doc examples and tests.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Caps(u16);

impl Caps {
    /// Nothing known: fallback markup everywhere.
    pub const NONE: Caps = Caps(0);

    /// Every capability, for examples and tests.
    pub fn all() -> Caps {
        Cap::ALL.into_iter().fold(Caps::NONE, Caps::with)
    }

    /// Whether `cap` is known to be supported.
    pub fn has(self, cap: Cap) -> bool {
        self.0 & cap.bit() != 0
    }

    /// Copy with `cap` added.
    pub fn with(self, cap: Cap) -> Caps {
        Caps(self.0 | cap.bit())
    }

    /// Names of every supported capability, in `Cap::ALL` order.
    pub fn names(self) -> Vec<&'static str> {
        Cap::ALL
            .into_iter()
            .filter(|c| self.has(*c))
            .map(Cap::name)
            .collect()
    }

    /// What an unprobed browser is assumed to support: features at baseline for over two
    /// years in every engine. It keeps the first and second page view looking the same.
    pub const ASSUMED: Caps = Caps(1 << (Cap::Popover as u16));

    /// Read the flags out of a raw `Cookie:` header value. Without a `probed` cookie the
    /// beacons have not fired yet and [`Caps::ASSUMED`] is returned.
    pub fn from_cookie_header(header: &str) -> Caps {
        let caps = header
            .split(';')
            .filter_map(|pair| pair.trim().split_once('='))
            .filter(|(_, value)| value.trim() == "1")
            .filter_map(|(name, _)| name.trim().strip_prefix(COOKIE_PREFIX))
            .filter_map(Cap::parse)
            .fold(Caps::NONE, Caps::with);
        if caps.has(Cap::Probed) {
            caps
        } else {
            Caps::ASSUMED
        }
    }

    /// Read a forced set out of a raw query string: `caps=popover,anchor` (names from
    /// [`Cap::name`], unknown ones ignored). `None` when there is no `caps` parameter, so the
    /// caller falls back to the cookies. A forced set counts as probed, since the point is to
    /// see exactly that variant: `?caps=` alone is an old browser with nothing.
    pub fn from_query(query: &str) -> Option<Caps> {
        let list = query_param(query, "caps")?;
        Some(
            list.split(',')
                .filter_map(|n| Cap::parse(n.trim()))
                .fold(Caps::NONE.with(Cap::Probed), Caps::with),
        )
    }
}

/// Value of `name` in a raw query string (`a=1&b=2`), without percent-decoding: flag and
/// capability names are plain ASCII words.
fn query_param<'a>(query: &'a str, name: &str) -> Option<&'a str> {
    query
        .split('&')
        .filter_map(|pair| pair.split_once('=').or(Some((pair, ""))))
        .find(|(k, _)| *k == name)
        .map(|(_, v)| v)
}

/// The beacon route without a framework: given the raw query string of
/// `GET /lui/caps?flag=<name>`, the `Set-Cookie` value to answer with, or `None` for an unknown
/// flag (answer 404). Either way answer without a body and with `Cache-Control: no-store`, so
/// every page view re-fires the beacons until the cookie exists. `secure` adds `Secure`: pass
/// [`is_https`] of the request.
pub fn beacon_cookie(query: &str, secure: bool) -> Option<String> {
    query_param(query, "flag")
        .and_then(Cap::parse)
        .map(|cap| cookie_for(cap, secure))
}

/// Whether the request came over HTTPS, so its cookies can be `Secure`: the request URI's
/// scheme when the server sees one, else the first `X-Forwarded-Proto` value, else
/// `proto=https` in a `Forwarded` header (RFC 7239). A proxy that terminates TLS must send one
/// of the two headers; without them a plain-HTTP hop reads as plain HTTP.
///
/// ```rust
/// use loco_ui_caps::is_https;
/// assert!(is_https(Some("https"), None, None));
/// assert!(is_https(None, Some("https, http"), None));
/// assert!(is_https(None, None, Some("for=10.0.0.1;proto=https;by=10.0.0.2")));
/// assert!(!is_https(None, None, None));
/// assert!(!is_https(Some("http"), Some("https"), None), "the server's own scheme wins");
/// ```
pub fn is_https(
    scheme: Option<&str>,
    x_forwarded_proto: Option<&str>,
    forwarded: Option<&str>,
) -> bool {
    if let Some(scheme) = scheme {
        return scheme.eq_ignore_ascii_case("https");
    }
    if let Some(proto) = x_forwarded_proto {
        return proto
            .split(',')
            .next()
            .is_some_and(|p| p.trim().eq_ignore_ascii_case("https"));
    }
    forwarded.is_some_and(|f| {
        f.split(',')
            .next()
            .unwrap_or("")
            .split(';')
            .filter_map(|pair| pair.split_once('='))
            .any(|(k, v)| {
                k.trim().eq_ignore_ascii_case("proto")
                    && v.trim().trim_matches('"').eq_ignore_ascii_case("https")
            })
    })
}

/// The `@supports` rules. Each one gives a beacon element a background image whose URL is the
/// beacon route with the flag name. Put it in the page's stylesheet; `loco_ui::stylesheet()` does.
pub fn beacon_css() -> String {
    let mut css = String::from(
        "\n.lui-caps { position: fixed; bottom: 0; right: 0; width: 1px; height: 1px; overflow: hidden; opacity: 0; pointer-events: none; }\n.lui-cap { display: block; width: 1px; height: 1px; }\n",
    );
    for cap in Cap::ALL {
        let rule = format!(
            ".lui-cap-{n} {{ background-image: url(\"{BEACON_PATH}?flag={n}\"); }}",
            n = cap.name()
        );
        match cap.supports() {
            Some(test) => css.push_str(&format!("@supports {test} {{ {rule} }}\n")),
            None => {
                css.push_str(&rule);
                css.push('\n');
            }
        }
    }
    css
}

/// The beacon elements. Empty once the browser has been probed, so a known browser pays
/// nothing. Put it at the end of `<body>`; `loco_ui::layout` does.
pub fn beacons(caps: &Caps) -> Markup {
    html! {
        @if !caps.has(Cap::Probed) {
            div class="lui-caps" aria-hidden="true" {
                @for cap in Cap::ALL {
                    i class={ "lui-cap lui-cap-" (cap.name()) } {}
                }
            }
        }
    }
}

/// `Set-Cookie` value that records `cap` for 30 days; `secure` adds `Secure`.
pub fn cookie_for(cap: Cap, secure: bool) -> String {
    format!(
        "{COOKIE_PREFIX}{}=1; Path=/; Max-Age={COOKIE_MAX_AGE}; SameSite=Lax{}",
        cap.name(),
        if secure { "; Secure" } else { "" }
    )
}

#[cfg(feature = "axum")]
mod axum_glue {
    use super::{BEACON_PATH, Caps, beacon_cookie, is_https};
    use axum::{
        Router,
        extract::FromRequestParts,
        http::{HeaderMap, StatusCode, Uri, header, request::Parts},
        routing::get,
    };

    /// `?caps=` first, then the cookies: a thin wrapper over the two plain parsers.
    impl<S: Send + Sync> FromRequestParts<S> for Caps {
        type Rejection = std::convert::Infallible;

        async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Caps, Self::Rejection> {
            if let Some(forced) = parts.uri.query().and_then(Caps::from_query) {
                return Ok(forced);
            }
            // Join every Cookie header first: `from_cookie_header` decides between the
            // parsed flags and `Caps::ASSUMED` from the whole set.
            let header = parts
                .headers
                .get_all(header::COOKIE)
                .iter()
                .filter_map(|v| v.to_str().ok())
                .collect::<Vec<_>>()
                .join("; ");
            Ok(Caps::from_cookie_header(&header))
        }
    }

    /// Serves `GET /lui/caps?flag=<name>`: sets the flag's cookie and answers `204`.
    /// Merge it into your app: `Router::new().merge(loco_ui::caps::router())`.
    pub fn router() -> Router {
        Router::new().route(BEACON_PATH, get(beacon))
    }

    async fn beacon(uri: Uri, request: HeaderMap) -> (StatusCode, HeaderMap) {
        let header = |name: &str| request.get(name).and_then(|v| v.to_str().ok());
        let secure = is_https(
            uri.scheme_str(),
            header("x-forwarded-proto"),
            header("forwarded"),
        );
        let mut headers = HeaderMap::new();
        headers.insert(header::CACHE_CONTROL, "no-store".parse().unwrap());
        match beacon_cookie(uri.query().unwrap_or(""), secure) {
            Some(cookie) => {
                headers.insert(header::SET_COOKIE, cookie.parse().unwrap());
                (StatusCode::NO_CONTENT, headers)
            }
            None => (StatusCode::NOT_FOUND, headers),
        }
    }
}

#[cfg(feature = "axum")]
pub use axum_glue::router;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cookie_round_trip() {
        let header = Cap::ALL
            .map(|c| cookie_for(c, false).split(';').next().unwrap().to_string())
            .join("; ");
        assert_eq!(Caps::from_cookie_header(&header), Caps::all());
        // Not probed yet: the assumed set, whatever else the header says.
        assert_eq!(
            Caps::from_cookie_header("lui-cap-bogus=1; lui-cap-popover=0"),
            Caps::ASSUMED
        );
        assert_eq!(Caps::from_cookie_header(""), Caps::ASSUMED);
        // Probed with nothing else: an old browser, no assumptions.
        assert_eq!(
            Caps::from_cookie_header("lui-cap-probed=1; lui-cap-popover=0"),
            Caps::NONE.with(Cap::Probed)
        );
    }

    #[test]
    fn query_forces_a_set_and_the_beacon_answers_by_hand() {
        assert_eq!(Caps::from_query("page=2"), None);
        assert_eq!(Caps::from_query(""), None);
        assert_eq!(
            Caps::from_query("page=2&caps=popover,anchor,bogus"),
            Some(
                Caps::NONE
                    .with(Cap::Probed)
                    .with(Cap::Popover)
                    .with(Cap::Anchor)
            )
        );
        assert_eq!(
            Caps::from_query("caps="),
            Some(Caps::NONE.with(Cap::Probed))
        );
        assert_eq!(
            beacon_cookie("flag=anchor", false),
            Some(cookie_for(Cap::Anchor, false))
        );
        assert_eq!(beacon_cookie("flag=nope", false), None);
        assert_eq!(beacon_cookie("", false), None);
        let secure = beacon_cookie("flag=anchor", true).unwrap();
        assert!(secure.ends_with("; SameSite=Lax; Secure"), "{secure}");
    }

    #[test]
    fn beacons_disappear_once_probed() {
        assert!(
            beacons(&Caps::NONE)
                .into_string()
                .contains("lui-cap-invokers")
        );
        assert_eq!(beacons(&Caps::NONE.with(Cap::Probed)).into_string(), "");
    }

    #[test]
    fn css_has_one_rule_per_cap() {
        let css = beacon_css();
        for cap in Cap::ALL {
            assert!(
                css.contains(&format!("?flag={}", cap.name())),
                "{}",
                cap.name()
            );
        }
    }
}
