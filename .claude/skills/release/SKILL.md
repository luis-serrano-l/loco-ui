---
name: release
description: Prepare a loco-ui release up to, and not including, the publish - cut the CHANGELOG, bump the three published crates together, regenerate the spec and README matrix, `cargo package` dry-run, docs.rs build, and check the BLOCKED.md gates. Use when asked to prepare, cut or tag a release or to check the crates are ready for crates.io. It stops before `cargo publish` and says so.
---

# Release

The three published crates share one version: `loco-ui-caps`, `loco-ui-macros` and
`loco-ui` (`loco-ui-test` and `demo` are not published). The first publish is 0.2.0 (owner,
2026-10-14). **This skill never runs `cargo publish`, `git push`, `git tag` on a remote, or
posts anywhere.** It ends with a report and the exact commands the owner would run.

## 1. Gates first

Read `BLOCKED.md`. If the publish is still "on hold" or any open question there touches the
crate names, the version or the license, stop and report it: the release waits for the owner.
Check `ROADMAP.md`: every milestone due before the release is ticked, or say which is not.

## 2. CHANGELOG

- Move everything under `## [Unreleased]` into `## [X.Y.Z] - YYYY-MM-DD` (today's date),
  keeping the `### loco-ui` / `### loco-ui-caps` groups; fold the `#### M<NN>` sub-headings
  into plain bullets unless they help the reader.
- Leave an empty `## [Unreleased]` above it.
- Every **Breaking** bullet says what to write instead. Run the `plain-writing` skill over
  the new section.

## 3. Versions

Bump together, in one commit:

- `version` in `loco-ui-caps/Cargo.toml`, `loco-ui-macros/Cargo.toml`, `loco-ui/Cargo.toml`;
- the `version = ".."` on the path dependencies in `loco-ui/Cargo.toml`
  (`loco-ui-caps`, `loco-ui-macros`) and anywhere else `grep -rn 'version = "OLD"'` finds one
  (the installer's templates under `loco-ui/loco-templates/`, `docs/`, the README's install
  lines);
- `cargo update -w` so `Cargo.lock` agrees.

## 4. Regenerate and check

```sh
cargo run -p demo -- spec write          # spec/components.json + README feature matrix
scripts/verify.sh                        # fmt, clippy, tests, <script> grep
cargo package -p loco-ui-caps -p loco-ui-macros -p loco-ui --allow-dirty --no-verify --list > /dev/null
cargo package -p loco-ui-caps -p loco-ui-macros -p loco-ui --allow-dirty
RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --no-deps --all-features -p loco-ui -p loco-ui-caps
```

- `cargo package` fails if a path dependency lacks a version or a packaged file is missing.
  Check the file lists: `loco-ui/src/_template.rs.txt` and `tests/fresh-loco-app` must not be
  in it, each crate carries its `LICENSE` and README.
- The nightly doc build needs the `docsrs` cfg spelling the crates use (`doc_cfg`); a
  failure here fails on docs.rs too.
- If `cargo-semver-checks` is installed: `cargo semver-checks -p loco-ui` against the last
  published version (none before 0.2.0: skip and say so).

## 5. Commit and stop

Commit as `Release X.Y.Z: changelog, versions, spec`. Do not tag on a remote and do not push.
Report:

1. The version, the CHANGELOG section's bullet count, and any Breaking bullets.
2. The output of each check above (pass/fail, with the first error).
3. **"Stopped before publish."** and the commands the owner runs after saying yes, in order:
   `cargo publish -p loco-ui-caps`, `cargo publish -p loco-ui-macros`,
   `cargo publish -p loco-ui`, then `git tag vX.Y.Z && git push --tags`.
