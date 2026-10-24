use crate::scan;
use syn::visit::Visit;

const OBSERVER_METHODS: [&str; 2] = ["add_observer", "observe"];

#[must_use]
pub fn observer_args(node: &scan::AppMethodCall) -> Option<Vec<String>> {
    if !OBSERVER_METHODS.contains(&node.method.to_string().as_str()) {
        return None;
    }
    let mut found = Vec::new();
    {
        let mut collector = scan::ObserverCollector(&mut found);
        for arg in &node.args {
            collector.visit_expr(arg);
        }
    }
    Some(found)
}

#[cfg(test)]
mod tests {
    use super::observer_args;
    use crate::scan;

    fn method_call(source: &str) -> syn::ExprMethodCall {
        let expr: syn::Expr = syn::parse_str(source).unwrap();
        let syn::Expr::MethodCall(call) = expr else {
            panic!("expected a method call");
        };
        call
    }

    #[test]
    fn test_usage() {
        assert_eq!(
            observer_args(&scan::AppMethodCall(&method_call(
                "app.add_observer(on_ready)"
            ))),
            Some(vec!["on_ready".to_string()])
        );
        assert_eq!(
            observer_args(&scan::AppMethodCall(&method_call("world.observe(on_hit)"))),
            Some(vec!["on_hit".to_string()])
        );
        assert!(
            observer_args(&scan::AppMethodCall(&method_call(
                "app.add_systems(Update, tick)"
            )))
            .is_none()
        );
    }
}
