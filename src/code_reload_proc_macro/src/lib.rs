mod build_profile;

#[proc_macro_attribute]
pub fn hotreload(
    proc_macro_attribute: proc_macro::TokenStream,
    proc_macro_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    if build_profile::is_debug() {
        return proc_macro_item;
    }

    return proc_macro_item;
}
