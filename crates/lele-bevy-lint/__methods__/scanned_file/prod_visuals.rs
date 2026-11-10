use std::collections::BTreeSet;

use syn::visit::Visit;

use crate::scan;

#[must_use]
pub fn prod_visuals(file: &scan::ScannedFile, extra: &BTreeSet<String>) -> Vec<scan::FoundVisual> {
    let module_fns = scan::ScannedFile::module_fns_of(&file.items);
    let mut found = Vec::new();
    let mut owner = String::new();
    {
        let mut scanner = scan::UiScanner {
            found: &mut found,
            owner: &mut owner,
            module_fns: &module_fns,
            extra,
        };
        for item in &file.items {
            if scan::ScannedItem(item).is_test_module() {
                continue;
            }
            scanner.visit_item(item);
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use crate::scan;
    use std::collections::BTreeSet;

    use super::prod_visuals;

    fn none() -> BTreeSet<String> {
        BTreeSet::new()
    }

    #[test]
    fn test_usage() {
        let file = syn::parse_str(
            "pub struct Spawner;
             pub fn setup(spawner: &mut Spawner) { spawner.spawn((Node, Text::new(\"x\"))); }
             #[cfg(test)]
             mod tests { fn test_usage() { spawner.spawn((Node,)); } }",
        )
        .unwrap();
        let found = prod_visuals(&scan::ScannedFile(&file), &none());
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].visual, "Node");
        assert_eq!(found[0].owner, "setup");
    }

    #[test]
    fn test_usage_camera_only_is_not_a_visual() {
        let file = syn::parse_str("pub fn setup(c: &mut S) { c.spawn((Camera2d,)); }").unwrap();
        assert_eq!(prod_visuals(&scan::ScannedFile(&file), &none()).len(), 0);
    }

    #[test]
    fn test_usage_visual_inside_children_macro_counts() {
        let file = syn::parse_str(
            "pub fn setup(s: &mut S) { s.spawn((Root, children![Text::new(\"x\")])); }",
        )
        .unwrap();
        let found = prod_visuals(&scan::ScannedFile(&file), &none());
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].visual, "Text");
    }

    #[test]
    fn test_usage_button_alone_is_a_visual() {
        let file = syn::parse_str("pub fn setup(s: &mut S) { s.spawn((Button,)); }").unwrap();
        assert_eq!(prod_visuals(&scan::ScannedFile(&file), &none()).len(), 1);
    }

    #[test]
    fn test_usage_insert_of_a_visual_counts() {
        let file =
            syn::parse_str("pub fn setup(s: &mut S) { s.entity(e).insert((Node,)); }").unwrap();
        let found = prod_visuals(&scan::ScannedFile(&file), &none());
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].visual, "Node");
    }

    #[test]
    fn test_usage_with_child_bundle_counts() {
        let file = syn::parse_str("pub fn setup(s: &mut S) { s.with_child((Node,)); }").unwrap();
        assert_eq!(prod_visuals(&scan::ScannedFile(&file), &none()).len(), 1);
    }

    #[test]
    fn test_usage_spawned_bundle_fn_counts() {
        let file = syn::parse_str(
            "fn row() -> impl Bundle { (Node::default(), Text::new(\"x\")) }
             pub fn setup(s: &mut S) { s.spawn(row()); }",
        )
        .unwrap();
        let found = prod_visuals(&scan::ScannedFile(&file), &none());
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].owner, "setup");
    }

    #[test]
    fn test_usage_require_component_counts() {
        let mut extra = BTreeSet::new();
        extra.insert(String::from("Cell"));
        let file = syn::parse_str("pub fn setup(s: &mut S) { s.spawn((Cell,)); }").unwrap();
        assert_eq!(prod_visuals(&scan::ScannedFile(&file), &extra).len(), 1);
    }
}
