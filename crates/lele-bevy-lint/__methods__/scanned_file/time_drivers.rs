use syn::visit::Visit;

use crate::scan;

#[must_use]
pub fn time_drivers(file: &scan::ScannedFile) -> Vec<String> {
    let mut found = Vec::new();
    {
        let mut scanner = scan::DriverScanner(&mut found);
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
    use super::time_drivers;
    use crate::scan;

    #[test]
    fn test_usage() {
        let file = syn::parse_str(
            "pub fn tick(time: Res<Time>, q: Query<&Interaction>) {
                 let d = time.delta_secs();
                 if q.is_changed() { let _ = d; }
             }",
        )
        .unwrap();
        let found = time_drivers(&scan::ScannedFile(&file));
        assert!(found.contains(&"Time".to_string()));
        assert!(found.contains(&"delta_secs".to_string()));
        assert!(!found.contains(&"Interaction".to_string()));
        assert!(!found.contains(&"is_changed".to_string()));
    }

    #[test]
    fn test_usage_change_detection_is_not_a_driver() {
        let file = syn::parse_str(
            "pub fn sync(snapshot: Res<Snapshot>, mut ready: Local<bool>, q: Query<Entity, Changed<Name>>) {
                 if snapshot.is_changed() { *ready = true; }
             }",
        )
        .unwrap();
        assert_eq!(time_drivers(&scan::ScannedFile(&file)).len(), 0);
    }

    #[test]
    fn test_usage_plain_spawn_has_no_driver() {
        let file = syn::parse_str("pub fn setup(s: &mut S) { s.spawn((Node,)); }").unwrap();
        assert_eq!(time_drivers(&scan::ScannedFile(&file)).len(), 0);
    }
}
