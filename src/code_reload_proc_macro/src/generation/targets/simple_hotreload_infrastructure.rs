use crate::caller_crate_name;
use not_enough_syntax::*;
use syn::spanned::Spanned;
use syn::*;

// fn usage(v: i32) -> i32 {
//     let library = LibraryWrapper::new("amogus.dll");
//     library.get::<fn(i32) -> i32>(b"kavo")(v)
// }
pub(crate) fn generate(item_fn: ItemFn) -> ItemFn {
    let caller_crate_name = caller_crate_name::get();
    let span = item_fn.span();
    let library_ident = Ident::new("library", span);
    let library_stmt = Stmt::Local(Local {
        attrs: Vec::new(),
        let_token: Token![let](span),
        modifiers: LocalModifiers::default(),
        pat: Pat::Ident(PatIdent {
            attrs: Vec::new(),
            by_ref: None,
            mutability: None,
            ident: library_ident.clone(),
            subpat: None,
        }),
        init: Some(LocalInit {
            eq_token: Token![=](span),
            expr: Box::new(Expr::Call(expr::call::new(
                span,
                Expr::Path(expr::path::new(
                    span,
                    ["code_reload", "LibraryWrapper", "new"],
                )),
                [expr::lit::string(span, &caller_crate_name)],
            ))),
            diverge: None,
        }),
        semi_token: Token![;](span),
    });
    let call_stmt = Stmt::Expr(
        Expr::MethodCall(ExprMethodCall {
            attrs: Vec::new(),
            receiver: Box::new(Expr::Path(ExprPath {
                attrs: Vec::new(),
                qself: None,
                path: path::from_ident(library_ident),
            })),
            dot_token: Token![.](span),
            method: Ident::new("get", span),
            turbofish: Some(),
            paren_token: token::Paren(span),
            args,
        }),
        None,
    );
    let result = item_fn;
    return result;
}
