//! # Enhancement script
//!
//! One small optional script that makes the same markup feel instant. Every page works
//! identically with it blocked, disabled or not yet loaded: it only intercepts what the
//! browser would otherwise do as a full navigation and does it in place.
//!
//! **Platform features:** `<script defer>` (baseline 2010), `fetch` (Chrome 42, Firefox 39,
//! Safari 10.1), `DOMParser` (baseline 2010), `history.pushState` (Chrome 5, Firefox 4,
//! Safari 5), `document.startViewTransition` where present (Chrome 111, Firefox 144,
//! Safari 18) so swapped parts morph when the answer took over 150 ms (a faster one lands at
//! once, a frame sooner; a root marked `data-lui-morph` always morphs), `CustomEvent` (Chrome 15, Firefox 11, Safari 6) for
//! `lui:swap`.
//!
//! **Fallback:** none needed. Without the script every form and link is a normal navigation;
//! the Blitz test suite renders every route with no script engine at all.
//!
//! **Contract:** an element with an `id` and `data-lui="swap"` is a swap root. Submitting a
//! form or following a same-origin link inside it fetches the response, parses it, and
//! replaces the root with the element of the same `id` from the new document. The flash
//! messages, the toast list, `<title>` and `data-theme` are synced too, and the URL follows the response.
//! A form or link anywhere may name its root instead with `data-lui-target="#id"`, and
//! `data-lui-swap="outer|inner|append|prepend"` (default `outer`) says how the new element
//! lands: replace the root, replace its children, or add them at the end or the start. The
//! request is the same either way, so the server may answer an enhanced request (headers
//! `Lui-Enhance: 1` and `Accept: text/html`) with only the fragment it needs to; without the
//! script it is a full navigation to the same URL. `slim` (feature `axum`) does that for every page: an
//! enhanced request gets the page without its inline stylesheet, which the document already
//! has, and every HTML answer says `Vary: Lui-Enhance, Cookie`. User actions fetch with
//! `priority: "high"`. Links inside an element marked `data-lui-prefetch` are fetched at low
//! priority on hover or focus, and a click within five seconds reuses that answer. A response may also carry elements marked `data-lui-oob`
//! (out of band): each replaces the element of the same `id` anywhere in the page, in the
//! mode the attribute names (`outer` by default), and is dropped from the main swap; the
//! full page without the script already shows them in place. While a request is in flight
//! the root and the form carry `data-lui-busy` and `aria-busy="true"`, the form's submit
//! buttons are disabled, and an element named by `data-lui-indicator="#id"` (authored with
//! `hidden`) is shown; the `--lui-busy` property sets how much the busy root fades. A request
//! that fails becomes the plain navigation the browser would have made, so the server's
//! answer is always seen. The URL follows the response (links push a history entry, forms
//! replace it); `data-lui-push="false"` keeps the URL as it is and `data-lui-replace` always
//! replaces. Every swap stores a copy of the roots in the history entry, so Back and Forward
//! restore them without a request. After every swap the root dispatches a bubbling `lui:swap`
//! event with `{ id, url, mode }` in `detail`.
//! Rapid actions on one root are queued, so a counter clicked five times counts five. The
//! script also mirrors `<input type=range>` and `type=color` values while they move, counts
//! characters into the `<output for>` of a field with `maxlength`, sends multipart forms as
//! `FormData` so files survive (through `XMLHttpRequest`, filling a `<progress data-lui-progress>`
//! inside the form as the upload goes out, when the form has one), opens
//! the `:target` dialog fallback as a real modal, moves through an open popover menu with the
//! arrow keys, searches a combobox as you type and walks its results with the arrow keys.
//! After a swap, an element marked `autofocus` in the new markup takes the focus (a form sent
//! back with an error summary starts there, as it would on a page load); otherwise the
//! element that had the focus gets it back, and when that element is gone the focus moves to
//! the new root's `h1`, or the root itself (`tabindex="-1"`), instead of falling to `<body>`.
//! A swap that changed the URL or the title says the new `document.title` through a
//! `role="status"` region the script adds to the page, as a screen reader hears a page load.
//! A tab title opens its panel and slides the underline on the click itself; the server's
//! answer replaces the strip quietly once the slide ends (a lazy panel fills in then). A
//! sortable list's item drags by its grip (HTML drag and drop); the drop posts the item's
//! own move form with its new index.
//!
//! [`script_tag`] goes at the end of `<body>`; `router` (feature `axum`) serves the file with a content
//! hash in the URL so it caches forever. It is compatible with `script-src 'self'`.
//!
//! ```rust
//! use loco_ui::enhance;
//! let tag = enhance::script_tag().into_string();
//! assert!(tag.starts_with("<script src=\"/lui/enhance.js?v="));
//! ```

use maud::{Markup, html};

/// Path the script is served from. [`script_url`] appends a content hash.
pub const SCRIPT_PATH: &str = "/lui/enhance.js";

/// The whole enhancement script. Plain ES2020, no build step, about 12 KB as [`served`].
pub const JS: &str = r##"(function () {
"use strict";
var roots = "[data-lui=swap]", queue = {}, cache = {};
// Every request says it is the script's (the server may answer with less) and wants HTML.
var init = function (priority) { return { credentials: "same-origin", priority: priority || "high", headers: { "Lui-Enhance": "1", Accept: "text/html" } }; };
var load = function (url, req) {
  // fetch cannot report upload progress: a form with a <progress data-lui-progress> posts
  // through XMLHttpRequest and fills the bar as the body goes out.
  if (req.progress) return new Promise(function (ok, fail) {
    var x = new XMLHttpRequest(), bar = req.progress;
    x.open(req.method, url);
    for (var h in req.headers) x.setRequestHeader(h, req.headers[h]);
    x.upload.onprogress = function (e) { bar.max = e.total; bar.value = e.loaded; };
    x.onload = function () { ok({ url: x.responseURL, html: x.responseText }); };
    x.onerror = fail;
    bar.hidden = false;
    x.send(req.body);
  });
  return fetch(url, req).then(function (res) { return res.text().then(function (html) { return { url: res.url, html: html }; }); });
};
var parse = function (html) { return new DOMParser().parseFromString(html, "text/html"); };
var reduced = matchMedia("(prefers-reduced-motion: reduce)").matches;

function focusState() {
  var a = document.activeElement;
  if (!a || a === document.body) return null;
  var key = a.id ? "#" + a.id : (a.name ? "[name='" + a.name + "']" : null);
  return key && { key: key, pos: a.selectionEnd };
}
function restoreFocus(f) {
  var el = f && document.querySelector(f.key);
  if (!el) return;
  el.focus({ preventScroll: true });
  if (f.pos != null && el.setSelectionRange) try { el.setSelectionRange(f.pos, f.pos); } catch (e) {}
}

function place(root, fresh, mode) {
  var kids = Array.prototype.slice.call(fresh.childNodes);
  if (mode === "inner") root.replaceChildren.apply(root, kids);
  else if (mode === "append") root.append.apply(root, kids);
  else if (mode === "prepend") root.prepend.apply(root, kids);
  else { root.replaceWith(fresh); return fresh; }
  return root;
}

// Copy of every swap root, kept in the history entry so Back and Forward restore in place.
function snapshot() {
  var lui = {};
  document.querySelectorAll(roots).forEach(function (r) { if (r.id) lui[r.id] = r.outerHTML; });
  return { lui: lui };
}
function swapped(el, url, mode) {
  history.replaceState(snapshot(), "", location.href);
  el.dispatchEvent(new CustomEvent("lui:swap", { bubbles: true, detail: { id: el.id, url: url, mode: mode } }));
}

// hist: "push" adds a history entry, "replace" rewrites the current one, "none" keeps the URL.
// fast: the answer came within 150 ms, so it lands at once instead of in a view transition
// (whose old-state capture costs a frame); a root marked data-lui-morph always morphs.
// A role=status region the script owns: after a swap that moved to another URL or title, it
// says the new title, as a page load would.
var said;
function announce(text) {
  if (!said) { said = document.createElement("div"); said.setAttribute("role", "status"); said.className = "lui-sr"; document.body.append(said); }
  said.textContent = "";
  setTimeout(function () { said.textContent = text; }, 50);
}
// Focus that was inside the replaced markup lands on the new root's h1, else the root.
function settle(anchor) {
  var a = document.activeElement;
  if (a && a !== document.body && a.isConnected) return;
  var to = anchor.querySelector("h1") || anchor;
  // A heading or div becomes focusable by script only.
  if (to.tabIndex < 0) to.tabIndex = -1;
  to.focus({ preventScroll: true });
}

function apply(doc, id, url, hist, mode, quiet, fast) {
  var root = document.getElementById(id), fresh = doc.getElementById(id);
  if (!root || !fresh) { if (url !== location.href) location.href = url; else location.reload(); return; }
  var f = focusState(), from = location.href, title = document.title;
  var swap = function () {
    doc.querySelectorAll("[data-lui-oob]").forEach(function (el) {
      var here = el.id && el !== fresh && document.getElementById(el.id), how = el.getAttribute("data-lui-oob");
      el.remove(); el.removeAttribute("data-lui-oob");
      if (here) place(here, el, how || "outer");
    });
    var anchor = place(root, fresh, mode);
    [".lui-flash", ".lui-toasts"].forEach(function (s, i) {
      var o = document.querySelector(s), n = doc.querySelector(s);
      if (o && n) o.replaceWith(n); else if (o) o.remove(); else if (n) i ? document.body.append(n) : anchor.before(n);
    });
    if (doc.title) document.title = doc.title;
    var theme = doc.documentElement.getAttribute("data-theme");
    if (theme) document.documentElement.setAttribute("data-theme", theme);
    var af = anchor.querySelector("[autofocus]");
    if (af) af.focus(); else { restoreFocus(f); settle(anchor); }
    if (document.title !== title || url !== from) announce(document.title);
    swapped(anchor, url, mode);
  };
  if (hist !== "none" && url !== location.href) {
    if (hist === "push") { history.replaceState(snapshot(), "", location.href); history.pushState(null, "", url); }
    else history.replaceState(null, "", url);
  }
  if (!quiet && document.startViewTransition && !reduced && (!fast || root.closest("[data-lui-morph]"))) document.startViewTransition(swap); else swap();
}

// The root an element acts on: the one named by data-lui-target on it or an ancestor, else
// the closest swap root. data-lui-swap picks the mode; data-lui-push="false" and
// data-lui-replace pick what happens to the URL (links push by default, forms replace).
function target(el, hist) {
  var t = el.closest("[data-lui-target]");
  var root = t ? document.querySelector(t.getAttribute("data-lui-target")) : el.closest(roots);
  if (!root || !root.id) return null;
  var m = el.closest("[data-lui-swap]"), p = el.closest("[data-lui-push]");
  if (p && p.getAttribute("data-lui-push") === "false") hist = "none";
  else if (el.closest("[data-lui-replace]")) hist = "replace";
  return { id: root.id, mode: m ? m.getAttribute("data-lui-swap") : "outer", hist: hist };
}

var pending = {};
// Busy marks on the root and the source (form or link): attributes for CSS and assistive
// tech, submit buttons disabled, the named indicator shown. Everything is undone after.
function busy(t, src, on) {
  var root = document.getElementById(t.id);
  [root, src].forEach(function (el) {
    if (!el) return;
    if (on) { el.setAttribute("data-lui-busy", ""); el.setAttribute("aria-busy", "true"); }
    else { el.removeAttribute("data-lui-busy"); el.removeAttribute("aria-busy"); }
  });
  if (src.tagName === "FORM") src.querySelectorAll(on ? "button:not([type=button]):not(:disabled),input[type=submit]:not(:disabled)" : "[data-lui-disabled]").forEach(function (b) {
    b.disabled = on; if (on) b.setAttribute("data-lui-disabled", ""); else b.removeAttribute("data-lui-disabled");
  });
  var i = src.closest("[data-lui-indicator]"), ind = i && document.querySelector(i.getAttribute("data-lui-indicator"));
  if (ind) ind.hidden = !on;
}

function request(t, src, url, req, fallback) {
  var id = t.id;
  pending[id] = (pending[id] || 0) + 1;
  busy(t, src, true);
  var run = function () {
    var hit = !req.method && cache[url], at = Date.now();
    delete cache[url];
    return (hit && Date.now() - hit.at < 5000 ? hit.got : load(url, req)).then(function (r) {
      var fast = Date.now() - at < 150;
      return Promise.resolve(t.quiet).then(function () { apply(parse(r.html), id, r.url, t.hist, t.mode, t.quiet, fast); });
    }).then(function () { done(); }, function () { done(); fallback(); });
  };
  var done = function () { pending[id]--; busy(t, src, false); if (pending[id]) busy(t, src, true); };
  queue[id] = (queue[id] || Promise.resolve()).then(run, run);
  return queue[id];
}

// A tab title opens its panel and moves the underline before the request; the answer
// replaces the strip once it has slid. Returns false, or a promise for the slide's end.
function openTab(a) {
  var l = a.closest(".lui-tabs-list");
  if (!l || a.dataset.state !== "inactive") return false;
  var ts = [...l.children], n = ts.indexOf(a), m = l.querySelector(".lui-tabs-mark");
  var show = function () {
    ts.forEach(function (t, i) { t.dataset.state = i === n ? "active" : "inactive"; l.parentNode.querySelectorAll(":scope>.lui-tabs-panel")[i].hidden = i !== n; });
    if (m) a.prepend(m);
  };
  if (document.startViewTransition && !reduced) return document.startViewTransition(show).finished;
  show();
  return true;
}

function submit(form, submitter) {
  var t = target(form, "replace");
  if (!t) return false;
  var data = new FormData(form);
  if (submitter && submitter.name) data.append(submitter.name, submitter.value);
  var at = function (a) { return submitter && submitter.getAttribute("form" + a) || form.getAttribute(a); };
  var url = new URL(at("action") || location.href, location.href);
  var req = init();
  var params = new URLSearchParams(data);
  if ((at("method") || "get").toLowerCase() === "post") { req.method = "POST"; req.body = form.enctype === "multipart/form-data" ? data : params; }
  else url.search = params.toString();
  if (req.body === data) req.progress = form.querySelector("progress[data-lui-progress]");
  request(t, form, url.href, req, function () { HTMLFormElement.prototype.submit.call(form); });
  return true;
}

document.addEventListener("submit", function (e) {
  if (e.defaultPrevented || e.target.method === "dialog") return;
  if (submit(e.target, e.submitter)) e.preventDefault();
});

document.addEventListener("click", function (e) {
  var a = e.target.closest("a[href]");
  if (!a || e.button || e.metaKey || e.ctrlKey || e.shiftKey || a.target) return;
  var d = a.closest(".lui-dialog");
  if (d && a.hash && a.hash.length > 1 && a.hash === "#" + (d.querySelector("dialog") || {}).id) {
    var dialog = d.querySelector("dialog");
    if (dialog.showModal) { e.preventDefault(); dialog.showModal(); }
    return;
  }
  if (d && a.getAttribute("href") === "#" && d.querySelector("dialog[open]")) {
    e.preventDefault(); d.querySelector("dialog").close(); return;
  }
  var t = target(a, "push");
  if (!t || a.origin !== location.origin) return;
  e.preventDefault();
  t.quiet = openTab(a);
  request(t, a, a.href, init(), function () { location.href = a.href; });
});

// data-lui-prefetch on a link or an ancestor: a hover or focus fetches the link's answer at
// low priority (not the page already shown, nor the tab already open), and a click within
// five seconds uses it instead of asking again.
["mouseover", "focusin"].forEach(function (type) {
  document.addEventListener(type, function (e) {
    var a = e.target.closest && e.target.closest("a[href]");
    if (!a || a.href === location.href || a.origin !== location.origin || !a.closest("[data-lui-prefetch]") || a.dataset.state === "active" || !target(a, "push")) return;
    var hit = cache[a.href];
    if (hit && Date.now() - hit.at < 5000) return;
    var got = load(a.href, init("low"));
    got.catch(function () { delete cache[a.href]; });
    cache[a.href] = { at: Date.now(), got: got };
  });
});

// The narrow-screen tab select submits on change (its Go button stays for everyone else).
document.addEventListener("change", function (e) {
  if (e.target.matches(".lui-tabs-select select") && e.target.form) submit(e.target.form, null);
});

var typing;
document.addEventListener("input", function (e) {
  var t = e.target, out = t.id && document.querySelector("output[for='" + t.id + "']");
  if (out) out.textContent = t.maxLength > 0 ? t.value.length + " / " + t.maxLength : t.value;
  if (t.type === "range") {
    var p = function (r) { return (r.value - r.min) * 100 / (r.max - r.min || 1); }, k = t.closest(".lui-range-track");
    if (k) { var v = [].map.call(k.querySelectorAll("input"), p).sort(function (a, b) { return a - b; });
      k.style.cssText = "--lui-range-lo:" + v[0] + "%;--lui-range-hi:" + v[1] + "%"; }
    else t.style.setProperty("--lui-range-fill", p(t) + "%");
  }
  if (t.classList.contains("lui-otp-input")) t.style.setProperty("--lui-otp-at", Math.min(t.value.length, t.maxLength - 1));
  if (t.type === "color") {
    var box = t.closest(".lui-color");
    if (box) {
      var sw = box.querySelector(".lui-color-swatch"), code = box.querySelector("code");
      if (sw) sw.style.setProperty("--lui-color-value", t.value);
      if (code) code.textContent = t.value;
    }
  } else if (t.type === "search" && t.form && t.closest(roots)) {
    clearTimeout(typing);
    // Submit through an adjacent button (a select filter's formmethod=get).
    var b = t.nextElementSibling;
    typing = setTimeout(function () { submit(t.form, b && b.type === "submit" ? b : null); }, 150);
  }
});

// Arrow keys walk the items of the open menu the focus is in (or that this button opened),
// and the combobox input plus its results.
document.addEventListener("keydown", function (e) {
  if (e.key !== "ArrowDown" && e.key !== "ArrowUp") return;
  var box = e.target.closest(".lui-popover"), menu = e.target.closest(".lui-popover nav"), items;
  if (!menu && box) menu = box.querySelector("nav:popover-open, details[open] > nav");
  var combo = e.target.closest(".lui-combobox");
  if (combo) items = Array.prototype.slice.call(combo.querySelectorAll("input[type=search], .lui-combobox-list a[href]"));
  else if (menu) items = Array.prototype.filter.call(menu.querySelectorAll("a[href], button:not(:disabled), summary"), function (el) {
    return el.closest("nav") === menu;
  });
  else return;
  if (!items.length) return;
  e.preventDefault();
  var i = items.indexOf(e.target), n = items.length;
  items[i < 0 ? (e.key === "ArrowDown" ? 0 : n - 1) : (i + (e.key === "ArrowDown" ? 1 : n - 1)) % n].focus();
});

document.addEventListener("click", function (e) {
  document.querySelectorAll("details.lui-popover-details[open]").forEach(function (d) {
    if (!d.contains(e.target)) d.removeAttribute("open");
  });
});

// Sortable list: a drag starts only from the grip. The item follows the pointer through its
// list; a drop posts the item's own form with to=<new index> (a hidden submit button), and a
// drag that ends anywhere else puts the item back.
var drag, from, dropped, near = function (e) { return e.target.closest && e.target.closest(".lui-sortable-item"); };
document.addEventListener("pointerdown", function (e) { if (e.target.closest && e.target.closest(".lui-sortable-grip")) near(e).draggable = true; });
document.addEventListener("dragstart", function (e) {
  var li = near(e);
  if (!li || !li.draggable) return;
  drag = li; from = li.nextElementSibling; dropped = 0;
  e.dataTransfer.setData("text/plain", "");
  li.setAttribute("data-lui-dragging", "");
});
document.addEventListener("dragover", function (e) {
  var o = drag && near(e), r;
  if (!o || o.parentNode !== drag.parentNode) return;
  e.preventDefault(); r = o.getBoundingClientRect();
  if (o !== drag) o[e.clientY > r.top + r.height / 2 ? "after" : "before"](drag);
});
document.addEventListener("drop", function (e) { if (drag) { e.preventDefault(); dropped = 1; } });
document.addEventListener("dragend", function () {
  var li = drag, l = li && li.parentNode, f, b;
  if (!li) return;
  drag = null; li.draggable = false; li.removeAttribute("data-lui-dragging");
  if (!dropped) return l.insertBefore(li, from);
  if (li.nextElementSibling === from) return;
  f = li.querySelector("form"); b = document.createElement("button");
  b.name = "to"; b.value = [].indexOf.call(l.children, li); b.hidden = true;
  f.append(b); f.requestSubmit(b); b.remove();
});

// Back and Forward: the entry's stored copy of each root when there is one, else a fetch.
addEventListener("popstate", function (e) {
  var lui = e.state && e.state.lui;
  if (lui) {
    Object.keys(lui).forEach(function (id) {
      var root = document.getElementById(id), fresh = parse(lui[id]).getElementById(id);
      if (root && fresh) { root.replaceWith(fresh); swapped(fresh, location.href, "outer"); }
    });
    return;
  }
  load(location.href, init())
    .then(function (r) {
      var doc = parse(r.html);
      document.querySelectorAll(roots).forEach(function (r) { apply(doc, r.id, location.href, "none", "outer"); });
    });
});
})();
"##;

/// [`JS`] as served: comment lines and indentation dropped, nothing else touched. The budget
/// (13 KB) applies to this; the source keeps its comments for the reader.
pub fn served() -> &'static str {
    static SERVED: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    SERVED.get_or_init(|| {
        JS.lines()
            .map(str::trim_start)
            .filter(|l| !l.is_empty() && !l.starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n")
    })
}

/// A strict `Content-Security-Policy` for pages that carry the enhancement script: scripts
/// only from this origin (the script is a same-origin file, never inline), no plugins, no
/// `<base>`, no framing, forms posting only here. Styles need `'unsafe-inline'`: the
/// stylesheet is inlined once per page and a few per-element custom properties travel in
/// `style` attributes. `csp` (feature `axum`) sends it.
pub const CSP: &str = "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; object-src 'none'; base-uri 'none'; form-action 'self'; frame-ancestors 'none'";

/// [`CSP`] with `script-src 'none'`, for a page built with `Page::without_script`.
pub const CSP_NO_SCRIPT: &str = "default-src 'self'; script-src 'none'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; object-src 'none'; base-uri 'none'; form-action 'self'; frame-ancestors 'none'";

/// The response extension `Page::without_script` leaves for `csp` (feature `axum`).
#[derive(Clone, Copy, Debug)]
pub struct NoScript;

/// The response extension a `Page::linked_stylesheet` page leaves for [`csp`]: the
/// `'sha256-…'` sources of the `<style>` elements it still inlines, space-separated.
#[derive(Clone, Debug)]
pub struct StyleHashes(pub String);

/// [`CSP`] for a page whose stylesheet is linked: `<style>` elements only by `hashes`
/// (from [`csp_hash`]), `style` attributes still allowed (`style-src-attr`, Chrome 75,
/// Firefox 108, Safari 15.4), since a few per-element custom properties travel in them.
pub fn csp_linked(hashes: &str, script: bool) -> String {
    let script = if script { "'self'" } else { "'none'" };
    let space = if hashes.is_empty() { "" } else { " " };
    format!(
        "default-src 'self'; script-src {script}; style-src 'self'{space}{hashes}; style-src-attr 'unsafe-inline'; img-src 'self' data:; object-src 'none'; base-uri 'none'; form-action 'self'; frame-ancestors 'none'"
    )
}

/// `'sha256-<base64>'` of `text`, a CSP source that allows a `<style>` (or `<script>`)
/// element with exactly that content. For an inline page, `csp_hash(loco_ui::stylesheet())`
/// plus the hashes of `Page::style_hashes` replace `'unsafe-inline'` in a policy of your own.
///
/// ```rust
/// assert_eq!(loco_ui::enhance::csp_hash(""), "'sha256-47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFU='");
/// ```
pub fn csp_hash(text: &str) -> String {
    format!("'sha256-{}'", base64(&sha256(text.as_bytes())))
}

/// SHA-256 (FIPS 180-4), for [`csp_hash`]: the crate takes no dependency for one hash.
fn sha256(data: &[u8]) -> [u8; 32] {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    let mut message = data.to_vec();
    message.push(0x80);
    while message.len() % 64 != 56 {
        message.push(0);
    }
    message.extend_from_slice(&((data.len() as u64).wrapping_mul(8)).to_be_bytes());
    for block in message.chunks_exact(64) {
        let mut w = [0u32; 64];
        for (i, word) in block.chunks_exact(4).enumerate() {
            w[i] = u32::from_be_bytes([word[0], word[1], word[2], word[3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = h;
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let t1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        for (state, add) in h.iter_mut().zip([a, b, c, d, e, f, g, hh]) {
            *state = state.wrapping_add(add);
        }
    }
    let mut out = [0u8; 32];
    for (i, word) in h.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    out
}

/// Standard base64 with padding.
fn base64(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let n = chunk
            .iter()
            .enumerate()
            .fold(0u32, |n, (i, b)| n | u32::from(*b) << (16 - 8 * i));
        for i in 0..4 {
            if i <= chunk.len() {
                out.push(char::from(ALPHABET[(n >> (18 - 6 * i) & 63) as usize]));
            } else {
                out.push('=');
            }
        }
    }
    out
}

/// FNV-1a hash of [`JS`]: the cache-busting version in [`script_url`].
fn version() -> String {
    let hash = JS.bytes().fold(0xcbf2_9ce4_8422_2325_u64, |h, b| {
        (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3)
    });
    format!("{hash:016x}")
}

/// `/lui/enhance.js?v=<hash>`: changes whenever the script does, so it may cache forever.
pub fn script_url() -> String {
    format!("{SCRIPT_PATH}?v={}", version())
}

/// `<script src=… defer>` for the end of `<body>`. [`crate::Ui::page`] includes it.
pub fn script_tag() -> Markup {
    html! { script src=(script_url()) defer {} }
}

/// The page an enhanced request needs: `html` without the `<style>` elements in its `<head>`,
/// since the document making the request already has them. Everything the script reads
/// (swap roots, flash, toasts, out-of-band elements, `<title>`, `data-theme`) is kept.
pub fn slim_html(html: &str) -> String {
    let head_end = html.find("</head>").unwrap_or(0);
    let mut out = String::with_capacity(html.len());
    let mut rest = html;
    let mut done = 0;
    while let Some(start) = rest.find("<style") {
        if done + start >= head_end {
            break;
        }
        let Some(len) = rest[start..].find("</style>") else {
            break;
        };
        out.push_str(&rest[..start]);
        let skip = start + len + "</style>".len();
        done += skip;
        rest = &rest[skip..];
    }
    out.push_str(rest);
    out
}

/// Build a swap-root id from a component prefix and a key such as a form action.
/// `swap_id("lui-counter", "/counter")` is `lui-counter--counter`.
pub fn swap_id(prefix: &str, key: &str) -> String {
    let key: String = key
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect();
    format!("{prefix}-{key}")
}

#[cfg(feature = "axum")]
mod axum_glue {
    use super::{SCRIPT_PATH, served, slim_html};
    use axum::{
        Router,
        body::{Body, HttpBody},
        extract::Request,
        http::{HeaderValue, header},
        middleware::Next,
        response::Response,
        routing::get,
    };

    /// Middleware for the whole router (`.layer(axum::middleware::from_fn(enhance::slim))`):
    /// an HTML answer to an enhanced request (`Lui-Enhance: 1`) loses its inline stylesheet
    /// ([`slim_html`]), and every HTML answer carries `Vary: Lui-Enhance, Cookie`, so a cache or
    /// a `<link rel="prefetch">` never serves one variant, or one cookie's page, for another. Streamed bodies (no known size) pass through untouched.
    pub async fn slim(request: Request, next: Next) -> Response {
        let enhanced = request.headers().contains_key("lui-enhance");
        let mut response = next.run(request).await;
        let html = response
            .headers()
            .get(header::CONTENT_TYPE)
            .is_some_and(|v| v.as_bytes().starts_with(b"text/html"));
        if !html {
            return response;
        }
        response.headers_mut().append(
            header::VARY,
            HeaderValue::from_static("lui-enhance, cookie"),
        );
        if !enhanced || response.body().size_hint().exact().is_none() {
            return response;
        }
        let (mut parts, body) = response.into_parts();
        let Ok(bytes) = axum::body::to_bytes(body, usize::MAX).await else {
            return Response::from_parts(parts, Body::empty());
        };
        let page = slim_html(&String::from_utf8_lossy(&bytes));
        parts.headers.remove(header::CONTENT_LENGTH);
        Response::from_parts(parts, Body::from(page))
    }

    /// Middleware: every HTML answer without a `Content-Security-Policy` of its own gets
    /// [`super::CSP`], or [`super::CSP_NO_SCRIPT`] when it is a `Page::without_script`. Layer it on the
    /// router: `.layer(axum::middleware::from_fn(loco_ui::enhance::csp))`.
    pub async fn csp(request: Request, next: Next) -> Response {
        let mut response = next.run(request).await;
        let html = response
            .headers()
            .get(header::CONTENT_TYPE)
            .is_some_and(|v| v.as_bytes().starts_with(b"text/html"));
        if html
            && !response
                .headers()
                .contains_key(header::CONTENT_SECURITY_POLICY)
        {
            let script = response.extensions().get::<super::NoScript>().is_none();
            let policy = match response.extensions().get::<super::StyleHashes>() {
                Some(hashes) => HeaderValue::from_str(&super::csp_linked(&hashes.0, script)).ok(),
                None if script => Some(HeaderValue::from_static(super::CSP)),
                None => Some(HeaderValue::from_static(super::CSP_NO_SCRIPT)),
            };
            if let Some(policy) = policy {
                response
                    .headers_mut()
                    .insert(header::CONTENT_SECURITY_POLICY, policy);
            }
        }
        response
    }

    /// Serves [`served`] at [`SCRIPT_PATH`] and [`crate::stylesheet`] at
    /// [`crate::layout::STYLESHEET_PATH`], each immutable for a year (their URLs carry a
    /// hash); the stylesheet also answers `If-None-Match` with `304`.
    pub fn router() -> Router {
        Router::new()
            .route(
                SCRIPT_PATH,
                get(|| async {
                    (
                        [
                            (header::CONTENT_TYPE, "text/javascript; charset=utf-8"),
                            (header::CACHE_CONTROL, "public, max-age=31536000, immutable"),
                        ],
                        served(),
                    )
                }),
            )
            .route(
                crate::layout::STYLESHEET_PATH,
                get(|headers: axum::http::HeaderMap| async move {
                    let etag = format!("\"{}\"", crate::layout::stylesheet_etag());
                    let fresh = (headers.get(header::IF_NONE_MATCH))
                        .is_some_and(|v| v.as_bytes() == etag.as_bytes());
                    let status = if fresh {
                        axum::http::StatusCode::NOT_MODIFIED
                    } else {
                        axum::http::StatusCode::OK
                    };
                    let body = if fresh { "" } else { crate::stylesheet() };
                    (
                        status,
                        [
                            (header::CONTENT_TYPE, "text/css; charset=utf-8".to_string()),
                            (
                                header::CACHE_CONTROL,
                                "public, max-age=31536000, immutable".to_string(),
                            ),
                            (header::ETAG, etag),
                        ],
                        body,
                    )
                }),
            )
    }
}
#[cfg(feature = "axum")]
pub use axum_glue::{csp, router, slim};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn script_is_small_and_plain() {
        assert!(
            served().len() < 13312,
            "enhance.js is {} bytes served",
            served().len()
        );
        assert!(
            JS.len() < 16384,
            "enhance.js source is {} bytes; trim before adding comments",
            JS.len()
        );
        assert!(!JS.contains("eval(") && !JS.contains("innerHTML"));
        assert!(script_url().starts_with("/lui/enhance.js?v="));
        assert_eq!(swap_id("lui-form", "/sign-up"), "lui-form--sign-up");
    }

    #[test]
    fn slim_drops_head_styles_only() {
        let page = "<html><head><title>T</title><style>a{}</style><style class=\"lui-tokens\">b{}</style></head><body><style>c{}</style><p id=x>hi</p></body></html>";
        assert_eq!(
            slim_html(page),
            "<html><head><title>T</title></head><body><style>c{}</style><p id=x>hi</p></body></html>"
        );
        assert_eq!(slim_html("<p>no head</p>"), "<p>no head</p>");
    }
}
