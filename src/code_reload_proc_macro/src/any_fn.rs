use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};
use syn::*;

pub enum AnyFn {
    Standalone(ItemFn),
    Associated(ImplItemFn),
}

impl Parse for AnyFn {
    fn parse(input: ParseStream) -> Result<Self> {
        let fork = input.fork();
        if let Ok(item_fn) = fork.parse::<ItemFn>() {
            input.advance_to(&fork);
            return Ok(Self::Standalone(item_fn));
        }

        let maybe_impl_item_fn = input.parse::<ImplItemFn>();
        return maybe_impl_item_fn.map(Self::Associated);
    }
}

impl ToTokens for AnyFn {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            AnyFn::Standalone(item_fn) => item_fn.to_tokens(tokens),
            AnyFn::Associated(impl_item_fn) => impl_item_fn.to_tokens(tokens),
        }
    }
}
