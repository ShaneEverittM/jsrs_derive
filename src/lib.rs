extern crate proc_macro;

mod object_derive;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Object, attributes(object_type, properties))]
pub fn object_derive(input: TokenStream) -> TokenStream {
    object_derive::derive_impl(input)
}

#[proc_macro_derive(Expression)]
pub fn expression_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let output = quote! {
        impl crate::ir::marker::Expression for #name {}
    };

    output.into()
}

#[proc_macro_derive(Statement)]
pub fn statement_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let output = quote! {
        impl crate::ir::marker::Statement for #name {}
    };

    output.into()
}

#[proc_macro_derive(Declaration)]
pub fn decl_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let output = quote! {
        impl crate::ir::marker::Declaration for #name {}
    };

    output.into()
}
