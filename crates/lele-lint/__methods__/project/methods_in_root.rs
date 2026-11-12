use std::collections::BTreeSet;

use crate::Project;

pub fn methods_in_root(project: &Project, type_snake: &str, root: &str) -> BTreeSet<String> {
    project
        .collect_shell_roots()
        .into_iter()
        .filter_map(|((found_type, method), roots)| {
            (found_type == type_snake && roots.iter().any(|name| name == root)).then_some(method)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::methods_in_root;
    use crate::Project;

    #[test]
    fn test_usage() {
        assert!(methods_in_root(&Project::default(), "entry", "src").is_empty());
    }
}
