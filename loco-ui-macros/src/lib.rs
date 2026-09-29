//! # lui!
//!
//! Maud's `html!` with components written like elements. Everything that is plain Maud passes
//! through untouched; a capitalized name is a component, and it expands to the same builder
//! chain a route would write by hand, so `lui!` and the dot form are one code path.
//!
//! ```rust
//! use loco_ui::prelude::*;
//!
//! let ui = Ui::from_request("/", "", "");
//! let narrow = true;
//! let projects = [("Site", 3), ("App", 0)];
//! let page = lui! {
//!     h1 { "Projects" }
//!     Tabs("projects") vertical[narrow] {
//!         @for (name, open) in projects {
//!             tab (name) badge=(open) { p { "Open issues: " (open) } }
//!         }
//!         lazy "Archive" || { p { "Rendered only when opened." } }
//!     }
//! };
//! let by_hand = html! {
//!     h1 { "Projects" }
//!     (ui.tabs("projects").vertical()
//!         .tab("Site", html! { p { "Open issues: " (3) } }).badge(3)
//!         .tab("App", html! { p { "Open issues: " (0) } }).badge(0)
//!         .lazy("Archive", || html! { p { "Rendered only when opened." } }))
//! };
//! assert_eq!(page.into_string(), by_hand.into_string());
//! ```
//!
//! ## The rules
//!
//! - **A component** is a capitalized name: `DatePicker(..)` is `ui.date_picker(..)` (the name
//!   in snake case), and `(..)` holds the required arguments, as in the method call. A
//!   component with none leaves the parentheses out (`Card title="Plan" { .. }`). It ends with
//!   `;` or with a block.
//! - **Attributes** are setters: `x="v"` or `x=(expr)` is `.x(v)`, `x=(a, b)` is `.x(a, b)`, a
//!   bare `x` is `.x()`, `x[cond]` calls `.x()` only when `cond` holds, and `x=[option]`
//!   calls `.x(v)` only when the option is `Some(v)` (Maud's toggle and optional syntax).
//!   `x=|i| { markup }` passes a closure returning markup (`rows=|i| { .. }` on a pager), and
//!   `x={ markup }` passes markup, itself `lui!` (`footer={ Button("Save") primary; }`).
//! - **A component's block** is either items or a body. It holds items when it starts with
//!   one: a lowercase name followed by its arguments (`tab "Use"`, `column "name" "Name"`,
//!   `tab (p.title)`, `separator()`). An item's attributes are the modifiers that apply to it
//!   (`tab "Use" badge=3`), its block is its last argument as markup, `|| { .. }` passes that
//!   markup as a closure instead (`lazy "Why" || { .. }`), and a block that itself holds items
//!   continues the chain (a kanban `column` with its `card`s). Otherwise the block is plain
//!   markup, passed to `.body(..)`. Any setter can stand in an items block the same way
//!   (`search "/shop";`, `inline();`), for the rare chain whose order matters; `body { .. }`
//!   among items is `.body(..)` with that markup (a form's controls between its fields).
//!   An item's arguments are values: a builder goes in parentheses (`stat (ui.stat(..))`),
//!   and a component written there is an error that says so.
//! - **`@for`, `@if` / `@else`, `@match` and `@let`** work among items as they do in Maud, so
//!   items built from data stay inline.
//! - **`ui`** is taken from the scope by that name; `lui!(ctx => ..)` names another.
//!
//! A misspelled attribute is rustc's own error at that attribute ("no method named `vertcal`
//! … a method with a similar name exists: `vertical`"), since every expanded call keeps the
//! span it came from.

use proc_macro2::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};
use quote::{quote, quote_spanned};

/// Maud's `html!` with components written like elements; see the crate documentation.
#[proc_macro]
pub fn lui(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut tokens: Vec<TokenTree> = TokenStream::from(input).into_iter().collect();
    let mut ui = Ident::new("ui", Span::call_site());
    if let [
        TokenTree::Ident(name),
        TokenTree::Punct(eq),
        TokenTree::Punct(gt),
        ..,
    ] = &tokens[..]
        && eq.as_char() == '='
        && eq.spacing() == Spacing::Joint
        && gt.as_char() == '>'
    {
        ui = name.clone();
        tokens.drain(..3);
    }
    let cx = Cx { ui };
    match cx.markup(&tokens) {
        Ok(markup) => quote!(::maud::html! { #markup }).into(),
        Err((span, message)) => quote_spanned!(span=> ::core::compile_error!(#message)).into(),
    }
}

type Result<T> = std::result::Result<T, (Span, String)>;

struct Cx {
    ui: Ident,
}

/// The builder variable of an expansion; hygienic, so it never meets a caller's name.
fn builder_ident() -> Ident {
    Ident::new("__lui_builder", Span::mixed_site())
}

fn punct(token: Option<&TokenTree>, c: char) -> bool {
    matches!(token, Some(TokenTree::Punct(p)) if p.as_char() == c)
}

fn brace(token: Option<&TokenTree>) -> Option<&Group> {
    match token {
        Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace => Some(g),
        _ => None,
    }
}

fn paren(token: Option<&TokenTree>) -> Option<&Group> {
    match token {
        Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Parenthesis => Some(g),
        _ => None,
    }
}

fn keyword(token: Option<&TokenTree>) -> Option<String> {
    match token {
        Some(TokenTree::Ident(i)) => Some(i.to_string()),
        _ => None,
    }
}

fn starts_upper(ident: &Ident) -> bool {
    ident
        .to_string()
        .starts_with(|c: char| c.is_ascii_uppercase())
}

/// `DatePicker` → `date_picker`.
fn snake(name: &str) -> String {
    let mut out = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_ascii_uppercase() {
            if i > 0 {
                out.push('_');
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

fn stream(tokens: &[TokenTree]) -> TokenStream {
    tokens.iter().cloned().collect()
}

/// `tokens` up to (not including) the first top-level brace group, and its index.
fn until_brace(tokens: &[TokenTree], from: usize, what: &str) -> Result<usize> {
    (from..tokens.len())
        .find(|&i| brace(tokens.get(i)).is_some())
        .ok_or_else(|| {
            (
                tokens[from.min(tokens.len() - 1)].span(),
                format!("`{what}` needs a `{{ .. }}` block"),
            )
        })
}

/// Whether `tokens[index]` starts an item: a lowercase name followed by an argument, `body`
/// followed by its block, or (a mistake [`Cx::items`] reports) a name followed by a component.
fn item_at(tokens: &[TokenTree], index: usize) -> bool {
    let Some(TokenTree::Ident(name)) = tokens.get(index) else {
        return false;
    };
    if starts_upper(name) {
        return false;
    }
    match tokens.get(index + 1) {
        Some(TokenTree::Literal(_)) => true,
        Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace => name == "body",
        Some(TokenTree::Group(_)) => true,
        Some(TokenTree::Ident(c)) => starts_upper(c) && paren(tokens.get(index + 2)).is_some(),
        _ => false,
    }
}

/// Whether a component's block holds items: its first entry (looking inside control flow)
/// is an item, or it is empty.
fn holds_items(tokens: &[TokenTree]) -> bool {
    let mut index = 0;
    while index < tokens.len() {
        if punct(tokens.get(index), '@') {
            if keyword(tokens.get(index + 1)).as_deref() == Some("let") {
                index = (index..tokens.len())
                    .find(|&j| punct(tokens.get(j), ';'))
                    .map_or(tokens.len(), |j| j + 1);
                continue;
            }
            return match (index..tokens.len()).find_map(|j| brace(tokens.get(j))) {
                Some(g) if keyword(tokens.get(index + 1)).as_deref() == Some("match") => {
                    let arms: Vec<TokenTree> = g.stream().into_iter().collect();
                    let at = arms
                        .windows(2)
                        .position(|w| punct(w.first(), '=') && punct(w.get(1), '>'));
                    at.is_some_and(|at| {
                        brace(arms.get(at + 2)).map_or_else(
                            || item_at(&arms, at + 2),
                            |g| holds_items(&g.stream().into_iter().collect::<Vec<_>>()),
                        )
                    })
                }
                Some(g) => holds_items(&g.stream().into_iter().collect::<Vec<_>>()),
                None => false,
            };
        }
        return item_at(tokens, index);
    }
    true
}

impl Cx {
    /// Plain Maud markup, with every component in it expanded to a splice.
    fn markup(&self, tokens: &[TokenTree]) -> Result<TokenStream> {
        let mut out = TokenStream::new();
        let mut index = 0;
        while index < tokens.len() {
            match &tokens[index] {
                TokenTree::Punct(p) if p.as_char() == '@' => {
                    let kw = keyword(tokens.get(index + 1)).unwrap_or_default();
                    if kw == "let" {
                        let end = (index..tokens.len())
                            .find(|&j| punct(tokens.get(j), ';'))
                            .ok_or_else(|| (p.span(), "`@let` needs a `;`".to_string()))?;
                        out.extend(stream(&tokens[index..=end]));
                        index = end + 1;
                        continue;
                    }
                    let at = until_brace(tokens, index, &format!("@{kw}"))?;
                    out.extend(stream(&tokens[index..at]));
                    let g = brace(tokens.get(at)).unwrap();
                    let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                    let body = if kw == "match" {
                        self.arms(&inner)?
                    } else {
                        self.markup(&inner)?
                    };
                    out.extend([regroup(g, body)]);
                    index = at + 1;
                }
                TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => {
                    let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                    out.extend([regroup(g, self.markup(&inner)?)]);
                    index += 1;
                }
                TokenTree::Ident(name) if starts_upper(name) => {
                    let (expr, next) = self.component(tokens, index)?;
                    out.extend([TokenTree::Group(Group::new(Delimiter::Parenthesis, expr))]);
                    index = next;
                }
                TokenTree::Ident(_) | TokenTree::Punct(_) => {
                    let mut j = index;
                    loop {
                        match tokens.get(j) {
                            None => {
                                out.extend(stream(&tokens[index..]));
                                return Ok(out);
                            }
                            Some(TokenTree::Punct(p)) if p.as_char() == ';' && j > index => {
                                out.extend(stream(&tokens[index..=j]));
                                break;
                            }
                            Some(TokenTree::Group(g))
                                if g.delimiter() == Delimiter::Brace
                                    && !punct(tokens.get(j - 1), '=') =>
                            {
                                out.extend(stream(&tokens[index..j]));
                                let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                                out.extend([regroup(g, self.markup(&inner)?)]);
                                break;
                            }
                            _ => j += 1,
                        }
                    }
                    index = j + 1;
                }
                other => {
                    out.extend([other.clone()]);
                    index += 1;
                }
            }
        }
        Ok(out)
    }

    /// The arms of a markup `@match`: patterns copied, each arm's markup expanded.
    fn arms(&self, tokens: &[TokenTree]) -> Result<TokenStream> {
        let mut out = TokenStream::new();
        let mut index = 0;
        while index < tokens.len() {
            let Some(arrow) = (index..tokens.len())
                .find(|&j| punct(tokens.get(j), '=') && punct(tokens.get(j + 1), '>'))
            else {
                out.extend(stream(&tokens[index..]));
                break;
            };
            out.extend(stream(&tokens[index..arrow + 2]));
            let end = (arrow + 2..tokens.len())
                .find(|&j| punct(tokens.get(j), ','))
                .unwrap_or(tokens.len());
            out.extend(self.markup(&tokens[arrow + 2..end])?);
            out.extend(tokens.get(end).cloned());
            index = end + 1;
        }
        Ok(out)
    }

    /// A component at `tokens[at]`: the builder expression and the index after it.
    fn component(&self, tokens: &[TokenTree], at: usize) -> Result<(TokenStream, usize)> {
        let TokenTree::Ident(name) = &tokens[at] else {
            unreachable!()
        };
        let method = Ident::new(&snake(&name.to_string()), name.span());
        let ui = &self.ui;
        let builder = builder_ident();
        let mut index = at + 1;
        let args = match paren(tokens.get(index)) {
            Some(g) => {
                index += 1;
                g.clone()
            }
            None => {
                let mut g = Group::new(Delimiter::Parenthesis, TokenStream::new());
                g.set_span(name.span());
                g
            }
        };
        let mut steps = vec![quote!(let #builder = #ui.#method #args;)];
        index = self.attributes(tokens, index, &mut steps)?;
        match tokens.get(index) {
            Some(TokenTree::Punct(p)) if p.as_char() == ';' => index += 1,
            Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace => {
                let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                if holds_items(&inner) {
                    self.items(&inner, &mut steps)?;
                } else {
                    let body = self.markup(&inner)?;
                    let call = Ident::new("body", g.span_open());
                    steps.push(quote!(let #builder = #builder.#call(::maud::html! { #body });));
                }
                index += 1;
            }
            None => {}
            Some(other) => {
                return Err((
                    other.span(),
                    format!("`{name}` ends with `;` or a `{{ .. }}` block"),
                ));
            }
        }
        Ok((quote!({ #(#steps)* #builder }), index))
    }

    /// Attributes from `tokens[index]` up to a `;`, a block or a closure body; each becomes a step.
    fn attributes(
        &self,
        tokens: &[TokenTree],
        mut index: usize,
        steps: &mut Vec<TokenStream>,
    ) -> Result<usize> {
        let builder = builder_ident();
        while let Some(TokenTree::Ident(name)) = tokens.get(index) {
            if name == "move" && punct(tokens.get(index + 1), '|') {
                break;
            }
            index += 1;
            match tokens.get(index) {
                Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Bracket => {
                    let cond = g.stream();
                    steps.push(
                        quote!(let #builder = if #cond { #builder.#name() } else { #builder };),
                    );
                    index += 1;
                }
                Some(TokenTree::Punct(p)) if p.as_char() == '=' => {
                    index += 1;
                    let (value, next) = self.value(tokens, index, name)?;
                    index = next;
                    match value {
                        Value::Optional(option) => {
                            let v = Ident::new("__lui_value", Span::mixed_site());
                            steps.push(quote!(let #builder = match #option {
                                ::core::option::Option::Some(#v) => #builder.#name(#v),
                                ::core::option::Option::None => #builder,
                            };));
                        }
                        Value::Args(args) => {
                            steps.push(quote!(let #builder = #builder.#name(#args);))
                        }
                    }
                }
                _ => steps.push(quote!(let #builder = #builder.#name();)),
            }
        }
        Ok(index)
    }

    /// An attribute's value at `tokens[index]` and the index after it.
    fn value(&self, tokens: &[TokenTree], index: usize, name: &Ident) -> Result<(Value, usize)> {
        match tokens.get(index) {
            Some(TokenTree::Literal(l)) => Ok((Value::Args(quote!(#l)), index + 1)),
            Some(TokenTree::Punct(p)) if p.as_char() == '-' => {
                let lit = tokens.get(index + 1).cloned();
                Ok((Value::Args(quote!(-#lit)), index + 2))
            }
            Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Parenthesis => {
                Ok((Value::Args(g.stream()), index + 1))
            }
            Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Bracket => {
                Ok((Value::Optional(g.stream()), index + 1))
            }
            Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace => {
                let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                let body = self.markup(&inner)?;
                let html = quote_spanned!(g.span()=> ::maud::html! { #body });
                Ok((Value::Args(html), index + 1))
            }
            Some(TokenTree::Punct(p)) if p.as_char() == '|' => {
                let (closure, next) = self.closure(tokens, index)?;
                Ok((Value::Args(closure), next))
            }
            Some(TokenTree::Ident(m)) if m == "move" => {
                let (closure, next) = self.closure(tokens, index)?;
                Ok((Value::Args(closure), next))
            }
            other => Err((
                other.map_or(name.span(), TokenTree::span),
                format!(
                    "`{name}=` takes a literal, `(expr)`, `[option]`, a markup block `{{ .. }}` \
                     or a closure `|..| {{ .. }}`"
                ),
            )),
        }
    }

    /// `move? |params| { markup }` at `tokens[index]` as a closure returning markup.
    fn closure(&self, tokens: &[TokenTree], mut index: usize) -> Result<(TokenStream, usize)> {
        let mut head = TokenStream::new();
        let span = tokens
            .get(index)
            .map_or_else(Span::call_site, TokenTree::span);
        if keyword(tokens.get(index)).as_deref() == Some("move") {
            head.extend(tokens.get(index).cloned());
            index += 1;
        }
        let start = index;
        let Some(open) = tokens.get(index).cloned() else {
            return Err((span, "a closure here is `move |..| { markup }`".to_string()));
        };
        let close = if matches!(&tokens[index], TokenTree::Punct(p) if p.spacing() == Spacing::Joint)
            && punct(tokens.get(index + 1), '|')
        {
            index + 1
        } else {
            (index + 1..tokens.len())
                .find(|&j| punct(tokens.get(j), '|'))
                .ok_or_else(|| {
                    (
                        open.span(),
                        "a closure's parameters end with `|`".to_string(),
                    )
                })?
        };
        head.extend(stream(&tokens[start..=close]));
        let Some(g) = brace(tokens.get(close + 1)) else {
            return Err((
                open.span(),
                "a closure here is `|..| { markup }`".to_string(),
            ));
        };
        let inner: Vec<TokenTree> = g.stream().into_iter().collect();
        let body = self.markup(&inner)?;
        Ok((quote!(#head ::maud::html! { #body }), close + 2))
    }

    /// The items of a component's block, each a step on the builder.
    fn items(&self, tokens: &[TokenTree], steps: &mut Vec<TokenStream>) -> Result<()> {
        let builder = builder_ident();
        let mut index = 0;
        while index < tokens.len() {
            if punct(tokens.get(index), '@') {
                index = self.control(tokens, index, steps)?;
                continue;
            }
            let Some(TokenTree::Ident(name)) = tokens
                .get(index)
                .filter(|_| !matches!(&tokens[index], TokenTree::Ident(n) if starts_upper(n)))
            else {
                return Err((
                    tokens[index].span(),
                    "a component's block holds items (`tab \"Title\" { .. }`) or markup, not both; \
                     put markup inside an item's block"
                        .to_string(),
                ));
            };
            index += 1;
            if let Some(TokenTree::Ident(c)) = tokens.get(index)
                && starts_upper(c)
            {
                return Err((
                    c.span(),
                    format!(
                        "items take arguments, not a component: write `{name} (ui.{}(..))`, \
                         the builder in parentheses",
                        snake(&c.to_string())
                    ),
                ));
            }
            let mut args: Vec<TokenStream> = Vec::new();
            let first = index;
            loop {
                match tokens.get(index) {
                    Some(TokenTree::Literal(l)) => args.push(quote!(#l)),
                    Some(TokenTree::Punct(p)) if p.as_char() == '-' => {
                        let lit = tokens.get(index + 1).cloned();
                        args.push(quote!(-#lit));
                        index += 1;
                    }
                    Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Parenthesis => {
                        if !g.stream().is_empty() {
                            args.push(g.stream());
                        }
                    }
                    _ => break,
                }
                index += 1;
            }
            if index == first && brace(tokens.get(index)).is_some() && name != "body" {
                return Err((
                    name.span(),
                    format!(
                        "`{name} {{ .. }}` is markup, but this block holds items; put markup inside \
                         an item's block (an item with no arguments is `{name}()`)"
                    ),
                ));
            }
            let mut modifiers = Vec::new();
            index = self.attributes(tokens, index, &mut modifiers)?;
            let mut nested = Vec::new();
            match tokens.get(index) {
                Some(TokenTree::Punct(p)) if p.as_char() == ';' => index += 1,
                Some(TokenTree::Punct(p)) if p.as_char() == '|' => {
                    let (closure, next) = self.closure(tokens, index)?;
                    args.push(closure);
                    index = next;
                }
                Some(TokenTree::Ident(m)) if m == "move" => {
                    let (closure, next) = self.closure(tokens, index)?;
                    args.push(closure);
                    index = next;
                }
                Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace => {
                    let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                    if name != "body" && !inner.is_empty() && holds_items(&inner) {
                        self.items(&inner, &mut nested)?;
                    } else {
                        let body = self.markup(&inner)?;
                        args.push(quote!(::maud::html! { #body }));
                    }
                    index += 1;
                }
                None => {}
                Some(other) => {
                    return Err((
                        other.span(),
                        format!("item `{name}` ends with `;` or a `{{ .. }}` block"),
                    ));
                }
            }
            let mut call = Group::new(Delimiter::Parenthesis, quote!(#(#args),*));
            call.set_span(name.span());
            steps.push(quote!(let #builder = #builder.#name #call;));
            steps.extend(modifiers);
            steps.extend(nested);
        }
        Ok(())
    }

    /// `@for`, `@if`/`@else`, `@match` or `@let` among items, at `tokens[at]` (the `@`).
    fn control(
        &self,
        tokens: &[TokenTree],
        at: usize,
        steps: &mut Vec<TokenStream>,
    ) -> Result<usize> {
        let builder = builder_ident();
        let kw = keyword(tokens.get(at + 1)).unwrap_or_default();
        let scoped = |cx: &Cx, group: &Group| -> Result<TokenStream> {
            let inner: Vec<TokenTree> = group.stream().into_iter().collect();
            let mut inner_steps = Vec::new();
            cx.items(&inner, &mut inner_steps)?;
            Ok(quote!({ #(#inner_steps)* #builder }))
        };
        match kw.as_str() {
            "let" => {
                let end = (at..tokens.len())
                    .find(|&j| punct(tokens.get(j), ';'))
                    .ok_or_else(|| (tokens[at].span(), "`@let` needs a `;`".to_string()))?;
                let binding = stream(&tokens[at + 1..=end]);
                steps.push(binding);
                Ok(end + 1)
            }
            "for" => {
                let open = until_brace(tokens, at, "@for")?;
                let head = stream(&tokens[at + 2..open]);
                let body = scoped(self, brace(tokens.get(open)).unwrap())?;
                steps.push(quote!(let mut #builder = #builder; for #head { #builder = #body; } let #builder = #builder;));
                Ok(open + 1)
            }
            "if" => {
                let mut chain = TokenStream::new();
                let mut index = at + 1;
                let mut closed = false;
                loop {
                    let open = until_brace(tokens, index, "@if")?;
                    let head = stream(&tokens[index..open]);
                    let body = scoped(self, brace(tokens.get(open)).unwrap())?;
                    chain.extend(quote!(#head #body));
                    index = open + 1;
                    if keyword(tokens.get(index)).is_none()
                        && punct(tokens.get(index), '@')
                        && keyword(tokens.get(index + 1)).as_deref() == Some("else")
                    {
                        chain.extend(quote!(else));
                        index += 2;
                        if keyword(tokens.get(index)).as_deref() == Some("if") {
                            continue;
                        }
                        let open = until_brace(tokens, index, "@else")?;
                        chain.extend(scoped(self, brace(tokens.get(open)).unwrap())?);
                        index = open + 1;
                        closed = true;
                    }
                    break;
                }
                if !closed {
                    chain.extend(quote!(else { #builder }));
                }
                steps.push(quote!(let #builder = #chain;));
                Ok(index)
            }
            "match" => {
                let open = until_brace(tokens, at, "@match")?;
                let scrutinee = stream(&tokens[at + 2..open]);
                let arms: Vec<TokenTree> = brace(tokens.get(open))
                    .unwrap()
                    .stream()
                    .into_iter()
                    .collect();
                let mut out = TokenStream::new();
                let mut index = 0;
                while index < arms.len() {
                    let Some(arrow) = (index..arms.len())
                        .find(|&j| punct(arms.get(j), '=') && punct(arms.get(j + 1), '>'))
                    else {
                        break;
                    };
                    out.extend(stream(&arms[index..arrow + 2]));
                    let (body, next) = match brace(arms.get(arrow + 2)) {
                        Some(g) => (scoped(self, g)?, arrow + 3),
                        None => {
                            let end = (arrow + 2..arms.len())
                                .find(|&j| punct(arms.get(j), ','))
                                .unwrap_or(arms.len());
                            let mut inner_steps = Vec::new();
                            self.items(&arms[arrow + 2..end], &mut inner_steps)?;
                            (quote!({ #(#inner_steps)* #builder }), end)
                        }
                    };
                    out.extend(body);
                    out.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone))]);
                    index = if punct(arms.get(next), ',') {
                        next + 1
                    } else {
                        next
                    };
                }
                steps.push(quote!(let #builder = match #scrutinee { #out };));
                Ok(open + 1)
            }
            other => Err((
                tokens[at].span(),
                format!(
                    "`@{other}` is not supported among items; use `@for`, `@if`, `@match` or `@let`"
                ),
            )),
        }
    }
}

enum Value {
    /// Arguments for the setter call.
    Args(TokenStream),
    /// `[option]`: the setter is called with the value when there is one.
    Optional(TokenStream),
}

/// `group` with new contents, keeping its delimiter and span.
fn regroup(group: &Group, inner: TokenStream) -> TokenTree {
    let mut out = Group::new(group.delimiter(), inner);
    out.set_span(group.span());
    TokenTree::Group(out)
}
