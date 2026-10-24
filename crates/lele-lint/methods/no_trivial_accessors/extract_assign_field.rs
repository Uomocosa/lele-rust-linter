use crate::rules;
use std::collections::HashSet;

pub fn extract_assign_field(
    assign: &syn::ExprAssign,
    sig: &syn::Signature,
    pub_fields: &HashSet<String>,
) -> Option<String> {
    let field_name = match &*assign.left {
        syn::Expr::Field(field) => {
            if let syn::Member::Named(named) = &field.member {
                let n = named.to_string();
                if rules::no_trivial_accessors::NoTrivialAccessors::is_self_ref(&field.base)
                    && pub_fields.contains(&n)
                {
                    n
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
        _ => {
            return None;
        }
    };

    let param_ident = sig.inputs.iter().nth(1).and_then(|arg| match arg {
        syn::FnArg::Typed(pat) => {
            if let syn::Pat::Ident(id) = &*pat.pat {
                Some(id.ident.to_string())
            } else {
                None
            }
        }
        syn::FnArg::Receiver(_) => None,
    })?;

    let right = &*assign.right;
    let right_str = quote::quote!(#right).to_string().replace(' ', "");
    let param_str = param_ident.replace(' ', "");
    if right_str == param_str
        || right_str == format!("{param_str}.clone()")
        || right_str == format!("{param_str}.to_owned()")
    {
        return Some(field_name);
    }
    None
}

// no test_usage necessary
