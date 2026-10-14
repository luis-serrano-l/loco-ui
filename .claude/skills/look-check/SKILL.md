---
name: look-check
description: Compare a loco-ui demo page with the library it copies - `scripts/look.sh --only <name>` shoots light and dark at 1280, 768 and 420 plus the reference page, then read each PNG, list the differences, and delete the shots. Use after changing a component's CSS or markup, before ticking a component box, or when asked how a page looks.
---

# Look check

`scripts/look.sh` drives headless Firefox against the debug demo on port 3009. Nothing in it
is a test: you are the comparison.

## Steps

1. Build the demo: `cargo build -p demo` (look.sh runs `target/debug/demo`, it does not
   build).
2. Find the page name in the `pages` list at the top of `scripts/look.sh`
   (`name|path|reference URL`). A new page needs a line there first. The name is not always
   the route: `/otp` is `otp`, `/form?errors=1` is `form-errors`.
3. Shoot it:
   ```sh
   scripts/look.sh --only <name>                 # into target/look/
   scripts/look.sh --only <name> --tag before    # into target/look/before/, to compare later
   scripts/look.sh --only <name> --no-ref        # skip the reference page
   ```
   It writes `<name>-{light,dark}-{1280,768,420}.png` and `<name>-ref.png` (the reference at
   1280, light). Each shot can take up to 90 seconds; give the command a ten-minute timeout.
4. Read every PNG with the Read tool: the six of ours and the reference. Check:
   - **Against the reference:** heights, paddings, radii, gaps, font sizes and weights,
     borders, shadows, the colour step of each state (hover and focus do not show in a still
     shot; say so instead of guessing).
   - **Dark:** contrast of muted text and borders, no light-mode colour left behind, tints
     that read on the dark surface.
   - **420 wide:** nothing overflows sideways (wide content scrolls inside its own box),
     text wraps or truncates, the layout steps down to its narrow form.
   - **768 wide:** the medium container layout, if the component has one.
5. List the deltas, one line each: what differs, at which width and theme, and whether it is
   a fix to make or a deliberate difference (that reason goes in the roadmap "Done:" note).
6. Delete the shots: `rm -rf target/look`. Keep a `--tag review` set only when the owner asked
   to see it, and say where it is.

## Notes

- The first visit in each profile teaches the server the browser's capabilities; look.sh
  makes it before shooting, so the shots are the modern variant. For the fallback variant,
  read the Blitz shots in `tests/shots/<page>-fallback.png` instead.
- Firefox here is a snap: output paths must be absolute and inside the project, which
  look.sh already does. Do not move the output to `/tmp`.
- Reduced motion is on in both profiles, so a dialog or drawer is never caught mid-animation.
- A reference page that changed or moved shows a 404 or a cookie banner: say so and compare
  with what you know of the library.
