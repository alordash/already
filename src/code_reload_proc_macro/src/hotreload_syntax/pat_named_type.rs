use syn::*;

pub(crate) struct PatNamedType {
    pub attrs: Vec<Attribute>,
    pub pat_ident: PatIdent,
    pub colon_token: Token![:],
    pub ty: Box<Type>,
}
