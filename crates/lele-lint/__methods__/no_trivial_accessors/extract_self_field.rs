use crate::rules;
use std::collections::HashSet;

pub fn extract_self_field(expr: &syn::Expr, pub_fields: &HashSet<String>) -> Option<String> {
    match expr {
        syn::Expr::Field(field) => {
            if let syn::Member::Named(named) = &field.member {
                let field_name = named.to_string();
                if rules::no_trivial_accessors::NoTrivialAccessors::is_self_ref(&field.base)
                    && pub_fields.contains(&field_name)
                {
                    return Some(field_name);
                }
            }
        }
        syn::Expr::Reference(ref_expr) => {
            return rules::no_trivial_accessors::NoTrivialAccessors::extract_self_field(
                &ref_expr.expr,
                pub_fields,
            );
        }
        syn::Expr::MethodCall(call)
            if rules::no_trivial_accessors::NoTrivialAccessors::is_plain_conversion(call) =>
        {
            return rules::no_trivial_accessors::NoTrivialAccessors::extract_self_field(
                &call.receiver,
                pub_fields,
            );
        }
        syn::Expr::Call(call)
            if rules::no_trivial_accessors::NoTrivialAccessors::is_ok_or_some(call) =>
        {
            if let [inner] = call.args.iter().collect::<Vec<_>>().as_slice() {
                return rules::no_trivial_accessors::NoTrivialAccessors::extract_self_field(
                    inner, pub_fields,
                );
            }
        }
        syn::Expr::Paren(paren) => {
            return rules::no_trivial_accessors::NoTrivialAccessors::extract_self_field(
                &paren.expr,
                pub_fields,
            );
        }
        _ => {}
    }
    None
}

// no test_usage necessary
