//! Lists every library source file for `llms.rs`: `$OUT_DIR/sources.rs` holds
//! `(module, include_str!(file))` for each `loco-ui/src/*.rs` and `blocks/*.rs` (and
//! `loco-ui-caps`, which is `loco_ui::caps`), so a new
//! component's `//!` header reaches `docs/llms-full.txt` without a list to edit, and cargo
//! rebuilds when a header changes.

use std::path::Path;

fn main() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("../loco-ui/src");
    let mut files = Vec::new();
    for dir in ["", "blocks"] {
        let Ok(entries) = std::fs::read_dir(src.join(dir)) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            if path.extension().is_some_and(|e| e == "rs") && stem != "mod" {
                let module = if dir.is_empty() {
                    stem.to_string()
                } else {
                    format!("{dir}/{stem}")
                };
                files.push((module, path.canonicalize().unwrap_or(path)));
            }
        }
    }
    files.sort();
    // `loco_ui::caps` is the `loco-ui-caps` crate, re-exported.
    let caps = Path::new(env!("CARGO_MANIFEST_DIR")).join("../loco-ui-caps/src/lib.rs");
    files.push(("caps".to_string(), caps.canonicalize().unwrap_or(caps)));
    let mut out = String::from("pub(crate) const LIBRARY_SOURCES: &[(&str, &str)] = &[\n");
    for (module, path) in &files {
        out.push_str(&format!(
            "    ({module:?}, include_str!({:?})),\n",
            path.display().to_string()
        ));
    }
    out.push_str("];\n");
    let dest = Path::new(&std::env::var("OUT_DIR").unwrap_or_default()).join("sources.rs");
    std::fs::write(dest, out).unwrap_or_else(|e| panic!("writing sources.rs: {e}"));
    println!("cargo::rerun-if-changed=../loco-ui/src");
    println!("cargo::rerun-if-changed=../loco-ui/src/blocks");
    println!("cargo::rerun-if-changed=../loco-ui-caps/src/lib.rs");
}
