//! The page every view renders into: a header with a home link (add yours), then the view.
//! Written by `cargo lui install`; edit freely.

use loco_ui::prelude::*;

/// `body` inside the app's header, as a full page titled `title`.
pub fn page(ui: &Ui, title: &str, body: Markup) -> Page {
    ui.page(
        title,
        html! {
            header {
                (ui.cluster().body(html! {
                    a href="/" { strong { "Home" } }
                }))
            }
            main { (body) }
        },
    )
}
