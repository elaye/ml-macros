extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::{TokenStream};
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, Field, Ident, Meta};

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

    let output = quote! {
        impl #name {
            pub fn to_vec(&self) -> Vec<f32> {
                vec![
                    #(self.#feature_fields_idents,)*
                ]
            }
        }
    };

    TokenStream::from(output)
}

// FIXME: find out why 'field_ignore_attr_ident' is considered unused
#[allow(unused_variables)]
fn ignored_field(field: &Field, field_ignore_attr_ident: &Ident) -> bool {
    field.attrs.iter().all(|attr| {
        match attr.parse_meta() {
            Ok(Meta::Word(field_ignore_attr_ident)) => {
                false
            },
            _ => true
        }
    })
}
