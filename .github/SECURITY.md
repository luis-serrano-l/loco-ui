# Security policy

## Supported versions

loco-ui is not on crates.io yet. Until the first release (0.2.0), only the `main` branch
gets fixes; after it, the latest minor version does.

## Reporting a vulnerability

Do not open a public issue. Use GitHub's private vulnerability reporting on this repository
(Security, then "Report a vulnerability") and include the version or commit, the route or
component, and the request that shows the problem.

You will get an answer within a week. A confirmed issue is fixed on `main` first and then
noted in `CHANGELOG.md`.

Examples of what counts: HTML or attribute injection through a component, a cookie or query
parser that panics or accepts a forged value, a `Content-Security-Policy` the enhancement
script breaks, and a form helper that allows a state change over GET.
