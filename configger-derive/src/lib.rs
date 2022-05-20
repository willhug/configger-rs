use std::any::Any;

use proc_macro2::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Attribute, DeriveInput, Field, Ident, Token,
};

#[proc_macro_derive(ConfiggerData, attributes(configger))]
#[proc_macro_error]
pub fn configger(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match input.data {
        syn::Data::Struct(ref data) => match data.fields {
            syn::Fields::Named(ref fields) => {
                let struct_name = input.ident;
                let field_defs: Vec<TokenStream> =
                    fields.named.iter().map(|f| field_definition(f)).collect();
                quote! {
                    impl ConfiggerData for #struct_name {
                        fn fields() -> Vec<ConfiggerField> {
                            vec![
                                #( #field_defs )*
                            ]
                        }
                    }
                }
            }
            syn::Fields::Unnamed(_) | syn::Fields::Unit => unimplemented!(),
        },
        syn::Data::Enum(_) | syn::Data::Union(_) => unimplemented!(),
    }.into()
}

fn field_definition(field: &Field) -> TokenStream {
    let name = format!("{}", field.ident.clone().unwrap());
    let require_on_create = parse_attrs(&field.attrs).iter().any(|a| match a {
        ConfiggerFieldAttr::RequireOnCreate(_) => true,
    });
    quote! {
        ConfiggerField {
            name: #name,
            // ty: "String", // TODO Syn type?
            require_on_create: #require_on_create,
        },
    }
}

#[derive(Debug)]
enum ConfiggerFieldAttr {
    RequireOnCreate(Ident),
}

impl Parse for ConfiggerFieldAttr {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let name_str = name.to_string();
        // Attributes represented with a sole identifier.
        match name_str.as_ref() {
            "require_on_create" => Ok(ConfiggerFieldAttr::RequireOnCreate(name)),

            _ => abort!(name, "unexpected attribute: {}", name_str),
        }
    }
}

fn parse_attrs(attrs: &[Attribute]) -> Vec<ConfiggerFieldAttr> {
    attrs
        .iter()
        .filter(|attr| attr.path.is_ident("configger"))
        .flat_map(|attr| {
            attr.parse_args_with(Punctuated::<ConfiggerFieldAttr, Token![,]>::parse_terminated)
                .unwrap()
        })
        .collect()
}
