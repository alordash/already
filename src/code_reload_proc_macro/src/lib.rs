use crate::generation::targets::*;
use quote::ToTokens;
use syn::*;

mod build_profile;
mod generation;

#[proc_macro_attribute]
pub fn hotreload(
    _: proc_macro::TokenStream,
    proc_macro_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item_fn = parse_macro_input!(proc_macro_item as ItemFn);

    if build_profile::is_debug() {
        return item_fn.to_token_stream().into();
    }

    let result = simple_hotreload_infrastructure::generate(item_fn);

    return result.to_token_stream().into();
}
