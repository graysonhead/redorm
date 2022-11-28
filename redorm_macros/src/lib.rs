extern crate proc_macro;
use bae::FromAttributes;
use proc_macro::TokenStream;


mod attributes;
mod hset;
mod errors;

#[derive(Debug, Eq, PartialEq, FromAttributes)]
struct FieldAttributes {
    id_field: Option<syn::LitBool>,
}

#[proc_macro_derive(DeriveHashSet, attributes(redorm))]
pub fn derive_macro_hset(input: TokenStream) -> TokenStream {
    hset::derive_proc_macro_impl(input)
}
