extern crate proc_macro;

use javascript_rs::runtime::ObjectType;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseBuffer};
use syn::{parenthesized, token, Error, Field, Ident, Result, Token};
use syn::{parse_macro_input, Attribute, DeriveInput};

#[proc_macro_derive(JsObject, attributes(object_type))]
pub fn my_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    // Get object_type attribute
    let struct_attr = match ast.attrs.first() {
        None => {
            return syn::Error::new_spanned(ast, "Expected only one attribute")
                .to_compile_error()
                .into()
        }
        Some(attr) => attr,
    };

    // Convert that to a type
    let _object_type = match parse_object_type(struct_attr) {
        Ok(t) => t,
        Err(e) => return e.to_compile_error().into(),
    };

    let tokens = quote!();

    tokens.into()
}

#[derive(Debug)]
struct ObjectTypeAttr {
    paren: token::Paren,
    variant: Ident,
}

impl Parse for ObjectTypeAttr {
    fn parse(input: &ParseBuffer) -> syn::Result<Self> {
        let _inner;
        Ok(ObjectTypeAttr {
            paren: parenthesized!(_inner in input),
            variant: input.parse()?,
        })
    }
}

fn parse_object_type(attr: &Attribute) -> syn::Result<ObjectType> {
    assert_eq!(attr.style, syn::AttrStyle::Outer);
    if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "object_type" {
        // found the attr
        let tokens: proc_macro::TokenStream = attr.tokens.clone().into();
        let obj_type_attr = syn::parse::<ObjectTypeAttr>(tokens)?;

        match obj_type_attr.variant.to_string().as_str() {
            "Global" => Ok(ObjectType::Global),
            "Function" => Ok(ObjectType::Function),
            "Array" => Ok(ObjectType::Array),
            "String" => Ok(ObjectType::String),
            "Object" => Ok(ObjectType::Object),
            _ => Err(syn::Error::new_spanned(attr, "Must match variant")),
        }
    } else {
        Err(syn::Error::new_spanned(
            attr,
            "expected `#[object_type(<type>)]`",
        ))
    }
}
