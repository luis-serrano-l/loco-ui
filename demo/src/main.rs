//! `cargo run -p demo` serves the component demo on http://127.0.0.1:3000 (`PORT` overrides
//! the port; the checks use 3001 so they never kill a server you are looking at).
//! `cargo run -p demo -- spec` prints `spec/components.json`; `-- spec write` regenerates
//! that file and the README feature matrix from `loco_ui::spec::SPECS`.
//! `cargo run -p demo -- snapshot <dir>` writes the static snapshot (`demo::snapshot`).

use std::path::Path;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        ["spec"] => print!("{}", loco_ui::spec::to_json()),
        ["spec", "write"] => write_spec(),
        ["snapshot", dir] => write_snapshot(Path::new(dir)).await,
        [] => serve().await,
        other => eprintln!(
            "unknown arguments {other:?}; try `spec`, `spec write`, `snapshot <dir>`, or nothing"
        ),
    }
}

async fn serve() {
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3000);
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", port))
        .await
        .unwrap();
    println!("http://127.0.0.1:{port}");
    axum::serve(listener, demo::router()).await.unwrap();
}

/// Regenerate `spec/components.json` and the README matrix between its markers.
fn write_spec() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    std::fs::write(root.join("spec/components.json"), loco_ui::spec::to_json()).unwrap();
    let readme_path = root.join("README.md");
    let readme = std::fs::read_to_string(&readme_path).unwrap();
    let (start_marker, end_marker) = ("<!-- matrix:start -->", "<!-- matrix:end -->");
    let start = readme
        .find(start_marker)
        .expect("README matrix start marker")
        + start_marker.len();
    let end = readme.find(end_marker).expect("README matrix end marker");
    let updated = format!(
        "{}\n{}{}",
        &readme[..start],
        loco_ui::spec::markdown_table(),
        &readme[end..]
    );
    std::fs::write(&readme_path, updated).unwrap();
    println!("wrote spec/components.json and README.md feature matrix");
}

/// Write every page of the static snapshot into `dir`, replacing what was there.
async fn write_snapshot(dir: &Path) {
    match std::fs::remove_dir_all(dir) {
        Err(e) if e.kind() != std::io::ErrorKind::NotFound => panic!("{}: {e}", dir.display()),
        _ => {}
    }
    std::fs::create_dir_all(dir).unwrap();
    let pages = demo::snapshot::pages().await;
    for page in &pages {
        std::fs::write(dir.join(&page.name), &page.html).unwrap();
    }
    // Pages would otherwise run Jekyll over the files.
    std::fs::write(dir.join(".nojekyll"), "").unwrap();
    println!("wrote {} pages to {}", pages.len(), dir.display());
}
