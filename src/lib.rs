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

const NO_FEATURE_IDENT: &'static str = "no_feature";

#[proc_macro_derive(Features, attributes(no_feature))]
pub fn derive_features(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);

    let name = &input.ident;

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
        impl #name {
            pub fn to_vec(&self) -> Vec<f32> {
                vec![
                    #(self.#feature_fields_idents),*
                ]
            }

            pub fn to_vec_without(&self, fields: &[&str]) -> Vec<f32> {
                let mut vec = Vec::new();

                #(if !fields.contains(&#feature_fields_names) {
                    vec.push(self.#feature_fields_idents);
                })*

                vec
            }

            pub fn names() -> Vec<&'static str> {
                vec![
                    #(#feature_fields_names),*
                ]
            }

            pub const fn nb_features() -> usize {
                #nb_features
            }
        }
    };

    TokenStream::from(output)
}

fn ignored_field(field: &Field, field_ignore_attr_ident: &Ident) -> bool {
    field.attrs.iter().all(|attr| {
        match attr.parse_meta() {
            Ok(Meta::Path(path)) => {
                match path.get_ident() {
                    Some(ident) => ident == field_ignore_attr_ident,
                    None => false
                }
            },
            _ => true
        }
    })
}
