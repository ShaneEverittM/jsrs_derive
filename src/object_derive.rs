use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseBuffer};
use syn::punctuated::Punctuated;
use syn::{parenthesized, Error, Field, Ident, Result};
use syn::{parse_macro_input, Attribute, DeriveInput};

pub(crate) fn derive_impl(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let fields = get_fields(&ast);

    let properties_ident = match find_property_store(fields) {
        Ok(ps) => ps,
        Err(e) => return e.to_compile_error().into(),
    };

    let object_type = match parse_object_type(&ast.attrs) {
        Ok(t) => t,
        Err(e) => return e.to_compile_error().into(),
    };

    let object_type_tokens = match object_type.variant.to_string().as_str() {
        "Global" => quote! {crate::runtime::ObjectType::Global},
        "Function" => quote! {crate::runtime::ObjectType::Function},
        "Array" => quote! {crate::runtime::ObjectType::Array},
        "String" => quote! {crate::runtime::ObjectType::String},
        "Object" => quote! {crate::runtime::ObjectType::Object},
        _ => Error::new_spanned(object_type.variant, "Must match variant").to_compile_error(),
    };

    let tokens = quote! {
        impl crate::runtime::Object for #name {
            fn put(&mut self, name: String, value: crate::runtime::Value) {
                self.#properties_ident.insert(name, value);
            }

            fn get(&self, name: &str) -> Option<crate::runtime::Value> {
                self.#properties_ident.get(name).cloned()
            }

            fn get_mut(&mut self, name: &str) -> Option<&mut crate::runtime::Value> {
                self.#properties_ident.get_mut(name)
            }

            fn get_type(&self) -> crate::runtime::ObjectType {
               #object_type_tokens
            }

            fn as_any(&mut self) -> &mut dyn std::any::Any {
                self
            }

            fn into_object(self: Box<Self>) -> Box<dyn crate::runtime::Object> {
                self as Box<dyn Object>
            }

            fn format_properties(&self) -> String {
                let mut buf = String::new();
                buf += "{\n";
                for (k, v) in self.#properties_ident.iter() {
                    buf += &format!("    {}: {}\n", k, v);
                }
                buf += "}";
                buf
            }
        }
    };

    tokens.into()
}

fn find_property_store(fields: &Punctuated<Field, syn::Token![,]>) -> Result<Ident> {
    let mut properties_ident = None;
    for field in fields {
        if field.ident.clone().unwrap() == "properties" {
            properties_ident = Some(field.ident.clone().unwrap());
        } else {
            let attrs = &field.attrs;
            for attr in attrs {
                if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "properties" {
                    // found the attr
                    properties_ident = Some(field.ident.clone().unwrap())
                } else {
                    return Err(Error::new_spanned(attr, "expected #[object_type(<type>)]"));
                }
            }
        }
    }
    properties_ident
        .ok_or_else(|| Error::new_spanned(fields, "Could not find object store in fields"))
}

fn get_fields(ast: &DeriveInput) -> &Punctuated<Field, syn::Token![,]> {
    // Extract fields of input struct into iterator.
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        // Must match the type of struct we can build, so a struct with named fields.
        unimplemented!();
    }
}

struct ObjectTypeAttr {
    variant: Ident,
}

impl Parse for ObjectTypeAttr {
    fn parse(input: &ParseBuffer) -> Result<Self> {
        let inner;
        parenthesized!(inner in input);
        Ok(ObjectTypeAttr {
            variant: inner.parse()?,
        })
    }
}

fn parse_object_type(attrs: &[Attribute]) -> Result<ObjectTypeAttr> {
    for attr in attrs {
        if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "object_type" {
            let tokens: proc_macro::TokenStream = attr.tokens.clone().into();
            return syn::parse::<ObjectTypeAttr>(tokens);
        }
    }
    return Err(Error::new_spanned(
        attrs.first().unwrap(),
        "could not find #[object_type(<type>)] attribute",
    ));
}
