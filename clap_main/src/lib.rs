use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn clap_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let f = parse_macro_input!(item as ItemFn);

    let item_type = f
        .sig
        .inputs
        .first()
        .expect("Need exactly one argument to the function");

    let name = f.sig.ident.clone();

    let ty = match item_type {
        syn::FnArg::Receiver(_) => panic!("Expected a type"),

        syn::FnArg::Typed(syn::PatType { ty, .. }) => match *ty.clone() {
            syn::Type::Path(ty) => ty.path.clone(),
            _ => panic!("Unexpected type here"),
        },
    };

    quote! {
        #f

        pub fn main() {
            use clap::Parser;
            let args = match #ty::try_parse() {
                Ok(args) => args,
                Err(e) => panic!("{e}"),
            };

            match #name(args) {
                Ok(()) => {},
                Err(e) => panic!("{e}"),
            }
        }
    }
    .into()
}
