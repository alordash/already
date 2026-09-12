use syn::*;

// fn usage(v: i32) -> i32 {
//     let library = LibraryWrapper::new("amogus.dll");
//     library.get::<fn(i32) -> i32>(b"kavo")(v)
// }
pub(crate) fn generate(item_fn: ItemFn) -> ItemFn {
    let result = item_fn;
    return result;
}