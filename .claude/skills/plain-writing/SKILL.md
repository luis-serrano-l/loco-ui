---
name: plain-writing
description: Remove signs of machine-written prose from loco-ui docs - README, ROADMAP "Done:" notes, CHANGELOG, FINDINGS, component `//!` headers, commit messages. Use after drafting any of these, or when asked to review or tidy their wording.
---

# Plain writing

Adapted from Zed's `humanizer` skill, which is based on Wikipedia's
[Signs of AI writing](https://en.wikipedia.org/wiki/Wikipedia:Signs_of_AI_writing). Zed's
version also asks for personality and opinions; that part is dropped here. Library docs should
be plain, exact and short, and read well for both a person and an agent.

## Process

1. Read the draft and mark every pattern below.
2. Rewrite those parts. Keep every fact, name, version number and link.
3. Ask: "what here still sounds machine-written?" and fix that too.
4. Check the result against the code: a smoother sentence that is now wrong is worse.

## Patterns to remove

**Inflation**
- Significance words: *crucial, pivotal, vital, key, robust, powerful, seamless, cutting-edge*.
  Say what the thing does instead.
- Promotional tone: *effortlessly, elegant, delightful*. Show it with an example.
- Fake depth: a trailing *-ing* clause that adds nothing (*"..., ensuring a smooth
  experience"*, *"..., highlighting its flexibility"*). Delete it.
- A generic upbeat closing sentence (*"With this, you're ready to build great apps"*).

**Word choices**
- *serves as, stands as, acts as* when *is* works.
- *delve, leverage, utilize, facilitate, harness, showcase, underscore, landscape, realm,
  tapestry, journey*. Use the plain word: *use, show, help*.
- Synonym cycling: calling one thing *the component*, *the element*, *the widget* and *the
  control* in four sentences. Pick one name and keep it; in this repo use the name the code
  uses.
- *Not just X, but Y* and *It's not X, it's Y* constructions.
- Triplets by reflex: three adjectives or three examples when one is enough.
- False ranges: *from simple forms to complex dashboards*.

**Formatting**
- Em dashes as the default punctuation. Use a comma, colon, parentheses or a new sentence.
- Bold scattered through a paragraph. Bold a term once where it is defined, if at all.
- Bullet lists where each item is `**Label:** sentence`. Write a paragraph or a table.
- Title Case Headings. Use sentence case, like the rest of this repo.
- Emojis. Curly quotes in code-adjacent text (use straight `"` and `'`).

**Chat leftovers**
- *Great question, Certainly!, I hope this helps, Let me know if...*
- Knowledge-cutoff hedges (*as of my last update*). State the version and date instead.
- Stacked hedges (*may potentially help in some cases*). Say it or leave it out.
- Filler: *it's worth noting that, it is important to remember, in order to, at the end of the
  day*. Delete; the sentence still says the same thing.
- Vague sources: *experts say, it is widely considered*. Name the spec, browser version or
  issue, or drop the claim.

## What good looks like here

- The first sentence says what the thing does.
- Numbers, versions and browser baselines instead of adjectives (*Firefox 139+* not *modern
  browsers*).
- Short sentences, one idea each; a longer one where the idea needs it.
- Code identifiers in backticks, spelled exactly as in the code.
- The fallback and the limits are stated as plainly as the features.
