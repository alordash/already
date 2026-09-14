use crate::generation::targets::*;
use code_reload_syn_core::*;
use quote::{ToTokens, quote};
use syn::*;

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

    let simple::Result {
        substitute_function,
        source_function,
    } = simple::generate(item_fn);

    let result = quote! {
        #substitute_function
        #source_function
    };
    return result.to_token_stream().into();
}
