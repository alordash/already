use quote::ToTokens;
use syn::*;

mod any_fn;
mod caller_crate;
mod hotreload_syntax;
mod source_code_id;

use any_fn::*;
use source_code_id::*;

#[proc_macro_attribute]
pub fn hotreload(
    _: proc_macro::TokenStream,
    proc_macro_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut any_fn = parse_macro_input!(proc_macro_item as AnyFn);

    match &mut any_fn {
        AnyFn::Standalone(item_fn) => {
            hotreload_syntax::apply(&mut item_fn.attrs, &item_fn.sig, &mut item_fn.block)
        }
        AnyFn::Associated(impl_item_fn) => hotreload_syntax::apply(
            &mut impl_item_fn.attrs,
            &impl_item_fn.sig,
            &mut impl_item_fn.block,
        ),
        _ => panic!("Can apply `#[hotreload]` only to standalone and associated functions."),
    };

    return any_fn.to_token_stream().into();
}
