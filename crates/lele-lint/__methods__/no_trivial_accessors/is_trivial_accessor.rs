use crate::rules;
use std::collections::HashSet;

pub fn is_trivial_accessor(
    method: &syn::ImplItemFn,
    pub_fields: &HashSet<String>,
) -> Option<String> {
    let sig = &method.sig;

    let receiver = sig.receiver()?;
    if receiver.reference.is_none() || receiver.mutability.is_some() {
        return None;
    }

    if sig.inputs.len() != 1 {
        return None;
    }

    let stmts = rules::no_trivial_accessors::NoTrivialAccessors::filtered_stmts(&method.block);
    let [stmt] = stmts.as_slice() else {
        return None;
    };

    let expr = match stmt {
        syn::Stmt::Expr(expr, _) => expr,
        _ => return None,
    };

    rules::no_trivial_accessors::NoTrivialAccessors::extract_self_field(expr, pub_fields)
}

// no test_usage necessary
