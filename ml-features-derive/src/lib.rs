extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::{TokenStream};
use proc_macro2::Span;
use quote::{
    quote,
    format_ident,
};
use syn::{
    parse_macro_input,
    ItemStruct,
    ItemEnum,
    Field,
    Ident,
    Meta,
};
use heck::SnakeCase;

const NO_FEATURE_IDENT: &'static str = "no_feature";

#[proc_macro_derive(Features, attributes(no_feature))]
pub fn derive_features(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);

    let name = &input.ident;
    let col_view_trait_name = format_ident!("{}ColView", name);

    let no_feature_ident = Ident::new(NO_FEATURE_IDENT, Span::call_site());

    let feature_fields_idents: Vec<Ident> = input.fields.into_iter()
        // Filter out fields with a #[no_feature] attribute
        .filter(|f| ignored_field(f, &no_feature_ident))
        .map(|f| f.ident.clone().unwrap())
        .collect();

    let feature_fields_names: Vec<String> = feature_fields_idents.iter()
        .map(Ident::to_string)
        .collect();

    let nb_features = feature_fields_idents.len();

    let output = quote! {
        impl Features for #name {
            fn to_vec(&self) -> Vec<f32> {
                vec![
                    #(self.#feature_fields_idents),*
                ]
            }

            fn to_vec_without(&self, fields: &[&str]) -> Vec<f32> {
                let mut vec = Vec::new();

                #(if !fields.contains(&#feature_fields_names) {
                    vec.push(self.#feature_fields_idents);
                })*

                vec
            }

            fn names() -> Vec<&'static str> {
                vec![
                    #(#feature_fields_names),*
                ]
            }
        }

        impl #name {
            pub const fn nb_features() -> usize {
                #nb_features
            }
        }

        pub trait #col_view_trait_name {
            #(fn #feature_fields_idents(&self) -> Vec<f32>;)*
        }

        impl #col_view_trait_name for Vec<#name> {
            #(fn #feature_fields_idents(&self) -> Vec<f32> {
                self.iter().map(|f| f.#feature_fields_idents).collect()
            })*
        }

        impl #col_view_trait_name for &[#name] {
            #(fn #feature_fields_idents(&self) -> Vec<f32> {
                self.iter().map(|f| f.#feature_fields_idents).collect()
            })*
        }
    };

    TokenStream::from(output)
}

fn ignored_field(field: &Field, field_ignore_attr_ident: &Ident) -> bool {
    // If all attributes are different from
    // 'field_ignore_attr_ident', then return 'true'.
    field.attrs.iter().all(|attr| {
        match attr.parse_meta() {
            Ok(Meta::Path(path)) => {
                match path.get_ident() {
                    Some(ident) => ident != field_ignore_attr_ident,
                    None => false
                }
            },
            _ => false
        }
    })
}

#[proc_macro_derive(ToOneHot)]
pub fn derive_to_one_hot(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemEnum);

    let name = &input.ident;

    let variants_idents: Vec<Ident> = input.variants.into_iter().map(|v| v.ident.clone()).collect();

    let variants_idents_lower: Vec<Ident> = variants_idents
        .iter()
        .map(|v| format_ident!("{}", v.to_string().to_snake_case()))
        .collect();

    let new_struct_ident = format_ident!("{}OneHot", name);

    let output = quote! {
        pub struct #new_struct_ident {
            #(pub #variants_idents_lower: f32),*
        }

        impl ToOneHot for #name {
            type OneHotStruct = #new_struct_ident;

            fn to_one_hot(&self) -> Self::OneHotStruct {
                Self::OneHotStruct {
                    #(#variants_idents_lower: if *self == #name::#variants_idents { 1. } else { 0. }),*
                }
            }
        }
    };

    TokenStream::from(output)
}
