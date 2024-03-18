use inflector::Inflector;
use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::Ident;
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, GenericArgument, ItemStruct, PathArguments,
    Token, Type, TypePath,
};

mod many_to_many;
mod standard;
mod utils;

#[derive(Debug)]
struct LeviosaArgs {
    many_to_many: bool,
}

impl Parse for LeviosaArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;

        let many_to_many = args.iter().any(|ident| ident == "many_to_many");

        Ok(LeviosaArgs { many_to_many })
    }
}

#[proc_macro_attribute]
pub fn leviosa(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let args = parse_macro_input!(_attr as LeviosaArgs);
    let name = &input.ident;

    if args.many_to_many {
        many_to_many::many_to_many_methods(name, &input)
    } else {
        standard::standard_methods(name, &input)
    }
}
