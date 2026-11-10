pub fn defining_module_path(qualifier: &[String]) -> Option<Vec<String>> {
    let head = qualifier.first().map(String::as_str);
    if matches!(head, Some("self" | "super")) {
        return None;
    }
    if matches!(head, Some("crate")) {
        let rooted_path = qualifier.iter().skip(1).cloned().collect::<Vec<String>>();
        return Some(rooted_path);
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
            defining_module_path(&path(&["crate", "discovery"])),
            Some(path(&["discovery"]))
        );
        assert_eq!(
            defining_module_path(&path(&["discovery"])),
            Some(path(&["discovery"]))
        );
        assert_eq!(defining_module_path(&[]), Some(Vec::new()));
    }

    #[test]
    fn test_usage_drops_the_leading_crate_keyword() {
        assert_eq!(
            defining_module_path(&path(&["crate", "direct_dial"])),
            Some(path(&["direct_dial"]))
        );
        assert_eq!(defining_module_path(&path(&["crate"])), Some(Vec::new()));
    }

    #[test]
    fn test_usage_leaves_self_and_super_paths_unresolvable() {
        assert_eq!(defining_module_path(&path(&["self", "discovery"])), None);
        assert_eq!(defining_module_path(&path(&["super", "discovery"])), None);
    }
}

// no test_usage necessary
