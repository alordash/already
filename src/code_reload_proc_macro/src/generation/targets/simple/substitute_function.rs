use code_reload_syn_core::*;
use not_enough_syntax::*;
use quote::format_ident;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::*;

mod named_fn_arg;
mod pat_named_type;

use named_fn_arg::*;
use pat_named_type::*;

pub(crate) fn generate(item_fn: &ItemFn, new_fn_ident_string: String) -> ItemFn {
    let caller_library_file_name = caller_crate::library_file_name();
    let span = item_fn.span();
    let named_fn_args = name_fn_args(&item_fn.sig.inputs);
    let args = named_fn_args
        .iter()
        .map(|x| match x {
            NamedFnArg::Receiver(r) => Expr::Path(expr::path::new(r.self_token.span, ["self"])),
            NamedFnArg::NamedTyped(nt) => Expr::Path(ExprPath {
                attrs: Vec::new(),
                qself: None,
                path: path::from_ident(nt.pat_ident.ident.clone()),
            }),
        })
        .collect();
    let call_stmt = Stmt::Expr(
        Expr::Call(ExprCall {
            attrs: Vec::new(),
            func: Box::new(Expr::Call(expr::call::new(
                span,
                Expr::Path(ExprPath {
                    attrs: Vec::new(),
                    qself: None,
                    path: path::new_generics_global(
                        span,
                        ["code_reload", "provide_fn"],
                        [GenericArgument::Type(Type::FnPtr(TypeFnPtr {
                            attrs: Vec::new(),
                            lifetimes: None,
                            unsafety: None,
                            abi: None,
                            fn_token: Token![fn](span),
                            paren_token: token::Paren(span),
                            inputs: named_fn_args
                                .iter()
                                .map(|x| NamedArg {
                                    attrs: Vec::new(),
                                    name: None,
                                    ty: match x {
                                        NamedFnArg::Receiver(r) => Type::Path(r#type::path::new(
                                            r.self_token.span,
                                            ["Self"],
                                        )),
                                        NamedFnArg::NamedTyped(nt) => *nt.ty.clone(),
                                    },
                                })
                                .collect(),
                            variadic: None,
                            output: item_fn.sig.output.clone(),
                        }))],
                    ),
                }),
                [
                    expr::lit::string(span, &caller_library_file_name),
                    expr::lit::byte_string(span, new_fn_ident_string.as_bytes()),
                ],
            ))),
            paren_token: token::Paren(span),
            args,
        }),
        None,
    );
    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![call_stmt],
    };
    let result = ItemFn {
        attrs: item_fn.attrs.clone(),
        vis: item_fn.vis.clone(),
        modifiers: item_fn.modifiers.clone(),
        sig: Signature {
            constness: item_fn.sig.constness,
            asyncness: item_fn.sig.asyncness,
            safety: item_fn.sig.safety.clone(),
            abi: item_fn.sig.abi.clone(),
            fn_token: item_fn.sig.fn_token,
            ident: item_fn.sig.ident.clone(),
            generics: item_fn.sig.generics.clone(),
            paren_token: token::Paren(span),
            inputs: named_fn_args
                .into_iter()
                .map(|x| match x {
                    NamedFnArg::Receiver(r) => FnArg::Receiver(r),
                    NamedFnArg::NamedTyped(nt) => FnArg::Typed(PatType {
                        attrs: nt.attrs,
                        pat: Box::new(Pat::Ident(nt.pat_ident)),
                        colon_token: nt.colon_token,
                        ty: nt.ty,
                    }),
                })
                .collect(),
            variadic: item_fn.sig.variadic.clone(),
            output: item_fn.sig.output.clone(),
        },
        block: Box::new(block),
    };
    return result;
}

fn name_fn_args<P>(fn_args: &Punctuated<FnArg, P>) -> Vec<NamedFnArg> {
    let result = fn_args
        .iter()
        .enumerate()
        .map(|(i, x)| match x {
            FnArg::Receiver(r) => NamedFnArg::Receiver(r.clone()),
            FnArg::Typed(t) => NamedFnArg::NamedTyped(PatNamedType {
                attrs: t.attrs.clone(),
                pat_ident: match t.pat.as_ref() {
                    Pat::Ident(pat_ident) => pat_ident.clone(),
                    _ => PatIdent {
                        attrs: Vec::new(),
                        by_ref: None,
                        mutability: None,
                        ident: format_ident!("__arg_{i}"),
                        subpat: None,
                    },
                },
                colon_token: t.colon_token,
                ty: t.ty.clone(),
            }),
        })
        .collect();
    return result;
}
