#![feature(default_free_fn)]
#![feature(extend_one)]

extern crate proc_macro;

use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, TokenStream, TokenTree};
use std::default::default;

fn unwrap_inner(input: TokenStream) -> TokenStream {
    let mut out: TokenStream = default();
    for token in input.into_iter() {
        match token {
            TokenTree::Group(group) => {
                let new_group = Group::new(group.delimiter(), unwrap_inner(group.stream()));
                out.extend_one(TokenTree::Group(new_group))
            }
            TokenTree::Punct(punct) if punct.as_char() == '?' => out.extend([
                TokenTree::Punct(Punct::new('.', Spacing::Alone)),
                TokenTree::Ident(Ident::new("unwrap", punct.span())),
                TokenTree::Group(Group::new(Delimiter::Parenthesis, default())),
            ]),
            other => out.extend_one(other),
        }
    }
    out
}

/// Automatically replaces every instance of the `?` operator with `.unwrap()`
///
/// # Example
///
/// ```ignore
/// #[unwrap]
/// fn normal_fn() {
///     let s = "does it detect this question mark? (no)";
///     println!(s);
///     let x: Result<i32, ()> = Ok(23);
///     x?; // gets replaced with x.unwrap();
/// }
/// ```
#[proc_macro_attribute]
pub fn unwrap(_args: TokenStream, input: TokenStream) -> TokenStream {
    unwrap_inner(input)
}
