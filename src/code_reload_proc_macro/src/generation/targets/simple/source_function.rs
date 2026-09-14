use not_enough_syntax::*;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn generate(mut item_fn: ItemFn, new_fn_ident: Ident) -> ItemFn {
    let span = item_fn.span();
    item_fn.sig.ident = new_fn_ident;
    let unsafe_export_name_attribute = Attribute {
        pound_token: Token![#](span),
        style: AttrStyle::Outer,
        bracket_token: token::Bracket(span),
        meta: Meta::List(MetaList {
            path: path::new(span, ["unsafe"]),
            delimiter: MacroDelimiter::Paren(token::Paren(span)),
            tokens: MetaNameValue {
                path: path::new(span, ["export_name"]),
                eq_token: Token![=](span),
                value: expr::lit::string(span, &item_fn.sig.ident.to_string()),
            }
            .to_token_stream(),
        }),
    };
    item_fn.attrs.insert(0, unsafe_export_name_attribute);
    item_fn
}
