use crate::rules;

pub fn pattern_bindings(pattern: &syn::Pat, bindings: &mut Vec<String>) {
    match pattern {
        syn::Pat::Ident(ident) => bindings.push(ident.ident.to_string()),
        syn::Pat::Tuple(tuple) => {
            for element in &tuple.elems {
                rules::test_usage_shape::TestUsageCalls::pattern_bindings(element, bindings);
            }
        }
        syn::Pat::TupleStruct(tuple) => {
            for element in &tuple.elems {
                rules::test_usage_shape::TestUsageCalls::pattern_bindings(element, bindings);
            }
        }
        syn::Pat::Struct(item) => {
            for field in &item.fields {
                rules::test_usage_shape::TestUsageCalls::pattern_bindings(&field.pat, bindings);
            }
        }
        syn::Pat::Slice(slice) => {
            for element in &slice.elems {
                rules::test_usage_shape::TestUsageCalls::pattern_bindings(element, bindings);
            }
        }
        syn::Pat::Reference(reference) => {
            rules::test_usage_shape::TestUsageCalls::pattern_bindings(&reference.pat, bindings);
        }
        syn::Pat::Paren(paren) => {
            rules::test_usage_shape::TestUsageCalls::pattern_bindings(&paren.pat, bindings);
        }
        syn::Pat::Type(typed) => {
            rules::test_usage_shape::TestUsageCalls::pattern_bindings(&typed.pat, bindings);
        }
        syn::Pat::Or(or) => {
            for case in &or.cases {
                rules::test_usage_shape::TestUsageCalls::pattern_bindings(case, bindings);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::pattern_bindings;
    use crate::rules;

    #[test]
    fn test_usage() {
        let function: syn::ItemFn = syn::parse_str("fn f() { let (a, b) = pair; }").unwrap();
        let Some(syn::Stmt::Local(local)) = function.block.stmts.first() else {
            panic!("expected a local");
        };
        let mut bindings = Vec::new();
        pattern_bindings(&local.pat, &mut bindings);
        assert_eq!(bindings, vec!["a".to_string(), "b".to_string()]);
        let mut shell_bindings = Vec::new();
        rules::test_usage_shape::TestUsageCalls::pattern_bindings(&local.pat, &mut shell_bindings);
        assert_eq!(shell_bindings, vec!["a".to_string(), "b".to_string()]);
    }
}
