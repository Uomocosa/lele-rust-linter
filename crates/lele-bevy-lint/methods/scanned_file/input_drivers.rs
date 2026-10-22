use derive_more::{Deref, DerefMut};
use syn::visit::Visit;

use crate::INPUT_DRIVERS;
use crate::scan;

#[must_use]
pub fn input_drivers(file: &scan::ScannedFile) -> Vec<String> {
    let mut found = Vec::new();
    {
        let mut scanner = InputScanner(&mut found);
        for item in &file.items {
            if scan::ScannedItem(item).is_test_module() {
                continue;
            }
            scanner.visit_item(item);
        }
    }
    found.sort();
    found.dedup();
    found
}

// needed helper: matches input driver tokens in paths and method calls
#[derive(Deref, DerefMut)]
struct InputScanner<'a>(&'a mut Vec<String>);

impl<'ast> Visit<'ast> for InputScanner<'_> {
    fn visit_path(&mut self, node: &'ast syn::Path) {
        for segment in &node.segments {
            let name = segment.ident.to_string();
            if INPUT_DRIVERS.contains(&name.as_str()) {
                self.push(name);
            }
        }
        syn::visit::visit_path(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        let name = node.method.to_string();
        if INPUT_DRIVERS.contains(&name.as_str()) {
            self.push(name);
        }
        syn::visit::visit_expr_method_call(self, node);
    }
}

#[cfg(test)]
mod tests {
    use super::input_drivers;
    use crate::scan;

    #[test]
    fn test_usage() {
        let file = syn::parse_str(
            "pub fn gate(q: Query<&Interaction>, keys: Res<ButtonInput<KeyCode>>) {
                 let _ = (q, keys);
             }
             pub fn tick(time: Res<Time>) { let _ = time.delta_secs(); }",
        )
        .unwrap();
        let found = input_drivers(&scan::ScannedFile(&file));
        assert!(found.contains(&"Interaction".to_string()));
        assert!(found.contains(&"ButtonInput".to_string()));
        assert!(found.contains(&"KeyCode".to_string()));
        assert!(!found.contains(&"Time".to_string()));
    }
}
