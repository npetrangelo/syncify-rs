//! Turn every async functional into sync by blocking on it.
//!
//! This crate provides the #\[syncify] macro to implicitly block on every async call.
//!
//! Web API libraries like rspotify struggle to support both async and sync use cases for
//! all of their clients. Ultimately, either the library must support a feature flag that
//! does the blocking for the client, or the client must block on everything themselves.
//! This crate aims to make it easier for the client to block on everything, so that
//! web API libraries can focus on async and recommend this crate instead of directly supporting sync.
//!
//! This crate is not intended to be used by web API libraries themselves or anything lower level.
//! This crate is only intended to be used by consumers of "top level" async libraries
//! who don't want to deal with it.

use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ExprCall, ItemFn, parse_macro_input, Stmt};

#[proc_macro_attribute]
pub fn syncify(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemFn);
    ast.block.stmts = ast.block.stmts.iter().map(|&stmt| -> Stmt {
        match stmt {
            Stmt::Expr(Expr::Await(a), s) => {
                let f = *a.base;
                let expanded = quote! {
                    futures::executor::block_on(#f);
                };
                let expanded: TokenStream = expanded.into();
                let expanded = parse_macro_input!(expanded as ExprCall);
                Stmt::Expr(Expr::Call(expanded), s)
            },
            a => a,
        }
    }).collect::<Vec<Stmt>>();
    eprintln!("{:#?}", ast);
    let expanded = quote! {
        #ast
    };
    expanded.into()
}
