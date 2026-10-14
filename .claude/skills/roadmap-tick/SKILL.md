---
name: roadmap-tick
description: Close a loco-ui roadmap box properly - tick it in ROADMAP.md with a "Done:" note, add the CHANGELOG Unreleased bullet, a FINDINGS entry with an issue link for any platform or Blitz gap, and regenerate the README matrix. Use when a ROADMAP box is finished, when finishing a milestone, or when asked to write or fix Done notes.
---

# Roadmap tick

`ROADMAP.md` is the work queue. A box counts as done only when the code, tests, demo and docs
all agree, and the note under it says what changed. A milestone is done when every box is
ticked, `cargo clippy --workspace --all-targets` is clean, the tests pass and the README is
updated (`CLAUDE.md`, "Roadmap").

## 1. The "Done:" note

Change `- [ ]` to `- [x]` and put the note directly under the box text, indented two spaces,
starting `Done:`. Write it in the voice of the earlier milestones (M32 to M36 are good
models):

- **What changed**, by name: files, builders, setters (`.icon_only()`), tests
  (`props_follow_the_conventions`), routes (`/table/minimal`). Names a reader can grep.
- **New API** in full: every new setter, constructor, type or flag, and every rename with
  the old name.
- **What was kept different** from the reference, or from the box's own wording, and why in
  one clause ("Blitz has no `@container`, so the shots show the narrow base").
- **Numbers that were asked for**, measured, never estimated, with what was measured
  ("502 lines in 57 blocks, `// code:` to `// end code` across `demo/src/routes/*.rs`").
- No praise, no "successfully", no "robust". Run the `plain-writing` skill over the note.

A box you cannot finish stays `- [ ]` with one line under it saying why (waits on the owner,
needs another milestone, blocked upstream with a link). Owner-only questions also go in
`BLOCKED.md`.

## 2. CHANGELOG

Under `## [Unreleased]`, a `#### M<NN>` sub-heading (create it if this milestone has none)
and one bullet per user-visible change: what a caller sees, the new names, and **Breaking**
in bold at the start of any bullet that breaks a caller, with what to write instead. Tooling
or docs-only milestones get one line.

## 3. FINDINGS

Every platform or Blitz gap found on the way gets an entry in `FINDINGS.md` in the section it
belongs to: what does not work, in which browser or engine and version, the workaround in the
code, and an issue link (Blitz: github.com/DioxusLabs/blitz/issues; browsers: their bug
tracker or the WPT/caniuse page). Never leave a skipped assertion in `loco-ui-test` without a
comment pointing at the entry. Do not file an upstream issue yourself: that needs the owner's
yes.

## 4. README

- A component or fallback change: `cargo run -p demo -- spec write` regenerates the feature
  matrix and `spec/components.json` (tests fail if they are stale). During the M37 to M46
  parallel push, leave the matrix to the merger.
- A number the box asked for (a line count, a size, a timing) goes in README where the box
  says, with the date or milestone it was measured at.
- A new file or directory a reader should know about goes in README "Layout".

## 5. Before the commit

`cargo fmt --all`, then `scripts/verify.sh`. Commit the tick together with the work it
describes, or right after it, in the repo's message style (`git log --oneline -20`), with no
`Co-Authored-By` or "Generated with" lines.

## Measuring the demo snippets

The demo's visible code is the lines between `// code: <href>` and `// end code` in the route
files. To count them:

```sh
awk '/^[ \t]*\/\/ code:/{on=1;next} /^[ \t]*\/\/ end code/{on=0} on{n++} END{print n}' demo/src/routes/*.rs
grep -cE '^[[:space:]]*// code:' demo/src/routes/*.rs | awk -F: '{s+=$2} END{print s, "blocks"}'
wc -l demo/src/routes/*.rs | tail -1
```

Anchor the markers at the start of a line: `routes/mod.rs` mentions `// code:` in a doc
comment, and an unanchored match counts the rest of that file.
