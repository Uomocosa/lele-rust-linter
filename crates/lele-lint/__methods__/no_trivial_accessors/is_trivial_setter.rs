use crate::rules;
use std::collections::HashSet;

pub fn is_trivial_setter(method: &syn::ImplItemFn, pub_fields: &HashSet<String>) -> Option<String> {
    let sig = &method.sig;

    let receiver = sig.receiver()?;
    if receiver.reference.is_none() || receiver.mutability.is_none() {
        return None;
    }

    if sig.inputs.len() != 2 {
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

    if let syn::Expr::Assign(assign) = expr {
        return rules::no_trivial_accessors::NoTrivialAccessors::extract_assign_field(
            assign, sig, pub_fields,
        );
    }

    None
}

// no test_usage necessary
