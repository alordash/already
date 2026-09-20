use super::PatNamedType;
use syn::*;

pub(crate) enum NamedFnArg {
    Receiver(Receiver),
    NamedTyped(PatNamedType),
}
