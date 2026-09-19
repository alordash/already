use quote::ToTokens;
use syn::*;

mod caller_crate;
mod hotreload_syntax;
mod source_code_id;

use source_code_id::*;

#[proc_macro_attribute]
pub fn hotreload(
    _: proc_macro::TokenStream,
    proc_macro_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut item_fn = parse_macro_input!(proc_macro_item as ItemFn);
    hotreload_syntax::apply(hotreload_syntax::Parameters {
        attributes: &mut item_fn.attrs,
        signature: &item_fn.sig,
        block: &mut item_fn.block,
    });

    return item_fn.to_token_stream().into();
}
