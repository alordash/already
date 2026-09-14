use code_reload_syn_core::*;
use quote::format_ident;
use syn::*;

mod source_function;
mod substitute_function;

// fn usage(v: i32) -> i32 {
//     let library = LibraryWrapper::new("amogus.dll");
//     library.get::<fn(i32) -> i32>(b"kavo")(v)
// }
// #[unsafe(export_name = "__code_reload_file_row_col_usage")]
// fn __code_reload_file_row_col_usage(v: i32) -> i32 { v + 1 }
pub(crate) struct Result {
    pub substitute_function: ItemFn,
    pub source_function: ItemFn,
}
pub(crate) fn generate(item_fn: ItemFn) -> Result {
    if !item_fn.sig.generics.params.is_empty() {
        panic!("Unsupported: functions with generics can not be hot-reloaded.")
    }

    let call_site = proc_macro::Span::call_site();
    let relative_file_path = call_site.local_file().unwrap_or_else(|| "UNKNOWN".into());
    let source_code_id =
        SourceCodeId::new(relative_file_path, call_site.line(), call_site.column());
    let fn_ident_prefix = source_code_id.to_fn_ident_prefix();
    let new_fn_ident = format_ident!("__code_reload_{}_{}", fn_ident_prefix, item_fn.sig.ident);

    let substitute_function = substitute_function::generate(&item_fn, new_fn_ident.to_string());
    let source_function = source_function::generate(item_fn, new_fn_ident);
    let result = Result {
        substitute_function,
        source_function,
    };
    return result;
}
