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
            if let syn::Fields::Named(fields) = &mut struct_data.fields {
                let field = syn::Field::parse_named
                    .parse2(quote! {comment: javadoc2adoc_types::BlockComment<'a>});
                match field {
                    Ok(field) => {
                        fields.named.push(field);
                    }
                    Err(error) => {
                        return error.into_compile_error().into();
                    }
                }

                let field = syn::Field::parse_named.parse2(quote! {node: tree_sitter::Node<'a>});
                match field {
                    Ok(field) => {
                        fields.named.push(field);
                    }
                    Err(error) => {
                        return error.into_compile_error().into();
                    }
                }

                let field = syn::Field::parse_named
                    .parse2(quote! {context: &'a javadoc2adoc_types::FileContext});
                match field {
                    Ok(field) => {
                        fields.named.push(field);
                    }
                    Err(error) => {
                        return error.into_compile_error().into();
                    }
                }
            }

            quote! {
                #ast
            }
            .into()
        }
        _ => Error::new(
            ast.span(),
            "default_javadocable_fields is only supported for structs",
        )
        .into_compile_error()
        .into(),
    }
}

#[proc_macro_derive(DefaultJavaDocable)]
pub fn default_java_docable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics javadoc2adoc_types::DefaultJavaDocable #ty_generics for #name #ty_generics #where_clause {
            fn get_node(&self) -> tree_sitter::Node<'_> {
                self.node
            }
            fn get_context(&self) -> &'a javadoc2adoc_types::FileContext {
                self.context
            }
            fn get_comment(&self) -> &'a javadoc2adoc_types::BlockComment {
                &self.comment
            }
        }
    };

    expanded.into()
}
