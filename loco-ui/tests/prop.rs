//! Property tests for the parsers that read a request: whatever arrives in a query string or
//! a cookie, they return a value (never panic), and what the library writes it reads back.

use loco_ui::calendar::Date;
use loco_ui::prelude::*;
use loco_ui::state::UiState;
use loco_ui::{href, minify_css, slug};
use proptest::prelude::*;

/// Text a browser could send: any characters, with the ones a query string gives meaning to
/// over-represented.
fn wild() -> impl Strategy<Value = String> {
    prop::collection::vec(
        prop_oneof![
            any::<char>().prop_map(String::from),
            prop::sample::select(vec![
                "&", "=", "%", "+", ";", ",", ".", "%2", "%zz", "%C3", " "
            ])
            .prop_map(String::from),
            "[a-z0-9.]{1,6}",
        ],
        0..24,
    )
    .prop_map(|parts| parts.concat())
}

/// A subset of every capability.
fn caps_subset() -> impl Strategy<Value = Vec<Cap>> {
    prop::sample::subsequence(Cap::ALL.to_vec(), 0..=Cap::ALL.len())
}

proptest! {
    #[test]
    fn any_request_builds_a_ui(path in wild(), query in wild(), cookie in wild()) {
        let ui = Ui::from_request(&path, &query, &cookie);
        let state = UiState::from_request(&path, &query, &cookie);
        let _ = (state.tab("x"), state.opens("x"), state.dialog(), state.set_cookies());
        let table = ui.table_query("files", &["name", "size"]);
        prop_assert!(table.page() >= 1);
        prop_assert!((1..=50).contains(&table.per_page()));
    }

    #[test]
    fn href_reads_back_through_the_query(key in "\\PC{1,12}", value in "\\PC{0,12}") {
        let link = href("/p", [(key.as_str(), value.as_str())]);
        let (_, query) = link.split_once('?').unwrap();
        let ui = Ui::from_request("/p", query, "");
        prop_assert_eq!(ui.param(&key), Some(value.as_str()));
    }

    #[test]
    fn state_survives_its_cookie(name in "[a-z0-9-]{1,8}", tab in 0usize..20, open in 0usize..20) {
        let query = format!("tab.{name}={tab}&open.{name}={open}");
        let state = UiState::parse("/", &query, "");
        prop_assert_eq!(state.tab(&name), tab);
        let cookie = state.cookie_value().unwrap();
        let back = UiState::parse("/", "", &cookie);
        prop_assert_eq!(back.entries(), state.entries());
    }

    #[test]
    fn caps_read_back_from_query_and_cookies(set in caps_subset(), noise in wild()) {
        let _ = (Caps::from_query(&noise), Caps::from_cookie_header(&noise));
        let names: Vec<_> = set.iter().map(|c| c.name()).collect();
        let forced = Caps::from_query(&format!("a=1&caps={}", names.join(","))).unwrap();
        let cookies: Vec<_> = set.iter().map(|c| format!("lui-cap-{}=1", c.name())).collect();
        let probed = Caps::from_cookie_header(&cookies.join("; "));
        for cap in Cap::ALL {
            prop_assert_eq!(forced.has(cap), cap == Cap::Probed || set.contains(&cap));
            if set.contains(&Cap::Probed) {
                prop_assert_eq!(probed.has(cap), set.contains(&cap));
            }
        }
        if !set.contains(&Cap::Probed) {
            prop_assert_eq!(probed, Caps::ASSUMED);
        }
    }

    #[test]
    fn dates_parse_what_they_print(year in 1i32..=9999, month in 1u8..=12, day in 1u8..=31, noise in wild()) {
        let _ = Date::parse(&noise);
        match Date::new(year, month, day) {
            Some(date) => prop_assert_eq!(Date::parse(&date.to_string()), Some(date)),
            None => prop_assert_eq!(Date::parse(&format!("{year:04}-{month:02}-{day:02}")), None),
        }
    }

    #[test]
    fn slugs_are_id_safe_and_stable(key in "\\PC{0,24}") {
        let id = slug(&key);
        prop_assert!(id.chars().all(|c| (c.is_alphanumeric() && !c.is_ascii_uppercase()) || c == '-' || c == '_'));
        prop_assert!(id.chars().count() >= key.chars().count());
        prop_assert_eq!(slug(&id), id);
    }

    /// Without comments (whose text is dropped, braces and all), minifying keeps every brace
    /// and minifying again changes nothing.
    #[test]
    fn minify_keeps_braces_and_is_idempotent(css in "[a-z0-9 \\n\\t{}:;,>()'\"\\\\.#-]{0,80}") {
        let once = minify_css(&css);
        prop_assert_eq!(once.matches('{').count(), css.matches('{').count());
        prop_assert_eq!(once.matches('}').count(), css.matches('}').count());
        prop_assert_eq!(minify_css(&once), once.clone());
    }
}

#[test]
fn the_stylesheet_is_already_minified() {
    let css = loco_ui::stylesheet();
    assert_eq!(minify_css(css), css);
}
