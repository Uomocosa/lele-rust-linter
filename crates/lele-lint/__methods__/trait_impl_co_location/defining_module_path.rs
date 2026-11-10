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

    fn path(names: &[&str]) -> Vec<String> {
        names.iter().map(|name| name.to_string()).collect()
    }

    #[test]
    fn test_usage() {
        assert_eq!(
            defining_module_path(&path(&["crate", "discovery"]), &[]),
            Some(path(&["discovery"]))
        );
        assert_eq!(
            defining_module_path(&path(&["discovery"]), &[]),
            Some(path(&["discovery"]))
        );
        assert_eq!(defining_module_path(&[], &[]), Some(Vec::new()));
    }

    #[test]
    fn test_usage_drops_the_leading_crate_keyword() {
        assert_eq!(
            defining_module_path(&path(&["crate", "direct_dial"]), &[]),
            Some(path(&["direct_dial"]))
        );
        assert_eq!(
            defining_module_path(&path(&["crate"]), &[]),
            Some(Vec::new())
        );
    }

    #[test]
    fn test_usage_anchors_super_at_the_impl_file_module() {
        assert_eq!(
            defining_module_path(&path(&["super", "config"]), &path(&["stock", "clock"])),
            Some(path(&["stock", "config"]))
        );
        assert_eq!(
            defining_module_path(&path(&["super"]), &path(&["stock", "clock"])),
            Some(path(&["stock"]))
        );
    }

    #[test]
    fn test_usage_self_imports_anchor_at_the_impl_file() {
        assert_eq!(
            defining_module_path(&path(&["self", "config"]), &path(&["stock", "clock"])),
            Some(path(&["stock", "clock", "config"]))
        );
    }

    #[test]
    fn test_usage_nested_super_super_walks_up_two_modules() {
        assert_eq!(
            defining_module_path(
                &path(&["super", "super", "config"]),
                &path(&["stock", "inner", "clock"])
            ),
            Some(path(&["stock", "config"]))
        );
    }

    #[test]
    fn test_usage_super_above_the_crate_root_stays_unresolvable() {
        assert_eq!(
            defining_module_path(&path(&["super", "super", "config"]), &path(&["stock"])),
            None
        );
        assert_eq!(defining_module_path(&path(&["super", "config"]), &[]), None);
    }
}

// no test_usage necessary
