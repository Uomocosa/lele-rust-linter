use syn::visit::Visit;

use crate::scan;

#[must_use]
pub fn input_drivers(file: &scan::ScannedFile) -> Vec<String> {
    let mut found = Vec::new();
    {
        let mut scanner = scan::InputScanner(&mut found);
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
