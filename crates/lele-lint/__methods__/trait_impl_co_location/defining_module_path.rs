pub fn defining_module_path(qualifier: &[String], anchor: &[String]) -> Option<Vec<String>> {
    let mut resolved = anchor.to_vec();
    let mut segments = qualifier;
    while let Some((head, rest)) = segments.split_first() {
        match head.as_str() {
            "self" => segments = rest,
            "super" => {
                resolved.pop()?;
                segments = rest;
            }
            _ => break,
        }
    }
    match qualifier.first().map(String::as_str) {
        Some("crate") => {
            let rooted_path = qualifier.iter().skip(1).cloned().collect::<Vec<String>>();
            return Some(rooted_path);
        }
        Some("self" | "super") => {
            return Some(
                resolved
                    .into_iter()
                    .chain(segments.iter().cloned())
                    .collect(),
            );
        }
        _ => {}
    }
    Some(qualifier.to_vec())
}

#[cfg(test)]
mod tests {
    use super::defining_module_path;
    use crate::testing;

    #[test]
    fn test_usage() {
        assert_eq!(
            defining_module_path(&testing::strings(&["crate", "discovery"]), &[]),
            Some(testing::strings(&["discovery"]))
        );
        assert_eq!(
            defining_module_path(&testing::strings(&["discovery"]), &[]),
            Some(testing::strings(&["discovery"]))
        );
        assert_eq!(defining_module_path(&[], &[]), Some(Vec::new()));
    }

    #[test]
    fn test_usage_drops_the_leading_crate_keyword() {
        assert_eq!(
            defining_module_path(&testing::strings(&["crate", "direct_dial"]), &[]),
            Some(testing::strings(&["direct_dial"]))
        );
        assert_eq!(
            defining_module_path(&testing::strings(&["crate"]), &[]),
            Some(Vec::new())
        );
    }

    #[test]
    fn test_usage_anchors_super_at_the_impl_file_module() {
        assert_eq!(
            defining_module_path(
                &testing::strings(&["super", "config"]),
                &testing::strings(&["stock", "clock"])
            ),
            Some(testing::strings(&["stock", "config"]))
        );
        assert_eq!(
            defining_module_path(
                &testing::strings(&["super"]),
                &testing::strings(&["stock", "clock"])
            ),
            Some(testing::strings(&["stock"]))
        );
    }

    #[test]
    fn test_usage_self_imports_anchor_at_the_impl_file() {
        assert_eq!(
            defining_module_path(
                &testing::strings(&["self", "config"]),
                &testing::strings(&["stock", "clock"])
            ),
            Some(testing::strings(&["stock", "clock", "config"]))
        );
    }

    #[test]
    fn test_usage_nested_super_super_walks_up_two_modules() {
        assert_eq!(
            defining_module_path(
                &testing::strings(&["super", "super", "config"]),
                &testing::strings(&["stock", "inner", "clock"])
            ),
            Some(testing::strings(&["stock", "config"]))
        );
    }

    #[test]
    fn test_usage_super_above_the_crate_root_stays_unresolvable() {
        assert_eq!(
            defining_module_path(
                &testing::strings(&["super", "super", "config"]),
                &testing::strings(&["stock"])
            ),
            None
        );
        assert_eq!(
            defining_module_path(&testing::strings(&["super", "config"]), &[]),
            None
        );
    }
}

// no test_usage necessary
