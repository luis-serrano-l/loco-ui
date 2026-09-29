---
name: rust-practices
description: Rust coding rules for loco-ui library, demo and example code - error handling without panics, when `let _ =` is allowed, naming, comments, clones in async code, where new code goes, and how to add a rule to CLAUDE.md. Use when writing or reviewing Rust in this repo.
---

# Rust practices

Adapted from the agent rules in the Zed repository (`.rules`), trimmed to what applies here.
`CLAUDE.md` wins where the two disagree.

## Priorities

Correctness and clarity first. Speed only when a benchmark shows it matters (see the `bench`
skill), and then measure before and after.

## Errors and panics

- Library code (`loco-ui/src`, `loco-ui-caps/src`, `loco-ui-macros/src`) outside tests and
  doctests does not call `unwrap()`, `expect()` or index a slice or map in a way that can
  panic. Propagate with `?`, or fall back to a sensible default the way components already do
  when a query parameter or cookie does not parse: a bad request renders the default state, it
  never crashes the handler.
- Use `.get(i)`, `.first()`, `split_once` and friends instead of `[i]` on input from a request.
- Tests, doctests, benches and `build.rs` may `unwrap()`: a panic there is a test failure.
- `let _ =` is only for operations that cannot fail in practice. `write!` into a `String`
  (`fmt::Write` for `String` never returns `Err`) is the common case in this repo and is fine.
  Never write `let _ =` in front of I/O, a network call, a database call or a send on a
  channel: propagate it, or handle the `Err` explicitly.
- Do not hide a failure behind `let _ = x?;`. If the value is unused, write `x?;`.
- In the Loco example app, errors reach the user as a page or a flash message, not as a log
  line nobody reads.

## Names and comments

- Full words for names: `queue`, not `q`; `request`, not `req`, in new code. Short names are
  fine for closure arguments over an obvious iterator (`|c|` for a char).
- Comments say *why*: a platform quirk, a browser version, a Blitz gap (with its `FINDINGS.md`
  entry), a choice that looks wrong but is not. Do not write comments that restate what the
  next line does or label sections of a function.
- The `//!` header of a component file is documentation, not a comment: it follows the
  component conventions in `CLAUDE.md`.

## Clones in async code

When a closure or `async move` block needs its own copy, shadow the name inside a block so the
clone's scope is obvious and the original stays usable:

```rust
tokio::spawn({
    let state = state.clone();
    async move {
        state.refresh().await;
    }
});
```

## Where code goes

- Put new functionality in an existing file unless it is a new logical piece. In this repo a
  new component is always its own file (`loco-ui/src/<name>.rs`), and a new demo route group
  is its own `demo/src/routes/<group>.rs`; anything smaller joins the file it belongs to.
- Do not add creative extras nobody asked for: no bonus setters, options or demo pages.

## Adding a rule to CLAUDE.md or a skill

These files are read by every agent session, so each line has a cost. Add a rule only when it
is:

1. **Non-obvious**: someone who knows the codebase would still get it wrong.
2. **Repeated**: it came up more than once.
3. **Actionable**: a concrete instruction, not a principle.

Rules describe traps to avoid, not maps of the code: module layouts and data flow go stale
and an agent can read them from the source. Editing or clarifying an existing rule is always
welcome.
