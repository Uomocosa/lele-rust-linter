use crate::scan;
use syn::visit::Visit;

#[must_use]
pub fn system_args(node: &scan::AppMethodCall) -> Option<Vec<String>> {
    if node.method != "add_systems" {
        return None;
    }
    let mut found = Vec::new();
    {
        let mut collector = scan::SystemCollector(&mut found);
        for arg in node.args.iter().skip(1) {
            collector.visit_expr(arg);
        }
    }
    Some(found)
}

#[cfg(test)]
mod tests {
    use super::system_args;
    use crate::scan;
    use crate::testing;

    #[test]
    fn test_usage() {
        assert_eq!(
            system_args(&scan::AppMethodCall(&testing::method_call(
                "app.add_systems(Update, (tick, sync))"
            ))),
            Some(vec!["tick".to_string(), "sync".to_string()])
        );
        assert_eq!(
            system_args(&scan::AppMethodCall(&testing::method_call(
                "app.add_systems(Startup, setup)"
            ))),
            Some(vec!["setup".to_string()])
        );
        assert!(
            system_args(&scan::AppMethodCall(&testing::method_call(
                "app.add_plugins((a, b))"
            )))
            .is_none()
        );
    }
}
