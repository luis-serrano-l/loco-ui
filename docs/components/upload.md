# Upload

`loco_ui::upload` · demo: `/upload`

## Constructors

| `lui!` | Constructor | Status |
|---|---|---|
| `Upload` | `ui.upload(action: &str, name: &str)` | stable |

## Props of `Upload`

| Prop | Kind | Arguments | Default | HTML | What it does |
|---|---|---|---|---|---|
| `accept` | value | `types: &'a str` |  | `accept` | `accept`. |
| `multiple` | switch |  | `off` | `multiple` | Several files at once. |
| `help` | value | `text: &'a str` |  |  | Small print in the drop zone. |
| `file` | item | `name: &'a str, size: u64` |  |  | A file the server already holds, `size` in bytes. |
| `preview` | modifier | `src: &'a str` |  |  | A thumbnail for the file added last (an image URL). |
| `href` | modifier | `href: &'a str` |  | `href` | A link to the file added last, on its name. |
| `remove` | value | `action: &'a str` |  |  | A Remove button per file, posting `<name>=<file name>` to `action`. |

## Platform

| Feature | Chrome | Firefox | Safari |
|---|---|---|---|
| `<input type="file" accept multiple>` | 1 | 1 | 1 |
| `<progress>` | 6 | 6 | 6 |
| `loading="lazy"` | 77 | 75 | 15.4 |

- Fallback: none needed: a plain multipart post; the progress bar needs the enhancement script
- Needs script: partly: upload progress and a preview before sending need script
- Responsive by: container
- Look after: Origin UI file upload

## Documentation

A file upload: a drop zone around a file input, an Upload button, and under it the files the
server already holds, with a thumbnail for images and a Remove button each. The list is what
the server says after the round trip, so it is always true.

The look follows the Origin UI file upload: a dashed drop zone with its icon in a round
tile, tinted on hover and while it has focus, roomier in a wide container; each kept file
a row of thumbnail, name (cut with an ellipsis, whole in its `title`) over its size, and
a remove button.

**Platform features:** `<input type="file" accept multiple>` (baseline 2015) in a
`<form method="post" enctype="multipart/form-data">`; a file dropped on the input is picked
like a chosen one, with no script; `<progress>` (baseline 2013) for the upload bar;
`loading="lazy"` thumbnails; Post/Redirect/Get after the upload and after each removal.

**Accessibility:** a native file input inside its label (Enter or Space opens the picker), a
labelled upload button, and remove buttons named with the file. Checked by axe-core in
headless Firefox on every demo route, both capability variants, light and dark (no serious
or critical violation).

**What it does not do without script:** show the upload's progress (the enhancement script
sends the form through `XMLHttpRequest` and fills the `<progress data-lui-progress>` bar;
without it the browser shows its own loading indicator), preview a file before it is sent,
or accept a drop anywhere but on the input.

**Fallback:** none needed: the form is a plain multipart post.

```rust
use loco_ui::prelude::*;
let ui = Ui::default();
let m = ui.upload("/files", "file").render().into_string();
assert!(m.contains(r#"enctype="multipart/form-data""#) && m.contains(r#"type="file""#));
// Images only, several at once, and what the server already has.
let m = ui.upload("/files", "file")
    .accept("image/*").multiple().help("PNG or JPEG, up to 200 KB.")
    .file("cat.png", 48_213).preview("/files/cat.png")
    .file("notes.txt", 1_024).href("/files/notes.txt")
    .remove("/files/remove");
let m = m.render().into_string();
assert!(m.contains(r#"accept="image/*" multiple"#) && m.contains(r#"src="/files/cat.png""#));
assert!(m.contains("47.1 KB") && m.contains(r#"name="file" value="notes.txt""#));

// The same in `lui!`:
let same = lui! { Upload("/files", "file")
    accept="image/*" multiple help="PNG or JPEG, up to 200 KB." {
    file "cat.png" 48_213 preview="/files/cat.png";
    file "notes.txt" 1_024 href="/files/notes.txt";
    remove "/files/remove";
} };
assert_eq!(same.into_string(), m);
```
