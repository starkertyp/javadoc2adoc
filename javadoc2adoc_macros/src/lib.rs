use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{self, Parser},
    parse_macro_input,
    spanned::Spanned,
    DeriveInput, Error,
};

#[proc_macro_attribute]
pub fn default_javadocable_fields(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(args as parse::Nothing);
    let mut ast = parse_macro_input!(input as DeriveInput);
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    let field = syn::Field::parse_named.parse2(quote! {comment: BlockComment<'a>});
                    match field {
                        Ok(field) => {
                            fields.named.push(field);
                        }
                        Err(error) => {
                            return error.into_compile_error().into();
                        }
                    }

                    let field = syn::Field::parse_named.parse2(quote! {node: Node<'a>});
                    match field {
                        Ok(field) => {
                            fields.named.push(field);
                        }
                        Err(error) => {
                            return error.into_compile_error().into();
                        }
                    }

                    let field = syn::Field::parse_named.parse2(quote! {context: &'a FileContext});
                    match field {
                        Ok(field) => {
                            fields.named.push(field);
                        }
                        Err(error) => {
                            return error.into_compile_error().into();
                        }
                    }
                }
                _ => (),
            }

            return quote! {
                #ast
            }
            .into();
        }
        _ => Error::new(
            ast.span(),
            "default_javadocable_fields is only supported for structs",
        )
        .into_compile_error()
        .into(),
    }
}
