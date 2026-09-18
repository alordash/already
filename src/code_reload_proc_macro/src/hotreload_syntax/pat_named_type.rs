use syn::*;

pub(crate) struct PatNamedType {
    pub pat_ident: PatIdent,
    pub ty: Box<Type>,
}
