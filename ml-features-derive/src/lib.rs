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
    DeriveInput,
    Data,
    DataStruct,
    DataEnum,
    Field,
    Ident,
    Meta,
};

const NO_FEATURE_IDENT: &'static str = "no_feature";

#[proc_macro_derive(Features, attributes(no_feature))]
pub fn derive_features(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    match input.data {
        Data::Enum(data) => features_enum_impl(&name, data),
        Data::Struct(data) => features_struct_impl(&name, data),
        _ => unimplemented!("Unhandled data type"),
    }
}

fn features_enum_impl(name: &Ident, data: DataEnum) -> TokenStream {
    let variants_idents: Vec<Ident> = data.variants.into_iter().map(|v| v.ident.clone()).collect();

    let output = quote! {
        impl Features for #name {
            fn to_vec(&self) -> Vec<f32> {
                vec![
                    #(if *self == #name::#variants_idents { 1. } else { 0. }),*
                ]
            }
        }
    };

    TokenStream::from(output)
}

fn features_struct_impl(name: &Ident, data: DataStruct) -> TokenStream {
    let col_view_trait_name = format_ident!("{}ColView", name);

    let no_feature_ident = Ident::new(NO_FEATURE_IDENT, Span::call_site());

    let feature_fields_idents: Vec<Ident> = data.fields.into_iter()
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
        }

        impl #name {
            pub const fn nb_features() -> usize {
                #nb_features
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

