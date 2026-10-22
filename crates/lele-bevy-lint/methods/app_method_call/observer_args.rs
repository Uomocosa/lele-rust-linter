use crate::scan;
use derive_more::{Deref, DerefMut};
use syn::visit::Visit;

const OBSERVER_METHODS: [&str; 2] = ["add_observer", "observe"];

#[must_use]
pub fn observer_args(node: &scan::AppMethodCall) -> Option<Vec<String>> {
    if !OBSERVER_METHODS.contains(&node.method.to_string().as_str()) {
        return None;
    }
    let mut found = Vec::new();
    {
        let mut collector = ObserverCollector(&mut found);
        for arg in &node.args {
            collector.visit_expr(arg);
        }
    }
    Some(found)
}

// needed helper: the last path segment of every path in an observer argument
#[derive(Deref, DerefMut)]
struct ObserverCollector<'a>(&'a mut Vec<String>);

impl<'ast> Visit<'ast> for ObserverCollector<'_> {
    fn visit_path(&mut self, node: &'ast syn::Path) {
        if let Some(last) = node.segments.last() {
            self.push(last.ident.to_string());
        }
        syn::visit::visit_path(self, node);
    }
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
