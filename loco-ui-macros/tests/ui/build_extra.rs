use loco_ui::prelude::*;

fn main() {
    let ui = Ui::from_request("/", "", "");
    // `build:` returns one builder: a second component has nowhere to go.
    let _ = lui!(build: Badge("New"); Badge("Old"););
    // And it needs a component, not markup.
    let _ = lui!(build: p { "text" });
}
