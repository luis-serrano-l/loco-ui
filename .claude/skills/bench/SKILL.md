---
name: bench
description: How to measure loco-ui performance and report it - criterion render benches (`cargo bench -p loco-ui`), the demo latency baseline (`scripts/bench.sh`) and click-to-paint of in-place updates (`scripts/bench-swap.mjs`). Use when a change claims to be faster, when adding or reviewing a benchmark, or before optimizing anything.
---

# Benchmarks

Method adapted from Zed's `gpui-bench` skill. The benches here:

| Tool | Measures |
|---|---|
| `cargo bench -p loco-ui` (`loco-ui/benches/render.rs`) | stylesheet, whole page, 1 000-row table, paged table, `UiState` parsing |
| `scripts/bench.sh [samples]` | p50/p95 time to first byte and full response of the release demo on port 3001, plus Firefox navigation timing |
| `node scripts/bench-swap.mjs [runs]` | click-to-paint of in-place updates: `enhance.js` beside htmx 2 on the same answers |

Results go in `FINDINGS.md` as a before → after table with what changed (see the M16 section).

## Start with the question

Before touching code, write down:

1. What is slow, as a user or caller sees it: time to first byte, render of a big table,
   swap latency, stylesheet size?
2. Which bench already covers it, or which input it needs (a realistic size, plus one severe
   size that shows how it scales).
3. What output proves the work is still complete (the rendered length, a row count, a
   substring). A faster result that drops work is not an improvement: assert it.

## Rules

- Measure before optimizing. In order: do less work (render fewer times, stop copying a whole
  `Markup`), pick a better algorithm or data structure, then cut allocations and clones
  (`String::with_capacity`, one buffer instead of many). Micro-tuning comes last.
- Add or extend the bench **before** changing the code, and prove it shows the problem on the
  baseline.
- Same bench code on baseline and candidate. Put the bench in its own commit, or run the
  baseline in a separate worktree (`git worktree add ../loco-ui-base <commit>`).
- Same machine, profile, features and lockfile for both runs. Close the demo server and heavy
  builds first. On a laptop, stay on the same power source.
- Smoke first: `cargo bench -p loco-ui -- --test` runs every bench once without measuring.
  Then a bounded measured run; `-- --quick` or a name filter (`-- "table 1000"`) keeps it
  short.
- Every bench command an agent runs has a hard timeout of at most five minutes.
- Never run two measured benchmarks at the same time, and never while `cargo` is building
  something else: they steal CPU from each other.
- If results are noisy, alternate baseline and candidate runs instead of doing all the
  baseline runs first.
- Never invent or estimate a number. Report what the tool printed.

## Reporting

Lead with whether the problem showed up on the baseline and whether the change fixed it. Then:

1. Commits compared, the exact command, machine (CPU, cores) and sample count.
2. Raw before and after numbers with criterion's confidence interval or the p50/p95 from
   `bench.sh`, and the percentage change.
3. Any regression elsewhere, stated plainly, even when small.
4. What is still expensive and what to look at next.

## Review checklist

- [ ] The bench states what it measures and why that matters to a user.
- [ ] Setup (building rows, parsing a request) is outside the timed closure unless setup is
      the thing measured.
- [ ] Inputs are fixed and realistic, and include a large case.
- [ ] Output completeness is asserted.
- [ ] The same bench code ran on baseline and candidate.
- [ ] Numbers in the report match the tool output and disclose regressions.

## References

- [Criterion: analysis](https://bheisler.github.io/criterion.rs/book/analysis.html)
- [Criterion: command-line options](https://bheisler.github.io/criterion.rs/book/user_guide/command_line_options.html)
- [The Rust Performance Book](https://nnethercote.github.io/perf-book/)
