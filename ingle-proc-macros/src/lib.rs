#![allow(clippy::let_and_return)]

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(EncodableDocument, attributes(cynic, arguments))]
pub fn encodable_document_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    let rv = ingle_codegen::encodable_document_derive(ast).into();

    //eprintln!("{}", rv);

    rv
}
