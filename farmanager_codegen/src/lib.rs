extern crate proc_macro;

use quote::*;
use syn;

use crate::proc_macro::TokenStream;

#[proc_macro_derive(Langpack, attributes(msg, language, langpack))]
pub fn langpack(input: TokenStream) -> TokenStream {
    let input: syn::DeriveInput = syn::parse(input).unwrap();
    let name: &syn::Ident = &input.ident;

    let expanded = quote! {
        impl basic::Langpack for #name {
            fn to_message_id(&self) -> isize {
                *self as isize
            }
        }
    };

    expanded.into()
}
