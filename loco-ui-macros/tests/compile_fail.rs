//! The errors a misspelled attribute, a missing required argument, a wrong value type, markup
//! among items, a component written as an item's argument and a misspelt choice (`tone="dangr"`) give, pinned so each keeps
//! pointing at what the caller wrote.

#[test]
fn errors_point_at_what_was_written() {
    trybuild::TestCases::new().compile_fail("tests/ui/*.rs");
}
